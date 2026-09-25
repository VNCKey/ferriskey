use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

fn grupo_bucles(
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

pub fn mostrar_tab_bucles(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "Rust ofrece tres formas principales de repetición: loop, while y for. Cada una expresa una intención diferente y puede combinarse con break y continue.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Tipos de bucles");
    ui.label(
        RichText::new(
            "Elige el bucle según cómo sabes que debe terminar la repetición.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_bucles(
        ui,
        (
            "loop",
            "Repite indefinidamente hasta que el código decide terminarlo con break.",
            "let resultado = loop {\n    break 42;\n};",
        ),
        (
            "while",
            "Comprueba una condición antes de cada vuelta y continúa mientras sea true.",
            "let mut n = 3;\n\nwhile n > 0 {\n    n -= 1;\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "for y rangos");
    ui.label(
        RichText::new(
            "for recorre los valores de un rango o los elementos de una colección sin que tengas que controlar manualmente cada índice.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_bucles(
        ui,
        (
            "for sobre un rango",
            "El rango 1..=3 incluye los dos límites y produce 1, 2 y 3.",
            "for numero in 1..=3 {\n    let _ = numero;\n}",
        ),
        (
            "for sobre una colección",
            "Una referencia permite recorrer los elementos sin tomar la propiedad de la colección.",
            "let numeros = [10, 20, 30];\n\nfor valor in &numeros {\n    let _ = valor;\n}",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_bucles(
        ui,
        (
            "Rango exclusivo",
            "1..5 termina antes de 5, por eso recorre 1, 2, 3 y 4.",
            "for n in 1..5 {\n    let _ = n;\n}",
        ),
        (
            "Rango inclusivo",
            "1..=5 también incluye el límite final y recorre hasta 5.",
            "for n in 1..=5 {\n    let _ = n;\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "break, continue y etiquetas");
    ui.label(
        RichText::new(
            "Estas instrucciones cambian el recorrido de un bucle sin tener que añadir condiciones innecesarias alrededor de todo el código.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_bucles(
        ui,
        (
            "break",
            "Termina el bucle inmediatamente cuando ya no necesitas más vueltas.",
            "loop {\n    if encontrado {\n        break;\n    }\n}",
        ),
        (
            "continue",
            "Salta el resto de la vuelta actual y comienza la siguiente.",
            "for valor in valores {\n    if invalido {\n        continue;\n    }\n\n    procesar(valor);\n}",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_bucles(
        ui,
        (
            "Etiqueta de bucle",
            "Una etiqueta permite controlar un bucle exterior cuando existen bucles anidados.",
            "'externo: for fila in filas {\n    for celda in fila {\n        if vacia(celda) {\n            break 'externo;\n        }\n    }\n}",
        ),
        (
            "break con valor",
            "loop puede devolver un valor mediante break valor; y guardarlo en una variable.",
            "let encontrado = loop {\n    if listo() {\n        break 42;\n    }\n};",
        ),
        state,
    );
}
