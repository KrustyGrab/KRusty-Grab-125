use eframe::{App, CreationContext, egui_glow::painter};
use egui::{CentralPanel, TopBottomPanel, Ui, Context, Label, Hyperlink, TextStyle, menu, Layout, Button, FontId, RichText, Visuals, Window, Grid, ColorImage, Pos2, Stroke, Color32, Frame, Rect, emath, Sense, Id, TextBuffer, Align, Widget, Vec2, TextureId, LayerId, Order, pos2, DragValue, Rgba, color_picker::{Alpha, color_edit_button_rgba}, Align2};
use egui_extras::RetainedImage;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};
use keyboard_types::{Code, Modifiers};
use serde::{Serialize, Deserialize};
use crate::icons::{icon_img, ICON_SIZE};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
enum Format {
    Jpeg,
    Png,
    Gif,
} 

// #[derive(Serialize, Deserialize)]
// struct HotKeys {
//     manager: GlobalHotKeyManager,
//     screen: HotKey,
// }

#[derive(Serialize, Deserialize)]
struct KrustyGrabConfig {
    dark_mode: bool,  
    save_folder: String, 
    save_format: Format, 
    // hotkeys: HotKeys,
}

impl Default for KrustyGrabConfig {
    fn default() -> Self {
        Self { dark_mode: true, save_folder: String::new(), save_format: Format::Png }
    }
}

impl KrustyGrabConfig {
    fn _new() -> Self {
        Self { dark_mode: true, save_folder: String::new(), save_format: Format::Png }
    }
}

pub struct KrustyGrab {
    config: KrustyGrabConfig,
    config_window: bool,
    pub screen: Option<ColorImage>,
    // paint: Painting,
}

