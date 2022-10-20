use eframe::egui::{
    Ui, Frame, Layout, Direction, Align, vec2, Color32, RichText, TextStyle
};
use cached_network_image::{
  ImageCache, ImageStore, Image
};
use crate::types::Artifact;
use crate::util::gen_artifact_icon;
use crate::theme::Icon;

pub struct ArtifactCard;

impl ArtifactCard {
    pub fn show(ui: &mut Ui, data: Artifact, images: &ImageCache) {
        ui.vertical(|ui| {
            ui.set_width(ui.available_width());
            let imgs = gen_artifact_icon(data.name.clone());
            ui.horizontal(|ui| {
              for img in imgs {
                Frame {
                  ..Frame::default()
                }
                .show(ui, |ui| {
                  ui.set_width(64.0);
                  ui.set_height(64.0);
                  if let Some(img_id) = ImageStore::<Image>::get_id(&img) {
                    if let Some(img) = images.get_id(img_id) {
                      let size = vec2(64.0, 64.0);
                      img.show_size(ui, size);
                    }
                  }
                });
              }
            });
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
              ui.vertical(|ui| {
                ui.horizontal(|ui| {
                  ui.label(data.name);
                  ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let mut star = String::new();
                    for _ in 0..data.max_rarity {
                      star.push_str(Icon::STAR.icon);
                    }
                    ui.label(RichText::new(format!("Rarity: {}", star)).text_style(TextStyle::Small).color(Color32::LIGHT_YELLOW));
                  });
                });
                ui.label(data.two_piece_bonus);
                ui.label(data.four_piece_bonus);
              });
            });
        });
    }
}