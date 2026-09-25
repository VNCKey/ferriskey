use crate::app::ui::*;
use crate::app::AppState;
use crate::views::funciones::grupo_funciones;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_retorno(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Una función puede devolver un único valor, agrupar varios resultados en una Tuple o indicar que solo realiza una acción.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Valores de retorno");
    ui.label(
        RichText::new(
            "El tipo escrito después de `->` describe el valor que la función entrega al código que la llamó.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_funciones(
        ui,
        (
            "Un valor",
            "La función declara un único tipo de retorno y la última expresión produce ese valor.",
            "fn duplicar(numero: i32) -> i32 {\n    numero * 2\n}\n\nlet resultado = duplicar(5);",
        ),
        (
            "Tuple de retorno",
            "Una `Tuple` agrupa varios resultados relacionados y permite recibirlos mediante un `Destructuring Pattern`.",
            "fn min_max(a: i32, b: i32) -> (i32, i32) {\n    if a < b { (a, b) } else { (b, a) }\n}\n\nlet (menor, mayor) = min_max(10, 5);",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Tipo de unidad");
    ui.label(
        RichText::new(
            "Cuando una función no necesita entregar un dato útil, su retorno es `()`, el Unit Type. Puede escribirse de forma explícita u omitirse.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_funciones(
        ui,
        (
            "Retorno explícito",
            "`-> ()` deja visible que la función no devuelve un valor con información para el llamador.",
            "fn mostrar(mensaje: &str) -> () {\n    println!(\"{mensaje}\");\n}",
        ),
        (
            "Retorno implícito",
            "Cuando el cuerpo termina con una instrucción que produce `()`, Rust permite omitir la anotación.",
            "fn mostrar(mensaje: &str) {\n    println!(\"{mensaje}\");\n}\n\nmostrar(\"Hola Rust\");",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Regla de la última expresión");
    ui.label(
        RichText::new(
            "Una expresión sin punto y coma al final del cuerpo puede convertirse en el resultado de la función. Si escribes `;`, la instrucción deja de devolver ese valor directamente.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
