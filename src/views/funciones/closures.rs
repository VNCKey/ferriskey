use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_closures(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un closure es una función anónima que puede guardar variables de su contexto circundante. Se escribe con la sintaxis de tuberías '|params| cuerpo'.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_closures_detalles")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Reglas & Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Fila 1: Closure Básica
                ui.label(egui::RichText::new("Closure Básica").strong().color(naranja));
                let code_basic = "let sumar_uno = |x: i32| x + 1;\nlet res = sumar_uno(5);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Closure Básica: |x| x + 1".to_string(), code_basic));
                }
                ui.label("Función anónima concisa. Los tipos de datos pueden inferirse automáticamente.");
                ui.end_row();

                // Fila 2: Captura por Préstamo (&)
                ui.label(egui::RichText::new("Captura por Préstamo").strong().color(naranja));
                let code_borrow = "let factor = 10;\nlet multiplicar = |x| x * factor; // Lee 'factor' de su entorno\n\nlet res = multiplicar(5); // Devuelve 50".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Captura por Préstamo (&)".to_string(), code_borrow));
                }
                ui.label("Accede a variables declaradas fuera de la closure mediante lectura prestada.");
                ui.end_row();

                // Fila 3: Captura por Movimiento (move)
                ui.label(egui::RichText::new("Captura con move").strong().color(naranja));
                let code_move = "let mensaje = String::from(\"Hola Rust\");\nlet imprimir = move || println!(\"{mensaje}\");\n\nimprimir(); // 'mensaje' se movió dentro del closure".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Captura por Movimiento (move)".to_string(), code_move));
                }
                ui.label("La palabra clave 'move' fuerza a la closure a tomar la propiedad (Ownership) de las variables capturadas.");
                ui.end_row();
            });
    });
}
