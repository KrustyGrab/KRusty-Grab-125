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

        for d in &drawings {
            match d.clone() {
                DrawingType::Brush { points, s, end } => {
                    for i in 1..points.len() {
                        painter.line_segment([points[i], points[i-1]], s);
                    }
                },
                DrawingType::Rectangle { r, s } => {
                    painter.rect(r, 0.0, s.color, s);
                },
                DrawingType::Circle { c, r, s } => {
                    painter.circle(c, r, s.color, s);
                },
                DrawingType::Arrow { p, v, s } => {
                    painter.arrow(p, v, s);
                },
                DrawingType::Text { p , t , s} => {
                    println!("{t:?}");
                    painter.text(p, Align2::CENTER_CENTER, t, FontId::new(15.0, egui::FontFamily::Proportional), s.color);
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

        match ctx.pointer_hover_pos()/* ctx.input(|i| i.pointer.hover_pos()) */ {
            Some(mut mouse) => {
                let hover_rect = match ctx.memory(|mem| mem.data.get_temp(Id::from("hover_rect"))){
                    Some(r) => r,
                    None => area,
                };

                if hover_rect.contains(mouse) && !color_picker_open {
                    
                    //TEXT
                    let mut text: String = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("TE_text"))) {
                        Some(text) => text,
                        None => Default::default(),
                    };
                    let te_window = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("TE_open"))) {
                        Some(te) => te,
                        None => false,
                    };
                    let te_pos = match ctx.memory_mut(|mem| mem.data.get_temp(Id::from("TE_pos"))) {
                        Some(pos) => pos,
                        None => Pos2::ZERO,
                    };

                    if te_window {
                        Window::new("")
                        .fixed_pos(te_pos)
                        .show(ctx, |ui| {
                            let mut text_box = ui.text_edit_singleline(&mut text);
                            if text_box.lost_focus() {
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("TE_open"), false));
                                ctx.memory_mut(|mem| mem.data.remove::<String>(Id::from("TE_text")));
                            }
                            // else if text_box.changed() {
                            //     ctx.memory_mut(|mem| mem.data.insert_temp((Id::from("TE_text")), text.clone()));
                            //     ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Drawing"), drawings.clone()));
                            //     // println!("{:?}", drawings.last());
                            //     drawings.push(DrawingType::Text { p: mouse, t: text, s: stroke });
                            // }
                        });
                    } 

                    if ctx.input(|i| i.pointer.primary_clicked()) && !te_window{
                        tracing::error!("Pointer primary clicked");
                        match drawing_mode {
                            DrawingMode::Text => {
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("TE_open"), true));
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("TE_pos"), mouse));
                                
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("Drawing"), drawings.clone()));
                            },
                            _ => {},
                        }
                    }

                    if ctx.input(|i| i.pointer.primary_down()) {
                        tracing::error!("Pointer primary down");
                        let mut p0 = match ctx.memory(|mem| mem.data.get_temp(Id::from("initial_pos"))) {
                            Some(p) => p,
                            None => {
                                let starting_pos = ctx.input(|i| i.pointer.press_origin()).unwrap();
                                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("initial_pos"), starting_pos));
                                starting_pos
                            }
                        };


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
                                if mouse.x < p0.x {
                                    (mouse.x, p0.x) = (p0.x, mouse.x);
                                }
                                if mouse.y < p0.y {
                                    (mouse.y, p0.y) = (p0.y, mouse.y);
                                }

                                painter.rect_stroke(Rect::from_min_max(p0, mouse), 0.0, stroke);
                                tracing::error!("Painted rect with p0 {:?}, mouse {:?}, stroke {:?}", p0, mouse, stroke);
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

                                painter.circle_stroke(center, radius, stroke);
                                tracing::error!("Painted circle with center {:?}, radius {:?}, stroke {:?}", center, radius, stroke);
                            },
                            DrawingMode::Arrow => {
                                painter.arrow(p0, Vec2::new(mouse.x - p0.x, mouse.y - p0.y), stroke);
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
                    if ctx.input(|i| i.pointer.primary_released()) {
                        ctx.memory_mut(|mem| {
                            if drawing_mode == DrawingMode::Brush {
                                match drawings.last_mut() {
                                    Some(d) => match d {
                                        DrawingType::Brush { points: _points, s: _s, end } => {
                                            *end = true;
                                            println!("ciaos - {end:?}");
                                        },
                                        _ => {},
                                    }
                                    _ => {},
                                }
                                mem.data.insert_temp(Id::from("Drawing"), drawings.clone());
                            };
    
                            mem.data.remove::<Pos2>(Id::from("previous_pos"));
                            mem.data.remove::<Pos2>(Id::from("initial_pos"));
                            mem.data.remove::<Rect>(Id::from("hover_rect"));
                        });
                    }
                }
            },
            None => {},
        }
    }
}
