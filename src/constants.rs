use once_cell::sync::Lazy;

pub static GENSHINDEV_URL: Lazy<&str> = Lazy::new(|| "https://api.genshin.dev");

pub static ARTIFACT_TYPE: Lazy<Vec<&str>> = Lazy::new(|| vec!["circlet-of-logos","flower-of-life","goblet-of-eonothem","plume-of-death","sands-of-eon"]);