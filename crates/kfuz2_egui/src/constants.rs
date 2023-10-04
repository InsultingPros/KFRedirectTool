pub const APP_NAME: &str = "KFUZ2 EGUI";

/// Initial size of windows: [`initial_window_size`](https://docs.rs/eframe/latest/eframe/struct.NativeOptions.html#structfield.initial_window_size)
pub const WINDOW_SIZE: Option<eframe::emath::Vec2> = Some(eframe::epaint::vec2(650.0, 290.0));

/// Default KF1 file extensions.
pub const DEFAULT_EXTENSIONS: [&str; 7] = ["u", "utx", "usx", "ukx", "uax", "rom", "uz2"];
/// Char limit for `TextEdit`.
pub const CHAR_LIMIT: usize = 50;

pub const BUTTON_SIZE_MEDIUM: eframe::emath::Vec2 = eframe::epaint::vec2(120.0, 15.0);
pub const BUTTON_SIZE_SMALL: eframe::emath::Vec2 = eframe::epaint::vec2(40.0, 15.0);

pub const LABEL_COLOR_EMPTY: egui::Color32 = egui::Color32::from_rgb(245, 66, 96);
pub const LABEL_COLOR_TEXT: egui::Color32 = egui::Color32::from_rgb(55, 200, 70);

pub const EXTENSION_COLOR: egui::Color32 = egui::Color32::from_rgb(42, 157, 143);

pub const PADDING_BIG: f32 = 15f32;
pub const PADDING_MEDIUM: f32 = 10f32;
pub const PADDING_SMALL: f32 = 5f32;

pub const STEAM_ICO: char = '';
