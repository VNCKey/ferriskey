use crate::app::AppState;
use crate::views::conceptos::mostrar_contenido_tipos_primitivos;
use eframe::egui;

/// Referencia de consulta rápida para los tipos primitivos y sus métodos.
/// El contenido detallado vive en Conceptos y se reutiliza aquí para mantener
/// una única fuente de información.
pub fn mostrar_referencia_tipos(ui: &mut egui::Ui, state: &mut AppState) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(8.0);
            ui.heading(
                egui::RichText::new("LIB · Tipos y métodos")
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(
                    "Una referencia de consulta para revisar los tipos primitivos, sus rangos, constantes y métodos sin sobrecargar el recorrido principal del Code Lab.",
                )
                .color(egui::Color32::from_rgb(190, 205, 225)),
            );
            ui.add_space(14.0);
            mostrar_contenido_tipos_primitivos(ui, state);
        });
}
