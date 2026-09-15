use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_errores_info(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Información teórica detallada sobre las estrategias de manejo de errores en Rust: Errores Recuperables (Result) vs Errores Irrecuperables (panic!).",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Errores Recuperables vs Irrecuperables
    ui.label(
        egui::RichText::new("Errores Recuperables (Result) vs Irrecuperables (panic!)")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_errores_estrategias")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Estrategia").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Mecanismo en Rust").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Caso de Uso Recomendado").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Recuperable").strong().color(texto));
                ui.label(egui::RichText::new("Result<T, E>").monospace().color(cyan));
                ui.label("Fallos esperados que la aplicación puede manejar (archivo no encontrado, red fuera de servicio, datos mal formateados).");
                ui.end_row();

                ui.label(egui::RichText::new("Irrecuperable").strong().color(texto));
                ui.label(egui::RichText::new("panic!()").monospace().color(cyan));
                ui.label("Bugs o estados inválidos irrecuperables de los que la aplicación no puede reponerse (acceso fuera de rango en array).");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Ausencia de Valores (Option<T>)
    ui.label(
        egui::RichText::new("Eliminación del Problema de Puntero Nulo (Null / Undefined)")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_option_null")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Modelado en Rust").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Garantía de Seguridad").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Ausencia de Valor").strong().color(texto));
                ui.label(egui::RichText::new("Option<T> (Some / None)").monospace().color(cyan));
                ui.label("El compilador obliga a manejar explícitamente el caso None antes de poder acceder al valor T.");
                ui.end_row();
            });
    });
}
