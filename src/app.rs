use std::sync::mpsc;
use std::thread;
use anyhow::Result;
use serde_json::Value;
use crate::api;
use crate::types::{Category};

enum Update {
    InitCategory,
    CategoriesLoaded(Result<Vec<Category>>),
}

enum State {
    Idle,
    Busy,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    update_tx: mpsc::Sender<Update>,
    #[serde(skip)]
    update_rx: mpsc::Receiver<Update>,
    #[serde(skip)]
    state: State,
    load_err: Option<String>,
    categories: Vec<Category>,
    category: Option<Category>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (update_tx, update_rx) = mpsc::channel();

        Self {
            update_tx,
            update_rx,
            state: State::Idle,
            load_err: None,
            categories: vec![],
            category: None,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let this: Self = Default::default();

        // load categories
        this.update_tx.send(Update::InitCategory).unwrap();

        this
    }

    fn load_category(&mut self, ctx: &egui::Context) {
        self.state = State::Busy;

        let update_tx = self.update_tx.clone();
        let ctx = ctx.clone();

        thread::spawn(move || {
            let data = api::load_category();
            update_tx.send(Update::CategoriesLoaded(data)).unwrap();
            ctx.request_repaint();
        });
    }

    fn load_data(&mut self, ctx: &egui::Context) {
        self.state = State::Busy;

        let update_tx = self.update_tx.clone();
        let ctx = ctx.clone();
        let category = self.category.clone();

        thread::spawn(move || {
            //
        });
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(update) = self.update_rx.try_recv() {
            match update {
                Update::InitCategory => {
                    self.load_category(ctx);
                }
                Update::CategoriesLoaded(result) => match result {
                    Ok(data) => {
                        self.categories = data;
                    }
                    Err(err) => {
                        self.state = State::Idle;
                        self.load_err = Some(err.to_string());
                    }
                },
            }
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            //

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.state {
                State::Idle => {

                }
                State::Busy => {
                    ui.spinner();
                    ui.heading("Loading data...");
                }
            }
        });
    }
}
