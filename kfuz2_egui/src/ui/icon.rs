/// Load app icon.
///
/// Reference: https://github.com/emilk/egui/discussions/1574#discussioncomment-5840144
///
/// Icon: https://icons8.com/icon/v6bWcVuiEKvy/box
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
