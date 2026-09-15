use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_option(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "El tipo Option<T> representa la presencia (Some) o ausencia (None) de un valor. Rust ofrece una completa familia de métodos combinadores (.unwrap_or, .unwrap_or_else, .expect, .map) para manejar la ausencia de valores de forma segura.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_option_familia_unwrap")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Variante / Método").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción & Caso de Uso").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Some(val) & None
                ui.label(egui::RichText::new("Some(T) / None").strong().color(naranja));
                let code_option_basic = "fn buscar_usuario(id: u32) -> Option<String> {\n    if id == 1 {\n        Some(String::from(\"Ana\"))\n    } else {\n        None\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Option<T>: Some vs None".to_string(), code_option_basic));
                }
                ui.label("Some(val) envuelve un valor existente; None indica la ausencia total de valor.");
                ui.end_row();

                // .unwrap_or()
                ui.label(egui::RichText::new(".unwrap_or()").strong().color(naranja));
                let code_unwrap_or = "let nombre_opt: Option<String> = None;\nlet nombre = nombre_opt.unwrap_or(String::from(\"Invitado\"));".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Manejo Seguro: .unwrap_or()".to_string(), code_unwrap_or));
                }
                ui.label("Extrae el valor si es Some; si es None, devuelve de forma segura un valor por defecto ya calculado.");
                ui.end_row();

                // .unwrap_or_else()
                ui.label(egui::RichText::new(".unwrap_or_else()").strong().color(naranja));
                let code_unwrap_else = "let saldo_opt: Option<u32> = None;\nlet saldo = saldo_opt.unwrap_or_else(|| consultar_banco_default());".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Manejo Lazy: .unwrap_or_else()".to_string(), code_unwrap_else));
                }
                ui.label("Ejecuta una closure para calcular el valor por defecto de forma perezosa (Lazy) solo si era None.");
                ui.end_row();

                // .unwrap_or_default()
                ui.label(egui::RichText::new(".unwrap_or_default()").strong().color(naranja));
                let code_unwrap_default = "let contador: Option<i32> = None;\nlet val = contador.unwrap_or_default(); // Devuelve 0".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Valor Estándar: .unwrap_or_default()".to_string(), code_unwrap_default));
                }
                ui.label("Devuelve el valor predeterminado estándar del tipo T (ej. 0 para enteros, \"\" para String).");
                ui.end_row();

                // .expect()
                ui.label(egui::RichText::new(".expect()").strong().color(naranja));
                let code_expect = "let config: Option<String> = Some(String::from(\"db.conf\"));\nlet ruta = config.expect(\"La ruta de configuración es obligatoria\");".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Desempaquetado con Mensaje: .expect()".to_string(), code_expect));
                }
                ui.label("Extrae el valor o provoca pánico (panic!) imprimiendo un mensaje explicativo personalizado.");
                ui.end_row();

                // .map() / .and_then()
                ui.label(egui::RichText::new(".map() / .and_then()").strong().color(naranja));
                let code_map_then = "let texto = Some(String::from(\"hola\"));\nlet long = texto.map(|s| s.len()); // Some(4)\n\nlet num_opt = Some(\"42\");\nlet parsed = num_opt.and_then(|s| s.parse::<i32>().ok()); // Some(42)".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Transformadores: .map() y .and_then()".to_string(), code_map_then));
                }
                ui.label(".map() transforma el valor interno; .and_then() encadena operaciones que retornan otro Option.");
                ui.end_row();
            });
    });
}
