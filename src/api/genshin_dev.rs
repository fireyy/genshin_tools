use reqwest::blocking::{get, Response};
use anyhow::Result;
use super::Api;
use crate::constants::GENSHINDEV_URL;

pub struct GenshinDev {}

impl Api for GenshinDev {
    fn build_queue(path: String) -> String {
        format!("{}/{}", *GENSHINDEV_URL, path)
    }

    fn fetch(path: String) -> Result<Response> {
        let url  = Self::build_queue(path);
        tracing::info!("Request url: {}", url);
        let res = get(url)?;
        Ok(res)
    }
}