use std::{borrow::Cow, thread, time::Duration};

use egui::{Context, TopBottomPanel, menu, RichText, TextStyle, Layout, Button, ColorImage, CentralPanel, Widget, Id, CursorIcon};
use image::open;
use crate::{krustygrab::{KrustyGrab, self}, painting::{icons::{icon_img, ICON_SIZE}, drawing::RedoList}, painting::drawing::DrawingType, screenshot::screen_capture::screens_number};
pub use crate::screenshot::screen_capture::take_screen;
use native_dialog::FileDialog;
use arboard::{Clipboard, ImageData};

impl KrustyGrab {
    ///It renders the maian window composed of the 3 panel. 
    pub fn main_window(&mut self, ctx: &Context, frame: &mut eframe::Frame){
        
        self.render_top_panel(ctx, frame);
        self.render_bottom_panel(ctx);
        self.render_central_panel(ctx);

        // If we are cropping we should be in full screen
        if self.is_window_status_crop(){
            frame.set_fullscreen(true);
        } else if self.is_window_status_save(){
            // frame.set_fullscreen(false);   //TO CHECK
        }

        // To hide the window while we are taking the screenshot
        if self.is_window_status_save() {
            frame.set_visible(true);
        }
        if self.screenshot_requested  {
            frame.set_visible(false);
        }
    }
    
    ///Render the top panel 
    fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        //define a TopBottomPanel widget
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(3.);
            menu::bar(ui, |ui| {
                
                // Option menu
                ui.menu_image_button(icon_img("gear", ctx), ICON_SIZE, |ui| {
                    
                    ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("SM_open"), true));

                    if ui
                        .button(RichText::new("📁 Open").text_style(TextStyle::Body))
                        .clicked()
                    {
                        if let Some(path) = FileDialog::new()
                            .add_filter("PNG", &["png"])
                            .add_filter("JPG", &["jpg"])
                            .add_filter("GIF", &["gif"])
                            .show_open_single_file()
                            .expect("Unable to visualize the file selection window") {
                                // Import the image selected
                                let open_image = open(path).expect("Unable to open the file");
                                let open_image_vec = open_image.clone().as_mut_rgba8().unwrap().clone().into_vec();
        
                                let new_image = ColorImage::from_rgba_unmultiplied(
                                    [open_image.width() as usize, open_image.height() as usize],
                                    &open_image_vec
                                );

                                self.set_temp_image(Some(new_image));
                                
                                //Remove eventual previous drawings 
                                ctx.memory_mut(|mem| {
                                    mem.data.remove::<RedoList>(Id::from("Redo_list"));
                                    mem.data.remove::<Vec<DrawingType>>(Id::from("Drawing"));
                                });
                                ui.close_menu();
                            }
                    }

                    // Select light theme or dark theme
                    ui.menu_button(
                        RichText::new("🌙 Theme").text_style(TextStyle::Body),
                        |ui| {
                            if ui
                                .button(RichText::new("Light Theme").text_style(TextStyle::Body))
                                .clicked()
                            {
                                self.config.dark_mode = false;
                                
                                if let Err(e) = confy::store(
                                    "krustygrab",
                                    None,
                                    self.config.clone(),
                                ) {
                                    tracing::error!("Failed saving app state: {}", e);
                                } else {
                                    tracing::info!("App state saved");
                                }
                            }
                            if ui
                                .button(RichText::new("Dark Theme").text_style(TextStyle::Body))
                                .clicked()
                            {
                                self.config.dark_mode = true;
                                
                                if let Err(e) = confy::store(
                                    "krustygrab",
                                    None,
                                    self.config.clone(),
                                ) {
                                    tracing::error!("Failed saving app state: {}", e);
                                } else {
                                    tracing::info!("App state saved");
                                }
                            }
                        },
                    );

                    // Preferences button. The config panel will be opened in next update. 
                    if ui
                        .button(RichText::new("💭 Preferences").text_style(TextStyle::Body))
                        .clicked()
                    {
                        self.config_window = true;
                        ui.close_menu();
                    }
                }).response
                .on_hover_cursor(CursorIcon::PointingHand)
                .on_hover_text_at_pointer("Settings");

