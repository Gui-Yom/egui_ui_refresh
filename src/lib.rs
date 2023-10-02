use egui::epaint::Shadow;
use egui::style::{WidgetVisuals, Widgets};
use egui::{Color32, FontData, FontDefinitions, FontFamily, Rounding, Stroke, Style, Visuals};

pub const ITALIC: &str = "italic";
pub const MEDIUM: &str = "medium";

pub fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::empty();
    fonts.font_data.insert(
        "Inter".to_string(),
        FontData::from_static(include_bytes!("inter/Inter-Regular.ttf")),
    );
    fonts.font_data.insert(
        "Inter-Italic".to_string(),
        FontData::from_static(include_bytes!("inter/Inter-Italic.ttf")),
    );
    fonts.font_data.insert(
        "Inter-Medium".to_string(),
        FontData::from_static(include_bytes!("inter/Inter-Medium.ttf")),
    );
    fonts.font_data.insert(
        "Jetbrains Mono".to_string(),
        FontData::from_static(include_bytes!("jetbrainsmono/JetBrainsMonoNL-Regular.ttf")),
    );

    // Font families
    fonts
        .families
        .insert(FontFamily::Proportional, vec!["Inter".to_owned()]);
    fonts.families.insert(
        FontFamily::Name(ITALIC.into()),
        vec!["Inter-Italic".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(MEDIUM.into()),
        vec!["Inter-Medium".to_owned()],
    );
    fonts
        .families
        .insert(FontFamily::Monospace, vec!["Jetbrains Mono".to_owned()]);

    fonts
}

pub fn style() -> Style {
    Style {
        visuals: Visuals {
            slider_trailing_fill: true,
            menu_rounding: Rounding::same(2.0),
            popup_shadow: Shadow {
                extrusion: 8.0,
                ..Shadow::default()
            },
            window_rounding: Rounding::same(2.0),
            window_shadow: Shadow {
                extrusion: 12.0,
                ..Shadow::default()
            },
            widgets: Widgets {
                hovered: WidgetVisuals {
                    weak_bg_fill: Color32::from_gray(70),
                    bg_fill: Color32::from_gray(70),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
                    fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
                    rounding: Rounding::same(2.0),
                    expansion: 1.0,
                },
                noninteractive: WidgetVisuals {
                    weak_bg_fill: Color32::from_gray(20),
                    bg_fill: Color32::from_gray(20),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),         // normal text color
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
                ..Widgets::default()
            },
            ..Visuals::default()
        },
        ..Style::default()
    }
}
