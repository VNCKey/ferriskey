use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_traits_info(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Información teórica sobre polimorfismo y mecanismos de resolución de métodos en Rust: Static Dispatch vs Dynamic Dispatch.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Static Dispatch vs Dynamic Dispatch
    ui.label(
        egui::RichText::new("Static Dispatch (Monomorfización) vs Dynamic Dispatch (dyn Trait)")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_traits_dispatch")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Mecanismo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Rendimiento & Memoria").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Static Dispatch").strong().color(texto));
                ui.label(egui::RichText::new("fn f<T: Trait>(val: T)").monospace().color(cyan));
                ui.label("Monomorfización en tiempo de compilación. Cero sobrecosto en ejecución (Zero-Cost).");
                ui.end_row();

                ui.label(egui::RichText::new("Dynamic Dispatch").strong().color(texto));
                ui.label(egui::RichText::new("Box<dyn Trait>").monospace().color(cyan));
                ui.label("Búsqueda en vtable en tiempo de ejecución. Permite heterogeneidad en colecciones.");
                ui.end_row();
            });
    });
}
