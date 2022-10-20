use egui::{Color32, Rounding, Ui};

#[derive(Clone)]
pub struct Style {
    pub separator_size: f32,
    pub separator_extra: f32,

    pub app_bg: Color32,

    pub top_base: Color32,
    pub top_text: Color32,
    pub top_active: Color32,
    pub top_disable: Color32,

    pub tab_rounding: Rounding,
    pub tab_bar: Color32,
    pub tab_base: Color32,
    pub tab_text: Color32,
    //pub tab_active: Color32,
    pub tab_outline: Color32,

    pub selection: Color32,
    pub selection_opaque: Color32,

    pub background: Color32,
    pub panel: Color32,
    pub tabbar: Color32,
    pub separator: Color32,
    pub input_stroke: Color32,
    pub input_fill: Color32,
    pub input_text: Color32,
}

impl Default for Style {
    fn default() -> Self {
        let background = Color32::from_gray(0x00);
        let panel = Color32::from_gray(0x2B);
        let tabbar = Color32::from_gray(0x14);
        let separator = Color32::from_gray(0x1E);

        let input_stroke = Color32::from_gray(0x37);
        let input_fill = Color32::from_gray(0x1B);
        let input_text = Color32::from_gray(0xEE);

        Self {
            background,
            panel,
            tabbar,
            separator,
            input_stroke,
            input_text,
            input_fill,

            selection: Color32::from_rgba_unmultiplied(61, 133, 224, 30),
            selection_opaque: Color32::from_rgb(61, 133, 224),

            separator_size: 4.0,
            separator_extra: 80.0,

            //app_bg: Color32::from_gray(0x14),
            app_bg: background,

            top_base: Color32::from_gray(0x2d),
            top_text: Color32::from_gray(0x9d),
            top_active: Color32::from_gray(0x55),
            top_disable: Color32::from_gray(0x20),

            tab_rounding: Rounding {
                ne: 2.0,
                nw: 2.0,
                se: 0.0,
                sw: 0.0,
            },

            //tab_bar: Color32::from_gray(0x20),
            //tab_base: Color32::from_gray(0x30),
            tab_bar: tabbar,
            tab_base: panel,

            tab_text: Color32::from_gray(0x9d),
            //tab_active: Color32::from_gray(0x40),
            tab_outline: Color32::from_gray(0x1c),
        }
    }
}

impl Style {
    pub fn set_theme_visuals(&self, ctx: &egui::Context) {
        let mut visuals = ctx.style().visuals.clone();

        let expansion = 0.0;
        visuals.widgets.noninteractive.expansion = expansion;
        visuals.widgets.inactive.expansion = expansion;
        visuals.widgets.hovered.expansion = expansion;
        visuals.widgets.active.expansion = expansion;
        visuals.widgets.open.expansion = expansion;

        visuals.widgets.noninteractive.bg_fill = self.input_fill;
        visuals.widgets.inactive.bg_fill = self.input_fill;
        visuals.widgets.hovered.bg_fill = self.input_fill;
        visuals.widgets.active.bg_fill = self.input_fill;
        visuals.widgets.open.bg_fill = self.input_fill;

        visuals.widgets.noninteractive.fg_stroke.color = self.input_text;
        visuals.widgets.inactive.fg_stroke.color = self.input_text;
        visuals.widgets.hovered.fg_stroke.color = self.input_text;
        visuals.widgets.active.fg_stroke.color = self.input_text;
        visuals.widgets.open.fg_stroke.color = self.input_text;

        let rounding = Rounding::same(2.0);

        visuals.extreme_bg_color = self.input_fill;
        visuals.widgets.noninteractive.rounding = rounding;
        visuals.widgets.inactive.rounding = rounding;
        visuals.widgets.hovered.rounding = rounding;
        visuals.widgets.active.rounding = rounding;
        visuals.widgets.open.rounding = rounding;

        visuals.selection.bg_fill = self.selection_opaque;
        visuals.selection.stroke.color = Color32::WHITE;

        ctx.set_visuals(visuals);
    }

    pub fn for_scrollbar(&self, ui: &mut Ui) {
        let spacing = ui.spacing_mut();
        spacing.scroll_bar_width = 4.0;

        let visuals = ui.visuals_mut();
        let rounding = Rounding::same(0.0);

        visuals.extreme_bg_color = self.panel;
        visuals.clip_rect_margin = 0.0;
        visuals.widgets.noninteractive.rounding = rounding;
        visuals.widgets.inactive.rounding = rounding;
        visuals.widgets.hovered.rounding = rounding;
        visuals.widgets.active.rounding = rounding;
        visuals.widgets.open.rounding = rounding;
    }

    pub fn scrollarea(&self, ui: &mut Ui) {
        let visuals = ui.visuals_mut();
        let rounding = Rounding::same(2.0);

        visuals.extreme_bg_color = self.input_fill;
        visuals.widgets.noninteractive.rounding = rounding;
        visuals.widgets.inactive.rounding = rounding;
        visuals.widgets.hovered.rounding = rounding;
        visuals.widgets.active.rounding = rounding;
        visuals.widgets.open.rounding = rounding;
    }
}
