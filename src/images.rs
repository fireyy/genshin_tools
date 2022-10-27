use std::collections::HashSet;
use egui_extras::RetainedImage;

#[cfg(not(target_arch = "wasm32"))]
use cached_network_image::{
    Image, ImageStore,
    FetchImage, FetchQueue, ImageCache,
    Uuid, ImageKind,
};

#[cfg(not(target_arch = "wasm32"))]
pub struct NetworkImages {
    pub fetch_queue: FetchQueue<Image>,
    pub caches: ImageCache,
    pub requested_images: HashSet<String>,
}
#[cfg(not(target_arch = "wasm32"))]
impl NetworkImages {
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            fetch_queue: FetchQueue::create(ctx.clone()),
            caches: ImageCache::default(),
            requested_images: HashSet::new(),
        }
    }

    pub fn add_all(&mut self, imgs: Vec<String>) {
        for img in imgs {
            self.add(img);
        }
    }

    pub fn add(&mut self, img: String) {
        if !self.requested_images.insert(img.clone()) {
            return;
        }
        self.fetch_queue.fetch(gen_image(img));
    }

    pub fn get_image(&self, url: String) -> Option<&RetainedImage> {
        if let Some(img_id) = ImageStore::<Image>::get_id(&url) {
            self.caches.get_id(img_id)
        } else {
            None
        }
    }

    pub fn try_fetch(&mut self) {
        let (image, data) = match self.fetch_queue.try_next() {
          Some((image, data)) => (image, data),
          _ => return,
        };
    
        let images = &mut self.caches;
        if images.has_id(image.id) {
          return;
        }
    
        match RetainedImage::from_image_bytes(image.url(), &data) {
          Ok(img) => {
            images.add(image.id, img);
            let _ = self.requested_images.remove(&image.url);
            ImageStore::<Image>::add(&image, &(), &data);
          }
          Err(err) => {
            tracing::error!("cannot create ({}) {} : {err}", image.id, image.url())
          }
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn gen_image(url: String) -> Image {
    let uuid = ImageStore::<Image>::get_id(&url) //
      .unwrap_or_else(Uuid::new_v4);

    Image{
      id: uuid,
      kind: ImageKind::Display,
      url: url.clone(),
      meta: (),
    }
}

#[cfg(target_arch = "wasm32")]
use std::{sync::{Arc, Mutex}, collections::HashMap};
#[cfg(target_arch = "wasm32")]
pub struct NetworkImages {
  ctx: egui::Context,
  caches: Arc<Mutex<HashMap<String, Vec<u8>>>>,
  requested_images: HashSet<String>,
}
#[cfg(target_arch = "wasm32")]
impl NetworkImages {
  pub fn new(ctx: egui::Context) -> Self {
    Self {
      ctx,
      caches: Arc::new(Mutex::new(HashMap::new())),
      requested_images: HashSet::new(),
    }
  }
  pub fn try_fetch(&mut self) {}
  pub fn add(&mut self, img: String) {
    if !self.requested_images.insert(img.clone()) {
      return;
    }
    self.fetch(img);
  }
  fn fetch(&mut self, url: String) {
    let req = ehttp::Request::get(url.clone());
    let caches_store = self.caches.clone();
    let ctx = self.ctx.clone();
    // let mut requested_images = self.requested_images.clone();
    ehttp::fetch(req, move |response| {
      match response {
        Err(err) => {
          tracing::error!("cannot create {} : {err}", url)
        }
        Ok(res) => {
          let mut pending = caches_store.lock().unwrap();
          pending.insert(url.clone(), res.bytes);
          // let _ = requested_images.remove(&url);
          ctx.request_repaint();
        }
      }
    });
  }
  pub fn add_all(&mut self, imgs: Vec<String>) {
    for img in imgs {
      self.add(img);
    }
  }
  pub fn get_image(&self, url: String) -> Option<RetainedImage> {
    let caches = self.caches.lock().unwrap();
    match caches.get(&url) {
      Some(b) => {
        match RetainedImage::from_image_bytes(url.clone(), &b) {
          Ok(img) => {
            Some(img)
          }
          Err(err) => {
            tracing::error!("cannot create {} : {err}", url);
            None
          }
        }
      }
      None => None
    }
  }
}