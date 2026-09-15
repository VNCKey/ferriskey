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
            "Tanto las Estructuras (Structs) como los Enumerados (Enums) pueden ser genéricos, permitiéndoles almacenar cualquier tipo de datos sin duplicar definiciones.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_genericos_structs")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Estructura Genérica").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Struct Genérica
                ui.label(egui::RichText::new("Struct Genérica").strong().color(naranja));
                let code_struct_gen = "struct Punto<T> {\n    x: T,\n    y: T,\n}\n\nlet entero = Punto { x: 5, y: 10 };\nlet flotante = Punto { x: 1.0, y: 4.0 };".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Struct Genérica <T>".to_string(), code_struct_gen));
                }
                ui.label("Define campos cuyos tipos se adaptan automáticamente según los datos inicializados.");
                ui.end_row();

                // Enum Genérico
                ui.label(egui::RichText::new("Enum Genérico").strong().color(naranja));
                let code_enum_gen = "enum Opcion<T> {\n    Alguno(T),\n    Ninguno,\n}\n\nlet a: Opcion<i32> = Opcion::Alguno(10);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Enum Genérico <T>".to_string(), code_enum_gen));
                }
                ui.label("Permite guardar cualquier tipo de datos T dentro de las variantes del enum.");
                ui.end_row();
            });
    });
}
