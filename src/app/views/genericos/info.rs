use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_genericos_info(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    _cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Información teórica sobre cómo el compilador de Rust optimiza el código genérico mediante el proceso de Monomorfización.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Monomorfización
    ui.label(
        egui::RichText::new("Monomorfización (Monomorphization)")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_genericos_mono")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Fase").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Mecanismo del Compilador").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Impacto en Rendimiento").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Compilación").strong().color(texto));
                ui.label("rustc analiza todos los tipos concretos (i32, String) usados con el genérico.");
                ui.label("Genera una copia optimizada del código máquina para cada tipo concreto.");
                ui.end_row();

                ui.label(egui::RichText::new("Ejecución").strong().color(texto));
                ui.label("No hay sobrecosto de búsqueda de métodos en tiempo de ejecución.");
                ui.label("Zero-Cost Abstractions: el código genérico corre tan rápido como el código escrito a mano.");
                ui.end_row();
            });
    });
}
