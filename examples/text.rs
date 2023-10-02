use eframe::{App, Frame, NativeOptions};
use egui::text::LayoutJob;
use egui::{CentralPanel, ComboBox, Context, FontFamily, FontId, Slider, TextFormat};

fn main() {
    eframe::run_native(
        "egui_ui_refresh - text rendering",
        NativeOptions {
            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(egui_ui_refresh::fonts());
            cc.egui_ctx.set_style(egui_ui_refresh::style());

            Box::new(ExampleApp {
                font_family: FontFamily::Proportional,
                font_size: 30.0,
                extra_spacing: false,
            })
        }),
    )
    .unwrap();
}

struct ExampleApp {
    font_family: FontFamily,
    font_size: f32,
    extra_spacing: bool,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(Slider::new(&mut self.font_size, 6.0..=80.0));
            ComboBox::new("font-family", "Font family")
                .selected_text(format!("{}", self.font_family))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.font_family,
                        FontFamily::Proportional,
                        "Proportional",
                    );
                    ui.selectable_value(&mut self.font_family, FontFamily::Monospace, "Monospace");
                    ui.selectable_value(
                        &mut self.font_family,
                        FontFamily::Name(egui_ui_refresh::ITALIC.into()),
                        egui_ui_refresh::ITALIC,
                    );
                    ui.selectable_value(
                        &mut self.font_family,
                        FontFamily::Name(egui_ui_refresh::MEDIUM.into()),
                        egui_ui_refresh::MEDIUM,
                    );
                });
            ui.add_space(4.0);
            ui.separator();

            let job = LayoutJob::single_section(
                "This is an example text".to_string(),
                TextFormat {
                    font_id: FontId::new(self.font_size, self.font_family.clone()),
                    extra_letter_spacing: 0.0,
                    line_height: None,
                    color: ui.style().visuals.text_color(),
                    background: Default::default(),
                    italics: false,
                    underline: Default::default(),
                    strikethrough: Default::default(),
                    valign: Default::default(),
                },
            );
            ui.label(job);
        });
    }
}
