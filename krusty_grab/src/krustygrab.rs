use std::{iter::Map, collections::{HashMap, BTreeMap}, path::Display, arch::x86_64::__cpuid};
#[allow(unused)]
use std::{path::{PathBuf, Path}, time::Instant, io::Write};

use crate::painting::icons::{icon_img, ICON_SIZE};
use eframe::{App, CreationContext};
use egui::{
    Button, ColorImage, Context, FontId, Grid, Layout, Rect,
    RichText, TextStyle, Visuals,
    Widget, Window, TextEdit,
    Key, Modifiers, Event, KeyboardShortcut, Sense, Align, Vec2,
};
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use egui_hotkey::{Hotkey, BindVariant};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Format {
    Jpeg,
    Png,
    Gif,
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Jpeg => "jpg".to_string(),
            Format::Png => "png".to_string(),
            Format::Gif => "gif".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyHotKey{
    pub modifier: Modifiers,
    pub key: Option::<Key>,

}
impl MyHotKey{
    fn new(modifier: Modifiers, key: Key)-> Self{
        if key == Key::Enter {
            return Self{modifier, key: None};
        }
        Self{modifier, key: Some(key)}
    }
    fn humanprint(& self) -> String{
        if self.modifier.is_none() {
            return format!("{:?}", self.key).to_string();
        }
        let mut m = "";
        if self.modifier.alt == true {
            m = "ALT";
        }
        if self.modifier.ctrl == true {
            m = "CTRL";
        }
        if self.modifier.command == true {
            m = "CMD";
        }
        if self.modifier.mac_cmd == true {
            m = "CMD";
        }
        if self.modifier.shift == true {
            m = "SHIFT";
        }
        format!("{:?} + {:?}", m , self.key).to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KrustyGrabConfig {
    pub dark_mode: bool,
    pub save_folder: PathBuf,
    pub save_format: Format,
    pub screenshot_delay: usize,
    pub myhotkeys: BTreeMap<String, MyHotKey>,
}

impl Default for KrustyGrabConfig {
    fn default() -> Self {  
        let mut myhotkeys = BTreeMap::new();
        let h1 = MyHotKey::new(Modifiers::NONE, Key::Enter); // Key::A);
        let h2 = MyHotKey::new(Modifiers::NONE, Key::Enter); // Key::S);
        let h3 = MyHotKey::new(Modifiers::NONE, Key::Enter); // Key::D);
        myhotkeys.insert("Screen".to_string(), h1);
        myhotkeys.insert("Screen Area".to_string(), h2);
        myhotkeys.insert("Undo".to_string(), h3);
        Self {
            dark_mode: true,
            save_folder: Path::new("~/Desktop").to_path_buf(),
            save_format: Format::Png,
            screenshot_delay: 0,
            myhotkeys,
        }
    }
}

impl KrustyGrabConfig {
    fn _new() -> Self {
        Default::default()
    }
}

///Used to track the current area manipulation.
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

///Used to select the window to be shown
pub enum WindowStatus {
    Main,
    Crop,
}

pub struct KrustyGrab {
    pub config: KrustyGrabConfig,
    pub config_window: bool,
    pub settingkey: bool, 
    pub screen: Option<ColorImage>,
    pub screenshot_requested: bool,
    grab_status: GrabStatus,
    window_status: WindowStatus,
    select: Option<Rect>,
    temp_image: Option<ColorImage>,
    selected_screen: usize,
    // paint: Painting,
}

impl Default for KrustyGrab {
    fn default() -> Self {
        Self {
            config: KrustyGrabConfig::_new(),
            config_window: false,
            settingkey: false,
            screen: None,
            grab_status: GrabStatus::None,
            window_status: WindowStatus::Main,
            select: None,
            temp_image: None,
            selected_screen: 0,
            screenshot_requested: false,
        }
    }
}

impl KrustyGrab {
    pub fn new(ctx: &CreationContext) -> Self {
        // Get current context style
        let mut style = (*ctx.egui_ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(30.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Name("Heading2".into()),
                FontId::new(25.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Name("Context".into()),
                FontId::new(23.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                FontId::new(15.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(10.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();

        // Mutate global style with above changes
        ctx.egui_ctx.set_style(style);

        let config: KrustyGrabConfig = confy::load("krustygrab", None).unwrap_or_default();

        Self {
            config,
            ..Default::default()
        }
    }

    pub fn get_grab_status(&self) -> GrabStatus {
        self.grab_status
    }
    pub fn get_selected_area(&self) -> Option<Rect> {
        self.select
    } 
    pub fn get_temp_image(&self) -> Option<ColorImage> {
        self.temp_image.clone()
    }
    pub fn get_selected_screen(&self) -> usize {
        self.selected_screen
    }

    pub fn set_grab_status(&mut self, new_status: GrabStatus) {
        self.grab_status = new_status;
    }
    pub fn set_window_status(&mut self, new_status: WindowStatus) {
        self.window_status = new_status;
    }
    pub fn set_select_area(&mut self, new_area: Option<Rect>) {
        self.select = new_area;
    }
    pub fn set_temp_image(&mut self, new_image: Option<ColorImage>) {
        self.screen = new_image.clone();
        self.temp_image = new_image.clone();
    }
    pub fn set_definitive_image(&mut self, new_image: Option<ColorImage>) {
        self.screen = new_image.clone();
    }
    pub fn set_selected_screen(&mut self, new_screen: usize){
        self.selected_screen = new_screen;
    }

    pub fn is_window_status_crop(&self) -> bool {
        match self.window_status {
            WindowStatus::Crop => true,
            _ => false
        }
    }

    fn render_config(&mut self, ctx: &Context) {
        Window::new(RichText::new("Configuration").text_style(TextStyle::Body)).show(ctx, |ui| {
            Grid::new("configGrid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Save folder:");
                    // let prev_save = self.config.save_folder.clone();
                    // let mut new_save = String::new();
                    // ui.add(egui::TextEdit::singleline(&mut new_save).hint_text(prev_save));
                    // ui.text_edit_singleline(&mut self.config.save_folder);
                    if Button::image_and_text(icon_img("folder", ctx), ICON_SIZE, "")
                        .ui(ui)
                        .clicked() {
                            if let Some(path) = FileDialog::new()
                                .set_location(&self.config.save_folder)
                                .show_open_single_dir()
                                .expect("Unable to visualize the folder selector") {
                                    self.config.save_folder = path.clone();
                                }
                        }
                    ui.shrink_width_to_current();
                    ui.add_space(180.0);
                    ui.label(self.config.save_folder.to_str().expect("Default folder path should be convertible into str"));
                    ui.add_space(5.0);
                    ui.end_row();

                    // if text_input.lost_focus() && ui.input(|i| {i.key_pressed(egui::Key::Enter)}) {
                    //     if let Err(e) = confy::store("krustygrab", None, KrustyGrabConfig {
                    //         dark_mode: self.config.dark_mode,
                    //         save_folder: self.config.save_folder.to_string(),
                    //         save_format: self.config.save_format.clone(),
                    //     }) {
                    //         tracing::error!("Failed saving app state: {}", e);
                    //     }
                    //     else {
                    //         tracing::error!("App state saved");
                    //     }
                    // }

                    ui.label("Save format:");
                    egui::ComboBox::from_label("Format")
                        .selected_text(
                            RichText::new(format!("{:?}", self.config.save_format))
                                .text_style(TextStyle::Body),
                        )
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            ui.selectable_value(
                                &mut self.config.save_format,
                                Format::Png,
                                RichText::new("Png").text_style(TextStyle::Body),
                            );
                            ui.selectable_value(
                                &mut self.config.save_format,
                                Format::Jpeg,
                                RichText::new("Jpeg").text_style(TextStyle::Body),
                            );
                            ui.selectable_value(
                                &mut self.config.save_format,
                                Format::Gif,
                                RichText::new("Gif").text_style(TextStyle::Body),
                            );
                        });
                    ui.end_row();
                    ui.separator();
                    ui.separator();
                    ui.end_row();
                    
                    //Shortcuts configuration
                    ui.label("Shortcuts:");
                    ui.end_row();

                    //for every possible shortcut
                    for (shortcut_name , mut my_hotkey) in self.config.myhotkeys.clone(){
                        ui.label(shortcut_name.clone() + ": ");

                        let text_edit = TextEdit::singleline(&mut my_hotkey.humanprint()).ui(ui);

                        if text_edit.has_focus(){
                            // Disable the capture of hotkeys
                            self.settingkey = true; 
                            
                            // Get the eventual new hotkey pressed
                            my_hotkey = ctx.input(|i|{
                                tracing::info!("Reading input for {}", shortcut_name ); 
                                if i.keys_down.iter().nth(0).is_some(){
                                    return MyHotKey::new(i.modifiers.clone(), i.keys_down.iter().nth(0).unwrap().clone());
                                }
                                my_hotkey
                            });

                            //Check that this combination is not present in any other hotkey 
                            let my_hotkey_exists = self.config.myhotkeys.values().any(|combo| {
                                combo.key == my_hotkey.key && combo.modifier == my_hotkey.modifier 
                            });

                            if !my_hotkey_exists {
                                tracing::info!("New key for {:?} = {:?}", shortcut_name.clone(), my_hotkey.humanprint());
                                // Save it locally
                                self.config.myhotkeys.insert(shortcut_name.clone(), my_hotkey);
                            }
                            else {
                                // ui.label("This hotkey is already used! Choose a different one");
                                tracing::warn!("This hotkey is already used!");
                            }
                        }
                        if text_edit.lost_focus(){
                            self.settingkey = false; 
                        }
                        ui.end_row();
                    }

                    ui.end_row();
                    ui.separator(); // in the first col
                    // ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui
                            .button(RichText::new("Close").text_style(TextStyle::Body))
                            .clicked()
                        {
                            self.config = confy::load("krustygrab", None).unwrap_or_default();
                            self.config_window = false;
                        } else if ui
                            .button(RichText::new("Apply").text_style(TextStyle::Body))
                            .clicked()
                        {
                            if let Err(e) = confy::store(
                                "krustygrab",
                                None,
                                self.config.clone(),
                            ) {
                                println!("{:?}", self.config);
                                tracing::error!("Failed saving app state: {}", e);
                            } else {
                                tracing::info!("App state saved");
                            }
                            self.config_window = false;
                        }
                    });
                    // });
                    ui.end_row();

                    // tracing::error!("{}", &self.config.save_folder.to_str().unwrap()); //log
                    // tracing::error!("{}", &self.config.save_folder.to_str().unwrap()); //log
                    // tracing::error!("{:?}", &self.config.save_format); //log
                });
  
        });
    }
}

impl App for KrustyGrab {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // let i = Instant::now();      //TODO rimuovere dalla release
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.config_window {
            self.render_config(ctx);
        }

        //Take the screenshot before turning on the visibility of the window
        if self.screenshot_requested {
            self.screenshot_requested = false;
            self.set_screenshot(ctx);
            frame.set_visible(true);
        }

        match self.window_status {
            WindowStatus::Main => self.main_window(ctx, frame),
            WindowStatus::Crop => self.crop_screen_window(ctx, frame),
        }
    
        //Handler for majour shortcuts
        if !self.settingkey {
            if let Some(hk) = ctx.input_mut(|x| {
                for s in self.config.myhotkeys.iter().filter(|x|x.1.key.is_some()).clone() {
                    let sh = KeyboardShortcut::new(s.1.modifier, s.1.key.unwrap());
                    if x.consume_shortcut(&sh){
                        return Some(s.clone());
                    }
                }
                return None;
            }){
                tracing::info!("Shortcut pressed: {:?}", hk.0);
                match hk.0.as_str() {
                    "Undo" => tracing::info!("Still not supported"),
                    "Screen" => {
                        self.screenshot_requested = true;
                    }
                    "Screen Area" => {
                        self.set_window_status(self::WindowStatus::Crop);
                        self.screenshot_requested = true;
                    },
                    _ => tracing::error!("Unknown shortcut pressed")
                }
            }
        }

        // Performance debug -> frame generation time
        // print!("\r{:?}      ", i.elapsed());
        // std::io::stdout().flush();
    }
}
