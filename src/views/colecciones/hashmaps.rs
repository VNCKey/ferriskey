use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_coleccion_hashmap(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    _naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un HashMap<K, V> almacena pares clave-valor (Key-Value) en el Heap usando una función hash para búsquedas rápidas en tiempo O(1). Requiere importar std::collections::HashMap.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    // Tabla de Operaciones Principales de HashMap
    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_hashmap_operaciones")
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

                ui.label(egui::RichText::new("Crear nuevo").strong().color(texto));
                ui.label(
                    egui::RichText::new("let mut map = HashMap::new();")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Instancia un HashMap vacío en el Heap.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Insertar clave-valor")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("map.insert(\"clave\", 10);")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Inserta o reemplaza el valor asociado a la clave.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Buscar por clave")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("map.get(&\"clave\")")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Busca el valor y devuelve Option<&V>.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Insertar si no existe")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("map.entry(\"k\").or_insert(0);")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Garantiza la presencia de la clave inicializando un valor por defecto.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Eliminar por clave")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("map.remove(&\"clave\");")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Elimina el par clave-valor y devuelve Option<V>.");
                ui.end_row();
            });
    });
}
