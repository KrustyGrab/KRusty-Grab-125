use std::borrow::Cow;

use egui::{Context, TopBottomPanel, menu, RichText, TextStyle, Layout, Button, ColorImage, CentralPanel, Widget, Id, Vec2, Pos2, pos2, CursorIcon, Rect};
use image::open;
use crate::{krustygrab::{KrustyGrab, KrustyGrabConfig, self}, painting::{icons::{icon_img, ICON_SIZE}, drawing::RedoList}, painting::drawing::DrawingType, screenshot::screen_capture::screens_number};
pub use crate::screenshot::screen_capture::take_screen;
use native_dialog::FileDialog;
use arboard::{Clipboard, ImageData};

impl KrustyGrab {
    pub fn main_window(&mut self, ctx: &Context, frame: &mut eframe::Frame){
        if ctx.memory(|mem| mem.data.get_temp::<Vec2>(Id::from("Window_size")).is_some()) {
            ctx.memory_mut(|mem| {
                let window_maximized = match mem
                    .data
                    .get_temp::<bool>(Id::from("Window_maximized"))
                {
                    Some(max) => max,
                    None => false,
                };

                if !window_maximized {
                    let window_sz = match mem
                        .data
                        .get_temp::<Vec2>(Id::from("Window_size"))
                    {
                        Some(size) => size,
                        None => Vec2::new(800., 450.),
                    };
                    let window_pos =
                        match mem.data.get_temp::<Pos2>(Id::from("Window_pos"))
                        {
                            Some(pos) => pos,
                            None => pos2(26., 26.),
                        };

                    frame.set_window_pos(window_pos);
                    frame.set_window_size(window_sz);

                    mem.data.remove::<Vec2>(Id::from("Window_size"));
                    mem.data.remove::<Pos2>(Id::from("Window_pos"));
                }
            });
        }

        self.render_top_panel(ctx, frame);
        self.render_bottom_panel(ctx);
        self.render_central_panel(ctx);
    }
    
    fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        //define a TopBottomPanel widget
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(3.);
            menu::bar(ui, |ui| {
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
                                let open_image = open(path).expect("Unable to open the file");
                                let open_image_vec = open_image.clone().as_mut_rgba8().unwrap().clone().into_vec();
        
                                let new_image = ColorImage::from_rgba_unmultiplied(
                                    [open_image.width() as usize, open_image.height() as usize],
                                    &open_image_vec
                                );
                                self.set_temp_image(Some(new_image));
                                ctx.memory_mut(|mem| {
                                    mem.data.remove::<RedoList>(Id::from("Redo_list"));
                                    mem.data.remove::<Vec<DrawingType>>(Id::from("Drawing"));
                                });
                            }

                    }

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
                                    KrustyGrabConfig {
                                        dark_mode: self.config.dark_mode,
                                        save_folder: self.config.save_folder.clone(),
                                        save_format: self.config.save_format.clone(),
                                    },
                                ) {
                                    tracing::error!("Failed saving app state: {}", e);
                                } else {
                                    tracing::error!("App state saved");
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
                                    KrustyGrabConfig {
                                        dark_mode: self.config.dark_mode,
                                        save_folder: self.config.save_folder.clone(),
                                        save_format: self.config.save_format.clone(),
                                    },
                                ) {
                                    tracing::error!("Failed saving app state: {}", e);
                                } else {
                                    tracing::error!("App state saved");
                                }
                            }
                        },
                    );

                    if ui
                        .button(RichText::new("💭 Preferences").text_style(TextStyle::Body))
                        .clicked()
                    {
                        self.config_window = true;
                        ui.close_menu();
                    }
                });

                //painting commands
                if self.screen.is_some() {
                    // tracing::error!("Painting buttons");

                    self.render_drawing_toolbar(ctx, ui, frame);
                }

                //controls
                ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                    if Button::image_and_text(icon_img("camera", ctx), ICON_SIZE, "")
                        .ui(ui)
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .on_hover_text_at_pointer("Take screenshot")
                        .clicked()
                    {
                        tracing::error!("Screen button clicked");
                        self.set_screenshot(ctx);
                    }

                    //Select area screenshot
                    if Button::image_and_text(icon_img("select", ctx), ICON_SIZE, "")
                        .ui(ui)
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .on_hover_text_at_pointer("Select area")
                        .clicked()
                    {
                        tracing::error!("DragScreen button clicked");
                        ctx.memory_mut(|mem| {
                            let window_maximized = frame.info().window_info.maximized;
                            println!("Window maximized? {window_maximized:?}");
        
                            if !window_maximized {
                                let window_size = frame.info().window_info.size;
                                let window_pos = frame.info().window_info.position.expect("Window position should be Some");
                                
                                mem.data.insert_temp(Id::from("Window_size"), window_size);
                                mem.data.insert_temp(Id::from("Window_pos"), window_pos);
                                println!("PRE Window size and pos: {window_size:?} - {window_pos:?}");
                            }
                            
                            mem.data.insert_temp(Id::from("Window_maximized"), window_maximized);

                            mem.data.remove::<Option<Rect>>(Id::from("Prev_area")); //??
                            self.set_select_area(None);
                        });
        
                        self.set_window_status(krustygrab::WindowStatus::Crop);
        
                        self.set_screenshot(ctx);
                    }

                    //Timer selection
                    ui.menu_image_button(icon_img("timer", ctx), ICON_SIZE, |ui| {
                        if ui.button(RichText::new("5 seconds").text_style(TextStyle::Body)).clicked() {
                            ui.close_menu();
                        }
                        if ui.button(RichText::new("10 seconds").text_style(TextStyle::Body)).clicked() {
                            ui.close_menu();
                        }
                        if ui.button(RichText::new("15 seconds").text_style(TextStyle::Body)).clicked() {
                            ui.close_menu();
                        }
                        if ui.button(RichText::new("30 seconds").text_style(TextStyle::Body)).clicked() {
                            ui.close_menu();
                        }
                    });

                    //Screen selection
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if screens_number() != 1 {
                            let screen_selected: usize = 1 + match ctx.memory(|mem| mem.data.get_temp(Id::from("Selected_screen"))) {
                                Some(s) => s,
                                None => {
                                    ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Selected_screen"), 0));
                                    0
                                },
                            };
        
                            ui.menu_button(RichText::new(screen_selected.to_string()).text_style(TextStyle::Body), |ui| {
                                for i in 0..screens_number() {
                                    if ui.button(RichText::new((i+1).to_string()).text_style(TextStyle::Body)).clicked() {
                                        ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Selected_screen"), i));
                                        ui.close_menu();
                                    }
                                }
                            });
                        }
                        else {
                            ui.label(RichText::new("1").text_style(TextStyle::Body));
                        }
                        ui.label("Screen");
                    }); 
                });
            });
            ui.add_space(3.);
        });
    }

    fn render_central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if let Some(screen) = &self.screen {
                let texture: egui::TextureHandle =
                    ui.ctx()
                        .load_texture("my-screen", screen.clone(), Default::default());

                // Show the image:
                // ui.image(&texture, ui.available_size());

                self.render_drawing(ctx, ui);
            }
        });
    }

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

    ///Used to take and set the screenshot to visualize. Used when screenshot button clicked and when select crop area is pressed while no screenshot was previously taken
    pub fn set_screenshot(&mut self, ctx: &Context) {
        //TODO rimuovere la visualizzazione della finestra durante l'acquisizione
        let screen_selected: usize = match ctx.memory(|mem| mem.data.get_temp(Id::from("Selected_screen"))) {
            Some(s) => s,
            None => 0,
        };
        let im = take_screen(screen_selected).expect("Problem taking the screenshot");

        self.set_temp_image(Some(im.clone()));
        
        let mut clipboard = Clipboard::new().expect("Unable to create clipboard");
        if let Err(e) = clipboard.set_image(ImageData { width: im.width(), height: im.height(), bytes: Cow::from(im.as_raw().clone())}) {
            tracing::error!("Unable to copy in the clipboard: {e:?}");
        }

        ctx.memory_mut(|mem| mem.data.remove::<Vec<DrawingType>>(Id::from("Drawing")));
    }
}