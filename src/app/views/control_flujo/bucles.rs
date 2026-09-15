use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_bucles(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.label(
        egui::RichText::new(
            "Rust ofrece 3 construcciones de bucles principales: 'loop' para ciclos infinitos con capacidad de retorno de valor, 'while' para repetición condicional y 'for' para iterar sobre rangos y colecciones.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Tabla de Sintaxis y Tipos de Bucles
    ui.label(
        egui::RichText::new("Tipos de Bucles y Sintaxis")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_bucles_rust_detalles")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Bucle / Control").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Reglas & Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Fila 1: loop
                ui.label(egui::RichText::new("loop").strong().color(texto));
                let code_loop = "let mut contador = 0;\n\nlet resultado = loop {\n    contador += 1;\n    if contador == 10 {\n        break contador * 2;\n    }\n};".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: loop con retorno".to_string(), code_loop));
                }
                ui.label("Ciclo infinito explícito. Permite devolver un valor mediante 'break valor;'.");
                ui.end_row();

                // Fila 2: while
                ui.label(egui::RichText::new("while").strong().color(texto));
                let code_while = "let mut numero = 3;\n\nwhile numero > 0 {\n    println!(\"{numero}!\");\n    numero -= 1;\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: while".to_string(), code_while));
                }
                ui.label("Se ejecuta repetidamente mientras la condición booleana sea 'true'.");
                ui.end_row();

                // Fila 3: for
                ui.label(egui::RichText::new("for").strong().color(texto));
                let code_for = "for i in 1..=5 {\n    println!(\"Número: {i}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: for básico".to_string(), code_for));
                }
                ui.label("Itera sobre rangos o colecciones sin riesgo de salirse de los límites de memoria.");
                ui.end_row();

                // Fila 4: break / continue y Etiquetas
                ui.label(egui::RichText::new("break / continue / 'etiqueta").strong().color(texto));
                let code_break = "'externo: for i in 1..=3 {\n    for j in 1..=3 {\n        if i == 2 && j == 2 {\n            break 'externo;\n        }\n    }\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: Etiquetas de Bucles ('label)".to_string(), code_break));
                }
                ui.label("Controlan la ejecución. Las etiquetas ('nombre) permiten romper o continuar bucles anidados específicos.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Formas Básicas del Bucle 'for'
    ui.label(
        egui::RichText::new("Formas de Usar el Bucle 'for' en Rust")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_variantes_for_rust")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Forma de for").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Demostración").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Rangos Exclusivos vs Inclusivos
                ui.label(egui::RichText::new("Rangos (a..b y a..=b)").strong().color(texto));
                let code_v1 = "// 1..5 -> Exclusivo (1, 2, 3, 4)\nfor i in 1..5 {\n    println!(\"{i}\");\n}\n\n// 1..=5 -> Inclusivo (1, 2, 3, 4, 5)\nfor i in 1..=5 {\n    println!(\"{i}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Uso de Rangos: Exclusivos vs Inclusivos".to_string(), code_v1));
                }
                ui.label("1..5 excluye el número 5 final; 1..=5 incluye el número 5 final.");
                ui.end_row();

                // Iteración sobre Colecciones (Arrays)
                ui.label(egui::RichText::new("Iterar sobre Colecciones").strong().color(texto));
                let code_v2 = "let numeros = [10, 20, 30];\n\n// Iteración simple por referencia de lectura\nfor num in &numeros {\n    println!(\"Elemento: {num}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Iterar Colecciones con for".to_string(), code_v2));
                }
                ui.label("Recorre directamente cada elemento del arreglo sin necesidad de manejar índices numéricos manuales.");
                ui.end_row();

                // Control de Flujo Interno (break / continue)
                ui.label(egui::RichText::new("Interrupción (break / continue)").strong().color(texto));
                let code_v3 = "for i in 1..=10 {\n    if i % 2 == 0 {\n        continue; // Salta los pares\n    }\n    if i == 7 {\n        break; // Detiene el bucle en 7\n    }\n    println!(\"{i}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Control con break y continue".to_string(), code_v3));
                }
                ui.label("'continue' salta inmediatamente a la siguiente vuelta; 'break' aborta el bucle por completo.");
                ui.end_row();
            });
    });
}
