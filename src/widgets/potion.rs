use crate::images::NetworkImages;
use crate::types::Potion;
use super::item;

pub struct PotionCard;

impl PotionCard {
    pub fn show(ui: &mut egui::Ui, data: Potion, images: &NetworkImages) {
        let response = ui.add(item(data.icon, data.name, data.rarity, images));
        response.on_hover_ui(|ui| {
            ui.label(data.effect);
            for recipe in data.crafting {
                ui.label(format!("{} x {}", recipe.item, recipe.quantity));
            }
        });
    }
}