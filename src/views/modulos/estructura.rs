use eframe::egui;

pub fn mostrar_teoria_modulos(ui: &mut egui::Ui) {
    ui.label(
        "Rust ofrece dos estilos principales para organizar módulos en archivos dentro de la carpeta 'src/'. La edición Rust 2018 introdujo un estilo más limpio sin necesidad de crear archivos 'mod.rs'.",
    );
    ui.add_space(10.0);

    ui.columns(2, |cols| {
        // Estilo Moderno
        let mut mod_frame = egui::Frame::new();
        mod_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        mod_frame.inner_margin = egui::Margin::same(12);
        mod_frame.corner_radius = egui::CornerRadius::same(8);
        mod_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        mod_frame.show(&mut cols[0], |ui| {
            ui.label(
                egui::RichText::new("Estilo Moderno (Rust 2018+)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("• El módulo principal de la carpeta comparte su mismo nombre.");
            ui.label("• Evita la acumulación de archivos 'mod.rs' duplicados.");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(
                    egui::RichText::new("src/")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("├── main.rs")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("├── redes.rs          // mod redes;")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("└── redes/")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("    └── http.rs      // mod http;")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
            });
        });

        // Estilo Clásico
        let mut clasico_frame = egui::Frame::new();
        clasico_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        clasico_frame.inner_margin = egui::Margin::same(12);
        clasico_frame.corner_radius = egui::CornerRadius::same(8);
        clasico_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        clasico_frame.show(&mut cols[1], |ui| {
            ui.label(
                egui::RichText::new("Estilo Clásico (mod.rs)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("• El archivo raíz de una subcarpeta se nombra obligatoriamente 'mod.rs'.");
            ui.label("• Compatible con código legado de Rust 2015.");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(
                    egui::RichText::new("src/")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("├── main.rs")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("└── redes/")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("    ├── mod.rs       // Archivo raíz de redes")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("    └── http.rs      // mod http;")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
            });
        });
    });
}