impl KrustyGrab {
    pub fn new(ctx: &CreationContext) -> Self {
        // Get current context style
        let mut style = (*ctx.egui_ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (TextStyle::Heading, FontId::new(30.0, egui::FontFamily::Proportional)),
            (TextStyle::Name("Heading2".into()), FontId::new(25.0, egui::FontFamily::Proportional)),
            (TextStyle::Name("Context".into()), FontId::new(23.0, egui::FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(15.0, egui::FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, egui::FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(20.0, egui::FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(10.0, egui::FontFamily::Proportional)),
        ].into();

        // Mutate global style with above changes
        ctx.egui_ctx.set_style(style);

        let config: KrustyGrabConfig = confy::load("krustygrab", None).unwrap_or_default();

        Self { config, config_window: false, screen: None }
    }

    fn render_top_panel(&mut self, ctx: &Context) {
        //define a TopBottomPanel widget
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(3.);
            menu::bar(ui, |ui| {
                ui.menu_image_button(icon_img("gear", ctx), ICON_SIZE, |ui| {
                    if ui.button(RichText::new("📁 Open").text_style(TextStyle::Body)).clicked() {
                        
                    }

                    ui.menu_button(RichText::new("🌙 Theme").text_style(TextStyle::Body), |ui| {
                        if ui.button(RichText::new("Light Theme").text_style(TextStyle::Body)).clicked() {
                            self.config.dark_mode = false;
                            if let Err(e) = confy::store("krustygrab", None, KrustyGrabConfig {
                                dark_mode: self.config.dark_mode,
                                save_folder: self.config.save_folder.to_string(),
                                save_format: self.config.save_format.clone(),
                            }) {
                                tracing::error!("Failed saving app state: {}", e);
                            }
                            else {
                                tracing::error!("App state saved");
                            }
                        }
                        if ui.button(RichText::new("Dark Theme").text_style(TextStyle::Body)).clicked() {
                            self.config.dark_mode = true;
                            if let Err(e) = confy::store("krustygrab", None, KrustyGrabConfig {
                                dark_mode: self.config.dark_mode,
                                save_folder: self.config.save_folder.to_string(),
                                save_format: self.config.save_format.clone(),
                            }) {
                                tracing::error!("Failed saving app state: {}", e);
                            }
                            else {
                                tracing::error!("App state saved");
                            }
                        }
                    });

                    if ui.button(RichText::new("💭 Preferences").text_style(TextStyle::Body)).clicked() {
                        self.config_window = true;
                        ui.close_menu();
                    }
                });

                //painting commands
                if self.screen.is_some() {
                    tracing::error!("Painting buttons");
                    
                    self.render_drawing_toolbar(ctx, ui);
                }

                //controls
                ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                    if Button::image_and_text(icon_img("camera", ctx), ICON_SIZE, "").ui(ui).clicked() {
                        tracing::error!("Screen button clicked");
                        self.screen = Some(ColorImage::example());
                    }
                });
            });
            ui.add_space(3.);
        });
    }   

    fn render_central_panel(&mut self, ctx: &Context){
        CentralPanel::default().show(ctx, |ui| {

            if let Some(screen) = &self.screen {
                let texture: egui::TextureHandle = ui.ctx().load_texture(
                    "my-screen",
                    screen.clone(),
                    Default::default()
                );
                
                // Show the image:    
                // ui.image(&texture, ui.available_size());

                self.render_drawing(ctx, ui);
            }

            self.render_bottom_panel(ctx);
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
    
    fn render_config(&mut self, ctx: &Context) {
        Window::new(RichText::new("Configuration").text_style(TextStyle::Body)).show(ctx, |ui| {
            
            Grid::new("configGrid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Save folder:");
                    // let prev_save = self.config.save_folder.clone();
                    // let mut new_save = String::new();
                    // ui.add(egui::TextEdit::singleline(&mut new_save).hint_text(prev_save));
                    ui.text_edit_singleline(&mut self.config.save_folder);
                    ui.end_row();
        
                    // if text_input.lost_focus() && ui.input(|i| {i.key_pressed(egui::Key::Enter)}) {
                    //     if let Err(e) = confy::store("krustygrab", None, KrustyGrabConfig {
                    //         dark_mode: self.config.dark_mode,
                    //         save_folder: self.config.save_folder.to_string(),
                    //         save_format: self.config.save_format.clone(),
                    //     }) {
                    //         tracing::error!("Failed saving app state: {}", e);
                    //     }
                    //     else {
                    //         tracing::error!("App state saved");
                    //     }
                    // }
                    tracing::error!("{}", &self.config.save_folder); //log 
        
                    ui.label("Save format:");
                    egui::ComboBox::from_label("Format")
                        .selected_text(RichText::new(format!("{:?}", self.config.save_format)).text_style(TextStyle::Body))
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            ui.selectable_value(&mut self.config.save_format, Format::Png, RichText::new("Png").text_style(TextStyle::Body));
                            ui.selectable_value(&mut self.config.save_format, Format::Jpeg, RichText::new("Jpeg").text_style(TextStyle::Body));
                            ui.selectable_value(&mut self.config.save_format, Format::Gif, RichText::new("Gif").text_style(TextStyle::Body));
                    });
                    ui.end_row();

                    ui.end_row();
                    ui.separator();
                    // ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                            if ui.button(RichText::new("Close").text_style(TextStyle::Body)).clicked() {
                                self.config_window = false;
                            }
                            else if ui.button(RichText::new("Apply").text_style(TextStyle::Body)).clicked() {
                                if let Err(e) = confy::store("krustygrab", None, KrustyGrabConfig {
                                    dark_mode: self.config.dark_mode,
                                    save_folder: self.config.save_folder.to_string(),
                                    save_format: self.config.save_format.clone(),
                                }) {
                                    tracing::error!("Failed saving app state: {}", e);
                                }
                                else {
                                    tracing::error!("App state saved");
                                }
                                self.config_window = false;
                            }
                        });
                    // });
                    ui.end_row();

                    tracing::error!("{}", &self.config.save_folder); //log 
                    tracing::error!("{:?}", &self.config.save_format); //log 
            });            
            
            
        });
    }   
}

impl App for KrustyGrab {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        }
        else {
            ctx.set_visuals(Visuals::light());
        }

        if self.config_window {
            self.render_config(ctx);
        }
        
        self.render_top_panel(ctx);
        self.render_central_panel(ctx);
    }  
}
