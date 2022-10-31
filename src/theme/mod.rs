use egui::{Color32, TextFormat, Align};
use eframe::epaint::text::{LayoutJob, TextWrapping};

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

pub fn text_ellipsis(name: &str, text_color: Color32, max_rows: usize) -> LayoutJob {
    let mut job = LayoutJob::single_section(name.to_string(), TextFormat {
        color: text_color,
    
        valign: Align::Center,
        ..TextFormat::default()
    });
    
    job.wrap = TextWrapping {
        max_rows,
        break_anywhere: true,
        overflow_character: Some('…'),
        ..TextWrapping::default()
    };
    
    job
}