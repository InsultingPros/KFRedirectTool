use crate::constants;

/// Render `center` panel of UI.
pub fn render_panel(
    ui_app: &mut super::app::MyApp,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(constants::PADDING_MEDIUM);

        egui::Grid::new("input_grid")
            .num_columns(2)
            .min_col_width(80f32)
            .spacing([15f32, 10f32])
            .striped(false)
            .show(ui, |ui| {
                if ui
                    .add(
                        egui::Button::new("Input directory")
                            .min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                    )
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        if path.exists() {
                            ui_app.input_dir = Some(path);
                        }
                    }
                }
                // label
                if let Some(input) = &ui_app.input_dir {
                    if input.exists() {
                        ui.monospace(
                            egui::RichText::new(input.display().to_string())
                                .color(crate::constants::LABEL_COLOR_TEXT),
                        );
                    } else {
                        ui.monospace(
                            egui::RichText::new("Select input folder!")
                                .color(crate::constants::LABEL_COLOR_EMPTY),
                        );
                    }
                } else {
                    ui.monospace(
                        egui::RichText::new("Select input folder!")
                            .color(crate::constants::LABEL_COLOR_EMPTY),
                    );
                }
                ui.end_row();

                if ui
                    .add(
                        egui::Button::new("Output directory")
                            .min_size(crate::constants::BUTTON_SIZE_MEDIUM),
                    )
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        if path.exists() {
                            ui_app.output_dir = Some(path);
                        }
                    }
                }
                // label
                if let Some(input) = &ui_app.output_dir {
                    if input.exists() {
                        ui.monospace(
                            egui::RichText::new(input.display().to_string())
                                .color(crate::constants::LABEL_COLOR_TEXT),
                        );
                    } else {
                        ui.monospace(
                            egui::RichText::new("Select output folder!")
                                .color(crate::constants::LABEL_COLOR_EMPTY),
                        );
                    }
                } else {
                    ui.monospace(
                        egui::RichText::new("Select output folder!")
                            .color(crate::constants::LABEL_COLOR_EMPTY),
                    );
                }
            });

        ui.add_space(constants::PADDING_MEDIUM);
        ui.separator();
        ui.add_space(constants::PADDING_MEDIUM);

        ui.horizontal(|ui| {
            ui.label("Disable KF Checks").on_hover_text(
                "Enable if you want to ignore KF1 core files / compress any UE2 based game files.",
            );
            ui.add(super::toggle_switch::toggle(&mut ui_app.disable_kf_check));

            ui.add_space(30f32);

            ui.label("Disable Multi Threading").on_hover_text(
                "Enable this if you process files on a slow hard drive.\nLeave disabled if you use an ssd.",
            );
            ui.add(super::toggle_switch::toggle(
                &mut ui_app.disable_multi_threading,
            ));

            ui.add_space(30f32);

            ui.label("Log level")
                .on_hover_text("Select log level for KFUZ2 CLI");
            egui::ComboBox::from_id_source(0)
                .selected_text(format!("{:?}", ui_app.log_level))
                .show_ui(ui, |ui| {
                    ui.set_min_width(60.0);

                    ui.selectable_value(
                        &mut ui_app.log_level,
                        kfuz2_lib::types::LogLevel::Verbose,
                        "Verbose",
                    )
                    .on_hover_text("Show additional log messages, with lots of details");
                    ui.selectable_value(
                        &mut ui_app.log_level,
                        kfuz2_lib::types::LogLevel::Default,
                        "Default",
                    )
                    .on_hover_text("Show only essential log messages");
                    ui.selectable_value(
                        &mut ui_app.log_level,
                        kfuz2_lib::types::LogLevel::Silent,
                        "Silent",
                    )
                    .on_hover_text("Show the bare minimum");
                });
        });

        ui.add_space(constants::PADDING_MEDIUM);

        ui.horizontal_wrapped(|ui| {
            ui.label("Extension List")
                .on_hover_text("Extension list to filter input files");

            let extension_response: egui::Response = ui.add(
                egui::TextEdit::singleline(&mut ui_app.text_edit_extensions)
                    .hint_text("Add at least one file extension!")
                    .char_limit(crate::constants::CHAR_LIMIT)
                    .text_color(constants::EXTENSION_COLOR),
            );

            if ui
                .add(egui::Button::new("Save").min_size(crate::constants::BUTTON_SIZE_SMALL))
                .clicked()
            {
                ui_app.extension_list = ui_app.text_edit_extensions.to_owned();
            }
            if ui
                .add(egui::Button::new("Reset").min_size(crate::constants::BUTTON_SIZE_SMALL))
                .clicked()
            {
                ui_app.text_edit_extensions = crate::constants::DEFAULT_EXTENSIONS.join(", ");
                ui_app.extension_list = crate::constants::DEFAULT_EXTENSIONS.join(", ");
            }

            // if we changed something but did not save -> revert everything
            if extension_response.lost_focus() {
                ui_app.text_edit_extensions = ui_app.extension_list.to_owned();
            }
        });
    });
}