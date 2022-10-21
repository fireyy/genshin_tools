use eframe::egui::{
    Ui, Frame, Color32
};
use cached_network_image::ImageCache;
use crate::types::Food;
use crate::util::{get_image, gen_star};
use crate::theme::text_ellipsis;

pub struct FoodCard;

impl FoodCard {
    pub fn show(ui: &mut Ui, data: Food, images: &ImageCache) {
        let initial_size = egui::vec2(
            100.0,
            150.0, // Assume there will be
        );
        let (rect, response) = ui.allocate_exact_size(initial_size, egui::Sense::click());
        if ui.is_rect_visible(rect) {
            ui.allocate_ui_at_rect(rect, |ui| {
                Frame {
                    rounding: egui::Rounding::same(10.0),
                    inner_margin: egui::style::Margin::same(10.0),
                    fill: egui::Color32::from_gray(50),
                    ..Frame::default()
                }
                .show(ui, |ui| {
                    ui.set_height(120.0);
                    ui.vertical_centered(|ui| {
                        Frame {
                            ..Frame::default()
                        }
                        .show(ui, |ui| {
                            ui.set_width(64.0);
                            ui.set_height(64.0);
                            if let Some(img) = get_image(images, data.icon) {
                                let size = egui::vec2(64.0, 64.0);
                                img.show_size(ui, size);
                            }
                        });
                        ui.label(gen_star(data.rarity));
                        ui.label(text_ellipsis(&data.name, Color32::WHITE, 2));
                    });
                });
            });
        }
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