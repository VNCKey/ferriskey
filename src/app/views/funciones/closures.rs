use crate::app::ui::*;
use crate::app::AppState;
use crate::views::funciones::grupo_funciones;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_closures(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Un Closure es una función anónima que puede guardarse en una variable, recibir parámetros y capturar valores del contexto donde fue creado.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Sintaxis y uso");
    ui.label(
        RichText::new(
            "Los parámetros se escriben entre barras verticales. El cuerpo puede ser una expresión corta o un bloque con varias instrucciones.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_funciones(
        ui,
        (
            "Closure básico",
            "Un Closure puede guardarse en una variable y llamarse como una función normal.",
            "let sumar_uno = |numero: i32| numero + 1;\nlet resultado = sumar_uno(5);",
        ),
        (
            "Inferencia de tipos",
            "Rust puede inferir los tipos cuando el contexto deja claro qué recibe y devuelve el Closure.",
            "let sumar_uno = |numero| numero + 1;\nlet resultado = sumar_uno(5);",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Captura del entorno");
    ui.label(
        RichText::new(
            "Un Closure puede utilizar variables declaradas fuera de su cuerpo. Rust elige la forma de captura según si las lee, las modifica o toma su propiedad.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_funciones(
        ui,
        (
            "Captura inmutable",
            "Cuando solo lee una variable externa, el Closure puede utilizarla sin modificarla.",
            "let factor = 10;\nlet multiplicar = |numero| numero * factor;\n\nlet resultado = multiplicar(5);",
        ),
        (
            "Captura mutable",
            "Si modifica una variable externa, el Closure necesita acceso mutable y normalmente se invoca como `FnMut`.",
            "let mut contador = 0;\nlet mut avanzar = || {\n    contador += 1;\n};\n\navanzar();",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_funciones(
        ui,
        (
            "Captura con move",
            "`move` obliga al Closure a tomar el `Ownership` de los valores capturados.",
            "let mensaje = String::from(\"Hola Rust\");\nlet imprimir = move || println!(\"{mensaje}\");\n\nimprimir();",
        ),
        (
            "Después del movimiento",
            "Cuando una `String` fue movida al Closure, el código exterior ya no puede utilizar la variable original.",
            "let mensaje = String::from(\"Rust\");\nlet imprimir = move || println!(\"{mensaje}\");\n\nimprimir();\n// println!(\"{mensaje}\"); // value moved",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Closures como parámetros");
    ui.label(
        RichText::new(
            "Una función puede recibir una operación para decidir qué transformación aplicar a sus datos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_funciones(
        ui,
        (
            "impl Fn",
            "`impl Fn(i32) -> i32` acepta un Closure que recibe un `i32` y devuelve otro `i32`.",
            "fn aplicar(valor: i32, op: impl Fn(i32) -> i32) -> i32 {\n    op(valor)\n}",
        ),
        (
            "Higher-Order Function",
            "Una función que recibe otra función o Closure puede reutilizar su estructura con operaciones diferentes.",
            "let cuadrado = aplicar(7, |numero| numero * numero);\nlet doble = aplicar(7, |numero| numero * 2);",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Regla esencial");
    ui.label(
        RichText::new(
            "Un Closure puede ser pequeño y directo, pero sigue las reglas de tipos, Ownership y Borrowing de Rust. La sintaxis es corta; las reglas del lenguaje siguen siendo las mismas.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
