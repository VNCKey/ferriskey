use eframe::egui;

pub fn mostrar_tabla_educativa(
    ui: &mut egui::Ui,
    id: &'static str,
    contenido: impl FnOnce(&mut egui::Ui),
) {
    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(14);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new(id)
            .striped(true)
            .spacing([22.0, 10.0])
            .show(ui, contenido);
    });
}
