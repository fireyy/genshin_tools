use eframe::egui::{
    Ui, Frame, Layout, Direction, Align, vec2, Color32, RichText, TextStyle
};
use cached_network_image::ImageCache;
use crate::types::Character;
use crate::util::get_image;
use crate::theme::Icon;

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
    }
}