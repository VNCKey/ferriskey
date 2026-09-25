use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar(ui: &mut egui::Ui) {
    session_title(ui, "Custom Types");
    session_intro(
        ui,
        "Esta sesión explica cómo crear tipos propios en Rust para representar datos relacionados, estados posibles y comportamientos compartidos.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Primero estudiaremos cómo una")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Struct");
        ui.label(
            RichText::new("agrupa campos bajo un nombre. Después veremos cómo un")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Enum");
        ui.label(
            RichText::new("representa una variante entre varias posibilidades.")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Finalmente aprenderemos a compartir comportamiento mediante")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Trait");
        ui.label(
            RichText::new("y a extraer datos de variantes con")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Pattern Matching");
        ui.label(RichText::new(".").font(Typography::body()).color(Colors::TEXT_PRIMARY));
    });

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Structs",
            "Un `Struct` es un tipo compuesto con campos nombrados. Sirve para representar una entidad con datos relacionados.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Enums",
            "Un `Enum` modela alternativas. Cada variante puede ser simple o contener datos propios.",
        );

        card_overview(
            &mut columns[1],
            "Traits",
            "Un `Trait` define un comportamiento compartido que distintos tipos pueden implementar.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Pattern Matching",
            "Los patrones permiten comprobar variantes y extraer sus valores con `match`, `if let` y `while let`.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    ui.label(
        RichText::new(
            "Los Custom Types permiten expresar el dominio del programa con nombres y reglas propias, en lugar de trabajar únicamente con valores primitivos sueltos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
