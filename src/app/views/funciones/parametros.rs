use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_parametros(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "En Rust existen 3 formas fundamentales de recibir parámetros en una función según la gestión de memoria (Ownership & Borrowing): por valor (transferencia de propiedad), por préstamo inmutable (&T) y por préstamo mutable (&mut T).",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_fn_parametros_ownership")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Forma de Recibir").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Efecto en la Memoria (Ownership)").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Valor (T)
                ui.label(egui::RichText::new("Valor (T)").strong().color(naranja));
                let code_val = "fn consumir(texto: String) {\n    println!(\"{texto}\");\n}\n\nlet s = String::from(\"Rust\");\nconsumir(s); // 's' se mueve a la función y deja de ser válida aquí".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Parámetro por Valor (T)".to_string(), code_val));
                }
                ui.label("La función toma la propiedad (Ownership). La variable original se Mueve (o Copia si es de tipo primitivo).");
                ui.end_row();

                // Borrowing (&T)
                ui.label(egui::RichText::new("Borrowing (&T)").strong().color(naranja));
                let code_ref = "fn calcular_longitud(texto: &String) -> usize {\n    texto.len()\n}\n\nlet s = String::from(\"Rust\");\nlet len = calcular_longitud(&s); // 's' sigue siendo válida después".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Parámetro por Borrowing (&T)".to_string(), code_ref));
                }
                ui.label("Presta el valor de solo lectura. La función lee el dato sin quitarle la propiedad a la variable dueña.");
                ui.end_row();

                // Borrowing Mutable (&mut T)
                ui.label(egui::RichText::new("Borrowing Mutable (&mut T)").strong().color(naranja));
                let code_mut = "fn agregar_saludo(texto: &mut String) {\n    texto.push_str(\", Hola!\");\n}\n\nlet mut s = String::from(\"Rust\");\nagregar_saludo(&mut s);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Parámetro por Borrowing Mutable (&mut T)".to_string(), code_mut));
                }
                ui.label("Presta el valor con permiso de modificación in-situ. Permite alterar la variable original en memoria.");
                ui.end_row();
            });
    });
}
