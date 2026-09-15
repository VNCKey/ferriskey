use crate::app::AppState;
use eframe::egui;

pub fn mostrar(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.label(
        "En Rust, un Bloque delimita un conjunto de instrucciones mediante llaves. Cada bloque crea un nuevo Scope que controla el ciclo de vida de las variables declaradas en su interior.",
    );
    ui.add_space(12.0);

    ui.columns(2, |cols| {
        // Columna 1: Bloques
        let mut block_frame = egui::Frame::new();
        block_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        block_frame.inner_margin = egui::Margin::same(12);
        block_frame.corner_radius = egui::CornerRadius::same(8);
        block_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        block_frame.show(&mut cols[0], |ui| {
            ui.label(
                egui::RichText::new("Bloques")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label(
                "Un bloque agrupa múltiples sentencias. En Rust, además, los bloques son expresiones que pueden devolver un valor evaluado:",
            );
            ui.add_space(4.0);
            ui.label("• Se delimita abriendo y cerrando '{}'.");
            ui.label("• Si la última línea no lleva ';', el bloque devuelve ese valor.");
            ui.label("• Si la última línea lleva ';', el bloque devuelve ().");
            ui.add_space(8.0);

            // Contenedor de Código estilo IDE
            let mut code_box1 = egui::Frame::new();
            code_box1.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box1.inner_margin = egui::Margin::same(10);
            code_box1.corner_radius = egui::CornerRadius::same(6);
            code_box1.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box1.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("let resultado = {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.indent("block_code_inner", |ui| {
                    ui.label(egui::RichText::new("let a = 10;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    ui.label(egui::RichText::new("let b = 20;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    ui.label(egui::RichText::new("a + b // Devuelve 30 (sin ';')").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                });
                ui.label(egui::RichText::new("};").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("println!(\"{resultado}\"); // Imprime: 30").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });
        });

        // Columna 2: Scope
        let mut scope_frame = egui::Frame::new();
        scope_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        scope_frame.inner_margin = egui::Margin::same(12);
        scope_frame.corner_radius = egui::CornerRadius::same(8);
        scope_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        scope_frame.show(&mut cols[1], |ui| {
            ui.label(
                egui::RichText::new("Scope")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label(
                "El Scope define la validez de una variable desde su declaración hasta el cierre del bloque:",
            );
            ui.add_space(4.0);
            ui.label("• La variable nace en la línea donde se declara con 'let'.");
            ui.label("• La variable es accesible dentro del bloque y sus sub-bloques.");
            ui.label("• Patrón RAII: Al llegar a '}', la variable sale de scope y Rust libera sus recursos de forma automática.");
            ui.add_space(8.0);

            // Contenedor de Código estilo IDE
            let mut code_box2 = egui::Frame::new();
            code_box2.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box2.inner_margin = egui::Margin::same(10);
            code_box2.corner_radius = egui::CornerRadius::same(6);
            code_box2.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box2.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("let exterior = 10;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("{").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.indent("scope_code_inner", |ui| {
                    ui.label(egui::RichText::new("let interior = 20;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    ui.label(egui::RichText::new("println!(\"{exterior} y {interior}\");").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                });
                ui.label(egui::RichText::new("} // interior sale de scope (liberación RAII)").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("// println!(\"{interior}\"); // Error: no existe fuera del bloque").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
            });
        });
    });
}
