use crate::constants;

/// Render `top` panel of UI.
pub fn render_panel(
    ui_app: &mut super::app::MyApp,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
) {
    egui::TopBottomPanel::top("top").show(ctx, |ui| {
        // menu bar
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    _frame.close();
                }
            });
            ui.add_space(constants::PADDING_SMALL);

            ui.menu_button("Advanced", |ui| {
                // reset app state on request
                if ui
                    .button("Reset preferences")
                    .on_hover_text("Reset all widgets, input-output fields to default")
                    .clicked()
                {
                    *ui_app = super::app::MyApp::default();
                    ui.close_menu();
                }
            });
            ui.add_space(constants::PADDING_SMALL);

            ui.menu_button("Help", |ui| {
                if ui
                    .hyperlink_to(
                        format!("{} What is a redirect?", crate::constants::STEAM_ICO),
                        "https://steamcommunity.com/sharedfiles/filedetails/?id=1522731903",
                    )
                    .clicked()
                {
                    ui.close_menu();
                };

                ui.separator();

                if ui
                    .hyperlink_to(
                        format!("{} KFUZ2 on GitHub", egui::special_emojis::GITHUB),
                        "https://github.com/InsultingPros/KFRedirectTool",
                    )
                    .clicked()
                {
                    ui.close_menu();
                };
            });
        });
    });
}