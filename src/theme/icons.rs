use eframe::egui;

pub const ICON_FONT: &[u8] = include_bytes!("../../assets/iconfont.ttf");

pub struct Icon {
  pub icon: &'static str,
  pub size: f32,
  color: Option<egui::Color32>,
  weak: bool,
}

const fn ic(c: &'static str) -> Icon {
  Icon {
    icon: c,
    size: 20.0,
    color: None,
    weak: false,
  }
}

impl Icon {
  pub const Star: Self = ic("\u{e693}");
  pub const Flower: Self = ic("\u{e606}");
  pub const Plume: Self = ic("\u{e607}");
  pub const Sands: Self = ic("\u{e608}");
  pub const Circle: Self = ic("\u{e60a}");
  pub const Goblet: Self = ic("\u{e60c}");
  pub const Anemo: Self = ic("\u{e60d}");

  pub fn color(self, color: egui::Color32) -> Self {
    let mut this = self;
    this.color = Some(color);
    this
  }

  pub fn size(self, sz: f32) -> Self {
    let mut this = self;
    this.size = sz;
    this
  }

  pub fn weak(self, weak: bool) -> Self {
    Self { weak, ..self }
  }
}

impl From<&Icon> for egui::WidgetText {
  fn from(ic: &Icon) -> egui::WidgetText {
    let mut rt = egui::RichText::new(ic.icon).font(egui::FontId::monospace(ic.size));
    if let Some(color) = ic.color {
      rt = rt.color(color);
    }
    if ic.weak {
      rt = rt.weak();
    }

    rt.into()
  }
}

impl Icon {
  pub fn show(&self, ui: &mut egui::Ui) -> egui::Response {
    let padding = egui::vec2(0.0, 0.0);
    let desired_size = egui::vec2(self.size + padding.x, self.size + padding.y);

    let (rect, resp) = ui.allocate_at_least(desired_size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
      let visuals = ui.style().interact(&resp);
      let wrap_width = ui.available_width();

      let icon_pos = egui::pos2(
        rect.min.x + padding.x,
        rect.center().y - self.size / 4.1 - 1.0,
      );

      let icon: egui::WidgetText = self.into();
      let icon = icon.into_galley(ui, Some(false), wrap_width, egui::TextStyle::Body);

      icon.paint_with_visuals(ui.painter(), icon_pos, visuals);
    }

    resp
  }
}
