use std::sync::mpsc;
use std::thread;
use anyhow::Result;
use serde_json::Value;
use tracing::{info};
use egui_extras::RetainedImage;
use std::collections::BTreeMap;
use crate::images::NetworkImages;
use crate::api;
use crate::types::{Category, Artifact, Character, Food};
use crate::widgets::{ArtifactCard, CharacterCard, FoodCard};
use crate::util::{gen_artifact_icon, gen_icon_from_type};
use crate::theme::{Icon, setup_custom_fonts, Style};

const LOGO: &[u8] = include_bytes!("../assets/logo.png");

enum Update {
    CategoriesLoaded(Result<Vec<Category>>),
    TabsLoaded(Result<Vec<String>>),
    DataLoaded(Result<Value>),
}

enum State {
    Idle,
    Busy,
}

pub struct TemplateApp {
    update_tx: mpsc::Sender<Update>,
    update_rx: mpsc::Receiver<Update>,
    state: State,
    load_err: Option<String>,
    categories: Vec<Category>,
    selected_category: String,
    tabs: Vec<String>,
    selected_tab: String,
    data: Value,
    net_images: NetworkImages,
    style: Style,
    logo: RetainedImage,
}

impl TemplateApp {
    const SAVE_KEY: &'static str = concat!(env!("CARGO_PKG_NAME"), "_", "categories");

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut categories =  vec![];
        if let Some(storage) = cc.storage {
            categories = eframe::get_value(storage, Self::SAVE_KEY).unwrap_or_default();
        }
        setup_custom_fonts(&cc.egui_ctx);

        let (update_tx, update_rx) = mpsc::channel();

        let mut this: Self = Self {
            update_tx,
            update_rx,
            state: State::Idle,
            load_err: None,
            categories,
            selected_category: String::new(),
            tabs: vec![],
            selected_tab: String::new(),
            data: Value::Null,
            net_images: NetworkImages::new(cc.egui_ctx.clone()),
            style: Style::default(),
            logo: RetainedImage::from_image_bytes("logo", LOGO).unwrap(),
        };

        this.style.set_theme_visuals(&cc.egui_ctx);

        if this.categories.is_empty() {
            // load categories
            this.load_category(&cc.egui_ctx);
        }

