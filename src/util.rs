use cached_network_image::{
    Image, ImageStore, ImageKind, Uuid, ImageCache,
};
use egui_extras::RetainedImage;

use crate::constants::{GENSHINDEV_URL, ARTIFACT_TYPE};
use crate::theme::Icon;


pub fn gen_image(url: String) -> Image {
    let uuid = ImageStore::<Image>::get_id(&url) //
      .unwrap_or_else(Uuid::new_v4);
    Image{
      id: uuid,
      kind: ImageKind::Display,
      url: url.clone(),
      meta: (),
    }
}

pub fn get_image(images: &ImageCache, url: String) -> Option<&RetainedImage> {
    if let Some(img_id) = ImageStore::<Image>::get_id(&url) {
        images.get_id(img_id)
    } else {
        None
    }
}

pub fn trans_string(name: String) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace("'", "-")
}

pub fn gen_artifact_icon(name: String) -> Vec<String> {
    let name = trans_string(name);
    let arr = ARTIFACT_TYPE
        .iter()
        .map(|a| gen_icon_from_type(format!("artifacts/{name}"), a.to_string()))
        .collect();
    arr
}

pub fn gen_icon_from_type(t: String, name: String) -> String {
    let t = trans_string(t);
    let name = trans_string(name);
    format!("{}/{}/{}.webp", *GENSHINDEV_URL, t, name)
}

pub fn gen_star(rarity: u8) -> String {
    let mut star = String::new();
    for _ in 0..rarity {
        star.push_str(Icon::STAR.icon);
    }
    star
}

pub fn show_vision(vision: &String) -> String {
    let str = match vision.as_str() {
        "ANEMO" => Icon::ANEMO.icon,
        "CRYO" => Icon::CRYO.icon,
        "DENDRO" => Icon::DENDRO.icon,
        "ELECTRO" => Icon::ELECTRO.icon,
        "GEO" => Icon::GEO.icon,
        "HYDRO" => Icon::HYDRO.icon,
        "PYRO" => Icon::PYRO.icon,
        _ => Icon::AMBER.icon, // return Amber :p
    };

    str.to_string()
}

#[macro_export]
macro_rules! custom_grid {
    ($ui:expr, $($label:expr),*) => {
        egui::Grid::new("_properties").num_columns(2).min_col_width(120.0).max_col_width(120.0).show($ui, |ui|{
            $(
                ui.label($label.0);
                ui.label($label.1);
                ui.end_row();
            )*
        });
    }
}