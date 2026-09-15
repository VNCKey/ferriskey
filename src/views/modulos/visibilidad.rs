use eframe::egui;

pub fn mostrar_tab_visibilidad(ui: &mut egui::Ui) {
    ui.label(
        "En Rust, todo es PRIVADO por defecto. Los elementos de un módulo solo son visibles fuera de él si se marcan explícitamente con la palabra clave 'pub'.",
    );
    ui.add_space(10.0);

    // Tabla Informativa: Modificadores de Visibilidad
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_modulos_visibilidad")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Modificador")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Alcance de Visibilidad")
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

                // Fila 1: Privado
                ui.label(
                    egui::RichText::new("Privado (por defecto)")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("Solo dentro del módulo actual");
                ui.label(
                    egui::RichText::new("fn secreto() {}")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("No accesible desde módulos externos ni desde main.rs.");
                ui.end_row();

                // Fila 2: pub
                ui.label(
                    egui::RichText::new("pub")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Público a todo el proyecto y externos");
                ui.label(
                    egui::RichText::new("pub fn conectar() {}")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Cualquiera puede invocar la función.");
                ui.end_row();

                // Fila 3: pub(crate)
                ui.label(
                    egui::RichText::new("pub(crate)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Visible en todo este crate");
                ui.label(
                    egui::RichText::new("pub(crate) fn helper() {}")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Público dentro de tu proyecto pero privado a usuarios externos.");
                ui.end_row();
            });
    });
}
