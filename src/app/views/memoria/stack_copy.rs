use crate::app::AppState;
use eframe::egui;

pub fn mostrar_tab_stack_copy(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        "La memoria Stack (Pila) es una estructura ultrarrápida gestionada directamente por el procesador. Los tipos de datos simples y de tamaño fijo viven aquí y se duplican automáticamente mediante el Trait Copy.",
    );
    ui.add_space(10.0);

    // Tabla Informativa: Stack & Copy
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_stack_copy")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Concepto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Detalle Técnico")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    let btn_color = if state.ui.show_railroad_modal == Some(6) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };
                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../../assets/diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color),
                            )
                            .frame(state.ui.show_railroad_modal == Some(6)),
                        )
                        .on_hover_text("Ver diagrama visual de la arquitectura del Stack")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(6) {
                            None
                        } else {
                            Some(6)
                        };
                    }
                });
                ui.label(
                    egui::RichText::new("Comportamiento")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Stack Pointer
                ui.label(
                    egui::RichText::new("Stack Pointer (SP)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Registro interno de la CPU");
                ui.label("Asignación instantánea mediante matemática simple de la CPU.");
                ui.end_row();

                // Fila 2: Tamaño de Datos
                ui.label(
                    egui::RichText::new("Tamaño de Datos")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Fijo y conocido al compilar");
                ui.label("Aplica a tipos primitivos: u8, i32, bool, f64, char, tuplas.");
                ui.end_row();

                // Fila 3: Trait Copy
                ui.label(
                    egui::RichText::new("Semántica Copy")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Duplicación de bits en Stack");
                ui.label("Al hacer 'let b = a;', ambas variables siguen siendo válidas.");
                ui.end_row();

                // Fila 4: Limpieza de Memoria (Scope)
                ui.label(
                    egui::RichText::new("Limpieza (Scope)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("El Stack Pointer retrocede");
                ui.label("Sin Garbage Collector ni llamadas al sistema operativo (OS).");
                ui.end_row();
            });
    });
}
