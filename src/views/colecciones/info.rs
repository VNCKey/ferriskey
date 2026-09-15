use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_coleccion_info(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Información teórica detallada sobre la estructura interna en memoria y los métodos esenciales de las Colecciones Dinámicas en Rust.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Estructura Interna de Vec<T> en Memoria (24 Bytes en Stack)
    ui.label(
        egui::RichText::new("Estructura Interna de Vec<T> (24 Bytes en Stack)")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_vec_layout_info")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Campo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Tamaño").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ubicación").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("ptr").monospace().strong().color(cyan));
                ui.label("8 Bytes");
                ui.label("Stack");
                ui.label("Puntero a la dirección inicial del buffer asignado en el Heap.");
                ui.end_row();

                ui.label(egui::RichText::new("cap").monospace().strong().color(cyan));
                ui.label("8 Bytes");
                ui.label("Stack");
                ui.label("Capacidad total de elementos que el buffer en Heap puede alojar sin reasignar.");
                ui.end_row();

                ui.label(egui::RichText::new("len").monospace().strong().color(cyan));
                ui.label("8 Bytes");
                ui.label("Stack");
                ui.label("Número de elementos actualmente ocupados e inicializados en el Vector.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Catálogo de Métodos de Vec<T>
    ui.label(
        egui::RichText::new("Catálogo de Métodos de Vec<T>")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_vec_metodos_info")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Categoría")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Método")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Firma de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Complejidad & Descripción")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Modificación
                ui.label(egui::RichText::new("Inserción").strong().color(texto));
                ui.label(egui::RichText::new("push").monospace().color(cyan));
                ui.label("v.push(val)");
                ui.label("O(1) amortizado. Inserta un elemento al final.");
                ui.end_row();

                ui.label(egui::RichText::new("Extracción").strong().color(texto));
                ui.label(egui::RichText::new("pop").monospace().color(cyan));
                ui.label("v.pop() -> Option<T>");
                ui.label("O(1). Elimina y devuelve el último elemento.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Inserción por Índice")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new("insert").monospace().color(cyan));
                ui.label("v.insert(idx, val)");
                ui.label("O(N). Inserta y desplaza los elementos hacia la derecha.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Eliminación Lenta")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new("remove").monospace().color(cyan));
                ui.label("v.remove(idx) -> T");
                ui.label("O(N). Elimina un elemento manteniendo el orden de los demás.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Eliminación Rápida")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new("swap_remove").monospace().color(cyan));
                ui.label("v.swap_remove(idx) -> T");
                ui.label("O(1). Reemplaza el elemento por el último. Cambia el orden.");
                ui.end_row();

                // Capacidad & Memoria
                ui.label(egui::RichText::new("Gestión Memoria").strong().color(texto));
                ui.label(egui::RichText::new("reserve").monospace().color(cyan));
                ui.label("v.reserve(additional)");
                ui.label("Reserva espacio en Heap para evitar reasignaciones futuras.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Optimización Memoria")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new("shrink_to_fit").monospace().color(cyan));
                ui.label("v.shrink_to_fit()");
                ui.label("Libera el exceso de capacidad no utilizada en el Heap.");
                ui.end_row();

                // Búsqueda & Inspección
                ui.label(egui::RichText::new("Inspección").strong().color(texto));
                ui.label(egui::RichText::new("first / last").monospace().color(cyan));
                ui.label("v.first() / v.last()");
                ui.label("Devuelve Option<&T> al primer o último elemento.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Búsqueda Binaria")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new("binary_search").monospace().color(cyan));
                ui.label("v.binary_search(&val)");
                ui.label("O(log N). Busca en vectores previamente ordenados.");
                ui.end_row();

                // Limpieza & Filtrado
                ui.label(egui::RichText::new("Limpieza").strong().color(texto));
                ui.label(egui::RichText::new("clear").monospace().color(cyan));
                ui.label("v.clear()");
                ui.label("Elimina todos los elementos vaciando el vector (len = 0).");
                ui.end_row();

                ui.label(egui::RichText::new("Filtrado").strong().color(texto));
                ui.label(egui::RichText::new("retain").monospace().color(cyan));
                ui.label("v.retain(|x| pred)");
                ui.label("Conserva únicamente los elementos que satisfacen el predicado.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 3. Aspectos Clave de HashMap<K, V>
    ui.label(
        egui::RichText::new("HashMap<K, V> & Entry API")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_hashmap_info_detalles")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Requisito / Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Requisitos de Clave").strong().color(texto));
                ui.label(egui::RichText::new("K: Eq + Hash").monospace().color(cyan));
                ui.label("Las claves deben implementar las traits Eq y Hash para calcular posiciones de balde.");
                ui.end_row();

                ui.label(egui::RichText::new("Entry API").strong().color(texto));
                ui.label(egui::RichText::new("map.entry(key).or_insert(val)").monospace().color(cyan));
                ui.label("Permite buscar e insertar en una sola operación optimizada sin doble lookup.");
                ui.end_row();

                ui.label(egui::RichText::new("Hasheador Por Defecto").strong().color(texto));
                ui.label(egui::RichText::new("SipHash 1-3").monospace().color(cyan));
                ui.label("Proporciona protección resistente contra ataques DoS por colisión de Hashes.");
                ui.end_row();
            });
    });
}
