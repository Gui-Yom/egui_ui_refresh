use egui::{Button, Color32, Context, Frame, RichText, Ui, ViewportCommand};

use crate::RefreshedTheme;

fn top_panel_frame(theme: &RefreshedTheme, custom_decorations: bool) -> Frame {
    let mut frame = Frame {
        inner_margin: theme.top_bar_margin,
        fill: Color32::from_gray(20),
        ..Default::default()
    };
    if custom_decorations {
        frame.rounding.nw = theme.native_window_rounding;
        frame.rounding.ne = theme.native_window_rounding;
    }
    frame
}

pub struct TopBarStyle {
    /// Height of the top bar
    pub height: f32,

    /// Extra horizontal space in the top left corner to make room for
    /// close/minimize/maximize buttons (on Mac)
    pub indent: f32,
}

fn top_bar_style(theme: &RefreshedTheme, ctx: &Context, style_like_web: bool) -> TopBarStyle {
    let egui_zoom_factor = ctx.zoom_factor();
    let fullscreen = ctx.input(|i| i.viewport().fullscreen).unwrap_or(false);

    // On Mac, we share the same space as the native red/yellow/green close/minimize/maximize buttons.
    // This means we need to make room for them.
    let make_room_for_window_buttons = !style_like_web && {
        #[cfg(target_os = "macos")]
        {
            crate::FULLSIZE_CONTENT && !fullscreen
        }
        #[cfg(not(target_os = "macos"))]
        {
            _ = fullscreen;
            false
        }
    };

    let native_buttons_size_in_native_scale = egui::vec2(64.0, 24.0); // source: I measured /emilk

    let height = if make_room_for_window_buttons {
        // On mac we want to match the height of the native red/yellow/green close/minimize/maximize buttons.
        // TODO(emilk): move the native window buttons to match our Self::title_bar_height

        // Use more vertical space when zoomed in…
        let height = native_buttons_size_in_native_scale.y;

        // …but never shrink below the native button height when zoomed out.
        height.max(native_buttons_size_in_native_scale.y / egui_zoom_factor)
    } else {
        theme.top_bar_height - theme.top_bar_margin.sum().y
    };

    let indent = if make_room_for_window_buttons {
        // Always use the same width measured in native GUI coordinates:
        native_buttons_size_in_native_scale.x / egui_zoom_factor
    } else {
        0.0
    };

    TopBarStyle { height, indent }
}

pub fn top_bar(egui_ctx: &Context, custom_decorations: bool) {
    let theme = RefreshedTheme::get();
    let top_bar_style = top_bar_style(theme, egui_ctx, false);

    egui::TopBottomPanel::top("top_bar")
        .frame(top_panel_frame(theme, custom_decorations))
        .exact_height(top_bar_style.height)
        .show(egui_ctx, |ui| {
            #[cfg(not(target_arch = "wasm32"))]
            if custom_decorations {
                // Interact with background first, so that buttons in the top bar gets input priority
                // (last added widget has priority for input).
                let title_bar_response = ui.interact(
                    ui.max_rect(),
                    ui.id().with("background"),
                    egui::Sense::click(),
                );
                if title_bar_response.double_clicked() {
                    let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
                } else if title_bar_response.is_pointer_button_down_on() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
                }
            }

            egui::menu::bar(ui, |ui| {
                ui.set_height(top_bar_style.height);
                ui.add_space(top_bar_style.indent);

                ui.menu_button("File", |ui| {} /*file_menu(ui, &self.command_sender)*/);

                top_bar_ui(ui, custom_decorations);
            });
        });
}

fn top_bar_ui(ui: &mut Ui, custom_decorations: bool) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // From right-to-left:

        if custom_decorations {
            ui.add_space(8.0);

            let button_height = 12.0;

            let close_response = ui
                .add(Button::new(RichText::new("\u{E4F6}").size(button_height)))
                .on_hover_text("Close the window");
            if close_response.clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }

            let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            if maximized {
                let maximized_response = ui
                    .add(Button::new(RichText::new("🗗").size(button_height)))
                    .on_hover_text("Restore window");
                if maximized_response.clicked() {
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::Maximized(false));
                }
            } else {
                let maximized_response = ui
                    .add(Button::new(RichText::new("\u{E626}").size(button_height)))
                    .on_hover_text("Maximize window");
                if maximized_response.clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
                }
            }

            let minimized_response = ui
                .add(Button::new(RichText::new("🗕").size(button_height)))
                .on_hover_text("Minimize the window");
            if minimized_response.clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
            }
            ui.separator();
        } else {
            ui.add_space(16.0);
        }

        // ui.medium_icon_toggle_button(
        //     &re_ui::icons::RIGHT_PANEL_TOGGLE,
        //     &mut self.show_right_panel,
        // );
        // ui.medium_icon_toggle_button(
        //     &re_ui::icons::BOTTOM_PANEL_TOGGLE,
        //     &mut self.show_bottom_panel,
        // );
        // ui.medium_icon_toggle_button(
        //     &re_ui::icons::LEFT_PANEL_TOGGLE,
        //     &mut self.show_left_panel,
        // );
    });
}
