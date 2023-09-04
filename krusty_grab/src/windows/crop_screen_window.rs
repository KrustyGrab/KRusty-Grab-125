use egui::{
    pos2, Button, CentralPanel, Color32, Context, CursorIcon, Id, LayerId, Layout, Painter, Pos2,
    Rect, Vec2,
};
use egui_extras::RetainedImage;

use crate::krustygrab::{GrabStatus, KrustyGrab, WindowStatus};

impl KrustyGrab {
    const OVERLAY_COLOR: Color32 = Color32::from_black_alpha(100);
    const ADJUST_POINTS_COLOR: Color32 = Color32::from_rgb(100, 10, 10);
    const GRABBABLE_POINTS_SIZE: f32 = 10.0;

    ///Manage the visualization of the area selection.
    pub fn crop_screen_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        //Set the frame to fullscreen and enable the visibility
        _frame.set_fullscreen(true);
        _frame.set_visible(true);

        CentralPanel::default().show(ctx, |_ui| {
            let window_size = _frame.info().window_info.size;
            let mut painter = ctx.layer_painter(LayerId::background());
            let image = RetainedImage::from_color_image(
                "Preview Image",
                self.get_temp_image().expect("Image must be defined"),
            );

            ctx.memory_mut(|mem| {
                if mem
                    .data
                    .get_temp::<Option<Rect>>(Id::from("Prev_area"))
                    .is_none()
                {
                    mem.data
                        .insert_temp(Id::from("Prev_area"), self.get_selected_area());
                }
            });

            //Changing cursor to selection one
            ctx.set_cursor_icon(CursorIcon::Crosshair);

            //TODO Decidere se dare la possibilità di rimuovere l'area
            //Buttons for confirming or cancelling area selection. Visibles only when no manipulation is ongoing
            let mut save_rect: Rect = Rect::NOTHING;
            let mut cancel_rect: Rect = Rect::NOTHING;
            //TODO decidere forma e posizione pulsanti. Decidere se fare una funzione separata
            if self.get_grab_status() == GrabStatus::None {
                _ui.with_layer_id(
                    LayerId::new(egui::Order::Foreground, Id::from("Save")),
                    |ui| {
                        ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                            let save = ui.add_sized([60., 20.], Button::new("Save"));
                            let cancel =
                                ui.add_sized([60., 20.], Button::new("Cancel").rounding(30.));

                            let mut pressed = false;

                            save_rect = save.rect;
                            cancel_rect = cancel.rect;

                            //Save button
                            if ctx.pointer_hover_pos().is_some()
                                && save_rect.contains(ctx.pointer_hover_pos().unwrap())
                            {
                                ctx.set_cursor_icon(CursorIcon::PointingHand);
                                save.highlight();

                                if ctx.input(|i| i.pointer.primary_clicked()) {
                                    if self.get_selected_area().is_some() {
                                        //Save the screen part inside the selected area.
                                        self.set_definitive_image(Some(
                                            self.get_temp_image()
                                                .unwrap()
                                                .region(&self.get_selected_area().unwrap(), None),
                                        ));
                                    }

                                    ctx.memory_mut(|mem| {
                                        mem.data.remove::<Option<Rect>>(Id::from("Prev_area"))
                                    });

                                    pressed = true;
                                }
                            }
                            //Cancel button
                            else if ctx.pointer_hover_pos().is_some()
                                && cancel_rect.contains(ctx.pointer_hover_pos().unwrap())
                            {
                                ctx.set_cursor_icon(CursorIcon::PointingHand);
                                cancel.highlight();

                                if ctx.input(|i| i.pointer.primary_clicked()) {
                                    let prev_area = ctx.memory_mut(|mem| {
                                        let p = match mem
                                            .data
                                            .get_temp::<Option<Rect>>(Id::from("Prev_area"))
                                        {
                                            Some(p) => p,
                                            None => None,
                                        };
                                        mem.data.remove::<Option<Rect>>(Id::from("Prev_area"));
                                        p
                                    });

                                    self.set_select_area(prev_area);

                                    pressed = true;
                                }
                            }

                            //Return to main window and reshape the window if any button pressed
                            if pressed {
                                self.set_window_status(WindowStatus::Main);

                                _frame.set_fullscreen(false);
                            }
                        });
                    },
                );
            }

            //TODO sistemare la visualizzazione dei disegni durante il ritaglio e dopo in maniera corretta
            //Setting the visualization area and the screenshot as background
            painter.set_clip_rect(Rect::from_min_size(pos2(0.0, 0.0), window_size));
            painter.image(
                image.texture_id(ctx),
                Rect::from_min_size(pos2(0.0, 0.0), window_size),
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );

            self.show_drawings_in_select(ctx, &painter);

