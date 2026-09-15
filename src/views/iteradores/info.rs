use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_iteradores_info(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Guía maestra de rendimiento, arquitectura y modelos de evaluación sobre el trait Iterator en Rust.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Zero-Cost Abstractions
    ui.label(
        egui::RichText::new("Zero-Cost Abstractions")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_zero_cost")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Concepto").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Optimizaciones del Compilador").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Impacto en Rendimiento").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Loop Unrolling").strong().color(texto));
                ui.label("rustc / LLVM desenrollan internamente las llamadas repetitivas.");
                ui.label("Velocidad equivalente o superior a bucles 'for' manuales en C/C++.");
                ui.end_row();

                ui.label(egui::RichText::new("Eliminación Bounds Checking").strong().color(texto));
                ui.label("Los iteradores conocen los límites exactos de la colección.");
                ui.label("Eliminan las verificaciones de desbordamiento en cada paso sin perder seguridad.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Lazy Evaluation
    ui.label(
        egui::RichText::new("Lazy Evaluation")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_lazy_eval")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Fase").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Métodos involucrados").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Comportamiento").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Construcción Pipeline").strong().color(texto));
                ui.label(egui::RichText::new(".map(), .filter(), .take()").monospace().color(cyan));
                ui.label("Crea una estructura ligera inactiva. Cero consumo de CPU o memoria en esta fase.");
                ui.end_row();

                ui.label(egui::RichText::new("Consumo Terminal").strong().color(texto));
                ui.label(egui::RichText::new(".collect(), .sum(), .find()").monospace().color(cyan));
                ui.label("Activa la iteración tirando ('pull') de los elementos uno a uno por el pipeline.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 3. Matriz de Decisión Rápida
    ui.label(
        egui::RichText::new("Matriz de Decisión: ¿Qué Modo de Iteración Elegir?")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_info_matriz_decision")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Caso de Uso")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Método Recomendado")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Razón de Selección")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(egui::RichText::new("Solo Lectura").strong().color(texto));
                ui.label(egui::RichText::new(".iter()").monospace().color(cyan));
                ui.label(
                    "Lee elementos por &T manteniendo la colección intacta para usos futuros.",
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("Modificación In-Situ")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new(".iter_mut()").monospace().color(cyan));
                ui.label(
                    "Modifica elementos por &mut T directamente en el Heap sin realojar memoria.",
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("Transformación / Consumo")
                        .strong()
                        .color(texto),
                );
                ui.label(egui::RichText::new(".into_iter()").monospace().color(cyan));
                ui.label(
                    "Transfiere el Ownership T a una nueva estructura consumiendo la original.",
                );
                ui.end_row();
            });
    });
}
