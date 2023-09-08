// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// to check: https://github.com/emilk/egui/tree/master/examples/file_dialog

fn main() -> Result<(), eframe::Error> {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let options: eframe::NativeOptions = eframe::NativeOptions {
        icon_data: kfuz2_egui::ui::icon::load_icon(),
        // drag_and_drop_support: true,
        initial_window_size: kfuz2_egui::constants::WINDOW_SIZE,
        max_window_size: kfuz2_egui::constants::WINDOW_SIZE,
        min_window_size: kfuz2_egui::constants::WINDOW_SIZE,
        follow_system_theme: true,
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        kfuz2_egui::constants::APP_NAME,
        options,
        Box::new(|cc| Box::new(kfuz2_egui::ui::app::MyApp::new(cc))),
    )
}
