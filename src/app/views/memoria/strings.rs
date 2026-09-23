use crate::app::AppState;
use eframe::egui;

fn tabla_metodos_string(
    ui: &mut egui::Ui,
    id: &str,
    titulo: &str,
    subtitulo: &str,
    filas: &[(&str, &str, &str)],
) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let texto = egui::Color32::from_rgb(205, 215, 230);
    let ejemplo_color = egui::Color32::from_rgb(180, 220, 180);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new(titulo)
                .strong()
                .size(15.0)
                .color(naranja),
        );
        ui.add_space(2.0);
        ui.label(
            egui::RichText::new(subtitulo)
                .size(12.5)
                .color(egui::Color32::from_rgb(160, 175, 195)),
        );
        ui.add_space(10.0);

        egui::Grid::new(id)
            .striped(true)
            .min_col_width(120.0)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Método")
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Descripción / Comportamiento")
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de uso")
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                for (metodo, desc, ejemplo) in filas {
                    ui.label(
                        egui::RichText::new(*metodo)
                            .monospace()
                            .strong()
                            .color(cyan),
                    );
                    ui.label(
                        egui::RichText::new(*desc)
                            .size(13.0)
                            .color(texto),
                    );
                    ui.label(
                        egui::RichText::new(*ejemplo)
                            .monospace()
                            .size(12.0)
                            .color(ejemplo_color),
                    );
                    ui.end_row();
                }
            });
    });
}

/// Sección teórica completa sobre String y &str en Rust
pub fn mostrar_teoria_string_y_str(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let texto = egui::Color32::from_rgb(205, 215, 230);

    ui.label(
        egui::RichText::new(
            "String es el tipo de texto dinámico y modificable por excelencia en Rust. Se almacena como un vector de bytes UTF-8 en el Heap y gestiona su memoria automáticamente sin recolector de basura.",
        )
        .size(14.0)
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(12.0);

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
                .size(15.0)
                .color(naranja),
            );
            ui.add_space(8.0);

            let btn_color = if state.ui.show_railroad_modal == Some(7) {
                naranja
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            };
            if ui
                .add(
                    egui::Button::image(
                        egui::Image::from_bytes(
                            "bytes://view.svg",
                            include_bytes!("../../../../assets/diagramas/view.svg"),
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
            .min_col_width(90.0)
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
                        .color(naranja),
                );
                ui.label("Heap");
                ui.label("Dinámico (cap bytes)");
                ui.label("Secuencia contigua de bytes donde residen las letras del texto.");
                ui.end_row();
            });
    });

    ui.add_space(18.0);

    ui.heading(
        egui::RichText::new("Arsenal de Métodos Directos de String")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Colección de operaciones integradas para consultar, modificar y transformar texto directamente sin necesidad de iteradores.",
        )
        .size(13.5)
        .color(texto),
    );
    ui.add_space(12.0);

    // =========================================================
    // TABLA 1: Inspección y Búsqueda (Solo lectura)
    // =========================================================
    let filas_inspeccion = [
        (
            ".len()",
            "Devuelve la longitud del texto en bytes (no en caracteres).",
            "texto.len() // usize",
        ),
        (
            ".capacity()",
            "Memoria RAM (en bytes) reservada actualmente en el Heap.",
            "texto.capacity()",
        ),
        (
            ".is_empty()",
            "Devuelve true si la longitud es 0 (\"\").",
            "\"\".is_empty() // true",
        ),
        (
            ".contains(str)",
            "Comprueba si una palabra o letra existe dentro del texto.",
            "texto.contains(\"Rust\")",
        ),
        (
            ".starts_with(str)",
            "Verifica si el texto comienza con el prefijo indicado.",
            "texto.starts_with(\"Al\")",
        ),
        (
            ".ends_with(str)",
            "Verifica si el texto termina con el sufijo indicado.",
            "texto.ends_with(\".\")",
        ),
        (
            ".find(str)",
            "Busca el texto y devuelve la posición (byte) de su inicio.",
            "texto.find(\"a\") // Option",
        ),
        (
            ".rfind(str)",
            "Igual que .find(), pero busca desde el final hacia el inicio.",
            "texto.rfind(\"a\")",
        ),
    ];

    tabla_metodos_string(
        ui,
        "grid_inspeccion_string",
        "1. Inspección y Búsqueda (Solo Lectura)",
        "Métodos que consultan propiedades del buffer sin modificar su contenido ni reservar nueva memoria.",
        &filas_inspeccion,
    );

    ui.add_space(14.0);

    // =========================================================
    // TABLA 2: Modificación en Memoria (Requieren let mut)
    // =========================================================
    let filas_modificacion = [
        (
            ".push(char)",
            "Añade un solo carácter al final del texto.",
            "texto.push('!')",
        ),
        (
            ".push_str(&str)",
            "Añade una cadena de texto completa al final.",
            "texto.push_str(\" Hola\")",
        ),
        (
            ".insert(idx, char)",
            "Inserta un carácter en una posición de byte específica.",
            "texto.insert(0, '¡')",
        ),
        (
            ".insert_str(idx, &str)",
            "Inserta una frase en una posición de byte específica.",
            "texto.insert_str(5, \"amigo\")",
        ),
        (
            ".remove(idx)",
            "Borra el carácter en esa posición exacta y lo devuelve.",
            "texto.remove(0) // char",
        ),
        (
            ".pop()",
            "Elimina el último carácter del final y lo devuelve.",
            "texto.pop() // Option",
        ),
        (
            ".truncate(n)",
            "Corta el texto conservando solo los primeros n bytes.",
            "texto.truncate(4)",
        ),
        (
            ".clear()",
            "Vacía el contenido del texto dejando su longitud en 0.",
            "texto.clear()",
        ),
    ];

    tabla_metodos_string(
        ui,
        "grid_modificacion_string",
        "2. Modificación en Memoria (Requieren let mut)",
        "Operaciones in-place que alteran directamente el buffer en el Heap. Los índices son posiciones en bytes.",
        &filas_modificacion,
    );

    ui.add_space(14.0);

    // =========================================================
    // TABLA 3: Transformación y Formato (Nuevos valores)
    // =========================================================
    let filas_transformacion = [
        (
            ".trim()",
            "Elimina espacios en blanco y saltos de línea en ambos extremos.",
            "texto.trim() // &str",
        ),
        (
            ".to_uppercase()",
            "Crea un nuevo String con todo el texto en MAYÚSCULAS.",
            "texto.to_uppercase() // String",
        ),
        (
            ".to_lowercase()",
            "Crea un nuevo String con todo el texto en minúsculas.",
            "texto.to_lowercase() // String",
        ),
        (
            ".replace(a, b)",
            "Busca todas las apariciones de 'a' y las reemplaza por 'b'.",
            "texto.replace(\"key\", \"kay\")",
        ),
        (
            ".replacen(a, b, n)",
            "Igual que .replace(), pero solo para las primeras n ocurrencias.",
            "texto.replacen(\"o\", \"a\", 2)",
        ),
        (
            ".repeat(n)",
            "Genera un nuevo String duplicando el texto n veces consecutivas.",
            "\"Ja\".repeat(3) // \"JaJaJa\"",
        ),
    ];

    tabla_metodos_string(
        ui,
        "grid_transformacion_string",
        "3. Transformación y Formato (Nuevos Valores)",
        "Operaciones que no mutan el original, sino que generan una nueva vista prestada (&str) o un nuevo String en Heap.",
        &filas_transformacion,
    );
}
