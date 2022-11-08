use crate::images::NetworkImages;
use crate::types::Element;
use crate::util::show_vision;
use eframe::egui::{vec2, Frame, Ui};

pub struct ElementCard;

impl ElementCard {
    pub fn show(ui: &mut Ui, data: Element, images: &NetworkImages) {
        ui.vertical(|ui| {
            Frame { ..Frame::default() }.show(ui, |ui| {
                ui.set_width(64.0);
                ui.set_height(64.0);
                if let Some(img) = images.get_image(data.icon) {
                    let size = vec2(64.0, 64.0);
                    img.show_size(ui, size);
                }
            });
            ui.heading(data.name);
            for d in data.reactions {
                ui.separator();
                ui.label(&d.name);
                ui.label(&d.description);
                let mut elements = String::new();
                for d in d.elements {
                    elements.push_str(&show_vision(&d))
                }
                ui.label(&elements);
            }
        });
    }
}
