pub mod declaracion;
pub mod estructura;
pub mod facade;
pub mod rutas;
pub mod visibilidad;

use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::conceptos::mostrar_selector_proyectos_estandar_con_archivos;
use eframe::egui;
#[allow(unused_imports)]
use std::sync::Arc;

pub fn mostrar_tutorial_modulos(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Módulos y Visibilidad")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Barra de navegación de pestañas
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        let tabs_practica = [
            (0, "Declaración & Árbol"),
            (1, "Visibilidad"),
            (2, "Importación & Rutas"),
            (3, "Re-exportación"),
        ];
        for (indice, texto) in tabs_practica {
            let es_activo = state.lessons.modulos_tab == indice;
            let color = if es_activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(texto).strong().color(color))
                        .frame(es_activo),
                )
                .clicked()
            {
                state.lessons.modulos_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let es_activo = state.lessons.modulos_tab == 4;
            let color = if es_activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("Estructura de Archivos")
                            .strong()
                            .color(color),
                    )
                    .frame(es_activo),
                )
                .clicked()
            {
                state.lessons.modulos_tab = 4;
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
            if state.lessons.modulos_tab < 4 {
                let code_target = if state.project.selected_project.is_some() {
                    &mut state.project.shared_project_code
                } else {
                    &mut state.lessons.modulos_code
                };

                mostrar_selector_proyectos_estandar_con_archivos(
                    ui,
                    &mut state.project.selected_project,
                    &mut state.project.selected_file,
                    &mut state.terminal.term_cwd,
                    "combo_proyectos_modulos",
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

            match state.lessons.modulos_tab {
                0 => declaracion::mostrar_tab_declaracion(ui),
                1 => visibilidad::mostrar_tab_visibilidad(ui),
                2 => rutas::mostrar_tab_rutas(ui),
                3 => facade::mostrar_tab_facade(ui),
                _ => estructura::mostrar_teoria_modulos(ui),
            }
        });
}
