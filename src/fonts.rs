use std::sync::OnceLock;

use egui::{FontData, FontDefinitions, FontFamily, FontId, FontTweak};

pub fn font_family_italic() -> FontFamily {
    static FF: OnceLock<FontFamily> = OnceLock::new();
    FF.get_or_init(|| FontFamily::Name("italic".into())).clone()
}

pub fn font_family_medium() -> FontFamily {
    static FF: OnceLock<FontFamily> = OnceLock::new();
    FF.get_or_init(|| FontFamily::Name("medium".into())).clone()
}

pub fn font_family_icons() -> FontFamily {
    static FF: OnceLock<FontFamily> = OnceLock::new();
    FF.get_or_init(|| FontFamily::Name("icons".into())).clone()
}

pub fn font_italic(size: f32) -> FontId {
    FontId::new(size, font_family_italic())
}

pub fn font_medium(size: f32) -> FontId {
    FontId::new(size, font_family_medium())
}

pub fn font_icons(size: f32) -> FontId {
    FontId::new(size, font_family_icons())
}

pub fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::empty();
    fonts.font_data.insert(
        "Inter-Regular".to_string(),
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
    fonts.font_data.insert(
        "Phosphor".to_string(),
        FontData::from_static(include_bytes!("phosphor/Phosphor.ttf")),
    );
    fonts.font_data.insert(
        "NotoEmoji".to_string(),
        FontData::from_static(include_bytes!("noto/NotoEmoji-Regular.ttf")).tweak(
            FontTweak {
                scale: 0.8, // make it smaller
                ..Default::default()
            },
        ),
    );

    // Font families
    fonts.families.insert(
        FontFamily::Proportional,
        vec![
            "Inter-Regular".to_owned(),
            "NotoEmoji".to_owned(),
            "Phosphor".to_owned(),
        ],
    );
    fonts.families.insert(
        font_family_italic(),
        vec![
            "Inter-Italic".to_owned(),
            "NotoEmoji".to_owned(),
            "Phosphor".to_owned(),
        ],
    );
    fonts.families.insert(
        font_family_medium(),
        vec![
            "Inter-Medium".to_owned(),
            "NotoEmoji".to_owned(),
            "Phosphor".to_owned(),
        ],
    );
    fonts.families.insert(
        FontFamily::Monospace,
        vec![
            "Jetbrains Mono".to_owned(),
            "NotoEmoji".to_owned(),
            "Phosphor".to_owned(),
        ],
    );
    fonts.families.insert(
        font_family_icons(),
        vec!["Phosphor".to_owned()],
    );

    fonts
}
