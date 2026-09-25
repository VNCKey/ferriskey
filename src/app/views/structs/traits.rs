use crate::app::AppState;
use crate::app::ui::*;
use crate::views::structs::grupo_custom_types;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_traits_custom(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Un Trait es un contrato de comportamiento compartido. Define qué puede hacer un tipo sin imponer cómo debe guardar sus datos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Declarar e implementar un Trait");
    ui.label(
        RichText::new(
            "Primero se define la capacidad y después se implementa para cada tipo que pueda ofrecerla.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Declaración",
            "El Trait contiene las firmas de los métodos que un tipo debe proporcionar.",
            "trait Describible {\n    fn describir(&self) -> String;\n}",
        ),
        (
            "Implementación",
            "`impl Trait for Tipo` conecta el contrato con una Struct o un Enum concreto.",
            "struct Usuario {\n    nombre: String,\n}\n\nimpl Describible for Usuario {\n    fn describir(&self) -> String {\n        self.nombre.clone()\n    }\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Métodos por defecto");
    ui.label(
        RichText::new(
            "Un Trait puede ofrecer una implementación base. El tipo puede utilizarla directamente o reemplazarla con su propia versión.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Implementación base",
            "El método tiene un cuerpo dentro del Trait y queda disponible para los tipos que lo implementan.",
            "trait Saludador {\n    fn saludar(&self) {\n        println!(\"Hola desde Rust\");\n    }\n}",
        ),
        (
            "Sobrescribir comportamiento",
            "El tipo puede definir el mismo método para adaptar el comportamiento a sus propios datos.",
            "struct Usuario;\n\nimpl Saludador for Usuario {\n    fn saludar(&self) {\n        println!(\"Hola, Usuario\");\n    }\n}",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_custom_types(
        ui,
        (
            "Trait para una Struct",
            "Las Structs suelen implementar Traits para añadir capacidades a sus campos y métodos propios.",
            "struct Persona {\n    nombre: String,\n}\n\nimpl Describible for Persona {\n    fn describir(&self) -> String {\n        self.nombre.clone()\n    }\n}",
        ),
        (
            "Trait para un Enum",
            "Los Enums pueden implementar el mismo contrato y decidir según sus variantes.",
            "enum Estado {\n    Activo,\n    Inactivo,\n}\n\nimpl Describible for Estado {\n    fn describir(&self) -> String {\n        String::from(\"Estado del sistema\")\n    }\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Idea esencial");
    ui.label(
        RichText::new(
            "Un Trait describe una capacidad; la implementación decide cómo esa capacidad funciona para cada tipo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
