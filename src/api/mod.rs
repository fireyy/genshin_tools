use reqwest::blocking::{Response};
use anyhow::Result;
use serde_json::Value;

use crate::types::Category;

mod genshin_dev;

pub use genshin_dev::GenshinDev;

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
struct LoadCategory {
    types: Vec<String>
}

pub trait Api {
    fn build_queue(path: String) -> String;
    fn fetch(path: String) -> Result<Response>;
}

pub fn load_category() -> Result<Vec<Category>> {
    let mut categories = vec![];
    let res = GenshinDev::fetch("".into())?;
    let val: LoadCategory = serde_json::from_str(&res.text()?)?;
    for d in val.types {
        categories.push(Category {
            name: d.clone(),
            value: d,
        });
    }

    Ok(categories)
}

pub fn load_all_data(path: String) -> Result<Value> {
    let res = GenshinDev::fetch(format!("{path}/all"))?;
    tracing::debug!("load data: {:?}", res);
    let data: Value = serde_json::from_str(&res.text()?)?;

    Ok(data)
}