use crate::constants;
use poll_promise::Promise;
use std::path::PathBuf;

/// Render `bottom` panel of UI.
pub fn render_panel(
    ui_app: &mut super::app::MyApp,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
) {
    egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
        ui.add_space(constants::PADDING_BIG);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.add_space(15f32);

            let empty_path = &PathBuf::new();
            let (enable_button, destination) = match &ui_app.output_dir {
                Some(value) => (value.is_dir(), value),
                None => (false, empty_path),
            };

            if ui
                .add_enabled(
                    enable_button,
                    egui::Button::new("Open Output").min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                )
                .on_disabled_hover_text("Select any output directory to activate this button")
                .clicked()
            {
                open_file_explorer(destination);
            }

            ui.add_space(100f32);

            let enable_button = match &ui_app.input_dir {
                Some(value) => value.is_dir(),
                None => false,
            };

            if ui
                .add_enabled(
                    enable_button,
                    egui::Button::new("Compress").min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                )
                .on_disabled_hover_text("Select any input folder to proceed")
                .clicked()
            {
                ui_app.pbar.animated_once = Some(true);
                let cp_ui_app = ui_app.clone();
                // we only use promise for non blocking behavior
                let _ = Promise::spawn_thread("slow_compression", move || {
                    crate::logic::start_compression(&cp_ui_app)
                });
            }

            ui.add_space(15f32);

            if ui
                .add_enabled(
                    enable_button,
                    egui::Button::new("Decompress").min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                )
                .on_disabled_hover_text("Select any input folder to proceed")
                .clicked()
            {
                ui_app.pbar.animated_once = Some(true);
                let cp_ui_app = ui_app.clone();
                // we only use promise for non blocking behavior
                let _ = Promise::spawn_thread("slow_decompression", move || {
                    crate::logic::start_decompression(&cp_ui_app)
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
        .map_err(|e| println!("ops! Error {}", e));
}

#[cfg(target_os = "linux")]
pub fn open_file_explorer(destination: &PathBuf) {
    let _ = std::process::Command::new("xdg-open")
        .arg(destination)
        .spawn()
        .map_err(|e| println!("ops! Error {}", e));
}

#[cfg(target_os = "macos")]
pub fn open_file_explorer(destination: &PathBuf) {
    let _ = std::process::Command::new("open")
        .arg("--")
        .arg(destination)
        .spawn()
        .map_err(|e| println!("ops! Error {}", e));
}
