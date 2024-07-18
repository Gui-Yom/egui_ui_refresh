use eframe::{App, Frame, NativeOptions};
use egui::{CentralPanel, Context};

use egui_ui_refresh::modal::{Modal, ModalHandler};
use egui_ui_refresh::RefreshedTheme;
use egui_ui_refresh::toasts::{Toast, ToastKind, ToastOptions, Toasts};
use egui_ui_refresh::top_bar::top_bar;

fn main() {
    eframe::run_native(
        "egui_ui_refresh - text rendering",
        NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_app_id("egui ui refresh - window example")
                .with_decorations(false) // Maybe hide the OS-specific "chrome" around the window
                .with_fullsize_content_view(true)
                .with_inner_size([1200.0, 800.0])
                .with_title_shown(false)
                .with_titlebar_buttons_shown(false)
                .with_titlebar_shown(false)
                .with_transparent(true), // To have rounded corners without decorations we need transparency

            follow_system_theme: false,
            default_theme: eframe::Theme::Dark,

            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(egui_ui_refresh::fonts::fonts());
            RefreshedTheme::init_default().apply(&cc.egui_ctx);

            Ok(Box::new(ExampleApp {
                toasts: Toasts::new(),
                modal_handler: Default::default(),
            }))
        }),
    )
        .unwrap();
}

struct ExampleApp {
    toasts: Toasts,
    modal_handler: ModalHandler,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        top_bar(ctx, true);

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Toast").clicked() {
                self.toasts.add(Toast {
                    kind: ToastKind::Info,
                    text: "toast content".to_string(),
                    options: ToastOptions { show_icon: true, ttl_sec: 3.0 },
                });
            }

            if ui.button("Modal").clicked() {
                self.modal_handler.open();
            }
            self.modal_handler.ui(ctx, || { Modal::new("Modal") }, |ui, close| {
                ui.label("modal content")
            });
        });

        self.toasts.show(ctx);
    }
}
