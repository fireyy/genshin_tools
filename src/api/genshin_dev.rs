use super::Api;
use crate::constants::{DATA_CACHE, GENSHINDEV_URL};
use anyhow::Result;

pub struct GenshinDev {}

impl Api for GenshinDev {
    fn build_queue(path: String) -> String {
        format!("{}/{}", *GENSHINDEV_URL, path)
    }

    fn fetch(path: String, callback: impl 'static + Send + FnOnce(Result<Vec<u8>>)) {
        let url = Self::build_queue(path);
        let mut cache = DATA_CACHE.lock().unwrap();
        if cache.contains(&url) {
            tracing::info!("Use cache: {}", url);
            match cache.get(&url) {
                Some(val) => callback(Ok(val.to_vec())),
                None => callback(Err(anyhow::Error::msg("Cache error".to_string()))),
            }
        } else {
            tracing::info!("Request url: {}", url);
            let req = ehttp::Request::get(url.clone());
            ehttp::fetch(req, move |response| match response {
                Ok(res) => {
                    let mut cache = DATA_CACHE.lock().unwrap();
                    cache.put(url, res.bytes.clone());
                    callback(Ok(res.bytes))
                }
                Err(err) => callback(Err(anyhow::Error::msg(format!("Request error: {}", err)))),
            });
        }
    }
}
