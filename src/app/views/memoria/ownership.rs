use eframe::egui;

pub fn mostrar_tab_ownership(ui: &mut egui::Ui) {
    ui.label(
        "Ownership es el sistema central de seguridad de memoria de Rust. Se rige por tres reglas simples verificadas en tiempo de compilación para garantizar cero fugas de memoria y prevenir el error de doble liberación (double free).",
    );
    ui.add_space(10.0);

    // Tabla Comparativa: Reglas de Ownership
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_reglas_ownership")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Regla de Oro")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis en Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Comportamiento en Memoria")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Garantía de Seguridad")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Un solo dueño
                ui.label(
                    egui::RichText::new("Owner")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("let s1 = String::from(\"a\");")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Cada valor tiene una única variable dueña.");
                ui.label("Evita punteros colgantes o compartición insegura.");
                ui.end_row();

                // Fila 2: Move
                ui.label(
                    egui::RichText::new("Move")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("let s2 = s1; // s1 queda inválido")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("El nuevo dueño es s2. s1 no puede volver a usarse.");
                ui.label("Previene double free al salir de scope.");
                ui.end_row();

                // Fila 3: Drop
                ui.label(
                    egui::RichText::new("Drop")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("} // fin del bloque léxico")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Al salir del scope, el valor se destruye automáticamente.");
                ui.label("Cero fugas de memoria (memory leaks).");
                ui.end_row();
            });
    });
}
