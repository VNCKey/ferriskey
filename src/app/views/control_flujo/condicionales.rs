use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_condicionales(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.label(
        egui::RichText::new(
            "En Rust, las condicionales no requieren paréntesis alrededor de la condición evaluada y deben ser estrictamente de tipo booleano (bool). Además, 'if' es una expresión que devuelve un valor asignable.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Tabla de Sintaxis y Estructuras Condicionales
    ui.label(
        egui::RichText::new("Sintaxis y Estructuras Condicionales")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_if_else_rust_detallada")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Construcción").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Reglas & Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Fila 1: if / else
                ui.label(egui::RichText::new("if / else").strong().color(texto));
                let code_if_else = "let edad = 18;\n\nif edad >= 18 {\n    println!(\"Mayor de edad\");\n} else {\n    println!(\"Menor de edad\");\n}".to_string();
                if ui
                    .button(
                        egui::RichText::new("Ver Código")
                            .strong()
                            .color(cyan),
                    )
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: if / else".to_string(), code_if_else));
                }
                ui.label("Evalúa una condición booleana. Las condiciones no llevan paréntesis obligatorios.");
                ui.end_row();

                // Fila 2: else if
                ui.label(egui::RichText::new("else if").strong().color(texto));
                let code_else_if = "let nota = 85;\n\nif nota >= 90 {\n    println!(\"Excelente\");\n} else if nota >= 70 {\n    println!(\"Aprobado\");\n} else {\n    println!(\"Reprobado\");\n}".to_string();
                if ui
                    .button(
                        egui::RichText::new("Ver Código")
                            .strong()
                            .color(cyan),
                    )
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: else if".to_string(), code_else_if));
                }
                ui.label("Encadena múltiples evaluaciones condicionales secuenciales.");
                ui.end_row();

                // Fila 3: if como Expresión
                ui.label(egui::RichText::new("if como Expresión").strong().color(texto));
                let code_if_expr = "let numero = 7;\n\nlet paridad = if numero % 2 == 0 {\n    \"par\"\n} else {\n    \"impar\"\n};".to_string();
                if ui
                    .button(
                        egui::RichText::new("Ver Código")
                            .strong()
                            .color(cyan),
                    )
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: if como Expresión".to_string(), code_if_expr));
                }
                ui.label("Devuelve un valor directamente a una variable. Ambas ramas DEBEN retornar el mismo tipo de dato.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Ejemplos Prácticos en Código
    ui.label(
        egui::RichText::new("Ejemplos Prácticos de Condicionales en Código")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_ejemplos_condicionales")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Caso de Uso").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Demostración").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Explicación Detallada").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Asignación Directa").strong().color(texto));
                let ex1 = "let numero = 7;\nlet resultado = if numero % 2 == 0 {\n    \"par\"\n} else {\n    \"impar\"\n};".to_string();
                if ui
                    .button(egui::RichText::new("Ver Ejemplo").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo: Asignación Directa con if".to_string(), ex1));
                }
                ui.label("Como 'if' es una expresión, la última línea de cada bloque sin punto y coma ';' es devuelta e inferida como tipo &str.");
                ui.end_row();

                ui.label(egui::RichText::new("Múltiples Condiciones").strong().color(texto));
                let ex2 = "let temperatura = 25;\nif temperatura > 30 {\n    println!(\"Calor\");\n} else if temperatura >= 15 {\n    println!(\"Agradable\");\n} else {\n    println!(\"Frío\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Ejemplo").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo: Múltiples Condiciones".to_string(), ex2));
                }
                ui.label("Permite evaluar un flujo continuo de alternativas excluyentes de arriba hacia abajo.");
                ui.end_row();

                ui.label(egui::RichText::new("Operadores Lógicos (&&, ||)").strong().color(texto));
                let ex3 = "let edad = 20;\nlet tiene_licencia = true;\n\nif edad >= 18 && tiene_licencia {\n    println!(\"Puede conducir\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Ejemplo").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo: Operadores Lógicos (&&, ||)".to_string(), ex3));
                }
                ui.label("Combina múltiples evaluaciones booleanas utilizando AND (&&), OR (||) y NOT (!).");
                ui.end_row();
            });
    });
}
