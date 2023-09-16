// #![allow(unused)]
use std::{
    fs::File,
    path::PathBuf,
    time::Instant,
};

use anyhow::Error;
use egui::ColorImage;
use image::{ImageBuffer, ImageFormat, Rgba};
use screenshots::Screen;

///Take a screenshot and converts it in a egui::ColorImage
pub fn take_screen(screen_src: usize) -> Result<ColorImage, Error> {
    let screen = Screen::all()?[screen_src];

    match screen.capture() {
        Ok(image) => Ok(ColorImage::from_rgba_unmultiplied(
            [image.width() as usize, image.height() as usize],
            image.rgba(),
        )),
        Err(e) => Err(e),
    }
}

///Return the number of connected screens
pub fn screens_number() -> usize {
    Screen::all()
        .expect("The screens should be retrieved")
        .len()
}

pub fn save_image(image: ColorImage, save_path: PathBuf) -> Result<(), Error> {
    //Formulazione temporanea per la conversione da ColorImage a Vec<u8> utilizzato per la conversione in ImageBuffer
    let pix: Vec<u8> = image
        .pixels
        .iter()
        .flat_map(|p| p.to_array().iter().copied().collect::<Vec<u8>>())
        .collect();

    let im: ImageBuffer<Rgba<u8>, Vec<_>> =
        ImageBuffer::from_vec(image.width() as u32, image.height() as u32, pix)
            .expect("Unable to obtain ImageBuffer from vec");

    let t = Instant::now();

    
    match save_path.clone().extension() {
        Some(ext) => {
            match ext.to_str().expect("Path string must be convertable") {
                "png" => {
                    im.save_with_format(save_path, ImageFormat::Png)
                        .expect("Unable to save the image");

                    println!("Inside {:?}", t.elapsed());

                    return Ok(());
                },
                "jpg" => {
                    im.save_with_format(save_path, ImageFormat::Jpeg)
                        .expect("Unable to save the image");
        
                    println!("Inside {:?}", t.elapsed());
        
                    return Ok(());
                },
                "gif" => {
                    let buffer = File::create(save_path).expect("Unable to create image file");
                    let mut gif_encoder = image::codecs::gif::GifEncoder::new_with_speed(buffer, 30);
        
                    let frame = image::Frame::new(im);
                    gif_encoder
                        .encode_frame(frame)
                        .expect("Unable to encode gif frame");
        
                    println!("Inside {:?}", t.elapsed());
        
                    return Ok(());
                },
                _ => todo!(),
            };
        },
        None => todo!(),
    }
}
