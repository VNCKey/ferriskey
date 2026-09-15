use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_bounds(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    // Explicación Fundamental: ¿Qué es un Trait Bound?
    ui.label(
        egui::RichText::new("¿Qué es un Trait Bound?")
            .strong()
            .size(17.0)
            .color(naranja),
    );
    ui.add_space(4.0);

    ui.label(
        egui::RichText::new(
            "Un Trait Bound (Restricción de Trait) es una condición que le pones a una función o estructura genérica para exigir que el tipo genérico T implemente un Trait determinado.\n\n\
             • El Problema: Si una función recibe cualquier tipo genérico T, el compilador no sabe qué métodos puede llamar sobre él.\n\
             • La Solución: Al poner <T: Describible>, le garantizas a Rust que solo aceptará tipos que tengan implementado ese Trait.\n\
             • Analogía: 'Puedes entrar a este club sin importar quién seas, siempre y cuando tengas credencial (Trait Bound)'.",
        )
        .color(texto),
    );
    ui.add_space(14.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_genericos_bounds")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Código").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // impl Trait
                ui.label(egui::RichText::new("impl Trait").strong().color(naranja));
                let code_impl_trait = "fn imprimir(item: &impl Describible) {\n    println!(\"{}\", item.describir());\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Sintaxis Concisa: impl Trait".to_string(), code_impl_trait));
                }
                ui.label("Forma concisa de indicar que un parámetro debe implementar un trait determinado.");
                ui.end_row();

                // Trait Bound T: Trait
                ui.label(egui::RichText::new("Trait Bound <T: Trait>").strong().color(naranja));
                let code_bound = "fn procesar<T: Describible>(item: &T) {\n    println!(\"{}\", item.describir());\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Trait Bound Explícito <T: Trait>".to_string(), code_bound));
                }
                ui.label("Sintaxis explícita con genéricos útil cuando múltiples parámetros comparten el mismo tipo T.");
                ui.end_row();

                // Cláusula where
                ui.label(egui::RichText::new("Cláusula where").strong().color(naranja));
                let code_where = "fn comparar<T, U>(a: &T, b: &U) -> bool\nwhere\n    T: Describible,\n    U: Describible,\n{\n    true\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Cláusula where para Múltiples Bounds".to_string(), code_where));
                }
                ui.label("Mantiene limpia la firma de la función agrupando las restricciones complejas al final.");
                ui.end_row();
            });
    });
}
