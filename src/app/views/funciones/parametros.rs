use crate::app::ui::*;
use crate::app::AppState;
use crate::views::funciones::grupo_funciones;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_parametros(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Los parámetros indican qué datos necesita una función. La forma de recibirlos define si la función toma Ownership, presta el valor o puede modificarlo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Formas de recibir datos");
    ui.label(
        RichText::new(
            "Compara las tres formas básicas antes de elegir cómo diseñar una función.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_funciones(
        ui,
        (
            "Por valor: T",
            "La función recibe el valor. Si el tipo no implementa `Copy`, la variable original deja de poder utilizarse después del `call`.",
            "fn consumir(texto: String) {\n    println!(\"{texto}\");\n}\n\nlet texto = String::from(\"Rust\");\nconsumir(texto);",
        ),
        (
            "Borrowing: &T",
            "La función recibe una referencia de solo lectura. El dueño conserva su `Ownership` y puede seguir utilizando el valor.",
            "fn longitud(texto: &String) -> usize {\n    texto.len()\n}\n\nlet texto = String::from(\"Rust\");\nlet cantidad = longitud(&texto);",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Permitir modificaciones");
    ui.label(
        RichText::new(
            "Una referencia mutable presta el valor con permiso explícito para cambiarlo en la función.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_funciones(
        ui,
        (
            "Borrowing Mutable: &mut T",
            "`&mut T` permite modificar el valor original. La variable dueña debe declararse con `let mut`.",
            "fn agregar_saludo(texto: &mut String) {\n    texto.push_str(\", hola\");\n}\n\nlet mut texto = String::from(\"Rust\");\nagregar_saludo(&mut texto);",
        ),
        (
            "Regla de acceso",
            "Durante el mismo acceso no se mezclan una referencia mutable y referencias inmutables al mismo valor.",
            "let mut texto = String::from(\"Rust\");\nlet lectura = &texto;\n\nprintln!(\"{lectura}\");\n\nlet cambio = &mut texto;\ncambio.push('!');",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Cómo elegir");
    ui.label(
        RichText::new(
            "Usa `T` cuando la función debe recibir la propiedad, `&T` cuando solo necesita leer y `&mut T` cuando debe modificar el valor original.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