                // Render painting commands if there is a screen
                if self.screen.is_some() {
                    self.render_drawing_toolbar(ctx, ui, frame);
                }

                // Buttons on the right to set the screenshot request
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    // Take a screenshot
                    if Button::image_and_text(icon_img("camera", ctx), ICON_SIZE, "")
                        .ui(ui)
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .on_hover_text_at_pointer("Take screenshot")
                        .clicked()
                    {
                        tracing::info!("Screen button clicked");
                        self.screenshot_requested = true;
                    }

                    //Select area screenshot
                    if Button::image_and_text(icon_img("select", ctx), ICON_SIZE, "")
                        .ui(ui)
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .on_hover_text_at_pointer("Select area")
                        .clicked()
                    {
                        tracing::info!("DragScreen button clicked");
                        self.set_window_status(krustygrab::WindowStatus::Crop);
                        self.screenshot_requested = true;
                    }

                    //Modify style (to have same font of the other button) 
                    let style = ui.style_mut();
                    style.drag_value_text_style = egui::TextStyle::Body;
                    //Timer button 
                    ui.add(
                        egui::DragValue::new(& mut self.config.screenshot_delay)
                            .speed(1)
                            .clamp_range(0..=120)
                            .prefix("Timer: "),
                    ).on_hover_text_at_pointer("Select delay");

                    //Screen selection
                    if screens_number() != 1 {
                        let screen_selected: usize = 1 + self.get_selected_screen();
    
                        ui.menu_button(RichText::new("Screen ".to_string() + screen_selected.to_string().as_str()).text_style(TextStyle::Body), |ui| {
                            for i in 0..screens_number() {
                                if ui.button(RichText::new("Screen ".to_string() + (i+1).to_string().as_str()).text_style(TextStyle::Body)).clicked() {
                                    self.set_selected_screen(i);
                                    ui.close_menu();
                                }
                            }
                        }).response
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .on_hover_text_at_pointer("Select screen");
                    }
                    else {
                        ui.label(RichText::new("1").text_style(TextStyle::Body));
                    }
                });
            });
            ui.add_space(3.);
        });
    }

    ///Render the central panel 
    fn render_central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            // Render the drawing if we have a screen saved
            if self.screen.is_some() {                
                self.render_canva(ctx, ui);
            }
        });
    }

    ///Render the bottom panel 
    fn render_bottom_panel(&self, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);

                ui.hyperlink_to(
                    RichText::new("KRusty-Grab-125 on GitHub").text_style(TextStyle::Small),
                    "https://github.com/Emanueleff/KRusty-Grab-125",
                );

                ui.add_space(10.);
            });
        });
    }

    ///Used to take and set the screenshot to visualize. Used when screenshot or select crop area buttons are pressed
    pub fn set_screenshot(&mut self, ctx: &Context) {
        //Insert a delay (150ms) in order to let the fade out animation of the application to be completed
        // + the eventual delay set from the user
        thread::sleep(Duration::from_millis(150) + Duration::from_secs(self.config.screenshot_delay as u64));

        //take the screenshot, set it in our struct, copy on the clipboard
        let screen_selected: usize = self.get_selected_screen();
        let im = take_screen(screen_selected).expect("Problem taking the screenshot");

        self.set_temp_image(Some(im.clone()));
        
        //Copy the taken screenshot to the clipboard
        let mut clipboard = Clipboard::new().expect("Unable to create clipboard");
        if let Err(e) = clipboard.set_image(ImageData { width: im.width(), height: im.height(), bytes: Cow::from(im.as_raw().clone())}) {
            tracing::error!("Unable to copy in the clipboard: {e:?}");
        }

        self.set_select_area(None);

        //Remove eventual previous drawings 
        ctx.memory_mut(|mem| {
            mem.data.remove::<RedoList>(Id::from("Redo_list"));
            mem.data.remove::<Vec<DrawingType>>(Id::from("Drawing"));
        });
    }
}