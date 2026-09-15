use crate::app::AppState;
use eframe::egui;

pub fn mostrar_modal_codigo(ctx: &egui::Context, state: &mut AppState) {
    if let Some((title, code)) = state.ui.show_code_modal.clone() {
        let mut abierto = true;
        egui::Window::new(
            egui::RichText::new(&title)
                .strong()
                .size(16.0)
                .color(egui::Color32::from_rgb(255, 160, 50)),
        )
        .open(&mut abierto)
        .collapsible(false)
        .resizable(true)
        .default_size([520.0, 300.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.add_space(6.0);
            let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
            let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
                crate::components::code_editor::rust_layouter(
                    ui,
                    text.as_str(),
                    wrap_width,
                    &state.editor.syntax_set,
                    theme,
                )
            };

            let mut code_mut = code.clone();
            let mut frame = egui::Frame::new();
            frame.fill = egui::Color32::from_rgb(13, 17, 23);
            frame.inner_margin = egui::Margin::same(12);
            frame.corner_radius = egui::CornerRadius::same(6);

            frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                egui::ScrollArea::vertical()
                    .max_height(240.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut code_mut)
                                .frame(egui::Frame::NONE)
                                .layouter(&mut layouter)
                                .code_editor()
                                .interactive(false)
                                .desired_width(f32::INFINITY),
                        );
                    });
            });
        });

        if !abierto {
            state.ui.show_code_modal = None;
        }
    }
}
