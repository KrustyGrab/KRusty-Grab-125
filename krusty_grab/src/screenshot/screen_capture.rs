use egui::ColorImage;
use screenshots::Screen;
use anyhow::Error;

pub struct Shape{
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize
}

impl Shape{
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self{
        Self { start_x: x, start_y: y, width: width, height: height }
    }
}

pub fn take_screen(screen_src: usize) -> Result<ColorImage, Error> {
    let screen = Screen::all()?[screen_src];

    match screen.capture() {
        Ok(image) => Ok(ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize], image.rgba())),
        Err(e) => Err(e),
    }
}

pub fn take_crop_screen(screen_src: usize, crop: Shape) -> Result<ColorImage, Error>{
    let screen = Screen::all()?[screen_src];

    match screen.capture_area(crop.start_x as i32, crop.start_y as i32, crop.width as u32, crop.height as u32){
        Ok(image) => Ok(ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize], image.rgba())),
        Err(e) => Err(e),
    }
}
