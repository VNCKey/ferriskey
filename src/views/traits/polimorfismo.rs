use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_tab_polimorfismo(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Rust ofrece dos formas de polimorfismo mediante Traits: Despacho Estático (monomorfización) y Despacho Dinámico (dyn Trait).",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_traits_polimorfismo")
            .striped(true)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Mecanismo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                ui.end_row();

                // Static Dispatch
                ui.label(egui::RichText::new("Despacho Estático").strong().color(naranja));
                let code_static = "fn renderizar(item: &impl Describible) {\n    println!(\"{}\", item.describir());\n}".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Despacho Estático (&impl Trait)".to_string(), code_static));
                }
                ui.label("Monomorfización en tiempo de compilación. Cero sobrecosto de rendimiento en ejecución.");
                ui.end_row();

                // Dynamic Dispatch (dyn Trait)
                ui.label(egui::RichText::new("Despacho Dinámico (dyn Trait)").strong().color(naranja));
                let code_dynamic = "let objetos: Vec<Box<dyn Describible>> = vec![\n    Box::new(Usuario { nombre: String::from(\"Ana\") }),\n];".to_string();
                if ui
                    .button(egui::RichText::new("Ver Código").strong().color(cyan))
                    .on_hover_text("Abrir modal centrado con el ejemplo de código de solo lectura")
                    .clicked()
                {
                    state.ui.show_code_modal = Some(("Despacho Dinámico con dyn Trait".to_string(), code_dynamic));
                }
                ui.label("Resolución de métodos en tiempo de ejecución mediante vtable. Permite vectores heterogéneos.");
                ui.end_row();
            });
    });
}
