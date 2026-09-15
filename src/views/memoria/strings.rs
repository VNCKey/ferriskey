use crate::app::AppState;
use eframe::egui;

/// Sección teórica completa sobre String y &str en Rust
pub fn mostrar_teoria_string_y_str(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        "String es el tipo de texto dinámico y modificable por excelencia en Rust. Se almacena como un vector de bytes UTF-8 en el Heap y gestiona su memoria automáticamente sin recolector de basura.",
    );
    ui.add_space(10.0);

    // Tabla 1: Anatomía de Memoria de String
    let mut table_mem = egui::Frame::new();
    table_mem.fill = egui::Color32::from_rgb(14, 18, 26);
    table_mem.inner_margin = egui::Margin::same(12);
    table_mem.corner_radius = egui::CornerRadius::same(8);
    table_mem.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_mem.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(
                    "Estructura Interna en Memoria (24 Bytes en Stack + Buffer en Heap)",
                )
                .strong()
                .size(14.0)
                .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(8.0);

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
                            include_bytes!("../../../diagramas/view.svg"),
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
        ui.add_space(8.0);

        egui::Grid::new("tabla_string_memoria_anatomia")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Campo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ubicación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito / Descripción")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("ptr")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Stack");
                ui.label("8 Bytes (64-bit)");
                ui.label("Puntero con la dirección de memoria exacta del buffer en el Heap.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("len")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Stack");
                ui.label("8 Bytes (usize)");
                ui.label("Longitud actual: cantidad de bytes UTF-8 válidos en uso.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("cap")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Stack");
                ui.label("8 Bytes (usize)");
                ui.label("Capacidad total: bytes reservados en Heap antes de requerir realloc.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Buffer UTF-8")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Heap");
                ui.label("Dinámico (cap bytes)");
                ui.label("Secuencia contigua de bytes donde residen las letras del texto.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Tabla 2: El Arsenal Completo de String
    let mut table_methods = egui::Frame::new();
    table_methods.fill = egui::Color32::from_rgb(14, 18, 26);
    table_methods.inner_margin = egui::Margin::same(12);
    table_methods.corner_radius = egui::CornerRadius::same(8);
    table_methods.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_methods.show(ui, |ui| {
        ui.label(
            egui::RichText::new("El Arsenal Completo de Métodos Directos de String")
                .strong()
                .size(16.0)
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
        ui.label(
            egui::RichText::new(
                "Todo lo que puedes hacer con texto sin necesidad de usar Iteradores.",
            )
            .color(egui::Color32::from_rgb(180, 180, 180)),
        );
        ui.add_space(10.0);

        let draw_row = |ui: &mut egui::Ui, metodo: &str, hace: &str, ejemplo: &str| {
            ui.label(
                egui::RichText::new(metodo)
                    .monospace()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.label(hace);
            ui.label(
                egui::RichText::new(ejemplo)
                    .monospace()
                    .color(egui::Color32::from_rgb(180, 220, 180)),
            );
            ui.end_row();
        };

        // 1. INSPECCION Y BUSQUEDA
        ui.label(
            egui::RichText::new("1. Inspección y Búsqueda (Solo leen)")
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.end_row();
        egui::Grid::new("grid_inspeccion_string")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                draw_row(
                    ui,
                    ".len()",
                    "Devuelve el tamaño del texto en bytes (no en letras).",
                    "texto.len() // usize",
                );
                draw_row(
                    ui,
                    ".capacity()",
                    "Memoria RAM (en bytes) reservada actualmente en el Heap.",
                    "texto.capacity()",
                );
                draw_row(
                    ui,
                    ".is_empty()",
                    "Devuelve true si la longitud es 0 (\"\").",
                    "\"\".is_empty() // true",
                );
                draw_row(
                    ui,
                    ".contains(str)",
                    "Busca si una palabra o letra existe dentro.",
                    "texto.contains(\"Rust\")",
                );
                draw_row(
                    ui,
                    ".starts_with(str)",
                    "Verifica si empieza exactamente con ese texto.",
                    "texto.starts_with(\"Al\")",
                );
                draw_row(
                    ui,
                    ".ends_with(str)",
                    "Verifica si termina exactamente con ese texto.",
                    "texto.ends_with(\".\")",
                );
                draw_row(
                    ui,
                    ".find(str)",
                    "Busca el texto y devuelve la posición (byte) inicial.",
                    "texto.find(\"a\") // Option",
                );
                draw_row(
                    ui,
                    ".rfind(str)",
                    "Igual que find, pero busca desde el final hacia atrás.",
                    "texto.rfind(\"a\")",
                );
            });

        ui.add_space(15.0);

        // 2. MODIFICACION
        ui.label(
            egui::RichText::new("2. Modificación (Requieren let mut)")
                .strong()
                .color(egui::Color32::from_rgb(255, 100, 100)),
        );
        ui.label(
            egui::RichText::new(
                "Alteran la memoria original. Cuidado con los índices (¡son en bytes!).",
            )
            .small()
            .color(egui::Color32::GRAY),
        );
        ui.end_row();
        egui::Grid::new("grid_modificacion_string")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                draw_row(
                    ui,
                    ".push(char)",
                    "Añade un solo carácter al final del texto.",
                    "texto.push('!')",
                );
                draw_row(
                    ui,
                    ".push_str(&str)",
                    "Añade una frase/texto al final del texto.",
                    "texto.push_str(\" Hola\")",
                );
                draw_row(
                    ui,
                    ".insert(idx, char)",
                    "Inserta un carácter en una posición (byte) específica.",
                    "texto.insert(0, '¡')",
                );
                draw_row(
                    ui,
                    ".insert_str(idx, &str)",
                    "Inserta una frase en una posición (byte) específica.",
                    "texto.insert_str(5, \"amigo\")",
                );
                draw_row(
                    ui,
                    ".remove(idx)",
                    "Borra el carácter en esa posición exacta y te lo devuelve.",
                    "texto.remove(0) // char",
                );
                draw_row(
                    ui,
                    ".pop()",
                    "Borra el último carácter del final y te lo devuelve.",
                    "texto.pop() // Option",
                );
                draw_row(
                    ui,
                    ".truncate(N)",
                    "Corta el texto, dejando solo los primeros N bytes.",
                    "texto.truncate(4)",
                );
                draw_row(
                    ui,
                    ".clear()",
                    "Vacía todo el texto (lo deja con longitud 0).",
                    "texto.clear()",
                );
            });

        ui.add_space(15.0);

        // 3. TRANSFORMACION
        ui.label(
            egui::RichText::new("3. Transformación (Devuelven un texto nuevo)")
                .strong()
                .color(egui::Color32::from_rgb(100, 255, 100)),
        );
        ui.end_row();
        egui::Grid::new("grid_transformacion_string")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                draw_row(
                    ui,
                    ".trim()",
                    "Elimina espacios en blanco y saltos de línea en los extremos.",
                    "texto.trim() // &str",
                );
                draw_row(
                    ui,
                    ".to_uppercase()",
                    "Convierte todo el texto a MAYÚSCULAS.",
                    "texto.to_uppercase() // String",
                );
                draw_row(
                    ui,
                    ".to_lowercase()",
                    "Convierte todo el texto a minúsculas.",
                    "texto.to_lowercase() // String",
                );
                draw_row(
                    ui,
                    ".replace(A, B)",
                    "Busca A y reemplaza TODAS sus apariciones por B.",
                    "texto.replace(\"key\", \"kay\")",
                );
                draw_row(
                    ui,
                    ".replacen(A, B, n)",
                    "Igual que replace, pero solo las primeras n veces.",
                    "texto.replacen(\"o\", \"a\", 2)",
                );
                draw_row(
                    ui,
                    ".repeat(n)",
                    "Copia el texto n veces seguidas.",
                    "\"Ja\".repeat(3) // \"JaJaJa\"",
                );
            });
    });
}
