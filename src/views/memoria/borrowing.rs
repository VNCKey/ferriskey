use eframe::egui;

pub fn mostrar_tab_borrowing(ui: &mut egui::Ui) {
    ui.label(
        "Borrowing (Préstamo) permite acceder y operar sobre los datos sin transferir su propiedad mediante referencias (&). El Borrow Checker verifica en compilación que nunca existan condiciones de carrera (data races).",
    );
    ui.add_space(10.0);

    // Tabla Comparativa: Borrowing
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_borrowing_referencias")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo de Préstamo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Permisos")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Cantidad Simultánea")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Regla de Seguridad")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Inmutable
                ui.label(
                    egui::RichText::new("Inmutable (&T)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("let r = &s;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Solo Lectura");
                ui.label("Ilimitadas referencias");
                ui.label("Múltiples lectores pueden observar los datos a la vez.");
                ui.end_row();

                // Fila 2: Mutable
                ui.label(
                    egui::RichText::new("Mutable (&mut T)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("let r = &mut s;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Lectura y Escritura");
                ui.label("Exactamente UNA sola");
                ui.label("Acceso exclusivo mientras vive el préstamo.");
                ui.end_row();

                // Fila 3: Concurrencia segura
                ui.label(
                    egui::RichText::new("Exclusividad")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("& y &mut a la vez")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Conflicto prohibido");
                ui.label("0 referencias mutables si hay lectores");
                ui.label("Previene lecturas inconsistentes y data races.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Dos Columnas: &T vs &mut T
    ui.columns(2, |cols| {
        // Columna Izquierda: Referencias Inmutables
        let mut imm_frame = egui::Frame::new();
        imm_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        imm_frame.inner_margin = egui::Margin::same(12);
        imm_frame.corner_radius = egui::CornerRadius::same(8);
        imm_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        imm_frame.show(&mut cols[0], |ui| {
            ui.label(
                egui::RichText::new("Referencias Inmutables (&T)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("Pasa datos a funciones o inspecciona estructuras sin ceder la propiedad:");
            ui.add_space(4.0);
            ui.label("• El valor original sigue perteneciendo a su variable declarada.");
            ui.label("• Puedes crear tantas referencias de solo lectura como desees.");
            ui.label("• No puedes modificar el dato a través de la referencia.");
            ui.add_space(8.0);

            // Contenedor de Código estilo IDE
            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(
                    egui::RichText::new("fn calcular_len(s: &String) -> usize {")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.indent("imm_code_inner", |ui| {
                    ui.label(
                        egui::RichText::new("s.len() // Lectura del valor sin moverlo")
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
        });

        // Columna Derecha: Referencias Mutables
        let mut mut_frame = egui::Frame::new();
        mut_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        mut_frame.inner_margin = egui::Margin::same(12);
        mut_frame.corner_radius = egui::CornerRadius::same(8);
        mut_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        mut_frame.show(&mut cols[1], |ui| {
            ui.label(
                egui::RichText::new("Referencias Mutables (&mut T)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("Permite modificar un valor ajeno de forma controlada y segura:");
            ui.add_space(4.0);
            ui.label("• La variable base debe haberse declarado con 'mut'.");
            ui.label("• Solo puede haber UNA referencia mutable activa a la vez.");
            ui.label("• Mientras exista '&mut T', no se permiten otras lecturas '&T'.");
            ui.add_space(8.0);

            // Contenedor de Código estilo IDE
            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(
                    egui::RichText::new("let mut texto = String::from(\"Hola\");")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("let ref_mut = &mut texto;")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("ref_mut.push_str(\" Mundo\"); // Modificacion exclusiva")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("println!(\"{texto}\"); // Imprime: Hola Mundo")
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
            });
        });
    });
}
