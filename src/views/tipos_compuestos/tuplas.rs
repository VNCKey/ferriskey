use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_compuesto_tupla(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Una Tupla agrupa una cantidad fija de valores que pueden ser de diferentes tipos. Vive en el Stack y se accede a sus elementos usando la sintaxis de punto.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    // Tabla 1: Especificación de Tuplas
    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_tupla_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Aspecto")
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

                // Fila 1: Declaración
                ui.label(
                    egui::RichText::new("Declaración")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("let t = (10, \"hola\");")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Combina tipos heterogéneos en una estructura fija.");
                ui.end_row();

                // Fila 2: Acceso por punto
                ui.label(
                    egui::RichText::new("Acceso por punto")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("t.0, t.1").monospace().color(cyan));
                ui.label("Acceso directo basado en posición (índice 0).");
                ui.end_row();

                // Fila 3: Retorno Múltiple
                ui.label(
                    egui::RichText::new("Retorno Múltiple")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("fn f() -> (i32, bool)")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Devuelve múltiples valores desde una función.");
                ui.end_row();

                // Fila 4: Unit Type
                ui.label(
                    egui::RichText::new("Unit Type ()")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("let vacio = ();")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Tupla de 0 elementos; representa ausencia de valor.");
                ui.end_row();

                // Fila 5: Tupla Unitaria
                ui.label(
                    egui::RichText::new("Tupla de 1 elemento")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("let t = (5,);").monospace().color(cyan));
                ui.label("Requiere coma final para distinguirla de una expresión.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Tabla 2: Ejemplos Prácticos
    ui.label(
        egui::RichText::new("Ejemplos Prácticos de Declaración e Inicialización")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(8.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_tupla_ejemplos")
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
                    egui::RichText::new("Resultado")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Ejemplo 1: Datos Mixtos
                ui.label(
                    egui::RichText::new("Datos heterogéneos")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("let persona = (\"Alice\", 30);")
                        .monospace()
                        .color(cyan),
                );
                ui.label("persona.0 = \"Alice\", persona.1 = 30");
                ui.end_row();

                // Ejemplo 2: Retorno Múltiple
                ui.label(
                    egui::RichText::new("Retorno de Función")
                        .strong()
                        .color(texto),
                );
                ui.label(
                    egui::RichText::new("let (lat, lon) = get_coords();")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Desestructura los dos valores devueltos.");
                ui.end_row();

                // Ejemplo 3: Unit Type
                ui.label(egui::RichText::new("Unit Type ()").strong().color(texto));
                ui.label(
                    egui::RichText::new("let res: () = main();")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Representa retorno vacío (void).");
                ui.end_row();

                // Ejemplo 4: Elemento Único
                ui.label(egui::RichText::new("Elemento Único").strong().color(texto));
                ui.label(
                    egui::RichText::new("let un solo = (42,);")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Tupla de tipo (i32,) con 1 elemento.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Nota Informativa sobre el Límite de 12 Elementos
    let mut warning_frame = egui::Frame::new();
    warning_frame.fill = egui::Color32::from_rgb(28, 22, 14);
    warning_frame.inner_margin = egui::Margin::same(12);
    warning_frame.corner_radius = egui::CornerRadius::same(8);
    warning_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 65, 30));

    warning_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Nota sobre el Límite Práctico de 12 Elementos:")
                    .strong()
                    .color(naranja),
            );
        });
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "La librería estándar de Rust implementa automáticamente contratos comunes para comparaciones con '==' 'Clone' y 'Default' únicamente para tuplas de hasta 12 elementos.",
            )
            .color(texto),
        );
    });
}
