use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_adaptadores(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Los Adaptadores de Iteradores (Iterator Adaptors) son métodos que transforman un iterador existente en un nuevo iterador. Son perezosos (Lazy): no ejecutan ningún cálculo hasta que son consumidos.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    // 1. Transformación
    ui.label(
        egui::RichText::new("Transformación")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_adaptadores_transformacion")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Adaptador").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new(".map()").strong().color(naranja));
                let code_map = "let nums = vec![1, 2, 3];\nlet dobles = nums.iter().map(|x| x * 2);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .map()".to_string(), code_map));
                }
                ui.label("Transforma cada elemento del iterador aplicando una closure.");
                ui.end_row();

                ui.label(egui::RichText::new(".flatten()").strong().color(naranja));
                let code_flatten = "let matriz = vec![vec![1, 2], vec![3, 4]];\nlet plano = matriz.into_iter().flatten();".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .flatten()".to_string(), code_flatten));
                }
                ui.label("Aplana colecciones anidadas (ej. Vec<Vec<T>>) en un único iterador plano.");
                ui.end_row();

                ui.label(egui::RichText::new(".cloned() / .copied()").strong().color(naranja));
                let code_cloned = "let refs = vec![&1, &2];\nlet copiados = refs.into_iter().cloned();".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptadores .cloned() y .copied()".to_string(), code_cloned));
                }
                ui.label("Convierte un iterador de referencias &T en valores T mediante clonación o copia.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 2. Filtrado & Selección
    ui.label(
        egui::RichText::new("Filtrado & Selección")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_adaptadores_filtrado")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Adaptador").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new(".filter()").strong().color(naranja));
                let code_filter = "let nums = vec![1, 2, 3, 4, 5];\nlet pares = nums.iter().filter(|x| **x % 2 == 0);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .filter()".to_string(), code_filter));
                }
                ui.label("Filtra los elementos evaluando una condición booleana.");
                ui.end_row();

                ui.label(egui::RichText::new(".take()").strong().color(naranja));
                let code_take = "let nums = vec![10, 20, 30, 40, 50];\nlet primeros_dos = nums.iter().take(2);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .take()".to_string(), code_take));
                }
                ui.label("Toma únicamente los primeros n elementos del iterador.");
                ui.end_row();

                ui.label(egui::RichText::new(".skip()").strong().color(naranja));
                let code_skip = "let nums = vec![10, 20, 30, 40, 50];\nlet omitir_dos = nums.iter().skip(2);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .skip()".to_string(), code_skip));
                }
                ui.label("Omite los primeros n elementos del iterador.");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // 3. Combinación & Orden
    ui.label(
        egui::RichText::new("Combinación & Orden")
            .strong()
            .size(16.0)
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_adaptadores_combinacion")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Adaptador").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new(".zip()").strong().color(naranja));
                let code_zip = "let a = vec![1, 2];\nlet b = vec![\"uno\", \"dos\"];\nlet pares = a.iter().zip(b.iter());".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .zip()".to_string(), code_zip));
                }
                ui.label("Combina dos iteradores distintos en un único iterador de tuplas pares.");
                ui.end_row();

                ui.label(egui::RichText::new(".chain()").strong().color(naranja));
                let code_chain = "let a = vec![1, 2];\nlet b = vec![3, 4];\nlet juntos = a.iter().chain(b.iter());".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .chain()".to_string(), code_chain));
                }
                ui.label("Encadena dos iteradores secuencialmente uno tras otro.");
                ui.end_row();

                ui.label(egui::RichText::new(".enumerate()").strong().color(naranja));
                let code_enum = "let frutas = vec![\"manzana\", \"banana\"];\nfor (idx, fruta) in frutas.iter().enumerate() {\n    println!(\"{idx}: {fruta}\");\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .enumerate()".to_string(), code_enum));
                }
                ui.label("Empareja cada elemento con su índice de posición (0, 1, 2...).");
                ui.end_row();

                ui.label(egui::RichText::new(".rev()").strong().color(naranja));
                let code_rev = "let inverso = (1..=5).rev();".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .rev()".to_string(), code_rev));
                }
                ui.label("Invierte el sentido de recorrido de la secuencia.");
                ui.end_row();

                ui.label(egui::RichText::new(".step_by()").strong().color(naranja));
                let code_step = "let saltos = (0..10).step_by(2);".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Adaptador .step_by()".to_string(), code_step));
                }
                ui.label("Avanza la iteración dando saltos de tamaño n.");
                ui.end_row();
            });
    });
}
