mod icons;
mod style;

pub use icons::{Icon, ICON_FONT};
pub use style::Style;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "iconfont".to_owned(),
        egui::FontData::from_static(ICON_FONT),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "iconfont".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("iconfont".to_owned());

    ctx.set_fonts(fonts);
}