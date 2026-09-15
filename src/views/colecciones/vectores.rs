use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_coleccion_vectores(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un Vector es una lista secuencial de elementos del mismo tipo almacenada en el Heap que puede crecer o reducirse dinámicamente en tiempo de ejecución.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    // Tabla de Operaciones Principales de Vectores
    ui.label(
        egui::RichText::new("Operaciones Principales de Vectores")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(8.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_vec_operaciones")
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

                ui.label(egui::RichText::new("Crear con macro").strong().color(texto));
                ui.label(
                    egui::RichText::new("let v = vec![1, 2, 3];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Crea un Vector inicializado en el Heap.");
                ui.end_row();

                ui.label(egui::RichText::new("Añadir al final").strong().color(texto));
                ui.label(
                    egui::RichText::new("v.push(valor);")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Inserta un elemento al final.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Extraer el último")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new("v.pop();").monospace().color(cyan));
                ui.label("Elimina y devuelve el último elemento en Option<T>.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Acceso por índice")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new("v[i]").monospace().color(cyan));
                ui.label("Acceso directo. Fuera de rango produce panic.");
                ui.end_row();

                ui.label(egui::RichText::new("Acceso seguro").strong().color(texto));
                ui.label(egui::RichText::new("v.get(i)").monospace().color(cyan));
                ui.label("Devuelve Option<&T> sin riesgo de panic.");
                ui.end_row();
            });
    });
}
