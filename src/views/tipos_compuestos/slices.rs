use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

#[allow(dead_code)]
pub fn mostrar_compuesto_slice(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un Slice es una vista prestada y dinámica sobre una secuencia contigua de elementos de un Array o Vector. Consiste en un puntero y una longitud.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_slice_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Operación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
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
                    egui::RichText::new("Crear Slice")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("&arr[1..4]").monospace().color(cyan));
                ui.label("Crea una vista desde el índice 1 hasta el 3 (exclusivo).");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Slice Completo")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("&arr[..]").monospace().color(cyan));
                ui.label("Referencia a todos los elementos del contenedor.");
                ui.end_row();
            });
    });
}
