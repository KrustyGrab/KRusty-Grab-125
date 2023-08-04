use eframe::{App, CreationContext};
use egui::{CentralPanel, TopBottomPanel, Ui, Context, Label, Hyperlink, TextStyle, menu, Layout, Button, FontId, RichText, Visuals, Window, Grid};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]

enum Format {
    Jpeg,
    Png,
    Gif,
} 

#[derive(Serialize, Deserialize)]
struct KrustyGrabConfig {
    dark_mode: bool,  
    save_folder: String, 
    save_format: Format, 
}

impl Default for KrustyGrabConfig {
    fn default() -> Self {
        Self { dark_mode: true, save_folder: String::new(), save_format: Format::Png}
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

        Self {config, config_window: false}
    }

    fn render_top_panel(&mut self, ctx: &Context) {
        //define a TopBottomPanel widget
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(3.);
            menu::bar(ui, |ui| {
                // //logo
                // ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                //     ui.label(RichText::new("📷").text_style(TextStyle::Button));
                // });

                ui.menu_button("⚙️", |ui| {
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

                    // ui.menu_button("SubMenu", |ui| {
                    //     ui.menu_button("SubMenu", |ui| {
                    //         if ui.button("Open...").clicked() {
                    //             ui.close_menu();
                    //         }
                    //         let _ = ui.button("Item");
                    //     });
                    //     ui.menu_button("SubMenu", |ui| {
                    //         if ui.button("Open...").clicked() {
                    //             ui.close_menu();
                    //         }
                    //         let _ = ui.button("Item");
                    //     });
                    //     let _ = ui.button("Item");
                    //     if ui.button("Open...").clicked() {
                    //         ui.close_menu();
                    //     }
                    // });
                });

                //controls
                // ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                //     //let close_bnt = ui.add(Button::new("❌"));
                //     let theme_bnt = ui.button("🌙");

                //     if theme_bnt.clicked() {
                //         self.config.dark_mode = !self.config.dark_mode;
                //     }
                // });
            });
            ui.add_space(3.);
        });
        //add a menù bar
        //2 layout widgets
        //to render the logo on the left
        //control buttons on the right
        //padding before and after the panel
    }   

    fn render_config(&mut self, ctx: &Context) {
        Window::new(RichText::new("Configuration").text_style(TextStyle::Body)).show(ctx, |ui| {
            
            Grid::new("configGrid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Save folder:");
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
                            if ui.button(RichText::new("Apply").text_style(TextStyle::Body)).clicked() {
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
        CentralPanel::default().show(ctx, |ui| {
            ui.label("testo");

            render_footer(ctx);
        });
    }


   
}

fn render_footer(ctx: &Context) {
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

