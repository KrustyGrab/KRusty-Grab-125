use egui::{Context, TopBottomPanel, menu, RichText, TextStyle, Layout, Button, ColorImage, CentralPanel, Widget, Id, Vec2, Pos2, pos2};
use crate::{krustygrab::{KrustyGrab, KrustyGrabConfig}, painting::icons::{icon_img, ICON_SIZE}, painting::drawing::DrawingType};
pub use crate::screenshot::screen_capture::take_screen;

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

                // frame.set_maximized(true);
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
                    if ui
                        .button(RichText::new("📁 Open").text_style(TextStyle::Body))
                        .clicked()
                    {}

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
                                        save_folder: self.config.save_folder.to_string(),
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
                                        save_folder: self.config.save_folder.to_string(),
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
                        .clicked()
                    {
                        tracing::error!("Screen button clicked");
                        self.set_screenshot(ctx);
                    }

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
        //TODO impostare la scelta dello schermo
        //TODO rimuovere la visualizzazione della finestra durante l'acquisizione
        let im = take_screen(0).expect("Problem taking the screenshot");

        self.set_temp_image(Some(im));
        ctx.memory_mut(|mem| mem.data.remove::<Vec<DrawingType>>(Id::from("Drawing")));
    }
}