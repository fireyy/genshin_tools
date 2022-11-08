use crate::custom_grid;
use crate::custom_list;
use crate::types::Domain;
use crate::util::show_vision;

pub struct DomainCard;

impl DomainCard {
    pub fn show(ui: &mut egui::Ui, data: Domain) {
        ui.vertical(|ui| {
            let mut elements = String::new();
            for d in data.recommended_elements {
                elements.push_str(&show_vision(&d))
            }
            custom_grid!(
                ui,
                ("Name", &data.name),
                ("Nation", &data.nation),
                ("Location", &data.location),
                ("Type", &data.domain_type),
                ("Recommended Elements", &elements)
            );
            ui.horizontal_wrapped(|ui| {
                ui.label(&data.description);
            });
            ui.collapsing("Requirements", |ui| {
                for req in data.requirements {
                    custom_list!(
                        ui,
                        ("Level", req.level.to_string()),
                        ("Recommended Level", req.adventure_rank.to_string()),
                        ("Adventure Rank", req.recommended_level.to_string())
                    );
                    ui.label("Ley Line Disorder:");
                    ui.vertical(|ui| {
                        for ley in req.ley_line_disorder {
                            ui.label(ley);
                        }
                    });
                }
            });
            ui.collapsing("Rewards", |ui| {
                for rew in data.rewards {
                    ui.label(rew.day);
                    for de in rew.details {
                        custom_list!(
                            ui,
                            ("Level", de.level.to_string()),
                            ("Adventure Experience", de.adventure_experience.to_string()),
                            (
                                "Companionship Experience",
                                de.companionship_experience.to_string()
                            ),
                            ("Mora", de.mora.to_string())
                        );
                        if !de.drops.is_empty() {
                            ui.label("Drops:");
                            for d in de.drops {
                                ui.label(format!("{}: {} ~ {}", d.name, d.drop_min, d.drop_max));
                            }
                        }
                        if !de.items.is_empty() {
                            ui.label("Items:");
                            for d in de.items {
                                ui.label(format!("{}: {} ~ {}", d.name, d.drop_min, d.drop_max));
                            }
                        }
                    }
                }
            });
        });
    }
}
