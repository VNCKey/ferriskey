use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_modos(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "En Rust existen 3 formas fundamentales de crear un iterador sobre una colección según la gestión de memoria (Ownership & Borrowing): .iter(), .iter_mut() e .into_iter().",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_iter_modos_detalles")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Método").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Efecto en la Memoria (Ownership)").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // .iter()
                ui.label(egui::RichText::new(".iter()").strong().color(naranja));
                let code_iter = "let numeros = vec![1, 2, 3];\n\nfor val in numeros.iter() {\n    println!(\"Lectura: {val}\"); // 'val' es de tipo &i32\n}\n\n// 'numeros' sigue siendo válida después del bucle".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Iterador por Referencia (&T): .iter()".to_string(), code_iter));
                }
                ui.label("Itera sobre referencias inmutables (&T). No consume la colección ni retira su propiedad.");
                ui.end_row();

                // .iter_mut()
                ui.label(egui::RichText::new(".iter_mut()").strong().color(naranja));
                let code_iter_mut = "let mut numeros = vec![1, 2, 3];\n\nfor val in numeros.iter_mut() {\n    *val *= 2; // 'val' es de tipo &mut i32\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Iterador por Referencia Mutable (&mut T): .iter_mut()".to_string(), code_iter_mut));
                }
                ui.label("Itera sobre referencias mutables (&mut T). Permite modificar los elementos in-situ en el Heap.");
                ui.end_row();

                // .into_iter()
                ui.label(egui::RichText::new(".into_iter()").strong().color(naranja));
                let code_into_iter = "let nombres = vec![String::from(\"Ana\"), String::from(\"Luis\")];\n\nfor nombre in nombres.into_iter() {\n    println!(\"{nombre}\"); // 'nombre' es de tipo String (propiedad movida)\n}\n\n// 'nombres' ya no existe aquí".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Iterador por Valor (T): .into_iter()".to_string(), code_into_iter));
                }
                ui.label("Itera por valor (T), transfiriendo el Ownership de cada elemento y destruyendo la colección original.");
                ui.end_row();
            });
    });
}
