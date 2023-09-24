use egui::{FontFamily::Proportional, FontId, TextStyle};

/// Set `font` sizes.
///
/// Reference: https://github.com/emilk/egui/discussions/1478#discussioncomment-2549924
pub fn edit_fonts(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Body, FontId::new(13.0, Proportional)),
        (TextStyle::Button, FontId::new(13.0, Proportional)),
        (TextStyle::Heading, FontId::new(20.0, Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, Proportional)),
        (TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();

    ctx.set_style(style);
}
