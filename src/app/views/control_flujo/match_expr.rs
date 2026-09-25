use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

fn grupo_match(
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

pub fn mostrar_tab_match(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        RichText::new(
            "match compara un valor con varios patrones y ejecuta el primer brazo que coincide. Rust exige que el conjunto de patrones cubra todos los casos posibles.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    section_heading(ui, "Patrones frecuentes");
    ui.label(
        RichText::new(
            "Cada brazo tiene un patrón, el operador => y una expresión. El brazo final suele usar _ para cubrir los casos restantes.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_match(
        ui,
        (
            "Literal",
            "Compara el valor con un dato exacto, como un número o un texto.",
            "match dado {\n    1 => \"uno\",\n    _ => \"otro\",\n}",
        ),
        (
            "Rango",
            "Coincide con cualquier valor incluido dentro del rango indicado.",
            "match edad {\n    0..=17 => \"menor\",\n    _ => \"adulto\",\n}",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_match(
        ui,
        (
            "Alternativas",
            "El operador | permite asociar varios patrones al mismo brazo.",
            "match tecla {\n    'q' | 'Q' => \"salir\",\n    _ => \"continuar\",\n}",
        ),
        (
            "Comodín _",
            "Cubre cualquier caso que no haya coincidido con los brazos anteriores.",
            "match valor {\n    0 => \"cero\",\n    _ => \"otro\",\n}",
        ),
        state,
    );

    ui.add_space(12.0);
    grupo_match(
        ui,
        (
            "Match guard",
            "Añade una condición if después del patrón para decidir si el brazo puede ejecutarse.",
            "match numero {\n    n if n % 2 == 0 => \"par\",\n    _ => \"impar\",\n}",
        ),
        (
            "Orden de los brazos",
            "Rust prueba los brazos de arriba abajo y utiliza el primero que coincide.",
            "match estado {\n    \"listo\" => \"continuar\",\n    _ => \"esperar\",\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "match como expresión");
    ui.label(
        RichText::new(
            "Al igual que if, match puede devolver un valor. Todas sus ramas deben producir el mismo tipo para que la asignación sea válida.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    grupo_match(
        ui,
        (
            "Asignación",
            "El brazo elegido produce directamente el valor que se guarda en una variable.",
            "let nota = 85;\nlet letra = match nota {\n    90..=100 => 'A',\n    70..=89 => 'B',\n    _ => 'F',\n};",
        ),
        (
            "Tuple Pattern",
            "El patrón puede separar los elementos de una Tuple y decidir qué valor devolver.",
            "let punto = (0, 5);\n\nmatch punto {\n    (0, y) => y,\n    (x, 0) => x,\n    (x, y) => x + y,\n}",
        ),
        state,
    );

    divider(ui);
    section_heading(ui, "Exhaustividad");
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Rust comprueba que")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip_color(ui, "match", Colors::CYAN_ACCENT);
        ui.label(
            RichText::new("cubra todos los valores posibles. El patrón")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip_color(ui, "_", Colors::CYAN_ACCENT);
        ui.label(
            RichText::new("es una forma sencilla de cubrir los casos restantes.")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
    });
}
