use std::fs::File;
use std::time::Instant;
use image::{ImageBuffer, Rgba};
use crate::screenshot::take_screen;

fn convert_gif(){
    let image = take_screen(0).expect("Errore screen");

    let pix: Vec<u8> = image
        .pixels
        .iter()
        .flat_map(|p| p.to_array().iter().copied().collect::<Vec<u8>>())
        .collect();

    let im: ImageBuffer<Rgba<u8>, Vec<_>> =
        ImageBuffer::from_vec(image.width() as u32, image.height() as u32, pix)
            .expect("Unable to obtain ImageBuffer from vec");

    let buffer = File::create("foo.gif").unwrap();
    let mut gif = image::codecs::gif::GifEncoder::new_with_speed(buffer, 30);

    let now = Instant::now();

    let frame = image::Frame::new(im);
    gif.encode_frame(frame).expect("oh no");
    println!("Finished after {:?}", now.elapsed());
}