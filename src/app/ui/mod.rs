use egui::{
    style::{Selection, Widgets},
    Color32, Stroke, Visuals,
};
use epaint::Shadow;

pub mod color {
    use egui::{Color32, Rgba};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref D_BG_000: Color32 = Color32::from_rgb(0x0e, 0x12, 0x17);
        pub static ref D_BG_00: Color32 = Color32::from_rgb(0x11, 0x16, 0x1b);
        pub static ref D_BG_0: Color32 = Color32::from_rgb(0x16, 0x1c, 0x23);
        pub static ref D_BG_1: Color32 = Color32::from_rgb(0x23, 0x2d, 0x38);
        pub static ref D_BG_2: Color32 = Color32::from_rgb(0x31, 0x3f, 0x4e);
        pub static ref D_BG_3: Color32 = Color32::from_rgb(0x41, 0x53, 0x67);
        pub static ref D_FG_0: Color32 = Color32::from_rgb(0xe5, 0xde, 0xd6);
        pub static ref D_BG_00_TRANSPARENT: Color32 = Rgba::from(*D_BG_00).multiply(0.96).into();
        pub static ref D_BG_0_TRANSPARENT: Color32 = Rgba::from(*D_BG_0).multiply(0.96).into();
        pub static ref D_BG_1_TRANSPARENT: Color32 = Rgba::from(*D_BG_1).multiply(0.96).into();
        pub static ref D_BG_2_TRANSPARENT: Color32 = Rgba::from(*D_BG_2).multiply(0.96).into();
        pub static ref D_BG_3_TRANSPARENT: Color32 = Rgba::from(*D_BG_3).multiply(0.96).into();
        pub static ref L_BG_0: Color32 = Color32::from_rgb(0xbf, 0xbf, 0xbf);
        pub static ref L_BG_1: Color32 = Color32::from_rgb(0xd4, 0xd3, 0xd4);
        pub static ref L_BG_2: Color32 = Color32::from_rgb(0xd9, 0xd9, 0xd9);
        pub static ref L_BG_3: Color32 = Color32::from_rgb(0xea, 0xea, 0xea);
        pub static ref L_BG_4: Color32 = Color32::from_rgb(0xf9, 0xf9, 0xf9);
        pub static ref L_BG_5: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
        pub static ref L_BG_0_TRANSPARENT: Color32 = Rgba::from(*L_BG_0).multiply(0.86).into();
        pub static ref L_BG_1_TRANSPARENT: Color32 = Rgba::from(*L_BG_1).multiply(0.86).into();
        pub static ref L_BG_2_TRANSPARENT: Color32 = Rgba::from(*L_BG_2).multiply(0.86).into();
        pub static ref L_BG_3_TRANSPARENT: Color32 = Rgba::from(*L_BG_3).multiply(0.86).into();
        pub static ref L_BG_4_TRANSPARENT: Color32 = Rgba::from(*L_BG_4).multiply(0.86).into();
        pub static ref L_BG_5_TRANSPARENT: Color32 = Rgba::from(*L_BG_5).multiply(0.86).into();
        pub static ref L_FG_0: Color32 = *D_BG_0;
    }
}

pub mod icon {
    pub const PACKAGE: &str = "\u{1F4E6}";
    pub const SCROLL: &str = "\u{1F4DC}";
    pub const INFO: &str = "\u{2139}";
    pub const DELETE: &str = "\u{1F5D9}";
    pub const PLAY: &str = "\u{25B6}";
    pub const PAUSE: &str = "\u{23F8}";
    pub const STOP: &str = "\u{23F9}";
    pub const SETTINGS: &str = "\u{2699}";
    pub const SAVE: &str = "\u{1F4BE}";
}

pub fn light_visuals() -> Visuals {
    use color::*;
    let mut widgets = Widgets::light();
    widgets.noninteractive.bg_fill = *L_BG_3_TRANSPARENT;
    widgets.inactive.bg_fill = *L_BG_3_TRANSPARENT;
    widgets.inactive.bg_stroke = Stroke::new(0.5, *D_BG_3);
    widgets.inactive.fg_stroke = Stroke::new(0.5, *D_BG_3);
    widgets.hovered.bg_fill = *L_BG_4_TRANSPARENT;
    widgets.hovered.bg_stroke = Stroke::new(1., *D_BG_1);
    widgets.hovered.fg_stroke = Stroke::new(1., *D_BG_1);
    widgets.active.bg_fill = *L_BG_5_TRANSPARENT;
    widgets.active.fg_stroke = Stroke::new(1.5, *D_BG_0);
    widgets.active.bg_stroke = Stroke::new(1.5, *D_BG_0);

    Visuals {
        dark_mode: false,
        extreme_bg_color: Color32::WHITE,
        selection: Selection {
            bg_fill: *L_BG_5,
            stroke: Stroke::new(0.7, *D_BG_0),
        },
        popup_shadow: Shadow::small_light(),
        widgets,
        ..Default::default()
    }
}

pub fn dark_visuals() -> Visuals {
    use color::*;
    let mut widgets = Widgets::dark();
    widgets.noninteractive.bg_fill = *D_BG_0_TRANSPARENT;
    widgets.inactive.bg_fill = *D_BG_1_TRANSPARENT;
    widgets.hovered.bg_fill = *D_BG_2_TRANSPARENT;
    widgets.active.bg_fill = *D_BG_0_TRANSPARENT;

    Visuals {
        dark_mode: true,
        extreme_bg_color: Color32::BLACK,
        selection: Selection {
            bg_fill: *D_BG_3_TRANSPARENT,
            stroke: Stroke::new(0.7, *D_FG_0),
        },
        popup_shadow: Shadow::small_dark(),
        widgets,
        ..Default::default()
    }
}

pub fn line(ui: &mut egui::Ui, frame: egui::Frame) -> egui::Response {
    frame
        .show(ui, |ui| {
            ui.set_max_height(1.);
            let available_space = ui.available_size();

            let size = egui::vec2(available_space.x, 0.);

            let (rect, response) = ui.allocate_at_least(size, egui::Sense::hover());
            let points = [
                egui::pos2(rect.left(), rect.bottom()),
                egui::pos2(rect.right(), rect.bottom()),
            ];

            let stroke = ui.visuals().widgets.noninteractive.fg_stroke;
            ui.painter().line_segment(points, stroke);
            response
        })
        .response
}

#[macro_export]
macro_rules! key {
    ($ui:ident, $k:expr) => {
        $ui.add(Label::new($k).strong());
    };
}

#[macro_export]
macro_rules! val {
    ($ui:ident, $v:expr) => {
        if $ui
            .add(Label::new($v).monospace().sense(egui::Sense {
                click: true,
                focusable: true,
                drag: false,
            }))
            .on_hover_text("secondary-click to copy")
            .secondary_clicked()
        {
            log::debug!("setting clipboard content to `{}`", $v);
            if let Err(e) = crate::save_to_clipboard($v.to_string()) {
                log::error!("failed to save content to clipboard - {}", e);
            }
        }
    };
}

#[macro_export]
macro_rules! key_val {
    ($ui:ident, $k:expr, $v:expr) => {
        key!($ui, $k);
        val!($ui, $v);
        $ui.end_row();
    };
}

pub use key;
pub use key_val;
pub use val;
