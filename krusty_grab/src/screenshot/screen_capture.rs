// #![allow(unused)]
use std::{
    ffi::OsString,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

use anyhow::Error;
use egui::{ColorImage};
use image::{ImageBuffer, ImageFormat, Rgba};
use screenshots::{Screen, Image};

pub enum SaveFormat {
    Png,
    Gif,
    Jpg,
}

impl ToString for SaveFormat {
    fn to_string(&self) -> String {
        match self {
            SaveFormat::Png => String::from(".png"),
            SaveFormat::Gif => String::from(".gif"),
            SaveFormat::Jpg => String::from(".jpg"),
        }
    }
}

pub struct SaveOptions<'a> {
    format: SaveFormat,
    path: Box<&'a Path>,
    file_name: OsString,
}

impl<'a> SaveOptions<'a> {
    pub fn new() -> Self {
        Self {
            format: SaveFormat::Png,
            path: Box::new(Path::new("./")),
            file_name: OsString::from_str("out").expect("The OsString must be correctly created"),
        }
    }

    pub fn new_with_details(format: SaveFormat, path: &'a Path, file_name: OsString) -> Self {
        Self {
            format,
            path: Box::new(path),
            file_name,
        }
    }

    pub fn change_format(&mut self, format: SaveFormat) -> Option<()> {
        self.format = format;
        Some(())
    }

    pub fn change_path(&mut self, path: &'a Path) -> Option<()> {
        self.path = Box::new(path);
        Some(())
    }

    pub fn change_file_name(&mut self, file_name: OsString) -> Option<()> {
        self.file_name = file_name;
        Some(())
    }

    pub fn save_file_name(&self) -> OsString {
        let mut file_name: OsString = self.file_name.clone();
        file_name.push(self.format.to_string());

        file_name
    }

    pub fn save_path(&self) -> PathBuf {
        let mut save_path = PathBuf::from(*self.path);
        save_path.push(
            self.save_file_name()
                .to_str()
                .expect("Should be a convertible string"),
        );

        save_path
    }
}

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
    Screen::all().expect("The screens should be retrieved").len()
}

pub fn save_image(image: ColorImage, save_options: SaveOptions) -> Result<(), Error> {
    //Formulazione temporanea per la conversione da ColorImage a Vec<u8> utilizzato per la conversione in ImageBuffer
    let pix: Vec<u8> = image
        .pixels
        .iter()
        .flat_map(|p| p.to_array().iter().copied().collect::<Vec<u8>>())
        .collect();

    let im: ImageBuffer<Rgba<u8>, Vec<_>> =
        ImageBuffer::from_vec(image.width() as u32, image.height() as u32, pix)
            .expect("Unable to obtain ImageBuffer from vec");

    let save_path = save_options.save_path();

    let t = Instant::now();

    match save_options.format {
        SaveFormat::Png => {
            im.save_with_format(save_path, ImageFormat::Png)
                .expect("Unable to save the image");

            println!("Inside {:?}", t.elapsed());

            return Ok(());
        }
        SaveFormat::Jpg => {
            im.save_with_format(save_path, ImageFormat::Jpeg)
                .expect("Unable to save the image");

            println!("Inside {:?}", t.elapsed());

            return Ok(());
        }
        SaveFormat::Gif => {
            let buffer = File::create(save_path).expect("Unable to create image file");
            let mut gif_encoder = image::codecs::gif::GifEncoder::new_with_speed(buffer, 30);

            let frame = image::Frame::new(im);
            gif_encoder
                .encode_frame(frame)
                .expect("Unable to encode gif frame");

            println!("Inside {:?}", t.elapsed());

            return Ok(());
        }
    }
}
