use eframe::egui;

pub fn mostrar_tab_rutas(ui: &mut egui::Ui) {
    ui.label(
        "La palabra clave 'use' permite traer elementos de un módulo al ámbito actual para invocarlos directamente sin escribir la ruta completa. Se pueden renombrar con 'as' y navegar de forma relativa con 'super::'.",
    );
    ui.add_space(10.0);

    ui.columns(2, |cols| {
        // Columna Izquierda: use y as
        let mut use_frame = egui::Frame::new();
        use_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        use_frame.inner_margin = egui::Margin::same(12);
        use_frame.corner_radius = egui::CornerRadius::same(8);
        use_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        use_frame.show(&mut cols[0], |ui| {
            ui.label(
                egui::RichText::new("Importación con 'use' y 'as'")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("• Trae ítems al ámbito: 'use crate::redes::http::conectar;'");
            ui.label("• Evita colisiones de nombres con 'as': 'use std::fmt::Result as FmtResult;'");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("use std::fmt::Result as FmtResult;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("use std::io::Result as IoResult;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });
        });

        // Columna Derecha: super::
        let mut super_frame = egui::Frame::new();
        super_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        super_frame.inner_margin = egui::Margin::same(12);
        super_frame.corner_radius = egui::CornerRadius::same(8);
        super_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        super_frame.show(&mut cols[1], |ui| {
            ui.label(
                egui::RichText::new("Navegación Relativa ('super::')")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("• 'super::' accede al módulo padre inmediato (similar a '..' en rutas de archivos).");
            ui.label("• Permite acceder a funciones privadas del módulo padre.");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("mod interno {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.indent("super_code_inner", |ui| {
                    ui.label(egui::RichText::new("fn ayuda() {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    ui.indent("super_fn", |ui| {
                        ui.label(egui::RichText::new("super::funcion_padre();").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                    ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                });
                ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });
        });
    });
}
