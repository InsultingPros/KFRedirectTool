// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::constants;
use eframe::egui;
use std::sync::atomic::Ordering;

/// Render `center` panel of UI.
#[inline]
pub fn render_panel(gui_app: &mut super::app::Kfuz2Egui, ctx: &eframe::egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(constants::PADDING_MEDIUM);

        render_input_fields(ui, gui_app);

        ui.add_space(constants::PADDING_MEDIUM);
        ui.separator();
        ui.add_space(constants::PADDING_MEDIUM);

        render_settings(ui, gui_app);

        ui.add_space(constants::PADDING_MEDIUM);
        ui.separator();
        ui.add_space(constants::PADDING_MEDIUM);

        render_progress(ui, gui_app);
    });
}

#[inline]
fn render_input_fields(ui: &mut egui::Ui, gui_app: &mut super::app::Kfuz2Egui) {
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
                gui_app.vars.persisted_vars.paths.input_dir = rfd::FileDialog::new().pick_folder();
            }
            // label
            if let Some(input) = &gui_app.vars.persisted_vars.paths.input_dir {
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
                        gui_app.vars.persisted_vars.paths.output_dir = Some(path);
                    }
                }
            }
            // label
            if let Some(input) = &gui_app.vars.persisted_vars.paths.output_dir {
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
}

#[inline]
fn render_settings(ui: &mut egui::Ui, gui_app: &mut super::app::Kfuz2Egui) {
    ui.horizontal(|ui| {
        ui.label("Ignore KF1 files")
            .on_hover_text("Enable if you want to ignore KF1 core files");
        ui.add(super::toggle_switch::toggle(
            &mut gui_app.vars.persisted_vars.ignore_kf_files,
        ));

        ui.add_space(30f32);

        ui.label("Disable Multi Threading").on_hover_text(
            "Enable this if you process files on a slow hdd.\nLeave disabled if you use an ssd",
        );
        ui.add(super::toggle_switch::toggle(
            &mut gui_app.vars.persisted_vars.disable_multi_threading,
        ));

        ui.add_space(30f32);

        ui.label("Log level")
            .on_hover_text("Select how much info you want to see in logs");
        egui::ComboBox::from_id_source(0)
            .selected_text(format!("{:?}", gui_app.vars.persisted_vars.log_level))
            .show_ui(ui, |ui| {
                ui.set_min_width(60.0);

                ui.selectable_value(
                    &mut gui_app.vars.persisted_vars.log_level,
                    kfuz2_lib::types::LogLevel::Verbose,
                    "Verbose",
                )
                .on_hover_text("Show additional log messages, with lots of details");
                ui.selectable_value(
                    &mut gui_app.vars.persisted_vars.log_level,
                    kfuz2_lib::types::LogLevel::Default,
                    "Default",
                )
                .on_hover_text("Show only essential log messages");
                ui.selectable_value(
                    &mut gui_app.vars.persisted_vars.log_level,
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
            egui::TextEdit::singleline(&mut gui_app.vars.persisted_vars.extensions.text_edit_extensions)
                .hint_text("Add at least one file extension!")
                .char_limit(crate::constants::CHAR_LIMIT)
                .text_color(constants::EXTENSION_COLOR),
        );

        if ui
            .add(egui::Button::new("Save").min_size(crate::constants::BUTTON_SIZE_SMALL))
            .clicked()
        {
            gui_app.vars.persisted_vars.extensions.extension_list = gui_app
                .vars.persisted_vars
                .extensions
                .text_edit_extensions
                .clone();
        }
        if ui
            .add(egui::Button::new("Reset").min_size(crate::constants::BUTTON_SIZE_SMALL))
            .clicked()
        {
            gui_app.vars.persisted_vars.extensions.reset();
        }

        // if we changed something but did not save -> revert everything
        if extension_response.lost_focus() {
            gui_app.vars.persisted_vars.extensions.text_edit_extensions =
                gui_app.vars.persisted_vars.extensions.extension_list.clone();
        }
    });
}

#[inline]
fn render_progress(ui: &mut egui::Ui, gui_app: &mut super::app::Kfuz2Egui) {
    ui.horizontal(|ui| {
        ui.label("Progress: ");
        // `cache` atomics
        let (success, fail, ignore, total) = (
            gui_app
                .vars.runtime_vars
                .pbar
                .file_num_success
                .load(Ordering::Relaxed),
            gui_app
                .vars.runtime_vars
                .pbar
                .file_num_failed
                .load(Ordering::Relaxed),
            gui_app
                .vars.runtime_vars
                .pbar
                .file_num_ignored
                .load(Ordering::Relaxed),
            gui_app
                .vars.runtime_vars
                .pbar
                .file_num_total
                .load(Ordering::Relaxed),
        );
        gui_app.vars.runtime_vars.pbar.animate = success + fail + ignore != total;
        let mut progress: f32 = f32::from(success + ignore) / f32::from(total);
        if progress.is_nan() {
            progress = 0f32;
        }
        let progress_bar = egui::ProgressBar::new(progress)
            .text(format!(
                "{:.1}% Time elapsed: {}.{} seconds.",
                progress * 100f32,
                gui_app
                    .vars.runtime_vars
                    .pbar
                    .time_elapsed
                    .0
                    .load(Ordering::Relaxed),
                gui_app
                    .vars.runtime_vars
                    .pbar
                    .time_elapsed
                    .1
                    .load(Ordering::Relaxed)
            ))
            .desired_width(421f32)
            .animate(gui_app.vars.runtime_vars.pbar.animate);

        let color = if gui_app
            .vars.runtime_vars
            .pbar
            .animated_once
            .is_some_and(|inner| inner)
        {
            ui.style().visuals.selection.bg_fill
        } else {
            egui::Color32::from_rgb(60, 60, 60)
        };

        ui.add(progress_bar.fill(color));
        ui.add_space(constants::PADDING_BIG);

        if ui
            .add(egui::Button::new("Show Logs").min_size(crate::constants::BUTTON_SIZE_MEDIUM))
            .clicked()
        {
            gui_app
                .vars.runtime_vars
                .show_logs_viewport
                .swap(true, Ordering::Relaxed);
        }
    });
}
