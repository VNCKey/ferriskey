use eframe::egui;
use crate::app::AppState;

pub fn mostrar_modal_template_creado(ctx: &egui::Context, state: &mut AppState) {
    let proj_name = match &state.project.created_project_name {
        Some(name) => name.clone(),
        None => return,
    };

    let mut open = true;
    egui::Window::new(format!("📦 Template {}", proj_name))
        .open(&mut open)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -410.0))
        .default_size([720.0, 260.0])
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            let is_lib = proj_name.contains("lib") || state.lessons.estructura_tab == 2;
            let src_file = if is_lib { "src/lib.rs" } else { "src/main.rs" };
            let src_desc = if is_lib {
                "Archivo raíz de la librería. No lleva fn main(), sino funciones y structs con pub."
            } else {
                "Archivo fuente principal ejecutable con la función de entrada fn main() { ... }."
            };

            ui.horizontal_top(|ui| {
                // Columna Izquierda: Tabla Desglose Template
                let mut info_frame = egui::Frame::new();
                info_frame.fill = egui::Color32::from_rgb(18, 22, 32);
                info_frame.inner_margin = egui::Margin::same(12);
                info_frame.corner_radius = egui::CornerRadius::same(8);
                info_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

                info_frame.show(ui, |ui| {
                    egui::Grid::new("desglose_template_grid_modal")
                        .striped(true)
                        .spacing([16.0, 8.0])
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Archivo / Carpeta").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                            ui.end_row();

                            ui.label(egui::RichText::new("Cargo.toml").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label("Manifiesto con metadatos de tu proyecto (nombre, versión, dependencias).");
                            ui.end_row();

                            ui.label(egui::RichText::new("Cargo.lock").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label("Registro de versiones fijadas de dependencias.");
                            ui.end_row();

                            ui.label(egui::RichText::new(src_file).monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(src_desc);
                            ui.end_row();

                            ui.label(egui::RichText::new("target/").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label("Carpeta binaria donde rustc compila los ejecutables.");
                            ui.end_row();
                        });
                });

                ui.add_space(15.0);

                // Columna Derecha: Imagen 7.png fija inamovible
                ui.add(
                    egui::Image::new(egui::include_image!("../../../../assets/taller/7.png"))
                        .fit_to_exact_size(egui::vec2(280.0, 180.0))
                        .corner_radius(8),
                );
            });
        });

    if !open {
        state.project.created_project_name = None;
        state.ui.show_cargo_output_modal.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
