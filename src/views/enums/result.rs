use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_result(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "El tipo Result<T, E> se utiliza para operaciones que pueden fallar. Representa el éxito (Ok) o un error recuperable (Err).",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_result_detalles")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Variante / Método").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Ok(T) & Err(E)
                ui.label(egui::RichText::new("Ok(T) / Err(E)").strong().color(naranja));
                let code_result_basic = "fn dividir(a: f64, b: f64) -> Result<f64, String> {\n    if b == 0.0 {\n        Err(String::from(\"División por cero\"))\n    } else {\n        Ok(a / b)\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Result<T, E>: Ok vs Err".to_string(), code_result_basic));
                }
                ui.label("Ok(val) contiene el resultado exitoso; Err(err) contiene la causa del fallo.");
                ui.end_row();

                // .is_ok() / .is_err()
                ui.label(egui::RichText::new(".is_ok() / .is_err()").strong().color(naranja));
                let code_is_ok = "let res: Result<i32, &str> = Ok(10);\nif res.is_ok() {\n    println!(\"Operación exitosa\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Comprobación: .is_ok() y .is_err()".to_string(), code_is_ok));
                }
                ui.label("Verifica el estado del resultado mediante un valor booleano sin consumir el contenido.");
                ui.end_row();
            });
    });
}
