pub mod borrowing;
pub mod heap_move;
pub mod ownership;
pub mod stack_copy;
pub mod strings;

use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
#[allow(unused_imports)]
use crate::views::conceptos::mostrar_selector_proyectos_estandar;
use eframe::egui;
#[allow(unused_imports)]
use std::sync::Arc;

pub fn mostrar_tutorial_strings_ownership(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Memoria")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Barra de navegación con el mismo patrón unificado que Comenzando
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        let tabs_practica = [
            (0, "Stack & Copy"),
            (1, "Heap & Move"),
            (2, "Ownership"),
            (3, "Borrowing"),
        ];
        for (indice, texto) in tabs_practica {
            let es_activo = state.lessons.strings_ownership_tab == indice;
            let color = if es_activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(texto).strong().color(color))
                        .frame(es_activo),
                )
                .clicked()
            {
                state.lessons.strings_ownership_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let es_activo = state.lessons.strings_ownership_tab == 4;
            let color = if es_activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("String & &str").strong().color(color))
                        .frame(es_activo),
                )
                .clicked()
            {
                state.lessons.strings_ownership_tab = 4;
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
            if state.lessons.strings_ownership_tab < 4 {
                let code_target = if state.project.selected_project.is_some() {
                    &mut state.project.shared_project_code
                } else {
                    &mut state.lessons.ownership_code
                };

                crate::views::conceptos::mostrar_selector_proyectos_estandar_con_archivos(
                    ui,
                    &mut state.project.selected_project,
                    &mut state.project.selected_file,
                    &mut state.terminal.term_cwd,
                    "combo_proyectos_strings_ownership",
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

            match state.lessons.strings_ownership_tab {
                0 => stack_copy::mostrar_tab_stack_copy(ui, state),
                1 => heap_move::mostrar_tab_heap_move(ui, state),
                2 => ownership::mostrar_tab_ownership(ui),
                3 => borrowing::mostrar_tab_borrowing(ui),
                _ => strings::mostrar_teoria_string_y_str(ui, state),
            }
        });
}
