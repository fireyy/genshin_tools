use eframe::egui::{
    Ui, Frame, vec2,
};
use cached_network_image::ImageCache;
use crate::types::Food;
use crate::util::{get_image, gen_star};

pub struct FoodCard;

impl FoodCard {
    pub fn show(ui: &mut Ui, data: Food, images: &ImageCache) {
        Frame {
            rounding: egui::Rounding::same(10.0),
            inner_margin: egui::style::Margin::same(10.0),
            fill: egui::Color32::from_gray(50),
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.set_width(150.0);
            ui.set_width(100.0);
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.set_height(64.0);
                if let Some(img) = get_image(images, data.icon) {
                    let size = egui::vec2(64.0, 64.0);
                    img.show_size(ui, size);
                }
                ui.label(data.name);
            });
        });
        // ui.vertical(|ui| {
        //     ui.horizontal(|ui| {
        //         custom_grid!(
        //             ui,
        //             ("Name", &data.name),
        //             ("Type", &data.food_type),
        //             ("Proficiency", &data.proficiency.to_string()),
        //             ("Rarity", gen_star(data.rarity)),
        //             ("Effect", &data.effect)
        //         );
    
        //         ui.with_layout(egui::Layout::from_main_dir_and_cross_align(egui::Direction::TopDown, egui::Align::RIGHT), |ui| {
        //             Frame {
        //                 ..Frame::default()
        //             }
        //             .show(ui, |ui| {
        //                 ui.set_width(118.5);
        //                 ui.set_height(128.0);
        //                 if let Some(img) = get_image(images, data.icon) {
        //                     let size = vec2(118.0, 128.0);
        //                     img.show_size(ui, size);
        //                 }
        //             });
        //         });
        //     });
        //     ui.horizontal_wrapped(|ui| {
        //         ui.label(&data.description);
        //     });
        //     // Recipe
        //     if data.has_recipe {
        //         ui.collapsing("Recipe", |ui| {
        //             for recipe in data.recipe {
        //                 ui.vertical(|ui| {
        //                     ui.label(format!("{} x {}", recipe.item, recipe.quantity));
        //                 });
        //             }
        //         });
        //     }
        // });
    }
}