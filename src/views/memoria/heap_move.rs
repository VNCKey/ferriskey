use crate::app::AppState;
use eframe::egui;

pub fn mostrar_tab_heap_move(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        "La memoria Heap (Montículo) se utiliza para almacenar datos dinámicos cuyo tamaño puede cambiar en tiempo de ejecución. Al asignar un String, la CPU guarda 24 bytes en el Stack (ptr, len, cap) que apuntan al buffer real en el Heap.",
    );
    ui.add_space(10.0);

    // Tabla Informativa: Heap & Move
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_heap_move")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Concepto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Detalle Técnico")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    let btn_color = if state.ui.show_railroad_modal == Some(7) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };
                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../assets/diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color),
                            )
                            .frame(state.ui.show_railroad_modal == Some(7)),
                        )
                        .on_hover_text("Ver diagrama visual de la arquitectura del Heap")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(7) {
                            None
                        } else {
                            Some(7)
                        };
                    }
                });
                ui.label(
                    egui::RichText::new("Comportamiento")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Asignación en Heap
                ui.label(
                    egui::RichText::new("Reserva Dinámica (Heap)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Solicitada al Sistema Operativo");
                ui.label("Almacena texto variable (ej: String::from(\"Hola\")).");
                ui.end_row();

                // Fila 2: Metadata en Stack
                ui.label(
                    egui::RichText::new("Metadata (24 Bytes)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("ptr (8b) + len (8b) + cap (8b)");
                ui.label("La tarjeta de control vive en Stack y apunta a los datos en Heap.");
                ui.end_row();

                // Fila 3: Transferencia (Move)
                ui.label(
                    egui::RichText::new("Semántica 'Move'")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Transferencia de propiedad al asignar");
                ui.label("Al hacer 'let s2 = s1;', la metadata se copia y 's1' queda inválida.");
                ui.end_row();

                // Fila 4: Prevención Double Free
                ui.label(
                    egui::RichText::new("Seguridad de Memoria")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Evita Double Free (Doble liberación)");
                ui.label(
                    "Rust garantiza que solo 1 variable ejecute drop() sobre la memoria Heap.",
                );
                ui.end_row();
            });
    });
}
