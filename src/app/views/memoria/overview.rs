use crate::app::ui::{
    cell_centered, inline_code_chip_color, section_heading, session_intro, session_title, Colors,
    EducationalTable, Typography,
};
use eframe::egui;

pub fn mostrar(ui: &mut egui::Ui) {
    session_title(ui, "Memory & Ownership");
    session_intro(
        ui,
        "Esta sesión explica cómo Rust organiza los valores en memoria y cómo controla su uso sin depender de un recolector de basura. El recorrido va desde Stack y Heap hasta Ownership, Borrowing y la diferencia entre String y &str.",
    );

    ui.label(
        egui::RichText::new(
            "La idea central es seguir el recorrido de un valor: dónde se guarda, quién es responsable de él, cuándo puede copiarse, cuándo se mueve y cómo puede utilizarse sin tomar su propiedad.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    section_heading(ui, "Mapa de la sesión");
    ui.label(
        egui::RichText::new(
            "Estos conceptos se relacionan entre sí. La tabla funciona como referencia rápida antes de entrar en los ejemplos del Code Lab.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(8.0);

    let max_col_width = ((ui.available_width() - 120.0) / 2.0).max(180.0);
    EducationalTable::new("tabla_mapa_memory_ownership", &["Concepto", "Idea central"])
        .min_col_width(120.0)
        .max_col_width(max_col_width)
        .spacing(18.0, 6.0)
        .show(ui, |body| {
            let recorrido = [
                (
                    "Stack",
                    "Zona de memoria rápida para valores locales cuyo tamaño puede conocerse con facilidad.",
                ),
                (
                    "Heap",
                    "Zona de memoria dinámica para datos que pueden crecer o cuyo tamaño se administra durante la ejecución.",
                ),
                (
                    "Copy",
                    "Algunos valores se copian al asignarse; la variable original continúa disponible.",
                ),
                (
                    "Move",
                    "Un valor cambia de propietario; la variable anterior deja de utilizarse para evitar dos dueños activos.",
                ),
                (
                    "Ownership",
                    "Cada valor tiene un responsable que determina quién puede usarlo y cuándo se libera.",
                ),
                (
                    "Borrowing",
                    "Permite acceder a un valor mediante una Reference sin transferir su Ownership.",
                ),
                (
                    "String",
                    "Texto dinámico que posee su buffer y puede crecer cuando la variable es mutable.",
                ),
                (
                    "&str",
                    "Vista prestada de texto UTF-8; observa datos que pertenecen a otro valor o al programa.",
                ),
            ];

            for (concepto, idea) in recorrido {
                body.row(|ui| {
                    cell_centered(ui, |ui| {
                        inline_code_chip_color(ui, concepto, Colors::ORANGE_RUST)
                    });
                    ui.label(
                        egui::RichText::new(idea)
                            .font(Typography::body_small())
                            .color(Colors::TEXT_PRIMARY),
                    );
                });
            }
        });

    ui.add_space(18.0);
    section_heading(ui, "Cómo se conectan");
    ui.label(
        egui::RichText::new(
            "Stack y Heap describen dónde pueden vivir los datos. Copy y Move describen qué ocurre cuando un valor se asigna. Ownership establece quién es responsable, mientras Borrowing permite compartir un acceso controlado. String y &str muestran estas reglas aplicadas al texto.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(
            "El Code Lab desarrolla cada concepto paso a paso con ejemplos pequeños. Esta pestaña funciona como el mapa conceptual y la pestaña String vs &str reúne la referencia específica sobre texto.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_MUTED)
        .line_height(Some(20.0)),
    );
}
