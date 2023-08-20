use egui::{Vec2, Context, TextureId};
use egui_extras::RetainedImage;

pub const ICON_SIZE: Vec2 = Vec2::splat(28.0);

pub fn icon_img(name: &str, ctx: &Context) -> TextureId {
    match name {
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
        _ => panic!("Invalid icon")
    }
}
