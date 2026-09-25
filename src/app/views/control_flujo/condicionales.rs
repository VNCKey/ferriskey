use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

fn grupo_condicionales(
    ui: &mut egui::Ui,
    izquierda: (&str, &str, &str),
    derecha: (&str, &str, &str),
    state: &AppState,
) {
    let syntax_set = &state.editor.syntax_set;
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];

    ui.columns(2, |columns| {
        columns[0].label(
            RichText::new(izquierda.0)
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        columns[0].add_space(6.0);
        columns[0].label(
            RichText::new(izquierda.1)
                .font(Typography::body_small())
                .color(Colors::TEXT_PRIMARY)
                .line_height(Some(19.0)),
        );

        columns[1].label(
            RichText::new(derecha.0)
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        columns[1].add_space(6.0);
        columns[1].label(
            RichText::new(derecha.1)
                .font(Typography::body_small())
                .color(Colors::TEXT_PRIMARY)
                .line_height(Some(19.0)),
        );
    });
    ui.add_space(10.0);

    ui.columns(2, |columns| {
        highlighted_code_block(&mut columns[0], izquierda.2, syntax_set, theme, "rs");
        highlighted_code_block(&mut columns[1], derecha.2, syntax_set, theme, "rs");
    });
}

pub fn mostrar_tab_condicionales(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Las condicionales permiten ejecutar distintos bloques según una condición booleana. En Rust, if puede actuar como una instrucción y también como una expresión que devuelve un valor.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "if, else if y else");
    ui.label(
        RichText::new(
            "Cada rama contiene un bloque. La condición debe producir bool y no necesita paréntesis alrededor.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_condicionales(
        ui,
        (
            "if / else",
            "Elige entre dos caminos según una condición. Si la condición es true se ejecuta el primer bloque; de lo contrario se ejecuta else.",
            "let edad = 20;\n\nif edad >= 18 {\n    println!(\"adulto\");\n} else {\n    println!(\"menor\");\n}",
        ),
        (
            "else if",
            "Permite comprobar varias alternativas en orden. Solo se ejecuta la primera rama cuya condición sea true.",
            "let nota = 85;\n\nif nota >= 90 {\n    println!(\"A\");\n} else if nota >= 70 {\n    println!(\"B\");\n} else {\n    println!(\"F\");\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "if como expresión");
    ui.label(
        RichText::new(
            "Una expresión produce un valor. Por eso el resultado de if puede guardarse directamente en una variable, siempre que sus ramas devuelvan el mismo tipo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_condicionales(
        ui,
        (
            "if devuelve un valor",
            "La última expresión de cada rama se convierte en el resultado de la construcción if.",
            "let activo = true;\n\nlet estado = if activo {\n    \"listo\"\n} else {\n    \"inactivo\"\n};",
        ),
        (
            "Ramas del mismo tipo",
            "Las dos ramas deben producir el mismo tipo. En este ejemplo, ambas devuelven un &str.",
            "let temperatura = 25;\n\nlet clima: &str = if temperatura > 20 {\n    \"cálido\"\n} else {\n    \"frío\"\n};",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Operadores lógicos");
    ui.label(
        RichText::new(
            "Puedes combinar condiciones booleanas sin crear bloques anidados innecesarios.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_condicionales(
        ui,
        (
            "AND y OR",
            "&& exige que ambas condiciones sean true. || devuelve true cuando al menos una condición se cumple.",
            "let edad = 20;\nlet permiso = true;\n\nlet puede_entrar = edad >= 18 && permiso;\nlet puede_salir = edad < 18 || permiso;",
        ),
        (
            "NOT",
            "! invierte un valor bool: true se convierte en false y false se convierte en true.",
            "let bloqueado = false;\n\nif !bloqueado {\n    println!(\"Acceso disponible\");\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Regla esencial");
    ui.label(
        RichText::new(
            "Una condición debe producir bool. Cuando if devuelve un valor, todas sus ramas deben devolver el mismo tipo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
