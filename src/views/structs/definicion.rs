use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_structs(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Las estructuras (Structs) permiten agrupar múltiples datos relacionados bajo un mismo nombre. Con los bloques impl se define su comportamiento mediante funciones asociadas y métodos.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_structs_definicion_impl")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Struct con Campos Nombrados
                ui.label(egui::RichText::new("Struct Tradicional").strong().color(naranja));
                let code_struct = "struct Usuario {\n    nombre: String,\n    edad: u32,\n    activo: bool,\n}\n\nlet u = Usuario {\n    nombre: String::from(\"Ana\"),\n    edad: 25,\n    activo: true,\n};".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Struct Tradicional con Campos Nombrados".to_string(), code_struct));
                }
                ui.label("Agrupa datos con nombres explícitos para cada campo.");
                ui.end_row();

                // Tuple Struct
                ui.label(egui::RichText::new("Tuple Struct").strong().color(naranja));
                let code_tuple_struct = "struct Color(u8, u8, u8);\nstruct Punto3D(f64, f64, f64);\n\nlet rojo = Color(255, 0, 0);\nlet p = Punto3D(1.0, 2.5, 0.0);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Tuple Struct".to_string(), code_tuple_struct));
                }
                ui.label("Estructura compacta que accede a sus campos por posición numérica (.0, .1, .2).");
                ui.end_row();

                // Bloque impl y Métodos
                ui.label(egui::RichText::new("Bloque impl & Métodos").strong().color(naranja));
                let code_impl = "struct Rectangulo {\n    ancho: u32,\n    alto: u32,\n}\n\nimpl Rectangulo {\n    // Función asociada (Constructor)\n    fn nuevo(ancho: u32, alto: u32) -> Self {\n        Self { ancho, alto }\n    }\n\n    // Método de instancia\n    fn area(&self) -> u32 {\n        self.ancho * self.alto\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Bloque impl: Constructor y Métodos".to_string(), code_impl));
                }
                ui.label("Agrega comportamiento. fn new() construye la instancia; &self accede a sus datos.");
                ui.end_row();
            });
    });
}
