use crate::app::AppState;
use crate::app::ui::*;
use crate::views::structs::grupo_custom_types;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_enums(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Un Enum define un tipo que puede tener una variante entre varias posibilidades. Las variantes pueden ser simples o guardar datos diferentes.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Variantes de un Enum");
    ui.label(
        RichText::new(
            "Cada variante representa un estado posible. El programa puede comprobar cuál está activa y actuar según ella.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Enum simple",
            "Las variantes no contienen datos adicionales. El valor solo indica cuál de las opciones está activa.",
            "enum EstadoServidor {\n    Activo,\n    Mantenimiento,\n    Apagado,\n}\n\nlet estado = EstadoServidor::Activo;",
        ),
        (
            "Enum con Tuple",
            "Una variante puede guardar un valor asociado y acceder a él mediante Pattern Matching.",
            "enum Mensaje {\n    Texto(String),\n    Codigo(u16),\n}\n\nlet mensaje = Mensaje::Codigo(200);",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_custom_types(
        ui,
        (
            "Enum con Struct",
            "Una variante también puede guardar campos nombrados para expresar mejor su significado.",
            "enum Evento {\n    Mover { x: i32, y: i32 },\n    Cerrar,\n}\n\nlet evento = Evento::Mover { x: 10, y: 20 };",
        ),
        (
            "Variantes distintas",
            "Cada variante puede tener una forma de datos diferente, pero todas pertenecen al mismo tipo Enum.",
            "enum Resultado {\n    Exito(String),\n    Error { codigo: u16, mensaje: String },\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "impl para un Enum");
    ui.label(
        RichText::new(
            "Un bloque impl puede añadir métodos al Enum. Dentro del método, match permite decidir según la variante actual.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Método de consulta",
            "Un método puede devolver información común para varias variantes, como si un estado está activo.",
            "enum Estado {\n    Conectado,\n    Desconectado,\n}\n\nimpl Estado {\n    fn activo(&self) -> bool {\n        match self {\n            Estado::Conectado => true,\n            Estado::Desconectado => false,\n        }\n    }\n}",
        ),
        (
            "Valor asociado",
            "El patrón puede extraer el dato guardado dentro de una variante.",
            "enum Mensaje {\n    Texto(String),\n    Fin,\n}\n\nlet mensaje = Mensaje::Texto(String::from(\"Hola\"));\nmatch mensaje {\n    Mensaje::Texto(texto) => println!(\"{texto}\"),\n    Mensaje::Fin => println!(\"Fin\"),\n}",
        ),
        state,
    );
}
