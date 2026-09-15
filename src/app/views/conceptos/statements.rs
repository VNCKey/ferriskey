use crate::app::AppState;
use eframe::egui;

pub fn mostrar(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.label(
                "Rust es un lenguaje basado en expresiones (expression-oriented). La diferencia entre realizar una acción (Statement) y producir un valor evaluado (Expression) es fundamental para estructurar funciones y bloques.",
            );
    ui.add_space(10.0);

    // Tabla Comparativa: Statements vs Expressions
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_statements_expressions")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Concepto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis de Ejemplo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("¿Produce Valor?")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("';' ?")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Statement (Sentencia)
                ui.label(
                    egui::RichText::new("Statement")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("let x = 6;\nfn suma() { }")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("No")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("Obligatorio");
                ui.label("Declara bindings, tipos o funciones.");
                ui.end_row();

                // Fila 2: Expression (Expresión)
                ui.label(
                    egui::RichText::new("Expression")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("5 + 6\n{ let a = 1; a + 2 }")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Sí")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Sin ';' al retornar");
                ui.label("Cálculos, bloques con retorno implícito.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Dos Columnas: Statements vs Expressions en profundidad
    ui.columns(2, |cols| {
                // Columna Izquierda: Statements
                let mut stmts_frame = egui::Frame::new();
                stmts_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                stmts_frame.inner_margin = egui::Margin::same(12);
                stmts_frame.corner_radius = egui::CornerRadius::same(8);
                stmts_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                stmts_frame.show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("Statements")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Son instrucciones completas que ejecutan una acción pero no devuelven un valor:",
                    );
                    ui.add_space(4.0);
                    ui.label("• La declaración con 'let' es una sentencia, por lo que NO retorna nada.");
                    ui.label("• En Rust es un error escribir 'let x = (let y = 6);' (a diferencia de C o Python).");
                    ui.label("• Terminan siempre con un punto y coma ';' al final.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("let x = 6; // Sentencia (no produce valor)").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("// let x = (let y = 6); // Error de sintaxis").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                    });
                });

                // Columna Derecha: Expressions
                let mut expr_frame = egui::Frame::new();
                expr_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                expr_frame.inner_margin = egui::Margin::same(12);
                expr_frame.corner_radius = egui::CornerRadius::same(8);
                expr_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                expr_frame.show(&mut cols[1], |ui| {
                    ui.label(
                        egui::RichText::new("Expressions")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Cualquier operación o bloque que evalúe y produzca un valor resultante:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Un bloque '{ ... }' es una expresión si su última línea no lleva ';'.");
                    ui.label("• '5 + 6' es una expresión que evalúa al número 11.");
                    ui.label("• Si agregas un ';' al final de una expresión, se convierte en sentencia y devuelve '()'.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("let y = {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("expr_code_inner", |ui| {
                            ui.label(egui::RichText::new("let x = 3;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label(egui::RichText::new("x + 1 // Expresión sin ';' devuelve 4").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        ui.label(egui::RichText::new("};").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("println!(\"{y}\"); // Imprime: 4").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });
            });
}
