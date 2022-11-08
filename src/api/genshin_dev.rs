use super::Api;
use crate::constants::GENSHINDEV_URL;
use anyhow::Result;

pub struct GenshinDev {}

impl Api for GenshinDev {
    fn build_queue(path: String) -> String {
        format!("{}/{}", *GENSHINDEV_URL, path)
    }

    fn fetch(path: String, callback: impl 'static + Send + FnOnce(Result<ehttp::Response>)) {
        let url = Self::build_queue(path);
        tracing::info!("Request url: {}", url);
        let req = ehttp::Request::get(url);
        ehttp::fetch(req, move |response| match response {
            Ok(res) => callback(Ok(res)),
            Err(err) => callback(Err(anyhow::Error::msg(format!("Request error: {}", err)))),
        });
    }
}
