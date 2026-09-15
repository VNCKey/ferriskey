use crate::app::AppState;
use crate::components::console_output::formatear_salida_consola;
use eframe::egui;
use std::sync::atomic::Ordering;

pub fn mostrar_modal_salida_cargo(ctx: &egui::Context, state: &mut AppState) {
    if !state.ui.show_cargo_output_modal.load(Ordering::Relaxed) {
        return;
    }

    let mut open = true;
    egui::Window::new("Salida")
        .open(&mut open)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(true)
        .default_size([650.0, 420.0])
        .collapsible(false)
        .show(ctx, |ui| {
            let output_text = state.obtener_output_activo().lock().unwrap().clone();

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    let mut out_frame = egui::Frame::new();
                    out_frame.fill = egui::Color32::from_rgb(10, 12, 18);
                    out_frame.inner_margin = egui::Margin::same(12);
                    out_frame.corner_radius = egui::CornerRadius::same(6);
                    out_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 55, 80));

                    out_frame.show(ui, |ui| {
                        ui.set_min_width(ui.available_width());

                        if output_text == "Compilando con Cargo..." {
                            ui.label(
                                egui::RichText::new(&output_text)
                                    .color(egui::Color32::YELLOW)
                                    .monospace(),
                            );
                        } else if let Some(idx) = output_text.find("[Errores/Warnings]:\n") {
                            let (stdout, stderr) = output_text.split_at(idx);
                            if !stdout.is_empty() {
                                ui.label(formatear_salida_consola(stdout, false));
                                ui.add_space(5.0);
                                ui.separator();
                                ui.add_space(5.0);
                            }
                            let solo_error = stderr
                                .strip_prefix("[Errores/Warnings]:\n")
                                .unwrap_or(stderr);
                            ui.label(formatear_salida_consola(solo_error, true));
                        } else if output_text.starts_with("Error") {
                            ui.label(formatear_salida_consola(&output_text, true));
                        } else {
                            ui.label(formatear_salida_consola(&output_text, false));
                        }
                    });
                });
        });

    if !open {
        state
            .ui
            .show_cargo_output_modal
            .store(false, Ordering::Relaxed);
    }
}
