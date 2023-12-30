// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> Result<(), eframe::Error> {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(kfuz2_egui::constants::WINDOW_SIZE)
            .with_max_inner_size(kfuz2_egui::constants::WINDOW_SIZE)
            .with_min_inner_size(kfuz2_egui::constants::WINDOW_SIZE)
            .with_resizable(false)
            .with_icon(kfuz2_egui::ui::theme::load_icon()),
        follow_system_theme: true,
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        kfuz2_egui::constants::APP_NAME,
        options,
        Box::new(|cc| Box::new(kfuz2_egui::ui::app::Kfuz2Egui::new(cc))),
    )
}
