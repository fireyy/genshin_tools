use std::sync::mpsc;
use std::thread;
use anyhow::Result;
use serde_json::Value;
use tracing::{info};
use egui_extras::RetainedImage;
use cached_network_image::{
    Image, ImageStore,
    FetchImage, FetchQueue, ImageCache, ImageKind,
};
use crate::api;
use crate::types::{Category, Artifact};
use crate::widgets::{ArtifactCard};
use crate::util::{gen_image, gen_artifact_icon, setup_custom_fonts};

enum Update {
    CategoriesLoaded(Result<Vec<Category>>),
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
    data: Value,
    fetch_queue: FetchQueue<Image>,
    net_images: ImageCache,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        let (update_tx, update_rx) = mpsc::channel();

        let mut this: Self = Self {
            update_tx,
            update_rx,
            state: State::Idle,
            load_err: None,
            categories: vec![],
            selected_category: String::new(),
            data: Value::Null,
            fetch_queue: FetchQueue::create(cc.egui_ctx.clone()),
            net_images: ImageCache::default(),
        };

        // load categories
        this.load_category(&cc.egui_ctx);

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

    fn load_all_data(&mut self, ctx: &egui::Context, path: String) {
        self.state = State::Busy;
        self.selected_category = path.clone();

        let update_tx = self.update_tx.clone();
        let ctx = ctx.clone();

        thread::spawn(move || {
            let data = api::load_all_data(path);
            update_tx.send(Update::DataLoaded(data)).unwrap();
            ctx.request_repaint();
        });
    }

    fn show_conent(&mut self, ui: &mut egui::Ui) -> Result<()> {
        let cate = self.selected_category.as_str();
        let data = self.data.clone();
        
        match cate {
            "artifacts" => {
                let data: Vec<Artifact> = serde_json::from_value(data)?;
                let row_height = 100.0;
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .id_source("content")
                    .show_rows(
                        ui,
                        row_height,
                        data.len(),
                        |ui, row_range| {
                        for row in row_range {
                            let d = data.get(row).unwrap();
                            let imgs = gen_artifact_icon(d.name.clone());
                            self.add_images(imgs);
                            ArtifactCard::show(ui, d.clone(), &self.net_images);
                        }
                    });
            }
            _ => {}                
        }

        Ok(())
    }

    fn add_images(&mut self, imgs: Vec<String>) {
        for img in imgs {
            self.fetch_queue.fetch(gen_image(img, ImageKind::Display));
        }
    }

    fn try_fetch_image(&mut self) {
        let (image, data) = match self.fetch_queue.try_next() {
          Some((image, data)) => (image, data),
          _ => return,
        };
    
        let images = &mut self.net_images;
        if images.has_id(image.id) {
          return;
        }
    
        match RetainedImage::from_image_bytes(image.url(), &data) {
          Ok(img) => {
            images.add(image.id, img);
            ImageStore::<Image>::add(&image, &(), &data);
          }
          Err(err) => {
            tracing::error!("cannot create ({}) {} : {err}", image.id, image.url())
          }
        }
      }
}

impl eframe::App for TemplateApp {
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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Category:");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    for cate in self.categories.clone() {
                        let mut is_open = self.selected_category == cate.name;
                        let resp = ui.toggle_value(&mut is_open, cate.name);
                        if resp.clicked() {
                            self.load_all_data(ctx, cate.value);
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.state {
                State::Idle => {
                    self.show_conent(ui).unwrap();
                }
                State::Busy => {
                    ui.spinner();
                    ui.heading("Loading data...");
                }
            }
        });

        self.try_fetch_image();
    }
}