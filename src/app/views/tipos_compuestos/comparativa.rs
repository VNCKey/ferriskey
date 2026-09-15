use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

#[allow(dead_code)]
pub fn mostrar_compuesto_comparar(
    ui: &mut egui::Ui,
    naranja: egui::Color32,
    _cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new("Resumen comparativo de las estructuras compuestas básicas en Rust:")
            .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_comparativa_comp")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("¿Tipos Mixtos?")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("¿Tamaño Dinámico?")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ubicación Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(egui::RichText::new("Array").strong().color(naranja));
                ui.label("❌ No (mismo tipo T)");
                ui.label("❌ No (fijo N)");
                ui.label("Stack");
                ui.end_row();

                ui.label(egui::RichText::new("Slice").strong().color(naranja));
                ui.label("❌ No (mismo tipo T)");
                ui.label("✅ Sí (vista dinámica)");
                ui.label("Referencia a Stack o Heap");
                ui.end_row();

                ui.label(egui::RichText::new("Tupla").strong().color(naranja));
                ui.label("✅ Sí (tipos variados)");
                ui.label("❌ No (fijo)");
                ui.label("Stack");
                ui.end_row();
            });
    });
}
