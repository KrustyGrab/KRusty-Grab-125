mod krustygrab;

use eframe::{run_native, NativeOptions};
use egui::{Vec2, Context};
use krustygrab::KrustyGrab;

fn main() {
    tracing_subscriber::fmt::init();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(960., 540.));
    run_native(
            "ScreenshotApp", 
            win_options, 
            Box::new(|context| Box::new(KrustyGrab::new(context)))).unwrap();
}