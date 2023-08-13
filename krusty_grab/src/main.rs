mod screenshot;

use std::{time::Instant, path::Path, ffi::OsString, str::FromStr};

use screenshot::{SaveOptions, take_screen, save_image};

//Esempio di creazione e salvataggio di screen, per evitare che lo screen venga caricato su GitHub inserire nel nome "out"
fn main() {
    let s = SaveOptions::new_with_details(screenshot::SaveFormat::Gif, Path::new("./"), OsString::from_str("out").expect("ciao"));
    let im = take_screen(0).expect("Errore screen");
    let t = Instant::now();
    save_image(im, s).expect("Salvataggio fallito");
    println!("Outside {:?}", t.elapsed());
}
