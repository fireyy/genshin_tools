#![allow(dead_code)]

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
    pub const STAR: Self = ic("\u{e693}");
    pub const FLOWER: Self = ic("\u{e606}");
    pub const PLUME: Self = ic("\u{e607}");
    pub const SANDS: Self = ic("\u{e608}");
    pub const CIRCLE: Self = ic("\u{e60a}");
    pub const GOBLET: Self = ic("\u{e60c}");
    pub const ANEMO: Self = ic("\u{e60d}");
    pub const CRYO: Self = ic("\u{e60e}");
    pub const DENDRO: Self = ic("\u{e60f}");
    pub const ELECTRO: Self = ic("\u{e610}");
    pub const GEO: Self = ic("\u{e611}");
    pub const HYDRO: Self = ic("\u{e612}");
    pub const PYRO: Self = ic("\u{e613}");
    pub const AMBER: Self = ic("\u{e615}");
    pub const WEAPON: Self = ic("\u{e616}");
    pub const ITEM: Self = ic("\u{e617}");
    pub const TASK: Self = ic("\u{e618}");
    pub const TOOL: Self = ic("\u{e619}");
    pub const ARTIFACT: Self = ic("\u{e61a}");
    pub const ORE: Self = ic("\u{e61b}");
    pub const MATERIAL: Self = ic("\u{e61c}");
    pub const CASTLE: Self = ic("\u{e863}");
    pub const MONSTER: Self = ic("\u{e664}");
    pub const COMPASS: Self = ic("\u{e629}");

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
