use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_match(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.label(
        egui::RichText::new(
            "La expresión 'match' en Rust compara un valor contra múltiples patrones y ejecuta el código del primer patrón coincidente. El compilador exige exhaustividad total (cubrir todos los casos posibles).",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Tabla de Patrones en Match
    ui.label(
        egui::RichText::new("Patrones Frecuentes en match")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_match_rust_detalles")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Patrón").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Reglas & Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Fila 1: Literal exacto
                ui.label(egui::RichText::new("Literal Exacto").strong().color(texto));
                let code_literal = "let dado = 4;\n\nmatch dado {\n    1 => println!(\"Uno\"),\n    2 => println!(\"Dos\"),\n    _ => println!(\"Otro número\"),\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: match con literales".to_string(), code_literal));
                }
                ui.label("Coincidencia exacta con un valor explícito (números, caracteres, strings).");
                ui.end_row();

                // Fila 2: Rangos inclusivos
                ui.label(egui::RichText::new("Rangos Inclusivos").strong().color(texto));
                let code_rango = "let edad = 15;\n\nmatch edad {\n    0..=12  => println!(\"Niño\"),\n    13..=17 => println!(\"Adolescente\"),\n    _       => println!(\"Adulto\"),\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: match con rangos".to_string(), code_rango));
                }
                ui.label("Coincidencia inclusiva con cualquier valor dentro del rango numérico.");
                ui.end_row();

                // Fila 3: Comodín _
                ui.label(egui::RichText::new("Comodín _").strong().color(texto));
                let code_comodin = "let caracter = 'z';\n\nmatch caracter {\n    'a' | 'e' | 'i' | 'o' | 'u' => println!(\"Vocal\"),\n    _ => println!(\"Consonante u otro\"),\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: Comodín _ en match".to_string(), code_comodin));
                }
                ui.label("Captura cualquier otro caso no listado previamente para cumplir la exhaustividad exigida por Rust.");
                ui.end_row();

                // Fila 4: Match Guards
                ui.label(egui::RichText::new("Guards (if)").strong().color(texto));
                let code_guard = "let numero = 8;\n\nmatch numero {\n    n if n % 2 == 0 => println!(\"Es par\"),\n    _ => println!(\"Es impar\"),\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo de Sintaxis: Match Guards (if)".to_string(), code_guard));
                }
                ui.label("Añade una condición booleana adicional (Match Guard) al patrón.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Ejemplos Prácticos en Código
    ui.label(
        egui::RichText::new("Ejemplos Prácticos de match en Código")
            .strong()
            .size(15.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_ejemplos_match")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Caso de Uso").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Demostración").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Explicación Detallada").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("1. Asignación con match").strong().color(texto));
                let ex1 = "let nota = 85;\nlet letra = match nota {\n    90..=100 => 'A',\n    80..=89  => 'B',\n    70..=79  => 'C',\n    _        => 'F',\n};\nprintln!(\"Calificación: {letra}\");".to_string();
                if ui
                    .button(egui::RichText::new("Ver Ejemplo").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo: Asignación con match".to_string(), ex1));
                }
                ui.label("Como 'match' es una expresión, evalúa el rango correspondiente y asigna 'B' directamente a la variable 'letra'.");
                ui.end_row();

                ui.label(egui::RichText::new("2. Coincidencia con Tuplas").strong().color(texto));
                let ex2 = "let punto = (0, 5);\nmatch punto {\n    (0, y) => println!(\"En eje Y (y={y})\"),\n    (x, 0) => println!(\"En eje X (x={x})\"),\n    (x, y) => println!(\"Punto ({x}, {y})\"),\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Ejemplo").strong().color(cyan))
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Ejemplo: Coincidencia con Tuplas".to_string(), ex2));
                }
                ui.label("Permite desestructurar tuplas directamente extrayendo sus componentes en patrones específicos.");
                ui.end_row();
            });
    });
}
