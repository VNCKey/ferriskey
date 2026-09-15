use eframe::egui;

pub fn mostrar_tab_declaracion(ui: &mut egui::Ui) {
    ui.label(
        "En Rust, los módulos permiten organizar el código en agrupaciones lógicas y controlar la privacidad. Un módulo se declara con la palabra clave 'mod' y forma un árbol jerárquico que parte desde la raíz del crate (main.rs o lib.rs).",
    );
    ui.add_space(10.0);

    // Tabla Informativa: Declaración de Módulos
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_modulos_declaracion")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Forma de Declaración")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis en Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ubicación del Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Uso Recomendado")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Inline
                ui.label(
                    egui::RichText::new("Módulo Inline")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("mod redes { ... }")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Dentro del mismo archivo .rs");
                ui.label("Ejemplos rápidos, módulos pequeños o pruebas.");
                ui.end_row();

                // Fila 2: Archivo separado
                ui.label(
                    egui::RichText::new("Módulo en Archivo")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("mod redes;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("En archivo src/redes.rs");
                ui.label("Proyectos medianos y grandes bien estructurados.");
                ui.end_row();

                // Fila 3: Raíz crate
                ui.label(
                    egui::RichText::new("Raíz del Crate")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("crate::...")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Origen raíz (main.rs / lib.rs)");
                ui.label("Rutas absolutas hacia cualquier ítem del proyecto.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Ejemplo de Código
    let mut code_box = egui::Frame::new();
    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
    code_box.inner_margin = egui::Margin::same(10);
    code_box.corner_radius = egui::CornerRadius::same(6);
    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

    code_box.show(ui, |ui| {
        ui.label(
            egui::RichText::new("Declaración de Módulo Inline en Rust")
                .strong()
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
        ui.add_space(4.0);
        ui.spacing_mut().item_spacing.y = 2.0;
        ui.label(
            egui::RichText::new("mod usuario {")
                .monospace()
                .size(12.0)
                .color(egui::Color32::from_rgb(100, 200, 255)),
        );
        ui.indent("mod_inline_inner", |ui| {
            ui.label(
                egui::RichText::new("pub fn crear() {")
                    .monospace()
                    .size(12.0)
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.indent("mod_inline_fn", |ui| {
                ui.label(
                    egui::RichText::new("println!(\"Usuario creado\");")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
            });
            ui.label(
                egui::RichText::new("}")
                    .monospace()
                    .size(12.0)
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
        });
        ui.label(
            egui::RichText::new("}")
                .monospace()
                .size(12.0)
                .color(egui::Color32::from_rgb(100, 200, 255)),
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("fn main() {")
                .monospace()
                .size(12.0)
                .color(egui::Color32::from_rgb(100, 200, 255)),
        );
        ui.indent("mod_inline_main", |ui| {
            ui.label(
                egui::RichText::new("usuario::crear();")
                    .monospace()
                    .size(12.0)
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
        });
        ui.label(
            egui::RichText::new("}")
                .monospace()
                .size(12.0)
                .color(egui::Color32::from_rgb(100, 200, 255)),
        );
    });
}
