use crate::app::AppState;
use eframe::egui;

pub fn mostrar(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.label(
                "En Rust, las funciones se declaran con 'fn' y utilizan la convención snake_case. Exigen declarar el tipo de cada parámetro obligatoriamente y devuelven el valor de su última expresión de forma implícita sin ';'.",
            );
    ui.add_space(10.0);

    // Tabla Comparativa: Funciones
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_funciones_rust")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Aspecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tipado")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Comportamiento")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Uso Ideal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Parámetros
                ui.label(
                    egui::RichText::new("Parámetros")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("fn sumar(a: i32, b: i32)")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Obligatorio")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Requiere tipo explícito en cada argumento.");
                ui.label("Paso de datos a la función.");
                ui.end_row();

                // Fila 2: Retorno Implícito
                ui.label(
                    egui::RichText::new("Retorno Implícito")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("-> i32 { a + b }")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Sin ';' final")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Evalúa la última expresión y la devuelve.");
                ui.label("Forma idiomática estándar en Rust.");
                ui.end_row();

                // Fila 3: Retorno Explícito
                ui.label(
                    egui::RichText::new("Salida Temprana")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("return valor;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Con ';' final")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Termina la función de inmediato.");
                ui.label("Condiciones de guardia o errores.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Dos Columnas: Parámetros & Retornos
    ui.columns(2, |cols| {
                // Columna Izquierda: Parámetros y Orden
                let mut fn_frame = egui::Frame::new();
                fn_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                fn_frame.inner_margin = egui::Margin::same(12);
                fn_frame.corner_radius = egui::CornerRadius::same(8);
                fn_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                fn_frame.show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("Parámetros y Orden de Declaración")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "En las firmas de función, el tipo de cada parámetro debe ser explícito:",
                    );
                    ui.add_space(4.0);
                    ui.label("• El compilador exige tipos explícitos para cada parámetro.");
                    ui.label("• Parámetros Mutables: Puedes anteponer 'mut' a un argumento para modificarlo localmente.");
                    ui.label("• En Rust no importa el orden: puedes llamar a funciones definidas más abajo.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("fn incrementar(mut contador: i32) -> i32 {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("fn_params_inner", |ui| {
                            ui.label(egui::RichText::new("contador += 1;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label(egui::RichText::new("contador").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });

                // Columna Derecha: Retornos de Valores
                let mut ret_frame = egui::Frame::new();
                ret_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                ret_frame.inner_margin = egui::Margin::same(12);
                ret_frame.corner_radius = egui::CornerRadius::same(8);
                ret_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                ret_frame.show(&mut cols[1], |ui| {
                    ui.label(
                        egui::RichText::new("Valores de Retorno")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "El tipo de retorno se declara después de una flecha '->':",
                    );
                    ui.add_space(4.0);
                    ui.label("• La última expresión sin punto y coma ';' es devuelta automáticamente.");
                    ui.label("• Si colocas un punto y coma ';' al final, la función devolverá () y fallará la compilación.");
                    ui.label("• 'return' solo se usa para salir anticipadamente.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("fn multiplicar(a: i32, b: i32) -> i32 {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("fn_return_inner", |ui| {
                            ui.label(egui::RichText::new("a * b // Retorno implícito (sin ';')").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });
            });
}
