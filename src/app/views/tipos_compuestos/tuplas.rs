use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar_compuesto_tupla(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    _cyan: egui::Color32,
    texto: egui::Color32,
) {
    let syntax_set = &state.editor.syntax_set;
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];

    ui.label(
        RichText::new(
            "Una Tupla agrupa una cantidad fija de valores relacionados. A diferencia de un Array, sus elementos pueden tener tipos diferentes y se accede a ellos mediante su posición.",
        )
        .font(Typography::body())
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    section_heading(ui, "Estructura de una Tupla");
    ui.label(
        RichText::new(
            "La posición de cada elemento forma parte de la estructura. La primera posición es 0 y el tipo completo describe el tipo de cada elemento.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_tupla_comp",
        &["Aspecto", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Declaración",
                "let persona = (\"Alice\", 30);",
                "(&str, i32)",
                "Combina valores de tipos diferentes en una estructura fija.",
            ),
            (
                "Acceso por posición",
                "persona.0",
                "&str",
                "Accede al primer elemento utilizando la sintaxis de punto.",
            ),
            (
                "Retorno múltiple",
                "fn limites() -> (i32, i32)",
                "(i32, i32)",
                "Una función puede devolver varios valores agrupados.",
            ),
            (
                "Unit Type",
                "let vacio = ();",
                "()",
                "Tupla de cero elementos; representa ausencia de un valor útil.",
            ),
            (
                "Tupla unitaria",
                "let numero = (5,);",
                "(i32,)",
                "La coma final distingue una Tupla de una expresión entre paréntesis.",
            ),
        ];

        for (aspecto, ejemplo, tipo, descripcion) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, aspecto, naranja)
                });
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, tipo, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Destructuring Pattern");
    ui.label(
        RichText::new(
            "Puedes extraer los elementos de una Tupla en nuevas variables. Esta forma resulta útil cuando una función devuelve varios valores relacionados y quieres dar un nombre claro a cada posición.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.horizontal_wrapped(|ui| {
        ui.label(
        RichText::new(
            "El patrón se lee de izquierda a derecha: cada nombre recibe el valor de su posición. Usa",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY),
        );
        inline_highlighted_code(ui, "_", syntax_set, theme, "rs");
        ui.label(
            RichText::new(
                "para ignorar una posición concreta y",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY),
        );
        inline_highlighted_code(ui, "..", syntax_set, theme, "rs");
        ui.label(
            RichText::new(
                "para ignorar las posiciones restantes.",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY),
        );
    });
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_tupla_ejemplos",
        &["Caso de uso", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Datos heterogéneos",
                "let persona = (\"Alice\", 30);",
                "persona.0 / persona.1",
                "Cada posición conserva su propio tipo.",
            ),
            (
                "Desestructuración",
                "let (nombre, edad) = persona;",
                "nombre, edad",
                "Crea variables a partir de las posiciones de la Tupla.",
            ),
            (
                "Ignorar una posición",
                "let (nombre, _) = persona;",
                "nombre",
                "El guion bajo descarta el valor que no necesitas conservar.",
            ),
            (
                "Conservar el primero",
                "let (primero, ..) = persona;",
                "primero",
                "Conserva el primer elemento e ignora las posiciones restantes.",
            ),
            (
                "Conservar el último",
                "let (.., ultimo) = persona;",
                "ultimo",
                "Ignora las posiciones anteriores y conserva el último elemento.",
            ),
            (
                "Conservar los extremos",
                "let (primero, .., ultimo) = (1, 2, 3, 4, 5);",
                "primero, ultimo",
                "Conserva el primer y el último elemento e ignora los valores intermedios.",
            ),
            (
                "Retorno múltiple",
                "let (min, max) = (2, 89);",
                "min = 2, max = 89",
                "Representa el resultado de una función que devuelve dos valores.",
            ),
            (
                "Unit Type",
                "let resultado: () = ();",
                "()",
                "Expresa que no existe un valor de retorno significativo.",
            ),
        ];

        for (caso, ejemplo, resultado, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(caso)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, resultado, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Detalles útiles");
    ui.label(
        RichText::new(
            "Estas formas muestran cómo una Tupla puede modificarse, anidarse o distinguirse de una expresión entre paréntesis.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_tupla_detalles",
        &["Caso", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Mutabilidad",
                "let mut persona = (\"Luis\", 25);\npersona.1 = 30;",
                "persona.1 = 30",
                "La Tupla completa debe ser mutable para cambiar una posición.",
            ),
            (
                "Tupla anidada",
                "let punto = ((1, 2), (3, 4));",
                "punto.0.1",
                "Una Tupla puede contener otras Tuplas y se accede siguiendo sus posiciones.",
            ),
            (
                "Un elemento",
                "let uno = (5,);",
                "(i32,)",
                "La coma final indica que es una Tupla de un elemento.",
            ),
            (
                "Solo paréntesis",
                "let numero = (5);",
                "i32",
                "Sin coma, los paréntesis solo agrupan una expresión; no crean una Tupla.",
            ),
        ];

        for (caso, ejemplo, resultado, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(caso)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, resultado, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    card(ui, |ui| {
        ui.label(
            RichText::new("Nota: las Tuplas no tienen un catálogo de métodos")
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);
        ui.label(
            RichText::new(
                "A diferencia de un Vec o un Slice, una Tupla se utiliza mediante sus posiciones, su Destructuring Pattern o como un valor completo. Por eso el acceso se escribe con expresiones como persona.0 y persona.1, no con métodos de recorrido.",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY)
            .line_height(Some(19.0)),
        );
    });
}
