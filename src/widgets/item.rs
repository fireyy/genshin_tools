use crate::images::NetworkImages;
use crate::util::gen_star;
use crate::theme::text_ellipsis;

pub fn item_ui(ui: &mut egui::Ui, icon: String, name: String, rarity: u8, images: &NetworkImages) -> egui::Response {
    let initial_size = egui::vec2(
        100.0,
        150.0, // Assume there will be
    );
    let (rect, response) = ui.allocate_exact_size(initial_size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        ui.allocate_ui_at_rect(rect, |ui| {
            egui::Frame {
                rounding: egui::Rounding::same(10.0),
                inner_margin: egui::style::Margin::same(10.0),
                fill: egui::Color32::from_gray(50),
                ..egui::Frame::default()
            }
            .show(ui, |ui| {
                ui.set_height(120.0);
                ui.vertical_centered(|ui| {
                    egui::Frame {
                        ..egui::Frame::default()
                    }
                    .show(ui, |ui| {
                        ui.set_width(64.0);
                        ui.set_height(64.0);
                        if let Some(img) = images.get_image(icon) {
                            let size = egui::vec2(64.0, 64.0);
                            img.show_size(ui, size);
                        }
                    });
                    ui.label(gen_star(rarity));
                    ui.label(text_ellipsis(&name, egui::Color32::WHITE, 2));
                });
            });
        });
    }

    response
}

pub fn item(icon: String, name: String, rarity: u8, images: &NetworkImages) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| item_ui(ui, icon, name, rarity, images)
}

#[macro_export]
macro_rules! custom_list {
    ($ui:expr, $($label:expr),*) => {
        egui::Frame {
            rounding: egui::Rounding::same(10.0),
            inner_margin: egui::style::Margin::same(10.0),
            fill: egui::Color32::from_gray(50),
            ..egui::Frame::default()
        }.show($ui, |ui| {
            ui.vertical(|ui| {
                $(
                    ui.horizontal(|ui| {
                        ui.label($label.0);
                        ui.label($label.1);
                    });
                )*
            })
        });
    }
}