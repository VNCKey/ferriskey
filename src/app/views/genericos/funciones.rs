use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_funciones(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Los Genéricos permiten escribir funciones abstractas que pueden operar con múltiples tipos de datos distintos mediante un parámetro de tipo entre corchetes angulares <T>.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_genericos_funciones")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Función Genérica Básica
                ui.label(egui::RichText::new("Función Genérica").strong().color(naranja));
                let code_fn_gen = "fn identidad<T>(valor: T) -> T {\n    valor\n}\n\nlet n = identidad(5); // T es i32\nlet s = identidad(\"Hola\"); // T es &str".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Función Genérica Básica <T>".to_string(), code_fn_gen));
                }
                ui.label("Define un parámetro de tipo abstracto <T> que se deduce automáticamente al llamar la función.");
                ui.end_row();

                // Múltiples Parámetros Genéricos
                ui.label(egui::RichText::new("Múltiples Genéricos").strong().color(naranja));
                let code_fn_multi = "fn par<T, U>(a: T, b: U) -> (T, U) {\n    (a, b)\n}\n\nlet p = par(1, \"uno\"); // T es i32, U es &str".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Múltiples Parámetros Genéricos <T, U>".to_string(), code_fn_multi));
                }
                ui.label("Permite combinar diferentes tipos independientes en la misma firma de función.");
                ui.end_row();
            });
    });
}
