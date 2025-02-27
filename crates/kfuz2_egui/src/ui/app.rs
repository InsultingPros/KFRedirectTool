// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::constants;
use eframe::egui;
use kfuz2_lib::types::LogLevel;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU64, Ordering},
        Arc,
    },
};

/// Link to lib's `LogLevel`
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(remote = "LogLevel")]
pub enum LogLevelDef {
    Verbose,
    Default,
    Minimal,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
///
/// Reference: <https://github.com/emilk/eframe_template/blob/master/src/app.rs>
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Kfuz2Egui {
    pub input_dir: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    /// Skip vanilla kf1 files.
    pub ignore_kf_files: bool,
    /// Single / multi- thread switch.
    pub disable_multi_threading: bool,
    /// How much info to show in logs / console.
    #[serde(with = "LogLevelDef")]
    pub log_level: LogLevel,
    /// Extension list used in file filtering.
    pub extension_list: String,
    /// Variable that accepts input from `TextEdit` field.
    pub text_edit_extensions: String,
    #[serde(skip)]
    pub pbar: ProgressBarStuff,
    #[serde(skip)]
    pub cancel_processing: Arc<AtomicBool>,
}

// progress bar related
#[derive(Debug, Clone)]
pub struct ProgressBarStuff {
    pub animate: bool,
    pub animated_once: Option<bool>,
    /// Total number of processed files.
    pub file_num_total: Arc<AtomicU16>,
    /// Successfuly processed files number.
    pub file_num_success: Arc<AtomicU16>,
    /// Failed files number.
    pub file_num_failed: Arc<AtomicU16>,
    /// Ignored files number.
    pub file_num_ignored: Arc<AtomicU16>,
    /// Time elapsed for current operation, as `seconds` + `milliseconds`
    pub time_elapsed: Arc<(AtomicU64, AtomicU32)>,
}

impl ProgressBarStuff {
    pub fn reset(&self) {
        self.file_num_success.swap(0u16, Ordering::Relaxed);
        self.file_num_failed.swap(0u16, Ordering::Relaxed);
        self.file_num_ignored.swap(0u16, Ordering::Relaxed);
        self.time_elapsed.0.swap(0u64, Ordering::Relaxed);
        self.time_elapsed.1.swap(0u32, Ordering::Relaxed);
    }
}

impl Default for ProgressBarStuff {
    fn default() -> Self {
        Self {
            animate: false,
            animated_once: Some(false),
            file_num_total: Arc::new(AtomicU16::new(0u16)),
            file_num_success: Arc::new(AtomicU16::new(0u16)),
            file_num_failed: Arc::new(AtomicU16::new(0u16)),
            file_num_ignored: Arc::new(AtomicU16::new(0u16)),
            time_elapsed: Arc::new((AtomicU64::new(0u64), AtomicU32::new(0u32))),
        }
    }
}

impl Default for Kfuz2Egui {
    fn default() -> Self {
        Self {
            input_dir: Some(PathBuf::new()),
            output_dir: Some(PathBuf::new()),
            ignore_kf_files: true,
            disable_multi_threading: false,
            log_level: LogLevel::default(),
            extension_list: constants::DEFAULT_EXTENSIONS.join(", "),
            text_edit_extensions: constants::DEFAULT_EXTENSIONS.join(", "),
            pbar: ProgressBarStuff::default(),
            cancel_processing: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Kfuz2Egui {
    /// Called once before the first frame.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        super::theme::edit_fonts(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for Kfuz2Egui {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        super::panel_top::render_panel(self, ctx, frame);
        super::panel_bottom::render_panel(self, ctx);
        // N.B. `center` must always be at the end!
        super::panel_center::render_panel(self, ctx);
    }
}