            //Show the selected area if present
            self.show_selected_area(ctx, _frame, &mut painter);

            if ctx.pointer_hover_pos().is_some()
                && !(save_rect.contains(ctx.pointer_hover_pos().unwrap())
                    || cancel_rect.contains(ctx.pointer_hover_pos().unwrap()))
            {
                self.select_area(ctx, _frame);
            }
        });
    }

    ///Check and set the coordinates of the selected area using drag and release.
    fn select_area(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        //Reset the status if the selection is ended
        if ctx.input(|i| i.pointer.primary_released())
            && self.get_grab_status() == GrabStatus::Select
        {
            self.set_grab_status(GrabStatus::None);
        }
        //Otherwhise start the selection, if it is not started and a mouse pression is detected, or continue it until the mouse is released
        else if ctx.input(|i| i.pointer.primary_down())
            && (self.get_grab_status() == GrabStatus::Select
                || self.get_grab_status() == GrabStatus::None)
        {
            self.set_grab_status(GrabStatus::Select);

            //Get drag initial and actual position
            let mut init_pos = ctx.input(|i| {
                i.pointer
                    .press_origin()
                    .expect("Press origin must be defined")
            });
            let mut drag_pos = ctx
                .pointer_hover_pos()
                .expect("Hover position must be some");

            //Update the saved area
            if init_pos != drag_pos {
                (init_pos, drag_pos) =
                    self.check_coordinates(init_pos, drag_pos, _frame.info().window_info.size);
                self.set_select_area(Some(Rect::from_min_max(init_pos, drag_pos)));
            }
        }
    }

    ///Draw the selected area if Some. Constant [`KrustyGrab::OVERLAY_COLOR`] is used as the color for the overlay.
    fn show_selected_area(
        &mut self,
        ctx: &Context,
        _frame: &mut eframe::Frame,
        painter: &mut Painter,
    ) {
        let window_size = _frame.info().window_info.size;

        //Check if the area is Some, otherwise draw the background overlay on all the screen
        match self.get_selected_area() {
            Some(sel) => {
                //Draw the points used for resizing
                self.grabbable_corners(ctx, _frame, painter);

                let min = sel.min;
                let max = sel.max;

                let min_x = min.x;
                let max_x = max.x;
                let min_y = min.y;
                let max_y = max.y;

                //Draw the overlay only on the screen parts that are not selected. Achieved using 4 rectangles
                painter.rect_filled(
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(min_x, window_size.y)),
                    0.0,
                    KrustyGrab::OVERLAY_COLOR,
                );
                painter.rect_filled(
                    Rect::from_min_max(pos2(max_x, 0.0), pos2(window_size.x, window_size.y)),
                    0.0,
                    KrustyGrab::OVERLAY_COLOR,
                );
                painter.rect_filled(
                    Rect::from_min_max(pos2(min_x, 0.0), pos2(max_x, min_y)),
                    0.0,
                    KrustyGrab::OVERLAY_COLOR,
                );
                painter.rect_filled(
                    Rect::from_min_max(pos2(min_x, max_y), pos2(max_x, window_size.y)),
                    0.0,
                    KrustyGrab::OVERLAY_COLOR,
                );
            }
            None => painter.rect_filled(
                //Draw the overlay on all the screen
                Rect::from_min_size(pos2(0.0, 0.0), window_size),
                0.0,
                KrustyGrab::OVERLAY_COLOR,
            ),
        }
    }

    ///Draw and manage the points used for the resizing of the selection area. Constant [`KrustyGrab::ADJUST_POINTS_COLOR`] is used as the color for the points ans [`KrustyGrab::GRABBABLE_POINTS_SIZE`] as the size.
    fn grabbable_corners(
        &mut self,
        ctx: &Context,
        _frame: &mut eframe::Frame,
        painter: &mut Painter,
    ) {
        let sel = self
            .get_selected_area()
            .expect("Selected area must be Some");
        let grab_status = self.get_grab_status();

        //The draggable points are drawn only when the selction sequence is not ongoing
        if grab_status != GrabStatus::Select {
            let point_dim = Vec2::splat(KrustyGrab::GRABBABLE_POINTS_SIZE);

            //Handle points shapes
            let tl_point = Rect::from_center_size(sel.left_top(), point_dim);
            let tm_point = Rect::from_center_size(sel.center_top(), point_dim);
            let tr_point = Rect::from_center_size(sel.right_top(), point_dim);
            let ml_point = Rect::from_center_size(sel.left_center(), point_dim);
            let mr_point = Rect::from_center_size(sel.right_center(), point_dim);
            let bl_point = Rect::from_center_size(sel.left_bottom(), point_dim);
            let bm_point = Rect::from_center_size(sel.center_bottom(), point_dim);
            let br_point = Rect::from_center_size(sel.right_bottom(), point_dim);

            //Handles are only drawn if the selection area is not being moved
            if grab_status != GrabStatus::Move {
                //Draw the handles for resizing
                painter.rect_filled(tl_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);
                painter.rect_filled(tr_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);
                painter.rect_filled(bl_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);
                painter.rect_filled(br_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);

                painter.rect_filled(tm_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);
                painter.rect_filled(ml_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);
                painter.rect_filled(mr_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);
                painter.rect_filled(bm_point, 0.0, KrustyGrab::ADJUST_POINTS_COLOR);
            }

            //Handle the handles interaction
            match ctx.pointer_hover_pos() {
                Some(pos) => {
                    let mut new_status = GrabStatus::None;

                    if tm_point.contains(pos) || grab_status == GrabStatus::TopMid {
                        ctx.set_cursor_icon(CursorIcon::ResizeNorth);
                        new_status = GrabStatus::TopMid;
                    } else if ml_point.contains(pos) || grab_status == GrabStatus::MidLeft {
                        ctx.set_cursor_icon(CursorIcon::ResizeWest);
                        new_status = GrabStatus::MidLeft;
                    } else if mr_point.contains(pos) || grab_status == GrabStatus::MidRight {
                        ctx.set_cursor_icon(CursorIcon::ResizeEast);
                        new_status = GrabStatus::MidRight;
                    } else if bm_point.contains(pos) || grab_status == GrabStatus::BotMid {
                        ctx.set_cursor_icon(CursorIcon::ResizeSouth);
                        new_status = GrabStatus::BotMid;
                    } else if tl_point.contains(pos) || grab_status == GrabStatus::TopLeft {
                        ctx.set_cursor_icon(CursorIcon::ResizeNorthWest);
                        new_status = GrabStatus::TopLeft;
                    } else if tr_point.contains(pos) || grab_status == GrabStatus::TopRight {
                        ctx.set_cursor_icon(CursorIcon::ResizeNorthEast);
                        new_status = GrabStatus::TopRight;
                    } else if bl_point.contains(pos) || grab_status == GrabStatus::BotLeft {
                        ctx.set_cursor_icon(CursorIcon::ResizeSouthWest);
                        new_status = GrabStatus::BotLeft;
                    } else if br_point.contains(pos) || grab_status == GrabStatus::BotRight {
                        ctx.set_cursor_icon(CursorIcon::ResizeSouthEast);
                        new_status = GrabStatus::BotRight;
                    } else if sel.contains(pos) || grab_status == GrabStatus::Move {
                        ctx.set_cursor_icon(CursorIcon::Grab);
                        new_status = GrabStatus::Move;
                    }

                    if new_status != GrabStatus::None {
                        self.update_area(ctx, _frame, pos, new_status);
                    }
                }
                None => ctx.set_cursor_icon(CursorIcon::Crosshair),
            }
        }
    }

    ///Update the area based on the handle used
    fn update_area(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        pos: Pos2,
        status: GrabStatus,
    ) {
        //Retrieve previous area values
        let sel = self
            .get_selected_area()
            .expect("Selected area must be Some when updating it");
        let mut new_min = sel.min;
        let mut new_max = sel.max;

        //Based on the handle the area is updated in a different way while mouse button is pressed
        if ctx.input(|i| i.pointer.primary_down()) {
            match self.get_grab_status() {
                GrabStatus::None => self.set_grab_status(status), //Set the passed status if enters with None
                GrabStatus::Select => panic!("Should not be in Select mode during area updating"), //Unreachable code, should panic if reached
                GrabStatus::TopLeft => new_min = pos,
                GrabStatus::TopMid => new_min = pos2(sel.min.x, pos.y),
                GrabStatus::TopRight => {
                    new_min = pos2(sel.min.x, pos.y);
                    new_max = pos2(pos.x, sel.max.y);
                }
                GrabStatus::MidLeft => new_min = pos2(pos.x, sel.min.y),
                GrabStatus::MidRight => new_max = pos2(pos.x, sel.max.y),
                GrabStatus::BotLeft => {
                    new_min = pos2(pos.x, sel.min.y);
                    new_max = pos2(sel.max.x, pos.y);
                }
                GrabStatus::BotMid => new_max = pos2(sel.max.x, pos.y),
                GrabStatus::BotRight => new_max = pos,
                GrabStatus::Move => {
                    ctx.set_cursor_icon(CursorIcon::Grabbing);

                    //Save the distance from center of the selected area and the cursor if the Move operation just started
                    let center_distance =
                        match ctx.memory(|mem| mem.data.get_temp(Id::from("Center_distance"))) {
                            Some(distance) => distance,
                            None => {
                                let start_coord = ctx
                                    .pointer_interact_pos()
                                    .expect("Pointer position must be found")
                                    .to_vec2();
                                let distance = start_coord - sel.center().to_vec2();
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp(Id::from("Center_distance"), distance)
                                });
                                distance
                            }
                        };

                    //Updated center considering the pointer position
                    let mut new_center = pos2(pos.x - center_distance.x, pos.y - center_distance.y);

                    //Check of the position of the new center in order to keep it inside the visualized window aka the screenshot area
                    {
                        let size = sel.size();
                        let window_size = _frame.info().window_info.size;

                        if new_center.x + size[0] / 2.0 > window_size[0] {
                            new_center.x = window_size[0] - size[0] / 2.0;
                        }
                        if new_center.y + size[1] / 2.0 > window_size[1] {
                            new_center.y = window_size[1] - size[1] / 2.0;
                        }
                        if new_center.x - size[0] / 2.0 < 0.0 {
                            new_center.x = size[0] / 2.0;
                        }
                        if new_center.y - size[1] / 2.0 < 0.0 {
                            new_center.y = size[1] / 2.0;
                        }
                    }

                    //Update the area withe the new center
                    self.set_select_area(Some(Rect::from_center_size(new_center, sel.size())));
                }
            }

            //Update the selected area, after checks, if not in Move mode (area updated in the match clause)
            if self.get_grab_status() != GrabStatus::Move {
                (new_min, new_max) =
                    self.check_coordinates(new_min, new_max, _frame.info().window_info.size);
                self.set_select_area(Some(Rect::from_min_max(new_min, new_max)));
            }
        }
        //Mouse button released -> reset status and used values
        else {
            if self.get_grab_status() == GrabStatus::Move {
                ctx.memory_mut(|mem| mem.data.remove::<Vec2>(Id::from("Center_distance")));
            }
            self.set_grab_status(GrabStatus::None);
        }
    }

    ///Checks if the coordinates are inside the visualized window
    fn check_coordinates(&mut self, start: Pos2, end: Pos2, window_size: Vec2) -> (Pos2, Pos2) {
        let mut init_pos = start;
        let mut end_pos = end;

        //Top - Left
        if init_pos.x < 0.0 {
            init_pos.x = 0.0;
        }
        if init_pos.y < 0.0 {
            init_pos.y = 0.0;
        }
        if end_pos.x < 0.0 {
            end_pos.x = 0.0;
        }
        if end_pos.y < 0.0 {
            end_pos.y = 0.0;
        }

        //Bottom - Right
        if init_pos.x > window_size.x {
            init_pos.x = window_size.x;
        }
        if init_pos.y > window_size.y {
            init_pos.y = window_size.y;
        }
        if end_pos.x > window_size.x {
            end_pos.x = window_size.x;
        }
        if end_pos.y > window_size.y {
            end_pos.y = window_size.y;
        }

        //Status needed during area manipolation in order to set the right one when min and max positions gets inverted
        let mut grab_status = self.get_grab_status();

        if init_pos.x > end_pos.x {
            let tmp = init_pos.x;
            init_pos.x = end_pos.x;
            end_pos.x = tmp;

            if grab_status == GrabStatus::MidLeft {
                grab_status = GrabStatus::MidRight;
            } else if grab_status == GrabStatus::MidRight {
                grab_status = GrabStatus::MidLeft;
            } else if grab_status == GrabStatus::TopLeft {
                grab_status = GrabStatus::TopRight;
            } else if grab_status == GrabStatus::TopRight {
                grab_status = GrabStatus::TopLeft;
            } else if grab_status == GrabStatus::BotLeft {
                grab_status = GrabStatus::BotRight;
            } else if grab_status == GrabStatus::BotRight {
                grab_status = GrabStatus::BotLeft;
            }
        }

        if init_pos.y > end_pos.y {
            let tmp = init_pos.y;
            init_pos.y = end_pos.y;
            end_pos.y = tmp;

            if grab_status == GrabStatus::TopMid {
                grab_status = GrabStatus::BotMid;
            } else if grab_status == GrabStatus::BotMid {
                grab_status = GrabStatus::TopMid;
            } else if grab_status == GrabStatus::TopLeft {
                grab_status = GrabStatus::BotLeft;
            } else if grab_status == GrabStatus::TopRight {
                grab_status = GrabStatus::BotRight;
            } else if grab_status == GrabStatus::BotLeft {
                grab_status = GrabStatus::TopLeft;
            } else if grab_status == GrabStatus::BotRight {
                grab_status = GrabStatus::TopRight;
            }
        }

        self.set_grab_status(grab_status);

        (init_pos, end_pos)
    }
}
