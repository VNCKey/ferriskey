use crate::app::AppState;
use eframe::egui;

pub fn mostrar(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(
        "En Rust, las variables son inmutables por defecto. Esto garantiza seguridad de memoria, previene errores de concurrencia y obliga a declarar explícitamente con 'mut' cuando un valor necesita cambiar.",
    );
    ui.add_space(12.0);

    // ==========================================
    // MUTABILIDAD Y DECLARACIONES
    // ==========================================
    ui.heading(
        egui::RichText::new("Mutabilidad y Declaraciones")
            .size(17.0)
            .strong()
            .color(egui::Color32::from_rgb(255, 160, 50)),
    );
    ui.add_space(6.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_declaraciones_rust")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                // Encabezados
                ui.label(
                    egui::RichText::new("Declaración")
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

                // Fila 1: let (Inmutable)
                let btn_color_0 = if state.ui.show_railroad_modal == Some(0) {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("let")
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(4.0);
                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_0),
                            )
                            .frame(state.ui.show_railroad_modal == Some(0)),
                        )
                        .on_hover_text("Ver diagrama Railroad de sintaxis (let inmutable)")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(0) {
                            None
                        } else {
                            Some(0)
                        };
                    }
                });
                ui.label(
                    egui::RichText::new("No")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label(
                    egui::RichText::new("let x = 5;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("No puede reasignarse. El valor permanece fijo.");
                ui.end_row();

                // Fila 2: let mut (Mutable)
                let btn_color_1 = if state.ui.show_railroad_modal == Some(1) {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("let mut")
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(4.0);
                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_1),
                            )
                            .frame(state.ui.show_railroad_modal == Some(1)),
                        )
                        .on_hover_text("Ver diagrama Railroad de sintaxis (let mut mutable)")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(1) {
                            None
                        } else {
                            Some(1)
                        };
                    }
                });
                ui.label(
                    egui::RichText::new("Sí")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("let mut x = 5;\nx = 10;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Permite cambiar el valor de la variable de forma explícita.");
                ui.end_row();

                // Fila 3: const
                ui.label(
                    egui::RichText::new("const")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
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
                ui.label("Constante de compilación. Su valor debe conocerse antes de ejecutar.");
                ui.end_row();

                // Fila 4: static
                ui.label(
                    egui::RichText::new("static")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
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
                ui.label("Dirección de memoria fija y global durante todo el programa.");
                ui.end_row();

                // Fila 5: type (Alias)
                ui.label(
                    egui::RichText::new("type")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("N/A")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label(
                    egui::RichText::new("type Metros = u64;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Crea un alias o nombre descriptivo para un tipo existente.");
                ui.end_row();
            });
    });

    ui.add_space(18.0);

    // ==========================================
    // SHADOWING
    // ==========================================
    ui.heading(
        egui::RichText::new("Shadowing")
            .size(17.0)
            .strong()
            .color(egui::Color32::from_rgb(255, 160, 50)),
    );
    ui.add_space(6.0);

    let mut shadow_frame = egui::Frame::new();
    shadow_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    shadow_frame.inner_margin = egui::Margin::same(14);
    shadow_frame.corner_radius = egui::CornerRadius::same(8);
    shadow_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    shadow_frame.show(ui, |ui| {
        ui.label(
            "El Shadowing (ensombrecimiento) consiste en declarar una nueva variable usando la palabra clave 'let' con el mismo nombre de una variable existente. La nueva variable oculta a la anterior.",
        );
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Puntos clave de Shadowing:").strong().color(egui::Color32::WHITE));
        ui.label("• Re-declaración con let: Cada vez que usas 'let', estás creando una nueva variable.");
        ui.label("• Cambio de tipo: Al crear una nueva variable, se le puede asignar un tipo de dato diferente.");
        ui.label("• Inmutabilidad conservada: El valor resultante sigue siendo inmutable salvo que se especifique 'mut'.");
        ui.add_space(10.0);

        ui.columns(2, |cols| {
            // Columna 1: Transformación de valor
            let mut code_box1 = egui::Frame::new();
            code_box1.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box1.inner_margin = egui::Margin::same(10);
            code_box1.corner_radius = egui::CornerRadius::same(6);
            code_box1.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box1.show(&mut cols[0], |ui| {
                ui.label(egui::RichText::new("Re-declarar y transformar valor").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                ui.add_space(4.0);
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("let x = 5;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("let x = x + 1; // x ahora es 6").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("let x = x * 2; // x ahora es 12").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("println!(\"{x}\"); // Imprime: 12").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });

            // Columna 2: Cambio de tipo
            let mut code_box2 = egui::Frame::new();
            code_box2.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box2.inner_margin = egui::Margin::same(10);
            code_box2.corner_radius = egui::CornerRadius::same(6);
            code_box2.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box2.show(&mut cols[1], |ui| {
                ui.label(egui::RichText::new("Cambio de tipo de dato").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                ui.add_space(4.0);
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("let espacios = \"   \"; // Tipo texto (&str)").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("let espacios = 3;     // Tipo número (i32)").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("// Con 'let mut' cambiar de tipo daría error").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.label(egui::RichText::new("println!(\"{espacios}\"); // Imprime: 3").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });
        });
    });
}
