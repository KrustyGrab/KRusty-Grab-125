use egui::{Vec2, Context, TextureId};
use egui_extras::RetainedImage;

pub const ICON_SIZE: Vec2 = Vec2::splat(28.0);

pub fn icon_img(name: &str, ctx: &Context) -> TextureId {
    match ctx.style().visuals.dark_mode {
        true => match name {
            "pencil" => RetainedImage::from_svg_bytes_with_size(
                "pencil",
                include_bytes!("./images/pencil.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "circle" => RetainedImage::from_svg_bytes_with_size(
                "circle",
                include_bytes!("./images/circle.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "rect" => RetainedImage::from_svg_bytes_with_size(
                "rect",
                include_bytes!("./images/rect.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "arrow" => RetainedImage::from_svg_bytes_with_size(
                "arrow",
                include_bytes!("./images/arrow.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "text" => RetainedImage::from_svg_bytes_with_size(
                "text",
                include_bytes!("./images/text.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "gear" => RetainedImage::from_svg_bytes_with_size(
                "gear",
                include_bytes!("./images/gear.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "camera" => RetainedImage::from_svg_bytes_with_size(
                "camera",
                include_bytes!("./images/camera.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "undo" => RetainedImage::from_svg_bytes_with_size(
                "undo",
                include_bytes!("./images/undo.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "redo" => RetainedImage::from_svg_bytes_with_size(
                "redo",
                include_bytes!("./images/redo.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "select" => RetainedImage::from_svg_bytes_with_size(
                "select",
                include_bytes!("./images/select.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "cut" => RetainedImage::from_svg_bytes_with_size(
                "cut",
                include_bytes!("./images/cut.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "timer" => RetainedImage::from_svg_bytes_with_size(
                "timer",
                include_bytes!("./images/timer.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "save" => RetainedImage::from_svg_bytes_with_size(
                "save",
                include_bytes!("./images/save.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "save_as" => RetainedImage::from_svg_bytes_with_size(
                "save_as",
                include_bytes!("./images/save_as.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "folder" => RetainedImage::from_svg_bytes_with_size(
                "folder",
                include_bytes!("./images/folder.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            _ => panic!("Invalid icon")
        },
        false => match name {
            "pencil" => RetainedImage::from_svg_bytes_with_size(
                "pencil",
                include_bytes!("./images/pencil_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "circle" => RetainedImage::from_svg_bytes_with_size(
                "circle",
                include_bytes!("./images/circle_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "rect" => RetainedImage::from_svg_bytes_with_size(
                "rect",
                include_bytes!("./images/rect_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "arrow" => RetainedImage::from_svg_bytes_with_size(
                "arrow",
                include_bytes!("./images/arrow_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "text" => RetainedImage::from_svg_bytes_with_size(
                "text",
                include_bytes!("./images/text_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "gear" => RetainedImage::from_svg_bytes_with_size(
                "gear",
                include_bytes!("./images/gear_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "camera" => RetainedImage::from_svg_bytes_with_size(
                "camera",
                include_bytes!("./images/camera_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "undo" => RetainedImage::from_svg_bytes_with_size(
                "undo",
                include_bytes!("./images/undo_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "redo" => RetainedImage::from_svg_bytes_with_size(
                "redo",
                include_bytes!("./images/redo_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "select" => RetainedImage::from_svg_bytes_with_size(
                "select",
                include_bytes!("./images/select_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "cut" => RetainedImage::from_svg_bytes_with_size(
                "cut",
                include_bytes!("./images/cut_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "timer" => RetainedImage::from_svg_bytes_with_size(
                "timer",
                include_bytes!("./images/timer_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "save" => RetainedImage::from_svg_bytes_with_size(
                "save",
                include_bytes!("./images/save_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "save_as" => RetainedImage::from_svg_bytes_with_size(
                "save_as",
                include_bytes!("./images/save_as_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            "folder" => RetainedImage::from_svg_bytes_with_size(
                "folder",
                include_bytes!("./images/folder_light.svg"),
                egui_extras::image::FitTo::Original).unwrap().texture_id(ctx),
            _ => panic!("Invalid icon")
        },
    }
}
