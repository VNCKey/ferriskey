use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar(ui: &mut egui::Ui) {
    session_title(ui, "Functions & Closures");
    session_intro(
        ui,
        "Esta sesión explica cómo las funciones reciben datos, devuelven resultados y cómo los Closures permiten guardar pequeñas operaciones que pueden capturar valores de su entorno.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Primero repasaremos cómo una función recibe valores mediante parámetros y cómo el")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Ownership");
        ui.label(
            RichText::new("o el")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Borrowing");
        ui.label(
            RichText::new("afectan ese intercambio. Después veremos retornos múltiples con")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Tuple");
        ui.label(
            RichText::new("y el tipo de unidad")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "()");
        ui.label(RichText::new(".").font(Typography::body()).color(Colors::TEXT_PRIMARY));
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Finalmente estudiaremos los")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Closures");
        ui.label(
            RichText::new(": su sintaxis, la captura del entorno,")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "move");
        ui.label(
            RichText::new("y la forma de recibirlos como parámetros mediante")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Fn");
        ui.label(RichText::new(".").font(Typography::body()).color(Colors::TEXT_PRIMARY));
    });

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Parameters & Ownership",
            "Una función puede recibir un valor, una referencia inmutable o una referencia mutable. La forma elegida determina si el dato se mueve, se presta o puede modificarse.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Closures",
            "Un Closure es una función anónima que puede guardarse en una variable y capturar valores del contexto donde fue creado.",
        );

        card_overview(
            &mut columns[1],
            "Return Values",
            "Una función puede devolver un valor, una Tuple con varios resultados o `()` cuando solo realiza una acción.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Fn & Higher-Order Functions",
            "Una función puede recibir otra operación como parámetro. `Fn` describe un Closure que puede invocarse sin modificar su entorno capturado.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    ui.label(
        RichText::new(
            "Una función organiza una operación con nombre; un Closure permite tratar una operación como un valor que puedes guardar, pasar y ejecutar cuando lo necesites.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
}
