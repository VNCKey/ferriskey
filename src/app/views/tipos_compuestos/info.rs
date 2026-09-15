use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_compuesto_info(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Sección Teórica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        ui.add_space(6.0);

        let sub_tabs = [
            (0, "Array Métodos"),
            (1, "Destructuring Pattern"),
            (2, "Comparativa"),
        ];

        for (idx, label) in sub_tabs {
            let activo = state.lessons.compuestos_info_tab == idx;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.compuestos_info_tab = idx;
            }
            ui.add_space(4.0);
        }
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    match state.lessons.compuestos_info_tab {
        0 => mostrar_metodos_array(ui, naranja, cyan, texto),
        1 => mostrar_destructuring_pattern(ui, naranja, cyan, texto),
        _ => mostrar_tabla_comparativa(ui, naranja, cyan, texto),
    }
}

fn mostrar_metodos_array(
    ui: &mut egui::Ui,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Los Arrays en Rust pueden invocar todos los métodos de los Slices mediante dereferenciación automática.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    // 1. Inspección y Consulta
    ui.label(
        egui::RichText::new("Inspección y Consulta")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_metodos_inspeccion")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Método")
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
                    egui::RichText::new(".len()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.len()").monospace().color(cyan));
                ui.label("Devuelve la cantidad fija de elementos.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".is_empty()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("arr.is_empty()")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Devuelve true si la longitud es 0.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".first()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.first()").monospace().color(cyan));
                ui.label("Devuelve el primer elemento en Option<&T>.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".last()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.last()").monospace().color(cyan));
                ui.label("Devuelve el último elemento en Option<&T>.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".get(i)")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.get(2)").monospace().color(cyan));
                ui.label("Acceso seguro por índice que devuelve Option<&T>.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // 2. Búsqueda y Verificación
    ui.label(
        egui::RichText::new("Búsqueda y Verificación")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_metodos_busqueda")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Método")
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
                    egui::RichText::new(".contains()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("arr.contains(&20)")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Devuelve true si el valor existe en el array.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".starts_with()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("arr.starts_with(&[10, 20])")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Verifica si el array comienza con una sub-secuencia.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".ends_with()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("arr.ends_with(&[30])")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Verifica si el array termina con una sub-secuencia.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // 3. Modificación u Ordenamiento (en arrays mutables)
    ui.label(
        egui::RichText::new("Modificación u Ordenamiento (arrays mutables)")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_metodos_modificacion")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Método")
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
                    egui::RichText::new(".sort()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.sort()").monospace().color(cyan));
                ui.label("Ordena los elementos de menor a mayor in-place.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".reverse()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.reverse()").monospace().color(cyan));
                ui.label("Invierte el orden de los elementos in-place.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".swap(i, j)")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("arr.swap(0, 2)")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Intercambia los elementos en las posiciones i y j.");
                ui.end_row();

                ui.label(
                    egui::RichText::new(".fill(val)")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.fill(0)").monospace().color(cyan));
                ui.label("Sobrescribe todo el array con el valor indicado.");
                ui.end_row();
            });
    });
}

fn mostrar_destructuring_pattern(
    ui: &mut egui::Ui,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "El Destructuring Pattern (desestructuración) es una característica fundamental de Rust que permite descomponer estructuras de datos (Array, Tupla, Slice, Struct) en variables individuales mediante coincidencia de patrones.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    // Tabla 1: Comparativa de Destructuring por Tipo
    ui.label(
        egui::RichText::new("Destructuring Pattern en Tipos Compuestos")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_destructuring_tipos")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo Compuesto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis de Patrón")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Características")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Array
                ui.label(
                    egui::RichText::new("Array")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("let [a, b, ..] = arr;")
                        .monospace()
                        .color(cyan),
                );
                ui.label(
                    "Elementos homogéneos (mismo tipo), tamaño fijo N conocido en compilación.",
                );
                ui.end_row();

                // Tupla
                ui.label(
                    egui::RichText::new("Tupla")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("let (x, y, z) = tupla;")
                        .monospace()
                        .color(cyan),
                );
                ui.label(
                    "Elementos heterogéneos (tipos mixtos), desestructuración posicional fija.",
                );
                ui.end_row();

                // Slice
                ui.label(
                    egui::RichText::new("Slice")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("let [head, tail @ ..] = slice;")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Vistas prestadas de tamaño dinámico, requiere patrones con resto (..).");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Tabla 2: Catálogo Completo de Patrones de Desestructuración
    ui.label(
        egui::RichText::new("Catálogo Práctico de Patrones de Desestructuración")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_destructuring_ejemplos")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Patrón")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Aplica A")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Código en Rust")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Efecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Exacto Array
                ui.label(
                    egui::RichText::new("Exacto Posicional")
                        .strong()
                        .color(texto),
                );
                ui.label("Array / Tupla");
                ui.label(
                    egui::RichText::new("let [x, y, z] = [10, 20, 30];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Extrae x = 10, y = 20, z = 30.");
                ui.end_row();

                // Heterogéneo Tupla
                ui.label(egui::RichText::new("Tipos Mixtos").strong().color(texto));
                ui.label("Tupla");
                ui.label(
                    egui::RichText::new("let (name, age) = (\"Alice\", 30);")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Extrae name = \"Alice\" (&str) y age = 30 (i32).");
                ui.end_row();

                // Omisión _
                ui.label(egui::RichText::new("Omisión (_)").strong().color(texto));
                ui.label("Array / Tupla");
                ui.label(
                    egui::RichText::new("let [r, _, b] = [255, 128, 0];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Extrae r = 255 y b = 0, ignorando la posición central.");
                ui.end_row();

                // Resto ..
                ui.label(
                    egui::RichText::new("Ignorar Resto (..)")
                        .strong()
                        .color(texto),
                );
                ui.label("Array / Slice");
                ui.label(
                    egui::RichText::new("let [first, ..] = [1, 2, 3, 4];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Extrae solo el primer elemento (first = 1).");
                ui.end_row();

                // Cabeza y Cola
                ui.label(
                    egui::RichText::new("Inicio y Final (..)")
                        .strong()
                        .color(texto),
                );
                ui.label("Array / Slice");
                ui.label(
                    egui::RichText::new("let [head, .., tail] = [1, 2, 3, 4];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Extrae head = 1 y tail = 4.");
                ui.end_row();

                // Sub-slice con @
                ui.label(
                    egui::RichText::new("Captura de Sub-slice (@ ..)")
                        .strong()
                        .color(texto),
                );
                ui.label("Slice / Array");
                ui.label(
                    egui::RichText::new("let [head, ref tail @ ..] = &[1, 2, 3];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Extrae head = 1 y tail = &[2, 3].");
                ui.end_row();
            });
    });
}

fn mostrar_tabla_comparativa(
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
