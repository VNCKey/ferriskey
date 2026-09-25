use crate::app::AppState;
use crate::app::ui::*;
use crate::views::structs::grupo_custom_types;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_structs(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Las Structs permiten agrupar datos relacionados bajo un tipo propio. Sus campos tienen nombres y sus bloques impl pueden añadir funciones asociadas y métodos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Formas de crear una Struct");
    ui.label(
        RichText::new(
            "Elige la forma que mejor describa los datos: campos nombrados, posiciones fijas o una marca sin campos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Campos nombrados",
            "Cada campo tiene un nombre y un tipo. Esta forma es clara cuando una instancia representa una entidad con varias propiedades.",
            "struct Usuario {\n    nombre: String,\n    edad: u32,\n}\n\nlet usuario = Usuario {\n    nombre: String::from(\"Ana\"),\n    edad: 25,\n};",
        ),
        (
            "Tuple Struct",
            "Sus campos se identifican por posición. Es útil para crear un tipo distinto sin escribir nombres para cada campo.",
            "struct Color(u8, u8, u8);\nstruct Punto3D(f64, f64, f64);\n\nlet rojo = Color(255, 0, 0);\nlet primer_canal = rojo.0;",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "impl y comportamiento");
    ui.label(
        RichText::new(
            "Un bloque impl conecta funciones y métodos con una Struct. Las funciones asociadas no necesitan una instancia; los métodos reciben self.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_custom_types(
        ui,
        (
            "Función asociada",
            "Una función como `nuevo` puede construir una instancia y llamarse mediante el nombre del tipo.",
            "struct Rectangulo {\n    ancho: u32,\n    alto: u32,\n}\n\nimpl Rectangulo {\n    fn nuevo(ancho: u32, alto: u32) -> Self {\n        Self { ancho, alto }\n    }\n}",
        ),
        (
            "Método con &self",
            "`&self` presta la instancia para leer sus campos sin consumirla.",
            "impl Rectangulo {\n    fn area(&self) -> u32 {\n        self.ancho * self.alto\n    }\n}\n\nlet figura = Rectangulo::nuevo(4, 3);\nlet area = figura.area();",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_custom_types(
        ui,
        (
            "Método con &mut self",
            "`&mut self` permite cambiar los campos. La instancia debe estar declarada con `let mut`.",
            "struct Contador {\n    valor: u32,\n}\n\nimpl Contador {\n    fn incrementar(&mut self) {\n        self.valor += 1;\n    }\n}",
        ),
        (
            "Unit Struct",
            "Una Unit Struct no tiene campos. Puede representar una marca o un tipo que solo necesita identidad propia.",
            "struct Marcador;\n\nlet _marcador = Marcador;",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Idea esencial");
    ui.label(
        RichText::new(
            "Una Struct define la forma de los datos y un bloque impl define qué puede hacer ese tipo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
