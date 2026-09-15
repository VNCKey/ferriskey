use crate::app::AppState;
use eframe::egui;

#[allow(dead_code)]
pub fn mostrar(ui: &mut egui::Ui, _state: &mut AppState) {
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_const_static")
                    .striped(true)
                    .spacing([25.0, 8.0])
                    .show(ui, |ui| {
                        // Encabezados
                        ui.label(
                            egui::RichText::new("Declaración")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Ubicación en Memoria")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Mutabilidad")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Ejemplo de Código")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Descripción")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.end_row();

                        // Fila 1: const
                        ui.label(
                            egui::RichText::new("const")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label("Valor de compilación");
                        ui.label(
                            egui::RichText::new("No")
                                .strong()
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                        ui.label(
                            egui::RichText::new("const MAX: u32 = 100;")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label("Su valor debe poder evaluarse durante la compilación.");
                        ui.end_row();

                        // Fila 2: static
                        ui.label(
                            egui::RichText::new("static")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label("Dirección Única en RAM");
                        ui.label(
                            egui::RichText::new("No por defecto")
                                .strong()
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                        ui.label(
                            egui::RichText::new("static VALOR: &str = \"OK\";")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label(
                            "Tiene una ubicación estable y vive durante todo el programa; static mut sí requiere unsafe.",
                        );
                        ui.end_row();
                    });
            });
}
