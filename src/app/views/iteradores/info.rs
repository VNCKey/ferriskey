use crate::app::AppState;
use crate::app::ui::*;
use crate::views::iteradores::grupo_iteradores;
use eframe::egui::{self, RichText};

pub fn mostrar_iteradores_info(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Esta pestaña reúne reglas prácticas para construir iteradores legibles, seguros y eficientes.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Lazy Evaluation");
    ui.label(
        RichText::new(
            "Los adaptadores preparan el pipeline y los consumidores solicitan los valores. Esta separación evita trabajo que nunca llega a utilizarse.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            "Pipeline sin consumidor",
            "Crear un adaptador no recorre la colección. El trabajo queda pendiente hasta que algo solicite un resultado.",
            "let numeros = [1, 2, 3];\nlet dobles = numeros.iter().map(|n| n * 2);",
        ),
        (
            "Pipeline ejecutado",
            "Un consumidor como `collect` hace avanzar el iterador y ejecuta las transformaciones encadenadas.",
            "let numeros = [1, 2, 3];\nlet dobles: Vec<_> = numeros\n    .iter()\n    .map(|n| n * 2)\n    .collect();",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Zero-Cost Abstractions");
    ui.label(
        RichText::new(
            "Los iteradores expresan operaciones de alto nivel y el compilador puede optimizarlas para producir código eficiente, sin renunciar a las comprobaciones de seguridad de Rust.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            "Legibilidad",
            "Divide un pipeline en pasos pequeños: filtrar, transformar y recolectar. Cada operación expresa una intención concreta.",
            "let resultado: Vec<_> = datos\n    .iter()\n    .filter(|dato| valido(dato))\n    .map(transformar)\n    .collect();",
        ),
        (
            "Medir antes de optimizar",
            "La abstracción suele ser eficiente, pero conviene medir cuando el pipeline forma parte de una ruta crítica.",
            "let inicio = std::time::Instant::now();\nlet resultado: Vec<_> = datos.iter().map(transformar).collect();\nlet duracion = inicio.elapsed();",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Buenas prácticas");
    ui.label(
        RichText::new(
            "Prefiere nombres claros, evita encadenamientos difíciles de leer y elige el modo de iteración que exprese correctamente tu relación con la colección.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_iteradores(
        ui,
        (
            "Leer sin consumir",
            "Utiliza `.iter()` cuando todavía necesitarás la colección después del recorrido.",
            "let valores = vec![1, 2, 3];\nlet suma: i32 = valores.iter().sum();\nlet cantidad = valores.len();",
        ),
        (
            "Consumir de forma intencional",
            "Utiliza `.into_iter()` cuando el siguiente paso debe quedarse con los valores y la colección original ya no es necesaria.",
            "let valores = vec![String::from(\"Rust\")];\nlet textos: Vec<_> = valores.into_iter().collect();",
        ),
        state,
    );
}
