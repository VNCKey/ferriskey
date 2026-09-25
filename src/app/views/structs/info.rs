use crate::app::AppState;
use crate::app::ui::*;
use crate::views::structs::grupo_custom_types;
use eframe::egui::{self, RichText};

pub fn mostrar_structs_info(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Structs y Enums son formas diferentes de modelar datos. Comprender su composición ayuda a elegir un tipo claro y a razonar sobre su representación.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Struct y Enum");
    ui.label(
        RichText::new(
            "Una Struct reúne todos sus campos al mismo tiempo. Un Enum representa una variante activa entre varias posibilidades.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Struct: tipo producto",
            "Una instancia contiene el campo A y el campo B y el campo C simultáneamente.",
            "struct Servidor {\n    host: String,\n    puerto: u16,\n    activo: bool,\n}",
        ),
        (
            "Enum: tipo suma",
            "Una instancia contiene la variante A o la variante B o la variante C, pero solo una a la vez.",
            "enum Estado {\n    Activo,\n    Error(String),\n    Apagado,\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Representación conceptual");
    ui.label(
        RichText::new(
            "La representación exacta depende de los tipos de los campos y de la alineación. La idea importante es cómo se compone el valor, no memorizar un cálculo de bytes para cada caso.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Campos contiguos",
            "Una Struct organiza sus campos en una composición única, respetando las reglas de tamaño y alineación del compilador.",
            "struct Punto {\n    x: i32,\n    y: i32,\n}\n\nlet punto = Punto { x: 10, y: 20 };",
        ),
        (
            "Discriminante y datos",
            "Un Enum necesita identificar su variante y puede transportar el contenido asociado a esa variante.",
            "enum Resultado {\n    Ok(u32),\n    Error(String),\n}\n\nlet respuesta = Resultado::Ok(200);",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Diseño de Custom Types");
    ui.label(
        RichText::new(
            "Crea una Struct cuando los datos existen juntos y una Enum cuando el valor debe representar alternativas. Añade Traits para expresar capacidades compartidas.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
