use egui::{CentralPanel, ColorImage, Context, Rect, Vec2};
use egui_extras::RetainedImage;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GrabStatus {
    None,
    Select,
    TopLeft,
    TopMid,
    TopRight,
    MidLeft,
    MidRight,
    BotLeft,
    BotMid,
    BotRight,
    Move,
}

pub enum WindowStatus {
    Main,
    Crop,
}

pub struct Status {
    temp_image: Option<ColorImage>,
    select: Option<Rect>,
    grab_status: GrabStatus,
    defintive_image: Option<ColorImage>,
    window_status: WindowStatus,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            temp_image: Some(ColorImage::example()),
            select: None,
            grab_status: GrabStatus::None,
            defintive_image: None,
            window_status: WindowStatus::Main,
        }
    }
}

impl Status {
    pub fn new() -> Self {
        Self {
            temp_image: None,
            ..Default::default()
        }
    }

    pub fn get_temp_image(&self) -> Option<ColorImage> {
        self.temp_image.clone()
    }
    pub fn get_definitive_image(&self) -> Option<ColorImage> {
        self.defintive_image.clone()
    }
    pub fn get_selected_area(&self) -> Option<Rect> {
        self.select
    }
    pub fn get_grab_status(&self) -> GrabStatus {
        self.grab_status
    }

    pub fn set_temp_image(&mut self, image: Option<ColorImage>) {
        self.temp_image = image;
    }
    pub fn set_definitive_image(&mut self, image: Option<ColorImage>) {
        self.defintive_image = image;
    }
    pub fn set_grab_status(&mut self, new_status: GrabStatus) {
        self.grab_status = new_status;
    }
    pub fn set_select_area(&mut self, new_select: Option<Rect>) {
        self.select = new_select;
    }
    pub fn set_window_status(&mut self, new_status: WindowStatus) {
        self.window_status = new_status;
    } 
}

impl eframe::App for Status {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match self.window_status {
            WindowStatus::Main => self.main_window(ctx, _frame),
            WindowStatus::Crop => self.crop_screen_window(ctx, _frame),
        }
    }
}

impl Status {
    fn main_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        _frame.set_fullscreen(false);
        _frame.set_window_size(Vec2::new(800., 430.)); //TODO non considerare in una versione finale
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Crop screenshot").clicked() {
                self.window_status = WindowStatus::Crop;
            }

            if self.defintive_image.is_some() {
                let image = RetainedImage::from_color_image(
                    "Preview Image",
                    self.get_definitive_image().expect("Image must be defined"),
                );
                ui.image(image.texture_id(ctx), Vec2::new(960.0, 540.0));
            }
        });
    }
}
