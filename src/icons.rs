use egui::RichText;

use crate::fonts::font_icons;

pub const CLOSE: &'static str = "\u{E4F6}";
pub const INFO: &'static str = "\u{E2CE}";
pub const WARNING: &'static str = "\u{E4E0}";
pub const WARNING_CIRCLE: &'static str = "\u{E4E2}";
pub const CHECK: &'static str = "\u{E182}";
pub const EXCLAMATION_MARK: &'static str = "\u{EE44}";

pub fn icon(icon: impl Into<String>, size: f32) -> RichText {
    RichText::new(icon).font(font_icons(size)).line_height(Some(size - 1.0))
}
