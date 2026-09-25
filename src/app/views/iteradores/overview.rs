use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar(ui: &mut egui::Ui) {
    session_title(ui, "Iteradores y Pipelines");
    session_intro(
        ui,
        "Esta sesión explica cómo recorrer colecciones en Rust, cómo elegir entre referencias y Ownership, y cómo construir pipelines claros mediante adaptadores y consumidores.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Primero veremos los modos de iteración con")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, ".iter()");
        ui.label(RichText::new(",").font(Typography::body()).color(Colors::TEXT_PRIMARY));
        inline_code_chip(ui, ".iter_mut()");
        ui.label(RichText::new("y").font(Typography::body()).color(Colors::TEXT_PRIMARY));
        inline_code_chip(ui, ".into_iter()");
        ui.label(
            RichText::new("para distinguir entre prestar, modificar y consumir una colección.")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Después construiremos pipelines con adaptadores como")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, ".filter()");
        ui.label(RichText::new("y").font(Typography::body()).color(Colors::TEXT_PRIMARY));
        inline_code_chip(ui, ".map()");
        ui.label(
            RichText::new("y los activaremos con consumidores como")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, ".collect()");
        ui.label(RichText::new(".").font(Typography::body()).color(Colors::TEXT_PRIMARY));
    });

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Iteration Modes",
            "`.iter()` presta referencias, `.iter_mut()` permite modificar elementos y `.into_iter()` transfiere valores consumiendo la colección.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Lazy Adaptors",
            "Métodos como `.filter()` y `.map()` describen transformaciones sin ejecutarlas inmediatamente.",
        );

        card_overview(
            &mut columns[1],
            "Consumers",
            "`.collect()`, `.sum()` y `.find()` avanzan el iterador y producen un resultado final.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Pipelines",
            "Encadenar operaciones pequeñas permite expresar una transformación completa de forma clara y eficiente.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    ui.label(
        RichText::new(
            "Un iterador no es necesariamente una colección nueva: es una forma controlada de pedir valores uno a uno, aplicar transformaciones y decidir cuándo obtener el resultado.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
