use lru::LruCache;
use once_cell::sync::Lazy;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

pub static GENSHINDEV_URL: Lazy<&str> = Lazy::new(|| "https://api.genshin.dev");

pub static ARTIFACT_TYPE: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "circlet-of-logos",
        "flower-of-life",
        "goblet-of-eonothem",
        "plume-of-death",
        "sands-of-eon",
    ]
});

type DataCache = Lazy<Arc<Mutex<LruCache<String, Vec<u8>>>>>;

pub static DATA_CACHE: DataCache =
    Lazy::new(|| Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(2).unwrap()))));
