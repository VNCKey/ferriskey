use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_structs_info(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Información teórica detallada sobre la disposición en memoria y diferencias entre Structs y Enums en Rust.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    ui.label(
        egui::RichText::new("¿Qué hace diferente a Rust?")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(
            "Rust es un lenguaje de programación de sistemas, multiparadigma y orientado a la composición. No utiliza un modelo clásico basado en clases y herencia; combina rendimiento de bajo nivel con abstracciones seguras.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_influencias_rust")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                for encabezado in ["Lenguaje / fuente", "Ideas que influyeron en Rust"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let influencias = [
                    (
                        "C y C++",
                        "Rendimiento, referencias, movimiento y control cercano al hardware.",
                    ),
                    (
                        "SML y OCaml",
                        "Enums, pattern matching e inferencia de tipos.",
                    ),
                    ("Haskell", "Traits y abstracciones de tipos."),
                    (
                        "Cyclone y ML Kit",
                        "Ideas relacionadas con la gestión segura de memoria.",
                    ),
                    ("Erlang", "Concurrencia y comunicación entre procesos."),
                    ("Scheme", "Macros higiénicas."),
                    (
                        "C# y Ruby",
                        "Atributos y una sintaxis cómoda para closures.",
                    ),
                ];

                for (fuente, ideas) in influencias {
                    ui.label(egui::RichText::new(fuente).strong().color(cyan));
                    ui.label(ideas);
                    ui.end_row();
                }
            });
    });

    ui.add_space(10.0);
    ui.label(
        egui::RichText::new(
            "Rust no inventó todos estos conceptos; los combinó con ownership, borrowing y comprobaciones en compile time para crear un lenguaje de sistemas con un enfoque propio.",
        )
        .italics()
        .color(texto),
    );
    ui.add_space(18.0);

    // 1. Representación en Memoria de Structs vs Enums
    ui.label(
        egui::RichText::new("Disposición de Tipos Personalizados en Memoria")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_custom_types_memoria")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Tipo Personalizado").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Representación Interna").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Cálculo del Tamaño en Memoria").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Struct").monospace().strong().color(cyan));
                ui.label("Secuencia contigua de campos en memoria Stack (con alineación de bytes).");
                ui.label("Suma del tamaño de todos sus campos individuales + padding de alineación.");
                ui.end_row();

                ui.label(egui::RichText::new("Enum").monospace().strong().color(cyan));
                ui.label("Etiqueta Discriminante (tag) + Carga Útil (payload de la variante más grande).");
                ui.label("Tamaño de la variante con mayor consumo de memoria + tamaño del discriminador.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Comparativa Tipo Producto vs Tipo Suma
    ui.label(
        egui::RichText::new("Tipo Producto vs Tipo Suma (Algebraic Data Types)")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_custom_types_algebraico")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Nombre Formal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Lógica de Composición")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(egui::RichText::new("Struct").strong().color(texto));
                ui.label("Tipo Producto");
                ui.label("Contiene el campo A Y el campo B Y el campo C al mismo tiempo.");
                ui.end_row();

                ui.label(egui::RichText::new("Enum").strong().color(texto));
                ui.label("Tipo Suma");
                ui.label("Contiene la variante A O la variante B O la variante C (exclusivo).");
                ui.end_row();
            });
    });
}
