use egui::{Context, LayerId, Rect, pos2, Color32};
use egui_extras::RetainedImage;

use crate::krustygrab::KrustyGrab;

impl KrustyGrab {
    pub fn save_window(&mut self, ctx: &Context, frame: &mut eframe::Frame){
        let window_size = frame.info().window_info.size;
        let mut painter = ctx.layer_painter(LayerId::background());
        let image = RetainedImage::from_color_image(
            "Preview Image",
            self.get_temp_image().expect("Image must be defined"),
        );
        
        painter.set_clip_rect(Rect::from_min_size(pos2(0.0, 0.0), window_size));
                painter.image(
                    image.texture_id(ctx),
                    Rect::from_min_size(pos2(0.0, 0.0), window_size),
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    Color32::WHITE,
                );

        self.show_drawings_in_select(ctx, &painter);
        tracing::info!("{:?}", frame.info().window_info.fullscreen); 
        frame.request_screenshot();
    }
}