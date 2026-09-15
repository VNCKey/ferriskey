use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_estandar(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    // Explicación Fundamental: Traits Estándar y el atributo #[derive]
    ui.label(
        egui::RichText::new("¿Qué es un Trait Estándar y qué es #[derive]?")
            .strong()
            .size(17.0)
            .color(naranja),
    );
    ui.add_space(4.0);

    ui.label(
        egui::RichText::new(
            "• Trait Estándar: Es un Trait que ya viene creado de fábrica en la librería estándar de Rust (ej. Debug, Clone, PartialEq, Display, From).\n\
             • Atributo #[derive]: Es un atajo automático del compilador. En lugar de escribir el bloque 'impl Trait for Tipo' manualmente a mano, le pides a Rust que genere el código de implementación por ti automáticamente.",
        )
        .color(texto),
    );
    ui.add_space(14.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_traits_estandar")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Mecanismo / Trait").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // #[derive(...)]
                ui.label(egui::RichText::new("#[derive(...)]").strong().color(naranja));
                let code_derive = "#[derive(Debug, Clone, PartialEq, Default)]\nstruct Punto {\n    x: i32,\n    y: i32,\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Derivación Automática con #[derive]".to_string(), code_derive));
                }
                ui.label("Atajo que implementa automáticamente Debug, Clone, PartialEq y Default sin escribir impl.");
                ui.end_row();

                // Display
                ui.label(egui::RichText::new("Display").strong().color(naranja));
                let code_display = "struct Punto { x: i32, y: i32 }\n\nimpl std::fmt::Display for Punto {\n    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n        write!(f, \"({}, {})\", self.x, self.y)\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Formateo Legible: Display".to_string(), code_display));
                }
                ui.label("Trait estándar para formatear e imprimir un tipo con println!(\"{}\").");
                ui.end_row();

                // From / Into
                ui.label(egui::RichText::new("From / Into").strong().color(naranja));
                let code_from = "struct Punto { x: i32, y: i32 }\n\nimpl From<i32> for Punto {\n    fn from(val: i32) -> Self {\n        Punto { x: val, y: val }\n    }\n}\n\nlet p: Punto = 5.into();".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Conversión de Tipos: From e Into".to_string(), code_from));
                }
                ui.label("Traits estándar de conversión de tipos. Implementar From otorga gratis el trait Into.");
                ui.end_row();
            });
    });
}
