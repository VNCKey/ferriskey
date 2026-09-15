use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_desempaquetado(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Técnicas idiomáticas de desempaquetado de Option y Result: coincidencia concisa con 'if let' / 'while let' y propagación rápida de errores con el operador '?'.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_desempaquetado_detalles")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Código").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // if let
                ui.label(egui::RichText::new("if let").strong().color(naranja));
                let code_if_let = "let usuario: Option<String> = Some(String::from(\"Ana\"));\n\nif let Some(nombre) = usuario {\n    println!(\"Hola {nombre}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Desempaquetado Conciso: if let".to_string(), code_if_let));
                }
                ui.label("Extrae el valor contenido en Some u Ok solo cuando la coincidencia tiene éxito.");
                ui.end_row();

                // while let
                ui.label(egui::RichText::new("while let").strong().color(naranja));
                let code_while_let = "let mut pila = vec![1, 2, 3];\n\nwhile let Some(val) = pila.pop() {\n    println!(\"Procesando: {val}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Bucle Conciso: while let".to_string(), code_while_let));
                }
                ui.label("Mantiene la iteración de un bucle mientras la expresión retorne Some o Ok.");
                ui.end_row();

                // Operador ?
                ui.label(egui::RichText::new("Operador ?").strong().color(naranja));
                let code_question = "fn leer_numero(s: &str) -> Result<i32, std::num::ParseIntError> {\n    let num = s.parse::<i32>()?; // Si falla, retorna Err inmediatamente\n    Ok(num * 2)\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Propagación de Errores: Operador ?".to_string(), code_question));
                }
                ui.label("Desempaqueta el valor si es Ok; si es Err, retorna tempranamente el error a la función llamadora.");
                ui.end_row();
            });
    });
}
