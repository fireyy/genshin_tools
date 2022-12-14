use anyhow::Result;
use serde_json::Value;

use crate::types::Category;

mod genshin_dev;

pub use genshin_dev::GenshinDev;

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
struct LoadCategory {
    types: Vec<String>,
}

pub trait Api {
    fn build_queue(path: String) -> String;
    fn fetch(path: String, callback: impl 'static + Send + FnOnce(Result<Vec<u8>>));
}

pub fn load_category(callback: impl 'static + Send + FnOnce(Result<Vec<Category>>)) {
    let mut categories = vec![];
    GenshinDev::fetch("".into(), |result| match result {
        Ok(res) => match std::str::from_utf8(&res).ok() {
            Some(text) => {
                let val: LoadCategory = serde_json::from_str(text).unwrap();
                for d in val.types {
                    categories.push(Category {
                        name: d.clone(),
                        value: d,
                    });
                }

                callback(Ok(categories))
            }
            None => callback(Err(anyhow::Error::msg("No Response"))),
        },
        Err(err) => callback(Err(err)),
    });
}

pub fn load_data(path: String, callback: impl 'static + Send + FnOnce(Result<Value>)) {
    GenshinDev::fetch(path, |result| match result {
        Ok(res) => match std::str::from_utf8(&res).ok() {
            Some(text) => {
                let val: Value = serde_json::from_str(text).unwrap();

                callback(Ok(val))
            }
            None => callback(Err(anyhow::Error::msg("No Response"))),
        },
        Err(err) => callback(Err(err)),
    });
}

pub fn load_tab_data(path: String, callback: impl 'static + Send + FnOnce(Result<Vec<String>>)) {
    GenshinDev::fetch(path, |result| match result {
        Ok(res) => match std::str::from_utf8(&res).ok() {
            Some(text) => {
                let val: Vec<String> = serde_json::from_str(text).unwrap();

                callback(Ok(val))
            }
            None => callback(Err(anyhow::Error::msg("No Response"))),
        },
        Err(err) => callback(Err(err)),
    });
}
