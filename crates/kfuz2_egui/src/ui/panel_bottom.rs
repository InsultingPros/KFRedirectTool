// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::constants;
use eframe::egui;
use poll_promise::Promise;
use std::{path::PathBuf, sync::atomic::Ordering};

const DISABLED_MSG: &str = "Select any output directory to activate this button";

/// Render `bottom` panel of UI.
pub fn render_panel(gui_app: &mut super::app::Kfuz2Egui, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
        ui.add_space(constants::PADDING_BIG);

        ui.horizontal(|ui| {
            let empty_path = &PathBuf::new();
            let output_destination = gui_app
                .output_dir
                .as_ref()
                .map_or(empty_path, |value| value);

            let output_selected: bool = gui_app
                .output_dir
                .as_ref()
                .is_some_and(|value| value.is_dir());
            let input_selected: bool = gui_app
                .input_dir
                .as_ref()
                .is_some_and(|value| value.is_dir());

            if ui
                .add_enabled(
                    output_selected,
                    egui::Button::new("Open Output").min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                )
                .on_disabled_hover_text(DISABLED_MSG)
                .clicked()
            {
                open_file_explorer(output_destination);
            }

            ui.separator();

            if ui
                .add(egui::Button::new("Cancel").min_size(crate::constants::BUTTON_SIZE_MEDIUM))
                .on_hover_text("You can only cancel active file processing.")
                .clicked()
            {
                gui_app.cancel_processing.swap(true, Ordering::Relaxed);
            }

            ui.separator();
            ui.add_space(70f32);
            ui.separator();

            if ui
                .add_enabled(
                    input_selected && output_selected,
                    egui::Button::new("Compress").min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                )
                .on_disabled_hover_text(DISABLED_MSG)
                .clicked()
            {
                gui_app.pbar.animated_once = Some(true);
                gui_app.cancel_processing.swap(false, Ordering::Relaxed);

                let cp_ui_app = gui_app.clone();
                // we only use promise for non blocking behavior
                let _ = Promise::spawn_thread("slow_compression", move || {
                    crate::logic::start_compression(&cp_ui_app);
                });
            }

            ui.add_space(15f32);

            if ui
                .add_enabled(
                    input_selected && output_selected,
                    egui::Button::new("Decompress").min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                )
                .on_disabled_hover_text(DISABLED_MSG)
                .clicked()
            {
                gui_app.pbar.animated_once = Some(true);
                gui_app.cancel_processing.swap(false, Ordering::Relaxed);

                let cp_ui_app = gui_app.clone();
                // we only use promise for non blocking behavior
                let _ = Promise::spawn_thread("slow_decompression", move || {
                    crate::logic::start_decompression(&cp_ui_app);
                });
            }
        });

        ui.add_space(constants::PADDING_BIG);
    });
}

#[cfg(target_os = "windows")]
fn open_file_explorer(destination: &PathBuf) {
    let _ = std::process::Command::new("explorer")
        .arg(destination)
        .spawn()
        .map_err(|e| println!("ops! Error {e}"));
}

#[cfg(target_os = "linux")]
pub fn open_file_explorer(destination: &PathBuf) {
    let _ = std::process::Command::new("xdg-open")
        .arg(destination)
        .spawn()
        .map_err(|e| println!("ops! Error {e}"));
}

#[cfg(target_os = "macos")]
pub fn open_file_explorer(destination: &PathBuf) {
    let _ = std::process::Command::new("open")
        .arg("--")
        .arg(destination)
        .spawn()
        .map_err(|e| println!("ops! Error {e}"));
}
