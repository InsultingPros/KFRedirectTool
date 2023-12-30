// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use eframe::egui::{self, FontFamily::Proportional, FontId, TextStyle};

/// Set `font` sizes.
///
/// Reference: <https://github.com/emilk/egui/discussions/1478#discussioncomment-2549924>
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

/// Load app icon.
///
/// Reference: <https://github.com/emilk/egui/discussions/1574#discussioncomment-5840144>
///
/// Icon: <https://icons8.com/icon/v6bWcVuiEKvy/box>
/// # Panics
///
/// Will panic if included icon is not found
#[must_use]
pub fn load_icon() -> Option<eframe::IconData> {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("..//static//icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Some(eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}
