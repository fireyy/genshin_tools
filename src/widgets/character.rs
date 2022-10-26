use eframe::egui::{
    Ui, Frame, vec2,
};
use crate::images::NetworkImages;
use crate::types::Character;
use crate::util::{gen_star, show_vision};
use crate::custom_grid;
use super::TalentCard;

pub struct CharacterCard;

impl CharacterCard {
    pub fn show(ui: &mut Ui, data: Character, images: &NetworkImages) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                custom_grid!(
                    ui,
                    ("Name", &data.name),
                    ("Title", &data.title),
                    ("Nation", &data.nation),
                    ("Birthday", &data.birthday),
                    ("Rarity", gen_star(data.rarity)),
                    ("Vision", format!("{} {}", show_vision(&data.vision_key), &data.vision)),
                    ("Weapon", &data.weapon),
                    ("Affiliation", &data.affiliation),
                    ("Constellation", &data.constellation)
                );
    
                ui.with_layout(egui::Layout::from_main_dir_and_cross_align(egui::Direction::TopDown, egui::Align::RIGHT), |ui| {
                    Frame {
                        ..Frame::default()
                    }
                    .show(ui, |ui| {
                        ui.set_width(118.5);
                        ui.set_height(128.0);
                        if let Some(img) = images.get_image(data.icon) {
                            let size = vec2(118.0, 128.0);
                            img.show_size(ui, size);
                        }
                    });
                });
            });
            ui.horizontal_wrapped(|ui| {
                ui.label(&data.description);
            });
            // Talent
            ui.collapsing("Constellations", |ui| {
                for talent in data.constellations {
                    TalentCard::show(ui, talent, images);
                }
            });
            ui.collapsing("PassiveTalents", |ui| {
                for talent in data.passive_talents {
                    TalentCard::show(ui, talent, images);
                }
            });
            ui.collapsing("skillTalents", |ui| {
                for talent in data.skill_talents {
                    TalentCard::show(ui, talent, images);
                }
            });
        });
    }
}