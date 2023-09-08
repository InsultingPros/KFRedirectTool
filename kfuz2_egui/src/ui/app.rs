use crate::constants;
use eframe::egui;
use kfuz2_lib::types::LogLevel;
use std::path::PathBuf;

// #[derive(serde::Deserialize, serde::Serialize, Default, Debug, PartialEq)]
// pub enum CLILogLevel {
//     Silent,
//     Verbose,
//     #[default]
//     Essential,
// }
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(remote = "LogLevel")]
pub enum LogLevelDef {
    Verbose,
    Default,
    Silent,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
///
/// Reference: https://github.com/emilk/eframe_template/blob/master/src/app.rs
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    pub disable_kf_check: bool,
    pub disable_multi_threading: bool,
    pub extension_list: String,
    pub input_dir: Option<PathBuf>,
    #[serde(with = "LogLevelDef")]
    pub log_level: LogLevel,
    pub output_dir: Option<PathBuf>,
    pub text_edit_extensions: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            disable_kf_check: false,
            disable_multi_threading: false,
            extension_list: constants::DEFAULT_EXTENSIONS.join(", "),
            input_dir: Some(PathBuf::new()),
            log_level: LogLevel::default(),
            output_dir: Some(PathBuf::new()),
            text_edit_extensions: constants::DEFAULT_EXTENSIONS.join(", "),
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        super::theme::edit_fonts(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        super::panel_top::render_panel(self, ctx, _frame);
        super::panel_bottom::render_panel(self, ctx, _frame);
        // N.B. `center` must always be at the end!
        super::panel_center::render_panel(self, ctx, _frame);
    }
}
