use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_compuesto_array(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un Array guarda valores del mismo tipo de forma contigua en el Stack con tamaño fijo. Un Slice (&[T]) es una vista prestada y dinámica sobre los elementos de un Array o Vector.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    // Tabla 1: Especificación del Array
    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_array_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Pieza")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Nota")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Tipo
                ui.label(
                    egui::RichText::new("Tipo")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("[i32; 5]").monospace().color(cyan));
                ui.label("T y N fijos; N es parte del tipo.");
                ui.end_row();

                // Fila 2: Acceso
                ui.label(
                    egui::RichText::new("Acceso")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr[i]").monospace().color(cyan));
                ui.label("Fuera de rango produce panic en runtime.");
                ui.end_row();

                // Fila 3: Longitud
                ui.label(
                    egui::RichText::new("Longitud")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.len()").monospace().color(cyan));
                ui.label("Siempre N; no crece como un Vec.");
                ui.end_row();

                // Fila 4: Repetición
                ui.label(
                    egui::RichText::new("Repetición")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("[val; N]").monospace().color(cyan));
                ui.label("Crea N elementos repetidos al instante.");
                ui.end_row();

                // Fila 5: Desestructuración
                ui.label(
                    egui::RichText::new("Desestructuración")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("let [a, b, ..] = arr;")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Extrae elementos o sub-secuencias por patrón.");
                ui.end_row();

                // Fila 6: Slice
                ui.label(
                    egui::RichText::new("Slice")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("&arr[..]").monospace().color(cyan));
                ui.label("Obtiene una vista prestada del array completo.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Tabla 2: Operaciones con Slices (&[T])
    ui.label(
        egui::RichText::new("Operaciones con Slices (&[T])")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(8.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_slice_operaciones")
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

                ui.label(egui::RichText::new("Slice Parcial").strong().color(texto));
                ui.label(egui::RichText::new("&arr[1..4]").monospace().color(cyan));
                ui.label("Vista prestada desde el índice 1 hasta el 3 (exclusivo).");
                ui.end_row();

                ui.label(egui::RichText::new("Slice Completo").strong().color(texto));
                ui.label(egui::RichText::new("&arr[..]").monospace().color(cyan));
                ui.label("Referencia a todos los elementos del contenedor.");
                ui.end_row();

                ui.label(egui::RichText::new("Slice Mutable").strong().color(texto));
                ui.label(
                    egui::RichText::new("&mut arr[1..3]")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Vista prestada con permiso de modificación sobre el rango.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Tabla 3: Ejemplos Prácticos de Declaración
    ui.label(
        egui::RichText::new("Ejemplos Prácticos de Declaración e Inicialización")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(8.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_array_ejemplos")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Caso de Uso")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Código en Rust")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Resultado en Memoria")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Ejemplo 1: Literal explícito
                ui.label(
                    egui::RichText::new("Literal explícito")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("let nums = [10, 20, 30];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("[10, 20, 30] (Array de tipo [i32; 3])");
                ui.end_row();

                // Ejemplo 2: Repetición de ceros
                ui.label(
                    egui::RichText::new("Repetición de ceros")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("let buffer = [0u8; 5];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("[0, 0, 0, 0, 0] (Array de tipo [u8; 5])");
                ui.end_row();

                // Ejemplo 3: Slice prestado
                ui.label(egui::RichText::new("Slice prestado").strong().color(texto));
                ui.label(
                    egui::RichText::new("let s = &nums[1..3];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("&[20, 30] (Slice de tipo &[i32])");
                ui.end_row();

                // Ejemplo 4: Tipo explícito
                ui.label(egui::RichText::new("Tipo explícito").strong().color(texto));
                ui.label(
                    egui::RichText::new("let coords: [f64; 2] = [1.5, 2.5];")
                        .monospace()
                        .color(cyan),
                );
                ui.label("[1.5, 2.5] (Array [f64; 2])");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Nota de advertencia sobre Stack Overflow
    let mut warning_frame = egui::Frame::new();
    warning_frame.fill = egui::Color32::from_rgb(28, 22, 14);
    warning_frame.inner_margin = egui::Margin::same(12);
    warning_frame.corner_radius = egui::CornerRadius::same(8);
    warning_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 65, 30));

    warning_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Nota sobre el Límite de Tamaño y Stack Overflow:")
                    .strong()
                    .color(naranja),
            );
        });
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Los arrays viven en la memoria Stack. Intentar crear un array extremadamente grande (por ejemplo, de varios Megabytes como [0u8; 10_000_000]) provocará un error de desbordamiento en el Stack (Stack Overflow) en tiempo de ejecución.",
            )
            .color(texto),
        );
    });
}
