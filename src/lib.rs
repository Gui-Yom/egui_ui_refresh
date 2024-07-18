use std::sync::OnceLock;

use egui::{Color32, Context, NumExt, Rounding, Stroke, Style, Visuals};
use egui::{Frame, Margin, TextStyle, Vec2};
use egui::epaint::Shadow;
use egui::style::NumberFormatter;

// pub mod rerun;
pub mod top_bar;
#[cfg(feature = "toasts")]
pub mod toasts;
#[cfg(feature = "modal")]
pub mod modal;
/// A few codepoints from Phosphor icons
pub mod icons;
/// New fonts packaged with this crate
pub mod fonts;

static REFRESHED_THEME: OnceLock<RefreshedTheme> = OnceLock::new();

#[derive(Debug)]
pub struct RefreshedTheme {
    pub bottom_bar_color: Color32,
    pub tab_bar_color: Color32,
    pub expansion: f32,
    pub top_bar_color: Color32,
    pub bottom_bar_stroke: Stroke,
    pub bottom_bar_rounding: Rounding,
    pub shadow_gradient_dark_start: Color32,
    pub panel_bg_color: Color32,
    pub selection: Color32,
    pub inactive: Color32,
    pub noninteractive: Color32,
    pub active: Color32,
    pub normal_text_size: f32,
    pub heading_text_size: f32,
    pub view_padding: f32,
    pub window_rounding: f32,
    pub normal_rounding: f32,
    pub small_rounding: f32,
    pub top_bar_margin: Margin,
    pub text_to_icon_padding: f32,
    pub top_bar_height: f32,
    pub title_bar_height: f32,
    pub list_item_height: f32,
    pub native_window_rounding: f32,
}

impl RefreshedTheme {
    pub const fn new() -> Self {
        Self {
            normal_text_size: 12.0,
            heading_text_size: 16.0,
            bottom_bar_color: Color32::from_rgb(0x14, 0x18, 0x19),
            tab_bar_color: Color32::from_rgb(0x18, 0x1c, 0x1e),
            panel_bg_color: Color32::from_rgb(0x0d, 0x10, 0x11),
            selection: Color32::from_rgb(0x00, 0x3d, 0xa1),
            inactive: Color32::from_rgb(0xca, 0xd8, 0xde),
            noninteractive: Color32::from_rgb(0x7d, 0x8c, 0x92),
            active: Color32::WHITE,
            expansion: 2.0,
            top_bar_color: Color32::from_gray(20),
            bottom_bar_stroke: Stroke { width: 1.0, color: Color32::from_gray(47) },
            bottom_bar_rounding: Rounding {
                nw: 6.0,
                ne: 6.0,
                sw: 0.0,
                se: 0.0,
            },
            shadow_gradient_dark_start: Color32::from_black_alpha(77),
            view_padding: 12.0,
            window_rounding: 12.0,
            normal_rounding: 6.0,
            small_rounding: 4.0,
            top_bar_margin: Margin::symmetric(8.0, 2.0),
            text_to_icon_padding: 4.0,
            top_bar_height: 28.0,
            title_bar_height: 24.0,
            list_item_height: 24.0,
            native_window_rounding: 10.0,
        }
    }
}

