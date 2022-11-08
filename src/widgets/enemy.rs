use crate::custom_grid;
use crate::images::NetworkImages;
use crate::types::Enemy;
use crate::util::{gen_star, show_vision};
use eframe::egui::{vec2, Frame, Ui};

pub struct EnemyCard;

impl EnemyCard {
    pub fn show(ui: &mut Ui, data: Enemy, images: &NetworkImages) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let mut elements = String::new();
                for d in data.elements {
                    elements.push_str(&show_vision(&d))
                }
                custom_grid!(
                    ui,
                    ("Name", &data.name),
                    ("Type", &data.enemy_type),
                    ("Region", &data.region),
                    ("Family", &data.family),
                    ("Faction", &data.faction),
                    ("Elements", &elements),
                    ("Mora Gained", &data.mora_gained.to_string())
                );

                ui.with_layout(
                    egui::Layout::from_main_dir_and_cross_align(
                        egui::Direction::TopDown,
                        egui::Align::RIGHT,
                    ),
                    |ui| {
                        Frame { ..Frame::default() }.show(ui, |ui| {
                            ui.set_width(118.5);
                            ui.set_height(128.0);
                            if let Some(img) = images.get_image(data.icon) {
                                let size = vec2(118.0, 128.0);
                                img.show_size(ui, size);
                            }
                        });
                    },
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.label(&data.description);
            });
            ui.collapsing("Elemental Description", |ui| {
                for d in data.elemental_description {
                    ui.label(format!("{}: {}", d.element, d.description));
                }
            });
            ui.collapsing("Drops", |ui| {
                for d in data.drops {
                    ui.label(format!(
                        "Level {}: {} {}",
                        d.minimum_level,
                        d.name,
                        gen_star(d.rarity)
                    ));
                }
            });
            ui.collapsing("Artifacts", |ui| {
                for d in data.artifacts {
                    ui.label(format!("Set {}: {} - {}", d.set, d.name, d.rarity));
                }
            });
        });
    }
}
