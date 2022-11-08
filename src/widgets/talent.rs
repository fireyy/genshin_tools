use crate::images::NetworkImages;
use crate::types::Talent;
use eframe::egui::{style::Margin, Color32, Frame, Rounding, Ui};

pub struct TalentCard;

impl TalentCard {
    pub fn show(ui: &mut Ui, data: Talent, _images: &NetworkImages) {
        Frame {
            rounding: Rounding::same(10.0),
            inner_margin: Margin::same(10.0),
            fill: Color32::from_gray(50),
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Level:");
                    ui.label(data.level.to_string());
                });
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.label(data.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Unlock:");
                    ui.label(data.unlock);
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label("Description:");
                    ui.label(data.description);
                });
            });
        });
    }
}