impl Default for RefreshedTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl RefreshedTheme {
    pub fn init_default() -> &'static Self {
        REFRESHED_THEME.get_or_init(|| RefreshedTheme::new())
    }

    pub fn init(self) -> &'static Self {
        REFRESHED_THEME.get_or_init(|| self)
    }

    pub fn get() -> &'static Self {
        REFRESHED_THEME.get().unwrap()
    }

    /// Apply style to the given egui context.
    pub fn apply(&self, ctx: &Context) {
        let mut egui_style = Style {
            visuals: Visuals::dark(),
            number_formatter: NumberFormatter::new(format_with_decimals_in_range),
            ..Default::default()
        };

        for text_style in [TextStyle::Body, TextStyle::Monospace, TextStyle::Button] {
            egui_style.text_styles.get_mut(&text_style).unwrap().size = self.normal_text_size;
        }

        egui_style
            .text_styles
            .get_mut(&TextStyle::Heading)
            .unwrap()
            .size = self.heading_text_size;

        // We want labels and buttons to have the same height.
        // Intuitively, we would just assign font_size to
        // the interact_size, but in practice text height does not match
        // font size (for unknown reason), so we fudge it for now:

        egui_style.spacing.interact_size.y = self.normal_text_size + 3.0;
        // egui_style.spacing.interact_size.y = font_size;

        // Used as the background of text edits, scroll bars and others things
        // that needs to look different from other interactive stuff.
        // We need this very dark, since the theme overall is very, very dark.
        egui_style.visuals.extreme_bg_color = Color32::BLACK;

        egui_style.visuals.widgets.noninteractive.weak_bg_fill = self.panel_bg_color;
        egui_style.visuals.widgets.noninteractive.bg_fill = self.panel_bg_color;

        egui_style.visuals.button_frame = true;
        egui_style.visuals.widgets.inactive.weak_bg_fill = Default::default(); // Buttons have no background color when inactive
        egui_style.visuals.widgets.inactive.bg_fill = Color32::from_gray(50); // Fill of unchecked radio buttons, checkboxes, etc. Must be brigher than the background floating_color.

        {
            // Background colors for buttons (menu buttons, blueprint buttons, etc) when hovered or clicked
            let hovered_color = Color32::from_gray(64);
            egui_style.visuals.widgets.hovered.weak_bg_fill = hovered_color;
            egui_style.visuals.widgets.hovered.bg_fill = hovered_color;
            egui_style.visuals.widgets.active.weak_bg_fill = hovered_color;
            egui_style.visuals.widgets.active.bg_fill = hovered_color;
            egui_style.visuals.widgets.open.weak_bg_fill = hovered_color;
            egui_style.visuals.widgets.open.bg_fill = hovered_color;
        }

        {
            // Turn off strokes around buttons:
            egui_style.visuals.widgets.inactive.bg_stroke = Default::default();
            egui_style.visuals.widgets.hovered.bg_stroke = Default::default();
            egui_style.visuals.widgets.active.bg_stroke = Default::default();
            egui_style.visuals.widgets.open.bg_stroke = Default::default();
        }

        {
            egui_style.visuals.widgets.hovered.expansion = self.expansion;
            egui_style.visuals.widgets.active.expansion = self.expansion;
            egui_style.visuals.widgets.open.expansion = self.expansion;
        }

        egui_style.visuals.selection.bg_fill = self.selection;
        egui_style.visuals.selection.stroke.color = Color32::from_rgb(173, 184, 255); // Brighter version of the above

        egui_style.visuals.widgets.noninteractive.bg_stroke.color = Color32::from_gray(30); // separator lines, panel lines, etc

        egui_style.visuals.widgets.inactive.fg_stroke.color = self.inactive; // button text
        egui_style.visuals.widgets.noninteractive.fg_stroke.color = self.noninteractive; // non-interactive text
        egui_style.visuals.widgets.active.fg_stroke.color = self.active; // strong text and active button text

        let wide_stroke_width = 2.0; // Make it a bit more visible, especially important for spatial primitives.
        egui_style.visuals.widgets.active.fg_stroke.width = wide_stroke_width;
        egui_style.visuals.selection.stroke.width = wide_stroke_width;

        // From figma
        let shadow = Shadow {
            offset: Vec2::new(0.0, 15.0),
            blur: 50.0,
            spread: 0.0,
            color: Color32::from_black_alpha(128),
        };
        egui_style.visuals.popup_shadow = shadow;
        egui_style.visuals.window_shadow = shadow;

        egui_style.visuals.window_fill = Color32::from_gray(35); // tooltips and menus
        egui_style.visuals.window_stroke = Stroke::NONE;
        egui_style.visuals.panel_fill = self.panel_bg_color;

        egui_style.visuals.window_rounding = self.window_rounding.into();
        egui_style.visuals.menu_rounding = self.window_rounding.into();
        let small_rounding = self.small_rounding.into();
        egui_style.visuals.widgets.noninteractive.rounding = small_rounding;
        egui_style.visuals.widgets.inactive.rounding = small_rounding;
        egui_style.visuals.widgets.hovered.rounding = small_rounding;
        egui_style.visuals.widgets.active.rounding = small_rounding;
        egui_style.visuals.widgets.open.rounding = small_rounding;

        egui_style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        egui_style.spacing.menu_margin = self.view_padding.into();
        egui_style.spacing.menu_spacing = 1.0;

        // avoid some visual glitches with the default non-zero value
        egui_style.visuals.clip_rect_margin = 0.0;

        // Add stripes to grids and tables?
        egui_style.visuals.striped = false;
        egui_style.visuals.indent_has_left_vline = false;
        egui_style.spacing.button_padding = Vec2::new(1.0, 0.0); // Makes the icons in the blueprint panel align
        egui_style.spacing.indent = 14.0;

        egui_style.spacing.combo_width = 8.0; // minimum width of ComboBox - keep them small, with the down-arrow close.

        egui_style.spacing.scroll.bar_inner_margin = 2.0;
        egui_style.spacing.scroll.bar_width = 6.0;
        egui_style.spacing.scroll.bar_outer_margin = 2.0;

        egui_style.spacing.tooltip_width = 720.0;

        // don't color hyperlinks #2733
        egui_style.visuals.hyperlink_color = self.inactive;

        egui_style.visuals.image_loading_spinners = false;

        ctx.set_style(egui_style);
    }

    pub fn panel_margin(&self) -> Margin {
        Margin::symmetric(self.view_padding, 0.0)
    }

    pub fn table_line_height(&self) -> f32 {
        self.normal_text_size + 3.0 + 1.0
    }

    pub fn table_header_height(&self) -> f32 {
        self.heading_text_size + 3.0 + 1.0
    }

    /// For the streams view (time panel)
    pub fn bottom_panel_frame(&self, custom_decorations: bool) -> Frame {
        // Show a stroke only on the top. To achieve this, we add a negative outer margin.
        // (on the inner margin we counteract this again)
        let margin_offset = self.bottom_bar_stroke.width * 0.5;

        let margin = self.top_bar_margin;

        let mut frame = Frame {
            fill: self.bottom_bar_color,
            inner_margin: margin + margin_offset,
            outer_margin: Margin {
                left: -margin_offset,
                right: -margin_offset,
                // Add a proper stoke width thick margin on the top.
                top: self.bottom_bar_stroke.width,
                bottom: -margin_offset,
            },
            stroke: self.bottom_bar_stroke,
            rounding: self.bottom_bar_rounding,
            ..Default::default()
        };
        if custom_decorations {
            frame.rounding.sw = self.native_window_rounding;
            frame.rounding.se = self.native_window_rounding;
        }
        frame
    }

    pub fn small_icon_size() -> Vec2 {
        Vec2::splat(14.0)
    }
}

