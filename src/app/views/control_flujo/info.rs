use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_control_flujo_info(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Información teórica y resumen comparativo de las construcciones de Control de Flujo en Rust.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Conceptos Clave
    ui.label(
        egui::RichText::new("Conceptos Clave de Control de Flujo")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_control_flujo_conceptos_info")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Concepto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo Sintáctico")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Descripción")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("Expresiones vs Sentencias")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("let x = if c { 5 } else { 10 };")
                        .monospace()
                        .color(cyan),
                );
                ui.label("En Rust 'if' y 'match' son expresiones que retornan un valor asignable.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Retorno desde loop")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("let res = loop { break 42; };")
                        .monospace()
                        .color(cyan),
                );
                ui.label("El comando break puede devolver un valor desde un bucle loop.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Exhaustividad en match")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("match val { ... _ => () }")
                        .monospace()
                        .color(cyan),
                );
                ui.label("El compilador exige evaluar absolutamente todas las variantes posibles.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Tabla Comparativa de Construcciones
    ui.label(
        egui::RichText::new("Tabla Comparativa de Construcciones")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_control_flujo_comp_info")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Construcción")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Evaluación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Retorno de Valor")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Caso de Uso Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(egui::RichText::new("if / else").strong().color(naranja));
                ui.label("Condición Booleana");
                ui.label("Sí (en ramas idénticas)");
                ui.label("Bifurcación simple o múltiple basada en booleanos.");
                ui.end_row();

                ui.label(egui::RichText::new("loop").strong().color(naranja));
                ui.label("Infinita");
                ui.label("Sí (mediante break val)");
                ui.label("Reintento de operaciones, servidores o workers continuos.");
                ui.end_row();

                ui.label(egui::RichText::new("while").strong().color(naranja));
                ui.label("Condición Booleana");
                ui.label("No");
                ui.label("Repetición mientras una condición siga siendo verdadera.");
                ui.end_row();

                ui.label(egui::RichText::new("for").strong().color(naranja));
                ui.label("Iterador / Rango");
                ui.label("No");
                ui.label("Iteración segura y finita sobre rangos y colecciones.");
                ui.end_row();

                ui.label(egui::RichText::new("match").strong().color(naranja));
                ui.label("Pattern Matching");
                ui.label("Sí");
                ui.label("Desestructuración exhaustiva de patrones y tipos de datos.");
                ui.end_row();
            });
    });
}
