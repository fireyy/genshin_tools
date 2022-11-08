use super::item;
use crate::images::NetworkImages;
use crate::types::Food;

pub struct FoodCard;

impl FoodCard {
    pub fn show(ui: &mut egui::Ui, data: Food, images: &NetworkImages) {
        let response = ui.add(item(data.icon, data.name, data.rarity, images));
        response.on_hover_ui(|ui| {
            ui.label(data.food_type);
            ui.label(data.effect);
            for recipe in data.recipe {
                ui.label(format!("{} x {}", recipe.item, recipe.quantity));
            }
            ui.label(data.description);
        });
    }
}
