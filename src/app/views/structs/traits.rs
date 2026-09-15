use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_traits_custom(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    // Explicación Fundamental: ¿Qué es un Trait?
    ui.label(
        egui::RichText::new("¿Qué es un Trait?")
            .strong()
            .size(17.0)
            .color(naranja),
    );
    ui.add_space(4.0);

    ui.label(
        egui::RichText::new(
            "Un Trait es un contrato de comportamiento compartido en Rust. \
             Define qué métodos o capacidades debe tener un tipo de datos, sin importar cómo esté guardado en memoria.\n\n\
             • Analogía: Piensa en el Trait 'Volador'. Un Avión, un Pájaro y un Dron son tipos totalmente distintos, pero todos comparten la habilidad de 'volar()'.\n\
             • Equivalente: En lenguajes como Java, C# o TypeScript, un Trait es equivalente a una Interfaz (interface).",
        )
        .color(texto),
    );
    ui.add_space(14.0);

    // 1. Declaración e Implementación de Traits
    ui.label(
        egui::RichText::new("Declaración e Implementación de Traits")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_traits_definicion")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Declaración de Trait
                ui.label(egui::RichText::new("Declaración de Trait").strong().color(naranja));
                let code_decl = "trait Describible {\n    fn describir(&self) -> String;\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Declaración de Trait".to_string(), code_decl));
                }
                ui.label("Define las firmas de métodos sin cuerpo que deben cumplir los tipos.");
                ui.end_row();

                // Implementación (impl)
                ui.label(egui::RichText::new("Implementación (impl)").strong().color(naranja));
                let code_impl = "struct Usuario {\n    nombre: String,\n}\n\nimpl Describible for Usuario {\n    fn describir(&self) -> String {\n        format!(\"Usuario: {}\", self.nombre)\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Implementar Trait en un Tipo".to_string(), code_impl));
                }
                ui.label("Conecta el contrato de la interfaz con los datos concretos de una Struct o Enum.");
                ui.end_row();

                // Método por Defecto
                ui.label(egui::RichText::new("Método por Defecto").strong().color(naranja));
                let code_default = "trait Saludador {\n    fn saludar(&self) {\n        println!(\"¡Hola desde Rust!\");\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Método con Implementación por Defecto".to_string(), code_default));
                }
                ui.label("Permite proveer una implementación base que los tipos pueden usar o sobrescribir.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Tipos de Datos Objetivo donde se Implementan Traits
    ui.label(
        egui::RichText::new("Tipos de Datos Objetivo donde se Implementan Traits")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_traits_tipos_objetivo")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Tipo Objetivo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de impl").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Caso de Uso").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Structs
                ui.label(egui::RichText::new("Structs").strong().color(texto));
                let code_t_struct = "struct Persona {\n    nombre: String,\n}\n\nimpl Describible for Persona {\n    fn describir(&self) -> String {\n        self.nombre.clone()\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("impl Trait para Structs".to_string(), code_t_struct));
                }
                ui.label("El uso más habitual: agregar contratos a estructuras de datos con campos.");
                ui.end_row();

                // Enums
                ui.label(egui::RichText::new("Enums").strong().color(texto));
                let code_t_enum = "enum Estado {\n    Activo,\n    Inactivo,\n}\n\nimpl Describible for Estado {\n    fn describir(&self) -> String {\n        String::from(\"Estado del sistema\")\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("impl Trait para Enums".to_string(), code_t_enum));
                }
                ui.label("Muy común para imprimir estados o desempaquetar variantes de enumerados.");
                ui.end_row();

                // Tipos Primitivos
                ui.label(egui::RichText::new("Tipos Primitivos").strong().color(texto));
                let code_t_prim = "impl Describible for i32 {\n    fn describir(&self) -> String {\n        format!(\"Número: {}\", self)\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("impl Trait para Tipos Primitivos (i32)".to_string(), code_t_prim));
                }
                ui.label("Extiende tipos nativos del lenguaje como i32, f64 o bool agregándoles métodos propios.");
                ui.end_row();
            });
    });
}
