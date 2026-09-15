use eframe::egui;

pub fn mostrar_tab_facade(ui: &mut egui::Ui) {
    ui.label(
        "La re-exportación combina 'pub' y 'use' ('pub use') para ofrecer una interfaz pública limpia (Patrón Facade), ocultando la complejidad interna de la estructura de carpetas.",
    );
    ui.add_space(10.0);

    let mut facade_frame = egui::Frame::new();
    facade_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    facade_frame.inner_margin = egui::Margin::same(14);
    facade_frame.corner_radius = egui::CornerRadius::same(8);
    facade_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    facade_frame.show(ui, |ui| {
        ui.label(egui::RichText::new("El Patrón Facade con 'pub use':").strong().color(egui::Color32::WHITE));
        ui.label("• Re-exporta un ítem interno a un nivel superior.");
        ui.label("• Permite al usuario importar directamente desde la raíz sin conocer la subcarpeta interna.");
        ui.add_space(8.0);

        let mut code_box = egui::Frame::new();
        code_box.fill = egui::Color32::from_rgb(8, 12, 18);
        code_box.inner_margin = egui::Margin::same(10);
        code_box.corner_radius = egui::CornerRadius::same(6);
        code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

        code_box.show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(egui::RichText::new("// En src/lib.rs o main.rs").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
            ui.label(egui::RichText::new("mod interno {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            ui.indent("facade_inner", |ui| {
                ui.label(egui::RichText::new("pub mod http {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.indent("facade_http", |ui| {
                    ui.label(egui::RichText::new("pub fn conectar() {}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                });
                ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });
            ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            ui.add_space(4.0);
            ui.label(egui::RichText::new("// Re-exportación: el usuario ahora puede llamar conectar() directamente").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
            ui.label(egui::RichText::new("pub use interno::http::conectar;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
        });
    });
}
