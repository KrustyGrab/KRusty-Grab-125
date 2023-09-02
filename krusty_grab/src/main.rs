pub mod screenshot;

use std::{time::Instant, path::Path, ffi::OsString, str::FromStr};

use eframe::{Theme, run_native};
use egui::ColorImage;
use krusty_grab::manage_window::Status;

use screenshot::{SaveOptions, take_screen, save_image};

//Example app for the area selection of a screenshot
fn main() {
    let im = take_screen(0).expect("Errore screen");
    save_example(im.clone());
    gui_test(im.clone());
}

#[allow(dead_code)]
fn save_example(im: ColorImage) {
    let s = SaveOptions::new_with_details(screenshot::SaveFormat::Gif, Path::new("./"), OsString::from_str("out").expect("ciao"));
    let t = Instant::now();
    save_image(im, s).expect("Salvataggio fallito");
    println!("Outside {:?}", t.elapsed());
}

#[allow(dead_code)]
fn gui_test(im: ColorImage) {
    let app_name = "Crop sample";
    let opt = eframe::NativeOptions {
        resizable: true,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        fullscreen: false,
        ..Default::default()
    };

    let mut _p = Status::new();
    _p.set_temp_image(Some(im.clone()));
    _p.set_definitive_image(Some(im.clone()));

    run_native(app_name, opt, Box::new(move |_cc| Box::<Status>::new(_p)))
        .expect("Unable to create app");
}