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

    fonts.families.insert(
        egui::FontFamily::Name("iconfont".into()),
        vec!["Hack".to_owned(), "iconfont".into()],
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .push("iconfont".to_owned());
    
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("iconfont".to_owned());

    ctx.set_fonts(fonts);
}