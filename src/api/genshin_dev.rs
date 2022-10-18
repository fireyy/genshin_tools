use reqwest::blocking::{get, Response};
use anyhow::Result;
use super::Api;

pub struct GenshinDev {}

impl Api for GenshinDev {
    fn build_queue(path: String) -> String {
        format!("https://api.genshin.dev/{}", path)
    }

    fn fetch(path: String) -> Result<Response> {
        let url  = Self::build_queue(path);
        let res = get(url)?;
        Ok(res)
    }
}