use egui::{Ui, ScrollArea, SelectableLabel, RichText, Context, Align, Layout};

pub fn tab_ui(ui: &mut Ui, data: Vec<String>, selected: String, mut handler: impl FnMut(&Context, String)) {
    let id = egui::Id::new("scrolled_tab");
    let scrolled_tab: String = ui.memory().data.get_temp(id).unwrap_or_default();
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        ui.set_height(30.0);
        let max_width = ui.available_width() - 70.0;
        ui.add_enabled_ui(data.len() > 1, |ui| {
            if ui.button(RichText::new("\u{23f4}").heading()).clicked() {
                let mut index = data.iter().position(|d| d == &selected).unwrap_or_default();
                index = if index == 0 {
                    data.len() - 1
                } else {
                    index - 1
                };
                let tab = data.get(index).unwrap();
                handler(ui.ctx(), tab.to_string());
            }
        });
        ScrollArea::horizontal()
            .max_width(max_width)
            // .auto_shrink([false; 2])
            .id_source("tabs")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for tab in data.clone() {
                        let select = SelectableLabel::new(
                            selected == tab,
                            RichText::new(&tab).heading()
                        );
                        let response = ui.add(select);
                        if selected == tab && scrolled_tab != tab {
                            ui.memory().data.insert_temp(id, tab.clone());
                            response.scroll_to_me(Some(Align::Center));
                        }
                        if response.clicked() && selected != tab {
                            handler(ui.ctx(), tab.clone());
                        }
                    }
                    if selected.is_empty() && !data.is_empty() {
                        handler(ui.ctx(), data.first().unwrap().to_string());
                    }
                });
            });

        ui.add_enabled_ui(data.len() > 1, |ui| {
            if ui.button(RichText::new("\u{23f5}").heading()).clicked() {
                let mut index = data.iter().position(|d| d == &selected).unwrap_or_default() + 1;
                if index == data.len() {
                    index = 0;
                }
                let tab = data.get(index).unwrap();
                handler(ui.ctx(), tab.to_string());
            }
        });
    });
}