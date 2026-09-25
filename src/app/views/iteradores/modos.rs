use crate::app::AppState;
use crate::app::ui::*;
use crate::views::iteradores::grupo_iteradores;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_modos(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Rust ofrece tres formas principales de recorrer una colección. La diferencia está en si el iterador presta referencias, permite modificar elementos o transfiere su Ownership.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Modos de iteración");
    ui.label(
        RichText::new(
            "Elige el modo según qué debe ocurrir con la colección después del recorrido.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            ".iter()",
            "Presta referencias inmutables. La colección conserva su `Ownership` y sigue disponible después del recorrido.",
            "let numeros = vec![1, 2, 3];\n\nfor numero in numeros.iter() {\n    let _ = numero;\n}\n\nlet cantidad = numeros.len();",
        ),
        (
            ".iter_mut()",
            "Presta referencias mutables. Permite modificar los elementos sin mover la colección.",
            "let mut numeros = vec![1, 2, 3];\n\nfor numero in numeros.iter_mut() {\n    *numero *= 2;\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, ".into_iter()");
    ui.label(
        RichText::new(
            "El iterador por valor entrega los elementos y consume la colección original. Es útil cuando el nuevo código necesita quedarse con los valores.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            "Mover valores",
            "Cada vuelta recibe el valor `T`, no una referencia `&T`. La colección deja de estar disponible después de consumirla.",
            "let nombres = vec![String::from(\"Ana\"), String::from(\"Luis\")];\n\nfor nombre in nombres.into_iter() {\n    let _ = nombre;\n}",
        ),
        (
            "Elegir el modo",
            "Usa `.iter()` para leer, `.iter_mut()` para modificar y `.into_iter()` cuando quieras consumir y mover los valores.",
            "let valores = vec![1, 2, 3];\n\nlet lectura = valores.iter();\nlet propios = valores.into_iter();",
        ),
        state,
    );
}
