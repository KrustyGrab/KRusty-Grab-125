mod screenshot;

use screenshot::{SaveOptions, take_screen, save_image};

//Esempio di creazione e salvataggio di screen, per evitare che lo screen venga caricato su GitHub inserire nel nome "out"
fn main() {
    let s = SaveOptions::new();
    let im = take_screen(0).expect("Errore screen");

    save_image(im, s).expect("Salvataggio fallito");
}
