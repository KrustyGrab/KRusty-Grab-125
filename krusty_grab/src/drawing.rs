use egui::{Context, Pos2, Stroke, Rect, Vec2, Rgba, Color32, Layout, Align, Button, Id, color_picker::{color_edit_button_rgba, Alpha}, DragValue, Ui, LayerId, Order, pos2, Align2, FontId, Widget, Window};
use egui_extras::RetainedImage;
use serde::{Serialize, Deserialize};
use crate::krustygrab::{self, KrustyGrab, };
use crate::icons::{icon_img, ICON_SIZE};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum DrawingMode {
    Brush,
    Rectangle,
    Circle,
    Arrow,
    Text,
}

#[derive(Clone, Debug)]
enum DrawingType {
    Brush {points: Vec<Pos2>, s: Stroke, end: bool},
    Rectangle {r: Rect, s: Stroke},
    Circle {c: Pos2, r: f32, s: Stroke},
    Arrow {p: Pos2, v: Vec2, s: Stroke},
    Text {p: Pos2, t: String, s: Stroke}, //???
}

impl KrustyGrab {
    pub fn render_drawing_toolbar(&self, ctx: &Context, ui: &mut Ui) {
        let mut color = match ctx.memory(|mem| mem.data.get_temp::<Rgba>(Id::from("Color"))){
            Some(c) => c,
            None => Rgba::from(Color32::GREEN)
        };

        let mut thickness: f32 = match ctx.memory(|mem| mem.data.get_temp(Id::from("Thickness"))) {
            Some(t) => t,
            None => 1.0,
        };
        
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            
            if Button::image_and_text(icon_img("pencil", ctx), ICON_SIZE, "")
            .stroke(Stroke::new(1.0,
                Color32::from_rgb(128, 106, 0)))
                .ui(ui).clicked() {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("DrawingMode"), DrawingMode::Brush));
                tracing::error!("Pencil selected");
            }

            if Button::image_and_text(icon_img("circle", ctx), ICON_SIZE, "")
            .stroke(Stroke::new(1.0,
                Color32::from_rgb(128, 106, 0)))
                .ui(ui).clicked() {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("DrawingMode"), DrawingMode::Circle));
                tracing::error!("Circle selected");
            }

            if Button::image_and_text(icon_img("rect", ctx), ICON_SIZE, "")
            .stroke(Stroke::new(1.0,
                Color32::from_rgb(128, 106, 0)))
                .ui(ui).clicked() {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("DrawingMode"), DrawingMode::Rectangle));
                tracing::error!("Rect selected");
            }

            if Button::image_and_text(icon_img("arrow", ctx), ICON_SIZE, "")
            .stroke(Stroke::new(1.0,
                Color32::from_rgb(128, 106, 0)))
                .ui(ui).clicked() {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("DrawingMode"), DrawingMode::Arrow));
                tracing::error!("Arrow selected");
            }

            if Button::image_and_text(icon_img("text", ctx), ICON_SIZE, "")
            .stroke(Stroke::new(1.0,
                Color32::from_rgb(128, 106, 0)))
                .ui(ui).clicked() {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("DrawingMode"), DrawingMode::Text));
                tracing::error!("Text selected");
            }

            let color_picker = color_edit_button_rgba(ui, &mut color, Alpha::BlendOrAdditive);
                        
            if ctx.memory(|mem| mem.any_popup_open()) {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("CP_open"), true));
            }
            else {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("CP_open"), false));
            }
            if color_picker.changed() {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Color"), color));
                tracing::error!("Color changed to {:?}", color);
            }

            ui.label("Thickness");
            if DragValue::new(&mut thickness)
                .speed(0.1)
                .clamp_range(1.0..=10.0)
                .ui(ui)
                .changed() {
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Thickness"), thickness));
                tracing::error!("Thickness changed to {:?}", thickness);
            }
        });
    }


    pub fn render_drawing(&mut self, ctx: &Context, ui: &mut Ui) {
        let screen = RetainedImage::from_color_image("Screenshot", self.screen.clone().unwrap());

        let mut painter = ctx.layer_painter(LayerId::new(Order::Background, Id::from("Painter")));

        let aspect_ratio = screen.width() as f32 / screen.height() as f32;
        let mut w = ui.available_width();  
        let mut h = w / aspect_ratio;
        if h > ui.available_height() {
            h = ui.available_height();
            w = h * aspect_ratio;
        }

        let mut area = ui.available_rect_before_wrap();
        if area.width() > w {
            area.min.x += (area.width() - w) / 2.0;
            area.max.x = area.min.x + w;
        }  
        if area.height() > h {
            area.min.y += (area.height() - h) / 2.0;
            area.max.y = area.min.y + h;
        }
        area.set_width(w);
        area.set_height(h);
        
        let visualization_ratio = screen.width() as f32 / w;

        ctx.memory_mut(|mem| {
            mem.data.insert_temp(Id::from("Visualization_ratio"), visualization_ratio);
            mem.data.insert_temp(Id::from("Visualization_pos"), area.min);
        });
    
        painter.set_clip_rect(area);
        painter.image(screen.texture_id(ctx), area, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);

        let mut drawings = match ctx.memory(|mem| mem.data.get_temp::<Vec<DrawingType>>(Id::from("Drawing"))) {
            Some(v) => v,
            None => Vec::<DrawingType>::new(),
        };

        let mut color = match ctx.memory(|mem| mem.data.get_temp::<Rgba>(Id::from("Color"))){
            Some(c) => c,
            None => Rgba::from(Color32::GREEN)
        };
        tracing::error!("Color from memory: {:?}", color);

        let mut thickness = match ctx.memory(|mem| mem.data.get_temp::<f32>(Id::from("Thickness"))){
            Some(t) => t,
            None => 1.0
        };
        tracing::error!("Thickness from memory: {}", thickness);

        let mut stroke = Stroke::new(thickness, color);

        //Visualizzazione disegni salvati
        for d in &drawings {
            match d.clone() {
                DrawingType::Brush { points, mut s, end } => {
                    // s.width /= (screen.width() as f32 / w); //TODO  mettere se si vuole scalare il tratto
                    for i in 1..points.len() {
                        let to_paint = [self.adjust_drawing_pos(ctx, points[i], true), self.adjust_drawing_pos(ctx, points[i-1], true)];
                        painter.line_segment(to_paint, s);
                    }
                },
                DrawingType::Rectangle { r, s } => {
                    let to_paint = Rect::from_min_max(self.adjust_drawing_pos(ctx, r.min, true), self.adjust_drawing_pos(ctx, r.max, true));
                    painter.rect(to_paint, 0.0, s.color, s);
                },
                DrawingType::Circle { mut c, mut r, s } => {
                    c = self.adjust_drawing_pos(ctx, c, true);
                    r /= visualization_ratio;
                    painter.circle(c, r, s.color, s);
                },
                DrawingType::Arrow { p, v, s } => {
                    let origin = self.adjust_drawing_pos(ctx, p, true);
                    let direction = v / visualization_ratio;
                    painter.arrow(origin, direction, s);
                },
                DrawingType::Text { mut p , t , s} => {
                    p = self.adjust_drawing_pos(ctx, p, true);
                    //Regolazione del font in base alla dimensione della finestra di render
                    let font_size = (15.0 + s.width) / 1.0; //visualization_ratio; //TODO vedere come rendere più funzionante la staticità del testo
                    painter.text(p, Align2::LEFT_CENTER, t, FontId::new(font_size, egui::FontFamily::Proportional), s.color);
                },
            }
        }

        let color_picker_open = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("CP_open"))){
            Some(c) => c,
            None => false
        };

        let drawing_mode = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("DrawingMode"))) {
            Some(m) => m,
            None => DrawingMode::Brush,    
        };

        
        //TEXT
        let te_window = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("TE_open"))) {
            Some(te) => te,
            None => false,
        };

        if drawing_mode != DrawingMode::Text {
            let last_was_text = match drawings.last() {
                Some(last) => match last {
                    DrawingType::Text { p, t, s } => true,
                    _ => false,
                },
                None => false,
            };
            if last_was_text {
                ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("TE_open")));
                ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("TE_continue")));
                ctx.memory_mut(|mem| mem.data.remove::<String>(Id::from("TE_text")));
            }
        }
        else if te_window {
            let mut text: String = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("TE_text"))) {
                Some(text) => text,
                None => Default::default(),
            };
            let text_pos = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("Text_pos"))) {
                Some(pos) => pos,
                None => Pos2::ZERO,
            };
            let te_continue = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("TE_continue"))) {
                Some(c) => c,
                None => false,
            };

            //Calcolo posizione text editor per non farlo uscire dallo schermo (valori ottenuti in modo sperimentale)
            let mut te_pos = self.adjust_drawing_pos(ctx, text_pos, true);

            if te_pos.y + 97.0 > area.size()[1] + area.min.y {
                te_pos = te_pos - Vec2::new(0.0, 95.0);
            }
            else {
                te_pos = te_pos + Vec2::new(0.0, 20.0);
            }
            
            Window::new("")
            .fixed_pos(te_pos)
            .show(ctx, |ui| {
                let text_box = ui.text_edit_singleline(&mut text);

                if text_box.lost_focus() {
                    ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("TE_open")));
                    ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("TE_continue")));
                    ctx.memory_mut(|mem| mem.data.remove::<String>(Id::from("TE_text")));
                }
                else if text_box.changed() {
                    ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("TE_text"), text.clone()));
                    if te_continue {
                        drawings.pop().unwrap_or(DrawingType::Text { p: text_pos, t: text.clone(), s: stroke });
                    }
                    else {
                        ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("TE_continue"), true));
                    }

                    drawings.push(DrawingType::Text { p: text_pos, t: text, s: stroke });
                    ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Drawing"), drawings.clone()));
                }
            });
        }

        match ctx.pointer_hover_pos()/* ctx.input(|i| i.pointer.hover_pos()) */ {
            Some(mut mouse) => {
                let hover_rect = match ctx.memory(|mem| mem.data.get_temp(Id::from("hover_rect"))){
                    Some(r) => r,
                    None => area,
                };

                if hover_rect.contains(mouse) && !color_picker_open {
                    //Rescaling della posizione del mouse sulla dimensione completa dello screen in modo da mantenere la posizione fissa sulla tela
                    mouse = self.adjust_drawing_pos(ctx, mouse, false);

                    if ctx.input(|i| i.pointer.primary_clicked()) && !te_window{
                        tracing::error!("Pointer primary clicked");
                        match drawing_mode {
                            DrawingMode::Text => {
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("TE_open"), true));
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("TE_continue"), false));
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Text_pos"), mouse));
                            },
                            _ => {},
                        }
                    }

                    if ctx.input(|i| i.pointer.primary_down()) {
                        tracing::error!("Pointer primary down");
                        let mut p0 = match ctx.memory(|mem| mem.data.get_temp(Id::from("initial_pos"))) {
                            Some(p) => p,
                            None => {
                                let mut starting_pos = ctx.input(|i| i.pointer.press_origin()).unwrap();
                                //Resize della posizione iniziale (e di conseguenza di quella precedente)
                                starting_pos = self.adjust_drawing_pos(ctx, starting_pos, false);
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("initial_pos"), starting_pos));
                                starting_pos
                            }
                        };

                        //Anteprime dei tratti disegnati durante il drag
                        match drawing_mode {
                            DrawingMode::Brush => {
                                let prev = match ctx.memory(|mem| mem.data.get_temp::<Pos2>(Id::from("previous_pos"))) {
                                    Some(p) => p,
                                    None => p0,
                                };

                                match drawings.last() {
                                    Some(d) => {
                                        match d.clone() {
                                            DrawingType::Brush { points, s, end } => {
                                                if !end {
                                                    let mut ps = points.clone();
                                                    drawings.pop();
                                                    ps.push(mouse);
                                                    drawings.push(DrawingType::Brush { points: ps, s: stroke, end: false });
                                                }
                                                else {
                                                    let mut ps = Vec::new();
                                                    ps.push(prev);
                                                    ps.push(mouse);
                                                    drawings.push(DrawingType::Brush { points: ps, s: stroke, end: false });
                                                }
                                            },
                                            _ => {
                                                let mut ps = Vec::new();
                                                ps.push(prev);
                                                ps.push(mouse);
                                                drawings.push(DrawingType::Brush { points: ps, s: stroke, end: false });
                                            },
                                        }
                                    },
                                    None => {
                                        let mut ps = Vec::new();
                                        ps.push(prev);
                                        ps.push(mouse);
                                        drawings.push(DrawingType::Brush { points: ps, s: stroke, end: false });
                                    },
                                };
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp(Id::from("previous_pos"), mouse);
                                    mem.data.insert_temp(Id::from("Drawing"), drawings.clone());
                                });
                            },
                            DrawingMode::Rectangle => {
                                if mouse.x < p0.x { //TODO possibile rimuovere un set di controllo di questo tipo dai rect, trovare quale (possibile anche per circle)
                                    (mouse.x, p0.x) = (p0.x, mouse.x);
                                }
                                if mouse.y < p0.y {
                                    (mouse.y, p0.y) = (p0.y, mouse.y);
                                }
                                let to_paint_border = Rect::from_min_max(self.adjust_drawing_pos(ctx, p0, true), self.adjust_drawing_pos(ctx, mouse, true));
                                painter.rect_stroke(to_paint_border, 0.0, stroke);
                                tracing::error!("Painted rect with p0 {:?}, mouse {:?}, stroke {:?}", p0, mouse, stroke);
                            },
                            DrawingMode::Circle => {
                                if mouse.x < p0.x {
                                    (mouse.x, p0.x) = (p0.x, mouse.x);
                                }
                                if mouse.y < p0.y {
                                    (mouse.y, p0.y) = (p0.y, mouse.y);
                                }

                                let radius = (mouse.x - p0.x) / visualization_ratio;
                                let mut center = pos2(p0.x + (mouse.x - p0.x) / 2.0, p0.y + (mouse.y - p0.y) / 2.0);
                                center = self.adjust_drawing_pos(ctx, center, true);

                                painter.circle_stroke(center, radius, stroke);
                                tracing::error!("Painted circle with center {:?}, radius {:?}, stroke {:?}", center, radius, stroke);
                            },
                            DrawingMode::Arrow => {
                                let origin = self.adjust_drawing_pos(ctx, p0, true);
                                let direction = Vec2::new(mouse.x - p0.x, mouse.y - p0.y) / visualization_ratio;
                                painter.arrow(origin, direction, stroke);
                                tracing::error!("Painted arrow with origin {:?}, vector {:?}, stroke {:?}", p0, Vec2::new(mouse.x - p0.x, mouse.y - p0.y), stroke);
                            },
                            _ => {},
                        }
                        ctx.memory_mut(|mem| {
                            mem.data.insert_temp(Id::from("mouse_pos"), mouse);
                            mem.data.insert_temp(Id::from("hover_rect"), area);
                        });
                    }

                    if ctx.input(|i| i.pointer.primary_released()) {
                        tracing::error!("Pointer primary released");
                        match ctx.memory(|mem| mem.data.get_temp::<Pos2>(Id::from("initial_pos"))) {
                            Some(mut p0) => {
                                match drawing_mode {
                                    DrawingMode::Brush => {
                                        match drawings.last_mut() {
                                            Some(d) => match d {
                                                DrawingType::Brush { points, s, end } => {
                                                    points.push(mouse);
                                                    *end = true;
                                                },
                                                _ => {},
                                            }
                                            _ => {},
                                        }
                                        ctx.memory_mut(|mem| mem.data.remove::<Pos2>(Id::from("previous_pos")));
                                    },
                                    DrawingMode::Rectangle => {
                                        if mouse.x < p0.x {
                                            (mouse.x, p0.x) = (p0.x, mouse.x);
                                        }
                                        if mouse.y < p0.y {
                                            (mouse.y, p0.y) = (p0.y, mouse.y);
                                        }
        
                                        drawings.push(DrawingType::Rectangle { r: Rect::from_min_max(p0, mouse), s: stroke });
                                        tracing::error!("Added rect with p0 {:?}, mouse {:?}, stroke {:?}", p0, mouse, stroke);
                                    },
                                    DrawingMode::Circle => {
                                        if mouse.x < p0.x {
                                            (mouse.x, p0.x) = (p0.x, mouse.x);
                                        }
                                        if mouse.y < p0.y {
                                            (mouse.y, p0.y) = (p0.y, mouse.y);
                                        }
        
                                        let radius = mouse.x - p0.x;
                                        let center = pos2(p0.x + (mouse.x - p0.x) / 2.0, p0.y + (mouse.y - p0.y) / 2.0);
                                        drawings.push(DrawingType::Circle { c: center, r: radius, s: stroke });
                                        tracing::error!("Added circle with center {:?}, radius {:?}, stroke {:?}", center, radius, stroke);
                                    },
                                    DrawingMode::Arrow => {
                                        drawings.push(DrawingType::Arrow { p: p0, v: Vec2::new(mouse.x - p0.x, mouse.y - p0.y), s: stroke });
                                        tracing::error!("Added arrow with origin {:?}, vector {:?}, stroke {:?}", p0, Vec2::new(mouse.x - p0.x, mouse.y - p0.y), stroke);
                                    },
                                    _ => {},
                                }
        
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp(Id::from("Drawing"), drawings.clone());
                                    mem.data.remove::<Pos2>(Id::from("initial_pos"));
                                    mem.data.remove::<Rect>(Id::from("hover_rect"));
                                });
                            },
                            None => {},
                        };

                        
                    }
                }
                else {
                    let primary_up = !ctx.input(|i| i.pointer.primary_down());

                    if drawing_mode == DrawingMode::Brush || primary_up {
                        ctx.memory_mut(|mem| {
                            if drawing_mode == DrawingMode::Brush {
                                match drawings.last_mut() {
                                    Some(d) => match d {
                                        DrawingType::Brush { points: _points, s: _s, end } => {
                                            *end = true;
                                        },
                                        _ => {},
                                    }
                                    _ => {},
                                }
                                mem.data.insert_temp(Id::from("Drawing"), drawings.clone());
                            }

                            if drawing_mode == DrawingMode::Brush && !primary_up {
                                //Rescaling della posizione del mouse sulla dimensione completa dello screen in modo da mantenere la posizione fissa sulla tela
                                // mouse = self.adjust_drawing_pos(ctx, mouse, false);  //TODO controllare perchè inserendo questo rescale si ha un crash in caso di uscita dalla tela
                                mem.data.insert_temp(Id::from("previous_pos"), mouse);
                            }
                            else {
                                mem.data.remove::<Pos2>(Id::from("previous_pos"));
                            }
                            
                            mem.data.remove::<Pos2>(Id::from("initial_pos"));
                            mem.data.remove::<Rect>(Id::from("hover_rect"));
                        });
                    }
                }
            },
            None => {},
        }
    }

    fn adjust_drawing_pos(&mut self, ctx: &Context, pos: Pos2, render: bool) -> Pos2{
        let adjusted_pos: Pos2;
        let v_ratio: f32 = ctx.memory(|mem| {
            match mem.data.get_temp::<f32>(Id::from("Visualization_ratio")){
                Some(ratio) => ratio,
                None => f32::default(),
            }
        });
        let v_pos: Pos2 = ctx.memory(|mem| {
            match mem.data.get_temp::<Pos2>(Id::from("Visualization_pos")){
                Some(ratio) => ratio,
                None => Pos2::default(),
            }
        });
        //TODO risolvere sbrego e calo fps
        if render {
            adjusted_pos = pos2((pos.x / v_ratio) + v_pos.x, (pos.y / v_ratio) + v_pos.y);
        } else {
            adjusted_pos = pos2((pos.x - v_pos.x) * v_ratio, (pos.y - v_pos.y) * v_ratio);
        }
        adjusted_pos
    }
}
