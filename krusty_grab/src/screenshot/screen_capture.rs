use std::str::FromStr;

use egui::ColorImage;
use screenshots::Screen;
use anyhow::Error;
use image::{ImageBuffer, Rgba, ImageFormat};

pub struct Shape{
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize
}

pub enum SaveFormat{
    Png,
    Gif,
    Jpeg
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

pub fn save_image(image: ColorImage, save_format: SaveFormat) -> Result<(), Error>{
    //Formulazione temporanea per la conversione da ColorImage a Vec<u8> utilizzato per la conversione in ImageBuffer
    let pix: Vec<u8> = image.pixels.iter().flat_map(|p| {p.to_array().iter().copied().collect::<Vec<u8>>()}).collect();
    let im: ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::from_vec(image.width() as u32, 
        image.height() as u32, 
        pix)
        .expect("Unable to obtain ImageBuffer from vec");
    
    let ext: &str;
    let format: ImageFormat;
    let file_name:String = String::from_str("out")?;

    match save_format{
        SaveFormat::Png => {
            ext = ".png";
            format = ImageFormat::Png
        },
        SaveFormat::Gif => {
            ext = ".gif";
            format = ImageFormat::Gif
        },
        SaveFormat::Jpeg => {
            ext = ".jpeg";
            format = ImageFormat::Jpeg
        },
        // _ => Err("Incompatible saving format"),
    }
    im.save_with_format(file_name + ext, format).expect("Unable to save the image");

    Ok(())
}