        this
    }

    fn load_category(&mut self, ctx: &egui::Context) {
        info!("load category");
        self.state = State::Busy;

        let update_tx = self.update_tx.clone();
        let ctx = ctx.clone();

        thread::spawn(move || {
            let data = api::load_category();
            update_tx.send(Update::CategoriesLoaded(data)).unwrap();
            ctx.request_repaint();
        });
    }

    fn load_tab_data(&mut self, ctx: &egui::Context, path: String) {
        self.state = State::Busy;
        self.selected_category = path.clone();
        self.selected_tab = String::new();
        self.tabs = vec![];

        let update_tx = self.update_tx.clone();
        let ctx = ctx.clone();

        thread::spawn(move || {
            let data = api::load_tab_data(path);
            update_tx.send(Update::TabsLoaded(data)).unwrap();
            ctx.request_repaint();
        });
    }

    fn load_data(&mut self, ctx: &egui::Context, path: String) {
        self.state = State::Busy;
        self.selected_tab = path.clone();

        let update_tx = self.update_tx.clone();
        let ctx = ctx.clone();
        let path = format!("{}/{}", self.selected_category, path);

        thread::spawn(move || {
            let data = api::load_data(path);
            update_tx.send(Update::DataLoaded(data)).unwrap();
            ctx.request_repaint();
        });
    }

    fn show_conent(&mut self, ui: &mut egui::Ui) -> Result<()> {
        let cate = self.selected_category.as_str();
        let data = self.data.clone();

        if data.is_null() {
            ui.centered_and_justified(|ui| {
                ui.label("There are no data to display.");
            });
            return Ok(())
        }
        
        match cate {
            "artifacts" => {
                let mut data: Artifact = serde_json::from_value(data)?;
                let imgs = gen_artifact_icon(data.name.clone());
                data.icon = imgs.clone();
                self.net_images.add_all(imgs);
                ArtifactCard::show(ui, data.clone(), &self.net_images);
            }
            "characters" => {
                let mut data: Character = serde_json::from_value(data)?;
                // let img = gen_character_icon(&data.name);
                let img = gen_icon_from_type(format!("characters/{}", data.name), "icon-big".into());
                data.icon = img.clone();
                self.net_images.add(img);
                CharacterCard::show(ui, data.clone(), &self.net_images);
            }
            "consumables" => {
                let data: BTreeMap<String, Food> = serde_json::from_value(data)?;       
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP)
                        .with_main_wrap(true),
                    |ui| {
                        // ui.spacing_mut().item_spacing.x = 0.0;
                        for (name, mut d) in data {
                            let img = gen_icon_from_type("consumables/food".to_string(), name.to_string());
                            d.icon = img.clone();
                            self.net_images.add(img);
                            FoodCard::show(ui, d.clone(), &self.net_images);
                        }
                    }
                );
            }
            _ => {}                
        }

        Ok(())
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, Self::SAVE_KEY, &self.categories);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(update) = self.update_rx.try_recv() {
            match update {
                Update::CategoriesLoaded(result) => match result {
                    Ok(data) => {
                        self.state = State::Idle;
                        self.categories = data;
                    }
                    Err(err) => {
                        self.state = State::Idle;
                        self.load_err = Some(err.to_string());
                    }
                },
                Update::TabsLoaded(result) => match result {
                    Ok(data) => {
                        self.state = State::Idle;
                        self.tabs = data;
                    }
                    Err(err) => {
                        self.state = State::Idle;
                        self.load_err = Some(err.to_string());
                    }
                },
                Update::DataLoaded(result) => match result {
                    Ok(data) => {
                        self.state = State::Idle;
                        self.data = data;
                    }
                    Err(err) => {
                        self.state = State::Idle;
                        self.load_err = Some(err.to_string());
                    }
                },
            }
        }

        egui::SidePanel::left("side_panel")
        .resizable(false)
        .min_width(150.0)
        .show(ctx, |ui| {
            self.logo.show_scaled(ui, 0.3);
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    for cate in &self.categories.clone() {
                        ui.style_mut().spacing.button_padding.y = 10.0;
                        let select = egui::SelectableLabel::new(
                            self.selected_category == cate.value, 
                            egui::RichText::new(format!("{} {}", Icon::WEAPON.icon, &cate.name)).heading()
                        );
                        if ui.add(select).clicked() && self.selected_category != cate.value {
                            self.load_tab_data(ctx, cate.value.clone());
                        }
                    }
                    if self.selected_category.is_empty() && !self.categories.is_empty() {
                        self.load_tab_data(ctx, self.categories.first().unwrap().value.clone());
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal()
                // .auto_shrink([false; 2])
                .id_source("tabs")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for tab in self.tabs.clone() {
                            let select = egui::SelectableLabel::new(
                                self.selected_tab == tab,
                                egui::RichText::new(&tab).heading()
                            );
                            if ui.add(select).clicked() && self.selected_tab != tab {
                                self.load_data(ctx, tab.clone());
                            }
                        }
                        if self.selected_tab.is_empty() && !self.tabs.is_empty() {
                            self.load_data(ctx, self.tabs.first().unwrap().to_string());
                        }
                    });
                });
            
            ui.separator();

            match &mut self.state {
                State::Idle => {
                    self.style.for_scrollbar(ui);
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        // .enable_scrolling(false)
                        .id_source("content_scroll")
                        .show(ui, |ui| {
                            self.style.scrollarea(ui);
                            self.show_conent(ui).unwrap();
                        });
                }
                State::Busy => {
                    ui.centered_and_justified(|ui| {
                        ui.spinner();
                    });
                }
            }
        });

        self.net_images.try_fetch();
    }
}