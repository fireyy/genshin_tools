use super::Api;
use crate::constants::GENSHINDEV_URL;

pub struct GenshinDev {}

impl Api for GenshinDev {
    fn build_queue(path: String) -> String {
        format!("{}/{}", *GENSHINDEV_URL, path)
    }

    fn fetch(path: String, callback: impl 'static + Send + FnOnce(ehttp::Response)) {
        let url  = Self::build_queue(path);
        tracing::info!("Request url: {}", url);
        let req = ehttp::Request::get(url);
        ehttp::fetch(req, move |response| {
            match response {
                Ok(res) => callback(res),
                Err(err) => tracing::error!("Request error: {}", err)
            }
        });
    }
}