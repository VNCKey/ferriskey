use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_consumidores(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Los Consumidores (o Métodos Terminales) avanzan el iterador hasta el final para evaluar los datos y producir un resultado final o una nueva colección en memoria.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Creación de Colecciones
    ui.label(
        egui::RichText::new("Creación de Colecciones")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_consumidores_colecciones")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Consumidor").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new(".collect()").strong().color(naranja));
                let code_collect = "let nums = vec![1, 2, 3];\nlet dobles: Vec<i32> = nums.iter().map(|x| x * 2).collect();".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidor .collect()".to_string(), code_collect));
                }
                ui.label("Transforma el iterador en una nueva colección en memoria (como Vec<T> o HashMap).");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Cálculo & Reducción
    ui.label(
        egui::RichText::new("Cálculo & Reducción")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_consumidores_calculo")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Consumidor").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new(".sum() / .product()").strong().color(naranja));
                let code_sum = "let nums = vec![1, 2, 3, 4];\nlet suma: i32 = nums.iter().sum(); // 10\nlet prod: i32 = nums.iter().product(); // 24".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidores .sum() y .product()".to_string(), code_sum));
                }
                ui.label(".sum() suma todos los elementos; .product() los multiplica todos.");
                ui.end_row();

                ui.label(egui::RichText::new(".count()").strong().color(naranja));
                let code_count = "let nums = vec![10, 20, 30];\nlet cantidad = nums.iter().count(); // 3".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidor .count()".to_string(), code_count));
                }
                ui.label("Cuenta el número total de elementos contenidos en el iterador.");
                ui.end_row();

                ui.label(egui::RichText::new(".fold() / .reduce()").strong().color(naranja));
                let code_fold = "let nums = vec![1, 2, 3];\nlet acumulado = nums.iter().fold(0, |acc, x| acc + x);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidores .fold() y .reduce()".to_string(), code_fold));
                }
                ui.label(".fold(init, f) realiza una acumulación con valor inicial; .reduce(f) sin valor inicial.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 3. Búsqueda & Comprobación
    ui.label(
        egui::RichText::new("Búsqueda & Comprobación")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_consumidores_busqueda")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Consumidor").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new(".find() / .position()").strong().color(naranja));
                let code_find = "let nums = vec![10, 20, 30];\nlet encontrado = nums.iter().find(|&&x| x == 20); // Option<&&i32>\nlet pos = nums.iter().position(|&x| x == 20); // Option<usize>".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidores .find() y .position()".to_string(), code_find));
                }
                ui.label(".find(pred) busca el primer elemento que satisface el predicado; .position(pred) devuelve su índice.");
                ui.end_row();

                ui.label(egui::RichText::new(".any() / .all()").strong().color(naranja));
                let code_any_all = "let nums = vec![1, 2, 3];\nlet hay_pares = nums.iter().any(|&x| x % 2 == 0); // true\nlet todos_positivos = nums.iter().all(|&x| x > 0); // true".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidores .any() y .all()".to_string(), code_any_all));
                }
                ui.label(".any(pred) verifica si al menos un elemento cumple; .all(pred) si todos cumplen.");
                ui.end_row();

                ui.label(egui::RichText::new(".max() / .min()").strong().color(naranja));
                let code_max_min = "let nums = vec![5, 2, 8, 1];\nlet maximo = nums.iter().max(); // Some(&8)\nlet minimo = nums.iter().min(); // Some(&1)".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidores .max() y .min()".to_string(), code_max_min));
                }
                ui.label("Devuelve el valor máximo o mínimo contenido en el iterador.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 4. Efectos Secundarios
    ui.label(
        egui::RichText::new("Efectos Secundarios")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_consumidores_efectos")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Consumidor").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new(".for_each()").strong().color(naranja));
                let code_foreach = "let nombres = vec![\"Ana\", \"Luis\"];\nnombres.iter().for_each(|nombre| println!(\"Hola {nombre}\"));".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Consumidor .for_each()".to_string(), code_foreach));
                }
                ui.label("Ejecuta una closure con efectos secundarios sobre cada elemento consumiendo el iterador.");
                ui.end_row();
            });
    });
}
