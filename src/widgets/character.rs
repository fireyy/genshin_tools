use eframe::egui::{
    Ui, Frame, vec2, Color32, Rounding, style::Margin,
};
use cached_network_image::ImageCache;
use crate::types::Character;
use crate::util::get_image;
use crate::theme::Icon;
use super::TalentCard;

macro_rules! character_grid {
    ($ui:expr, $($label:expr),*) => {
        egui::Grid::new("_properties").num_columns(2).min_col_width(120.0).max_col_width(120.0).show($ui, |ui|{
            $(
                ui.label($label.0);
                ui.label($label.1);
                ui.end_row();
            )*
        });
    }
}

pub struct CharacterCard;

impl CharacterCard {
    pub fn show(ui: &mut Ui, data: Character, images: &ImageCache) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                character_grid!(
                    ui,
                    ("Name", &data.name),
                    ("Title", &data.title),
                    ("Nation", &data.nation),
                    ("Birthday", &data.birthday),
                    ("Rarity", format!("{}", data.rarity)),
                    ("Vision", &data.vision),
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
                        if let Some(img) = get_image(images, data.icon) {
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