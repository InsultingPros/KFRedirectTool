use crate::constants;
use eframe::egui;
use std::sync::atomic::Ordering;

/// # Panics
/// fixme
#[inline]
pub fn render_viewport(gui_app: &mut super::app::Kfuz2Egui, ctx: &egui::Context) {
    let show_logs_viewport = gui_app.vars.runtime_vars.show_logs_viewport.clone();

    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of(constants::LOGS_VIEWPORT_NAME),
        egui::ViewportBuilder::default()
            .with_title(constants::LOGS_VIEWPORT_NAME)
            .with_inner_size(constants::LOGS_VIEWPORT_SIZE),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Hello from deferred viewport");
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                show_logs_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
