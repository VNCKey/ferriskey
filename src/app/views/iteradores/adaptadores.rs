use crate::app::AppState;
use crate::app::ui::*;
use crate::views::iteradores::grupo_iteradores;
use eframe::egui::{self, RichText};

pub fn mostrar_tab_adaptadores(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Los adaptadores transforman un iterador en otro. Son perezosos: preparan el recorrido, pero esperan a que un consumidor solicite los valores.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Transformar y filtrar");
    ui.label(
        RichText::new(
            "`map` cambia cada elemento y `filter` conserva únicamente los elementos que cumplen una condición.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            ".map()",
            "Aplica un Closure a cada elemento y produce un valor transformado.",
            "let numeros = vec![1, 2, 3];\nlet dobles = numeros\n    .iter()\n    .map(|numero| numero * 2);",
        ),
        (
            ".filter()",
            "Evalúa una condición y conserva los elementos para los que devuelve `true`.",
            "let numeros = vec![1, 2, 3, 4];\nlet pares = numeros\n    .iter()\n    .filter(|numero| **numero % 2 == 0);",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Limitar y combinar");
    ui.label(
        RichText::new(
            "Otros adaptadores controlan cuántos valores recorres o combinan varias fuentes en un solo pipeline.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            ".take() y .skip()",
            "`take` limita la cantidad de valores y `skip` ignora los primeros elementos.",
            "let numeros = 1..=6;\nlet siguientes = numeros\n    .skip(2)\n    .take(2);",
        ),
        (
            ".zip() y .chain()",
            "`zip` combina posiciones de dos iteradores y `chain` los recorre uno después del otro.",
            "let a = [1, 2];\nlet b = [\"uno\", \"dos\"];\nlet pares = a.iter().zip(b.iter());",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_iteradores(
        ui,
        (
            ".enumerate()",
            "Añade un índice a cada elemento y devuelve pares como `(indice, valor)`.",
            "let frutas = [\"manzana\", \"pera\"];\n\nfor (indice, fruta) in frutas.iter().enumerate() {\n    let _ = (indice, fruta);\n}",
        ),
        (
            ".rev() y .step_by()",
            "`rev` invierte el recorrido y `step_by` avanza usando saltos regulares.",
            "let inverso = (1..=5).rev();\nlet saltos = (0..10).step_by(2);",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Regla de los adaptadores");
    ui.label(
        RichText::new(
            "Encadenar adaptadores no crea por sí solo una colección final. El pipeline se ejecuta cuando llega un consumidor como `collect`, `sum` o `find`.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
