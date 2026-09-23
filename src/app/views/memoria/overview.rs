use eframe::egui;
use crate::views::pilares::anatomy::codigo_inline_chip;

fn titulo_overview(ui: &mut egui::Ui, titulo: &str, color: egui::Color32) {
    ui.heading(
        egui::RichText::new(titulo)
            .size(18.0)
            .strong()
            .color(color),
    );
    ui.add_space(8.0);
}

fn card_overview(
    ui: &mut egui::Ui,
    titulo: &str,
    descripcion: &str,
    naranja: egui::Color32,
    texto: egui::Color32,
) {
    let mut card = egui::Frame::new();
    card.fill = egui::Color32::from_rgb(14, 18, 26);
    card.inner_margin = egui::Margin::same(12);
    card.corner_radius = egui::CornerRadius::same(8);
    card.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    card.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(
            egui::RichText::new(titulo)
                .size(15.0)
                .strong()
                .color(naranja),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new(descripcion)
                .size(13.0)
                .color(texto)
                .line_height(Some(19.0)),
        );
    });
}

pub fn mostrar(ui: &mut egui::Ui) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let texto = egui::Color32::from_rgb(205, 215, 230);

    ui.heading(
        egui::RichText::new("Memory & Ownership")
            .size(24.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new(
            "Esta sesión presenta cómo Rust organiza los valores en memoria y cómo controla su uso. El recorrido comienza con Stack y Heap, continúa con Copy, Move, Ownership y Borrowing, y termina diferenciando String de &str.",
        )
        .size(14.5)
        .color(texto)
        .line_height(Some(21.0)),
    );
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new(
            "El Code Lab contiene los ejemplos, ejercicios y comprobaciones prácticas de cada tema. Este Overview funciona como el mapa general de la sesión.",
        )
        .size(13.5)
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    titulo_overview(ui, "Recorrido de la sesión", naranja);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Primero veremos cómo Rust organiza los valores entre").size(13.5).color(texto));
        codigo_inline_chip(ui, "Stack");
        ui.label(egui::RichText::new("y").size(13.5).color(texto));
        codigo_inline_chip(ui, "Heap");
        ui.label(egui::RichText::new(". Después diferenciaremos").size(13.5).color(texto));
        codigo_inline_chip(ui, "Copy");
        ui.label(egui::RichText::new(", que permite duplicar ciertos valores, de").size(13.5).color(texto));
        codigo_inline_chip(ui, "Move");
        ui.label(egui::RichText::new(", que transfiere la propiedad para evitar que existan dos dueños del mismo recurso.").size(13.5).color(texto));
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("A continuación estudiaremos").size(13.5).color(texto));
        codigo_inline_chip(ui, "Ownership");
        ui.label(egui::RichText::new("y").size(13.5).color(texto));
        codigo_inline_chip(ui, "Borrowing");
        ui.label(egui::RichText::new(": quién es responsable de cada valor y cómo podemos acceder a él mediante referencias. Finalmente veremos la diferencia entre").size(13.5).color(texto));
        codigo_inline_chip(ui, "String");
        ui.label(egui::RichText::new(", que posee sus datos, y").size(13.5).color(texto));
        codigo_inline_chip(ui, "&str");
        ui.label(egui::RichText::new(", que funciona como una vista prestada.").size(13.5).color(texto));
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Los ejemplos, ejercicios y comprobaciones prácticas de todos estos conceptos están organizados en").size(13.5).color(texto));
        codigo_inline_chip(ui, "Code Lab");
        ui.label(egui::RichText::new(".").size(13.5).color(texto));
    });

    ui.add_space(10.0);
    titulo_overview(ui, "Al finalizar", naranja);
    ui.columns(2, |columnas| {
        let resultados = [
            (
                "Organización de la memoria",
                "Comprenderás qué tipo de información se relaciona con Stack y Heap.",
            ),
            (
                "Copy y Move",
                "Diferenciarás una copia de una transferencia de propiedad.",
            ),
            (
                "Ownership y Borrowing",
                "Entenderás quién puede utilizar un valor y bajo qué reglas.",
            ),
            (
                "String y &str",
                "Reconocerás cuándo un texto posee sus datos y cuándo solo los observa.",
            ),
        ];

        for (indice, (titulo, descripcion)) in resultados.iter().enumerate() {
            card_overview(
                &mut columnas[indice % 2],
                titulo,
                descripcion,
                naranja,
                texto,
            );
            columnas[indice % 2].add_space(8.0);
        }
    });
}
