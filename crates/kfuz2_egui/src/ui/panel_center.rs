use crate::constants;
use std::sync::atomic::Ordering;

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
                    ui_app.input_dir = rfd::FileDialog::new().pick_folder();
                }
                // label
                if let Some(input) = &ui_app.input_dir {
                    if input.exists() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(input.display().to_string())
                                    .monospace()
                                    .color(crate::constants::LABEL_COLOR_TEXT),
                            )
                            .truncate(true),
                        );
                    } else {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Select input folder!")
                                    .monospace()
                                    .color(crate::constants::LABEL_COLOR_EMPTY),
                            )
                            .truncate(true),
                        );
                    }
                } else {
                    // yay copy-paste
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Select input folder!")
                                .monospace()
                                .color(crate::constants::LABEL_COLOR_EMPTY),
                        )
                        .truncate(true),
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
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(input.display().to_string())
                                    .monospace()
                                    .color(crate::constants::LABEL_COLOR_TEXT),
                            )
                            .truncate(true),
                        );
                    } else {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Select output folder!")
                                    .monospace()
                                    .color(crate::constants::LABEL_COLOR_EMPTY),
                            )
                            .truncate(true),
                        );
                    }
                } else {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Select output folder!")
                                .monospace()
                                .color(crate::constants::LABEL_COLOR_EMPTY),
                        )
                        .truncate(true),
                    );
                }
            });

        ui.add_space(constants::PADDING_MEDIUM);
        ui.separator();
        ui.add_space(constants::PADDING_MEDIUM);

        ui.horizontal(|ui| {
            ui.label("Ignore KF1 files")
                .on_hover_text("Enable if you want to ignore KF1 core files");
            ui.add(super::toggle_switch::toggle(&mut ui_app.ignore_kf_files));

            ui.add_space(30f32);

            ui.label("Disable Multi Threading").on_hover_text(
                "Enable this if you process files on a slow hdd.\nLeave disabled if you use an ssd",
            );
            ui.add(super::toggle_switch::toggle(
                &mut ui_app.disable_multi_threading,
            ));

            ui.add_space(30f32);

            ui.label("Log level")
                .on_hover_text("Select how much info you want to see in logs");
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
                        kfuz2_lib::types::LogLevel::Minimal,
                        "Minimal",
                    )
                    .on_hover_text("Show the bare minimum");
                });
        });

        ui.add_space(constants::PADDING_MEDIUM);

        ui.horizontal(|ui| {
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

        ui.add_space(constants::PADDING_MEDIUM);
        ui.separator();
        ui.add_space(constants::PADDING_MEDIUM);

        ui.horizontal(|ui| {
            ui.label("Progress: ");
            // `cache` atomics
            let (success, fail, ignore, total) = (
                ui_app.pbar.file_num_success.load(Ordering::Acquire),
                ui_app.pbar.file_num_failed.load(Ordering::Acquire),
                ui_app.pbar.file_num_ignored.load(Ordering::Acquire),
                ui_app.pbar.file_num_total.load(Ordering::Acquire),
            );
            ui_app.pbar.animate = success + fail + ignore != total;
            let progress: f32 = (success + ignore) as f32 / total as f32;
            let progress_bar = egui::ProgressBar::new(progress)
                .show_percentage()
                .animate(ui_app.pbar.animate);

            let color = if ui_app.pbar.animated_once.is_some_and(|inner| inner) {
                ui.style().visuals.selection.bg_fill
            } else {
                egui::Color32::from_rgb(60, 60, 60)
            };

            ui.add(progress_bar.fill(color));
        });
    });
}