fn format_with_decimals_in_range(
    value: f64,
    decimal_range: std::ops::RangeInclusive<usize>,
) -> String {
    fn format_with_decimals(value: f64, decimals: usize) -> String {
        re_format::FloatFormatOptions::DEFAULT_f64
            .with_decimals(decimals)
            .with_strip_trailing_zeros(false)
            .format(value)
    }

    let epsilon = 16.0 * f32::EPSILON; // margin large enough to handle most peoples round-tripping needs

    let min_decimals = *decimal_range.start();
    let max_decimals = *decimal_range.end();
    debug_assert!(min_decimals <= max_decimals);
    debug_assert!(max_decimals < 100);
    let max_decimals = max_decimals.at_most(16);
    let min_decimals = min_decimals.at_most(max_decimals);

    if min_decimals < max_decimals {
        // Try using a few decimals as possible, and then add more until we have enough precision
        // to round-trip the number.
        for decimals in min_decimals..max_decimals {
            let text = format_with_decimals(value, decimals);
            if let Some(parsed) = re_format::parse_f64(&text) {
                if egui::emath::almost_equal(parsed as f32, value as f32, epsilon) {
                    // Enough precision to show the value accurately - good!
                    return text;
                }
            }
        }
        // The value has more precision than we expected.
        // Probably the value was set not by the slider, but from outside.
        // In any case: show the full value
    }

    // Use max decimals
    format_with_decimals(value, max_decimals)
}
