pub mod bounds;
pub mod funciones;
pub mod info;
pub mod structs;

use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::conceptos::mostrar_selector_proyectos_estandar_con_archivos;
use eframe::egui;

pub fn mostrar_tutorial_genericos(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Generics")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Barra de Navegación de Pestañas de Genericos
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs_izq = [
            (0, "Funciones Genéricas"),
            (1, "Estructuras Genéricas"),
            (2, "Trait Bounds"),
        ];
        for (indice, label) in tabs_izq {
            let activo = state.lessons.genericos_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.genericos_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.lessons.genericos_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Info").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.genericos_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Teórico:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(10.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state.lessons.genericos_tab < 3 {
                let code_target = if state.project.selected_project.is_some() {
                    &mut state.project.shared_project_code
                } else {
                    &mut state.lessons.genericos_code
                };

                mostrar_selector_proyectos_estandar_con_archivos(
                    ui,
                    &mut state.project.selected_project,
                    &mut state.project.selected_file,
                    &mut state.terminal.term_cwd,
                    "combo_proyectos_genericos",
                    code_target,
                );

                ui.add_space(10.0);

                let syntax_set = state.editor.syntax_set.clone();
                let theme = state.editor.theme_set.themes["base16-ocean.dark"].clone();
                let (code_ref, output_arc) = state.obtener_editor_activo_mut();
                mostrar_editor_interactivo(
                    ui,
                    code_ref,
                    output_arc,
                    "",
                    ejecutar_codigo_rust,
                    &syntax_set,
                    &theme,
                );

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(12.0);
            }

            match state.lessons.genericos_tab {
                0 => funciones::mostrar_tab_funciones(ui, state, naranja, cyan, texto),
                1 => structs::mostrar_tab_structs(ui, state, naranja, cyan, texto),
                2 => bounds::mostrar_tab_bounds(ui, state, naranja, cyan, texto),
                _ => info::mostrar_genericos_info(ui, state, naranja, cyan, texto),
            }
        });
}
