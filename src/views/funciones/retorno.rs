use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_retorno(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Además de retornar un único valor, en Rust es habitual retornar múltiples valores agrupados en Tuplas -> (T1, T2) y controlar las salidas de las funciones.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_fn_retorno_detalles")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Forma de Retorno").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción & Uso").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Retorno Múltiple con Tuplas
                ui.label(egui::RichText::new("Retorno Múltiple (Tupla)").strong().color(naranja));
                let code_tuple = "fn calcular_min_max(a: i32, b: i32) -> (i32, i32) {\n    if a < b { (a, b) } else { (b, a) }\n}\n\nlet (menor, mayor) = calcular_min_max(10, 5);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Retorno Múltiple con Tuplas -> (T1, T2)".to_string(), code_tuple));
                }
                ui.label("Agrupa y devuelve varios valores a la vez. Permite desestructurarlos fácilmente en el punto de llamada.");
                ui.end_row();

                // Retorno de Unidad ()
                ui.label(egui::RichText::new("Retorno Unidad ()").strong().color(naranja));
                let code_unit = "fn imprimir_mensaje(msg: &str) -> () {\n    println!(\"{msg}\");\n}\n\n// El tipo de retorno '-> ()' se puede omitir".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Retorno de Tipo Unidad ()".to_string(), code_unit));
                }
                ui.label("Cuando una función solo realiza efectos secundarios (como imprimir), devuelve implícitamente el tipo de unidad ().");
                ui.end_row();
            });
    });
}
