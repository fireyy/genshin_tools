use cached_network_image::{
    Image, ImageStore, ImageKind, Uuid,
};

use crate::constants::{GENSHINDEV_URL, ARTIFACT_TYPE};
use crate::theme::ICON_FONT;

pub fn gen_image(url: String, kind: ImageKind) -> Image {
    let uuid = ImageStore::<Image>::get_id(&url) //
      .unwrap_or_else(Uuid::new_v4);
    Image{
      id: uuid,
      kind,
      url: url.clone(),
      meta: (),
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
        .map(|a| format!("{}/artifacts/{}/{}.png", *GENSHINDEV_URL, name, a))
        .collect();
    arr
}

#[inline(always)]
pub fn map_to_pixel(point: f32, ppi: f32, map: fn(f32) -> f32) -> f32 {
    map(point * ppi) / ppi
}