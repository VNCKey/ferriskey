use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_pattern_matching(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "La sintaxis concisa de coincidencias de patrones (Pattern Matching) con 'if let' y 'while let' permite evaluar y desempaquetar variantes de un Enum cuando solo nos interesa una coincidencia específica.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_pattern_matching_if_let")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Patrón").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // if let
                ui.label(egui::RichText::new("Sintaxis if let").strong().color(naranja));
                let code_if_let = "enum Config {\n    Modo(u8),\n}\n\nlet config = Config::Modo(7);\n\nif let Config::Modo(nivel) = config {\n    println!(\"Nivel activo: {nivel}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Coincidencia Concisa: if let".to_string(), code_if_let));
                }
                ui.label("Evalúa y desempaqueta una variante específica sin necesidad de escribir un match exhaustivo.");
                ui.end_row();

                // while let
                ui.label(egui::RichText::new("Bucle while let").strong().color(naranja));
                let code_while_let = "let mut pila = vec![1, 2, 3];\n\nwhile let Some(val) = pila.pop() {\n    println!(\"Procesando: {val}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Bucle Conciso: while let".to_string(), code_while_let));
                }
                ui.label("Repite el bucle mientras la expresión coincida con el patrón especificado.");
                ui.end_row();
            });
    });
}
