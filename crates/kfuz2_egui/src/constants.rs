// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

pub const MAIN_VIEWPORT_NAME: &str = "KFUZ2";
pub const LOGS_VIEWPORT_NAME: &str = "KFUZ2 Logs";

pub const MAIN_VIEWPORT_SIZE: [f32; 2] = [650.0, 290.0];
pub const LOGS_VIEWPORT_SIZE: [f32; 2] = [640.0, 480.0];

/// Default KF1 file extensions.
pub const DEFAULT_EXTENSIONS: [&str; 7] = ["u", "utx", "usx", "ukx", "uax", "rom", "uz2"];
/// Char limit for `TextEdit`.
pub const CHAR_LIMIT: usize = 50;

pub const BUTTON_SIZE_MEDIUM: eframe::egui::Vec2 = eframe::egui::vec2(120.0, 15.0);
pub const BUTTON_SIZE_SMALL: eframe::egui::Vec2 = eframe::egui::vec2(40.0, 15.0);

pub const LABEL_COLOR_EMPTY: eframe::egui::Color32 = eframe::egui::Color32::from_rgb(245, 66, 96);
pub const LABEL_COLOR_TEXT: eframe::egui::Color32 = eframe::egui::Color32::from_rgb(55, 200, 70);

pub const EXTENSION_COLOR: eframe::egui::Color32 = eframe::egui::Color32::from_rgb(42, 157, 143);

pub const PADDING_BIG: f32 = 15f32;
pub const PADDING_MEDIUM: f32 = 10f32;
pub const PADDING_SMALL: f32 = 5f32;

pub const STEAM_ICO: char = '';
