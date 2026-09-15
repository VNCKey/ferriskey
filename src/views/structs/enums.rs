use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_enums(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Los Enumerados (Enums) permiten definir un tipo que solo puede tomar una variante a la vez. En Rust, las variantes pueden guardar datos y tener métodos asociados mediante bloques impl.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_enums_definicion_impl")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Enum Simple
                ui.label(egui::RichText::new("Enum Simple").strong().color(naranja));
                let code_enum_simple = "enum EstadoServidor {\n    Activo,\n    Mantenimiento,\n    Apagado,\n}\n\nlet e = EstadoServidor::Activo;".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Enum Simple".to_string(), code_enum_simple));
                }
                ui.label("Conjunto discreto de variantes exclusivas.");
                ui.end_row();

                // Enum con Datos Asociados
                ui.label(egui::RichText::new("Enum con Datos").strong().color(naranja));
                let code_enum_data = "enum Mensaje {\n    Salir,\n    Mover { x: i32, y: i32 },\n    Escribir(String),\n}\n\nlet m = Mensaje::Mover { x: 10, y: 20 };".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Enum con Datos Asociados".to_string(), code_enum_data));
                }
                ui.label("Cada variante puede guardar diferentes estructuras de datos o valores internos.");
                ui.end_row();

                // Bloque impl en Enums
                ui.label(egui::RichText::new("Bloque impl en Enum").strong().color(naranja));
                let code_enum_impl = "enum Estado {\n    Conectado,\n    Desconectado,\n}\n\nimpl Estado {\n    fn esta_activo(&self) -> bool {\n        match self {\n            Estado::Conectado => true,\n            Estado::Desconectado => false,\n        }\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Bloque impl en Enums".to_string(), code_enum_impl));
                }
                ui.label("Los bloques impl permiten agregar métodos propios a un Enum igual que a una Struct.");
                ui.end_row();
            });
    });
}
