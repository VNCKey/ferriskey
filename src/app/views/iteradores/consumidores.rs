use crate::app::AppState;
use crate::app::ui::*;
use crate::views::iteradores::grupo_iteradores;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_consumidores(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Los consumidores avanzan un iterador y producen un resultado final. Son los que hacen que un pipeline perezoso ejecute su trabajo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Crear resultados");
    ui.label(
        RichText::new(
            "`collect` materializa los valores en una colección y `sum` o `product` los reducen a un resultado numérico.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            ".collect()",
            "Reúne los valores en una colección como `Vec<T>`. El tipo de destino ayuda a Rust a saber qué construir.",
            "let numeros = [1, 2, 3];\nlet dobles: Vec<i32> = numeros\n    .iter()\n    .map(|numero| numero * 2)\n    .collect();",
        ),
        (
            ".sum() y .product()",
            "Calculan la suma o el producto de todos los valores del iterador.",
            "let numeros = [1, 2, 3, 4];\nlet suma: i32 = numeros.iter().sum();\nlet producto: i32 = numeros.iter().product();",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Buscar y comprobar");
    ui.label(
        RichText::new(
            "Estos consumidores pueden detenerse antes de recorrer todos los valores cuando ya obtuvieron la respuesta necesaria.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            ".find()",
            "Devuelve el primer elemento que cumple una condición.",
            "let numeros = [3, 8, 11];\nlet encontrado = numeros\n    .iter()\n    .find(|numero| **numero > 5);",
        ),
        (
            ".any() y .all()",
            "`any` comprueba si al menos un elemento cumple y `all` si todos cumplen.",
            "let numeros = [2, 4, 6];\nlet hay_impar = numeros.iter().any(|n| n % 2 != 0);\nlet todos_pares = numeros.iter().all(|n| n % 2 == 0);",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_iteradores(
        ui,
        (
            ".count() y .last()",
            "`count` cuenta los elementos y `last` devuelve el último valor del recorrido.",
            "let numeros = [10, 20, 30];\nlet cantidad = numeros.iter().count();\nlet ultimo = numeros.iter().last();",
        ),
        (
            ".for_each()",
            "Ejecuta un Closure con cada elemento y se utiliza cuando buscas un efecto secundario.",
            "let nombres = [\"Ana\", \"Luis\"];\n\nnombres.iter().for_each(|nombre| {\n    println!(\"Hola {nombre}\");\n});",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Consumir o conservar");
    ui.label(
        RichText::new(
            "Antes de consumir una colección, decide si necesitas conservarla para utilizarla después o si el nuevo resultado puede tomar sus valores.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
