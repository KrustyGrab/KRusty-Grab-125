use crate::icons::{icon_img, ICON_SIZE};
use eframe::{egui_glow::painter, App, CreationContext};
use egui::{
    color_picker::{color_edit_button_rgba, Alpha},
    emath, menu, pos2, Align, Align2, Button, CentralPanel, Color32, ColorImage, Context,
    DragValue, FontId, Frame, Grid, Hyperlink, Id, Label, LayerId, Layout, Order, Pos2, Rect, Rgba,
    RichText, Sense, Stroke, TextBuffer, TextStyle, TextureId, TopBottomPanel, Ui, Vec2, Visuals,
    Widget, Window,
};
use egui_extras::RetainedImage;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};
use keyboard_types::{Code, Modifiers};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Format {
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
pub struct KrustyGrabConfig {
    pub dark_mode: bool,
    pub save_folder: String,
    pub save_format: Format,
    // hotkeys: HotKeys,
}

impl Default for KrustyGrabConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            save_folder: String::new(),
            save_format: Format::Png,
        }
    }
}

impl KrustyGrabConfig {
    fn _new() -> Self {
        Self {
            dark_mode: true,
            save_folder: String::new(),
            save_format: Format::Png,
        }
    }
}

///Used to track the current area manipulation.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GrabStatus {
    None,
    Select,
    TopLeft,
    TopMid,
    TopRight,
    MidLeft,
    MidRight,
    BotLeft,
    BotMid,
    BotRight,
    Move,
}

///Used to select the window to be shown
pub enum WindowStatus {
    Main,
    Crop,
}

pub struct KrustyGrab {
    pub config: KrustyGrabConfig,
    pub config_window: bool,
    pub screen: Option<ColorImage>,
    grab_status: GrabStatus,
    window_status: WindowStatus,
    select: Option<Rect>,
    temp_image: Option<ColorImage>,
    // paint: Painting,
}

impl Default for KrustyGrab {
    fn default() -> Self {
        Self {
            config: KrustyGrabConfig::_new(),
            config_window: false,
            screen: None,
            grab_status: GrabStatus::None,
            window_status: WindowStatus::Main,
            select: None,
            temp_image: None,
        }
    }
}

impl KrustyGrab {
    pub fn new(ctx: &CreationContext) -> Self {
        // Get current context style
        let mut style = (*ctx.egui_ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(30.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Name("Heading2".into()),
                FontId::new(25.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Name("Context".into()),
                FontId::new(23.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                FontId::new(15.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(10.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();

        // Mutate global style with above changes
        ctx.egui_ctx.set_style(style);

        let config: KrustyGrabConfig = confy::load("krustygrab", None).unwrap_or_default();

        Self {
            config,
            ..Default::default()
        }
    }

    pub fn get_grab_status(&self) -> GrabStatus {
        self.grab_status
    }
    pub fn get_selected_area(&self) -> Option<Rect> {
        self.select
    } 
    pub fn get_temp_image(&self) -> Option<ColorImage> {
        self.temp_image.clone()
    }

    pub fn set_grab_status(&mut self, new_status: GrabStatus) {
        self.grab_status = new_status;
    }
    pub fn set_window_status(&mut self, new_status: WindowStatus) {
        self.window_status = new_status;
    }
    pub fn set_select_area(&mut self, new_area: Option<Rect>) {
        self.select = new_area;
    }
    pub fn set_temp_image(&mut self, new_image: Option<ColorImage>) {
        self.screen = new_image.clone();
        self.temp_image = new_image.clone();
    }
    pub fn set_definitive_image(&mut self, new_image: Option<ColorImage>) {
        self.screen = new_image.clone();
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
                        .selected_text(
                            RichText::new(format!("{:?}", self.config.save_format))
                                .text_style(TextStyle::Body),
                        )
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            ui.selectable_value(
                                &mut self.config.save_format,
                                Format::Png,
                                RichText::new("Png").text_style(TextStyle::Body),
                            );
                            ui.selectable_value(
                                &mut self.config.save_format,
                                Format::Jpeg,
                                RichText::new("Jpeg").text_style(TextStyle::Body),
                            );
                            ui.selectable_value(
                                &mut self.config.save_format,
                                Format::Gif,
                                RichText::new("Gif").text_style(TextStyle::Body),
                            );
                        });
                    ui.end_row();

                    ui.end_row();
                    ui.separator();
                    // ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui
                            .button(RichText::new("Close").text_style(TextStyle::Body))
                            .clicked()
                        {
                            self.config_window = false;
                        } else if ui
                            .button(RichText::new("Apply").text_style(TextStyle::Body))
                            .clicked()
                        {
                            if let Err(e) = confy::store(
                                "krustygrab",
                                None,
                                KrustyGrabConfig {
                                    dark_mode: self.config.dark_mode,
                                    save_folder: self.config.save_folder.to_string(),
                                    save_format: self.config.save_format.clone(),
                                },
                            ) {
                                tracing::error!("Failed saving app state: {}", e);
                            } else {
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
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.config_window {
            self.render_config(ctx);
        }

        match self.window_status {
            WindowStatus::Main => self.main_window(ctx, frame),
            WindowStatus::Crop => self.crop_screen_window(ctx, frame),
        }
    }
}
