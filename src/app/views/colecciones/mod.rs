pub mod hashmaps;
pub mod info;
pub mod simulador;
pub mod vectores;

use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::conceptos::mostrar_selector_proyectos_estandar_con_archivos;
use eframe::egui;
#[allow(unused_imports)]
use std::sync::Arc;

/// Renderiza únicamente el contenido de Colecciones para reutilizarlo como
/// una pestaña dentro de Tipos Compuestos, sin crear otra sesión visual.
pub fn mostrar_contenido_colecciones(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Colecciones:").small().color(gris_tab));
        for (indice, label) in [(0, "Vec<T>"), (1, "HashMap")] {
            let activo = state.lessons.colecciones_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.colecciones_tab = indice;
            }
            ui.add_space(3.0);
        }
    });
    ui.add_space(10.0);

    match state.lessons.colecciones_tab {
        0 => vectores::mostrar_coleccion_vectores(ui, state, naranja, cyan, texto),
        _ => hashmaps::mostrar_coleccion_hashmap(ui, state, naranja, cyan, texto),
    }
}

#[allow(dead_code)]
pub fn mostrar_tutorial_colecciones(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Colecciones")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Barra de navegación de pestañas de Colecciones
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        let tabs_practica = [(0, "Vectores"), (1, "HashMap")];
        for (indice, label) in tabs_practica {
            let activo = state.lessons.colecciones_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.colecciones_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Laboratorio:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        let activo_lab = state.lessons.colecciones_tab == 2;
        let color_lab = if activo_lab { naranja } else { gris_tab };
        if ui
            .add(
                egui::Button::new(egui::RichText::new("Simulador").strong().color(color_lab))
                    .frame(activo_lab),
            )
            .clicked()
        {
            state.lessons.colecciones_tab = 2;
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.lessons.colecciones_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Info").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.colecciones_tab = 3;
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
            if state.lessons.colecciones_tab < 2 {
                let code_target = if state.project.selected_project.is_some() {
                    &mut state.project.shared_project_code
                } else {
                    &mut state.lessons.colecciones_code
                };

                mostrar_selector_proyectos_estandar_con_archivos(
                    ui,
                    &mut state.project.selected_project,
                    &mut state.project.selected_file,
                    &mut state.terminal.term_cwd,
                    "combo_proyectos_colecciones",
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

            match state.lessons.colecciones_tab {
                0 => vectores::mostrar_coleccion_vectores(ui, state, naranja, cyan, texto),
                1 => hashmaps::mostrar_coleccion_hashmap(ui, state, naranja, cyan, texto),
                2 => simulador::mostrar_simulador_vectores(ui, state, naranja, cyan, texto),
                _ => info::mostrar_coleccion_info(ui, state, naranja, cyan, texto),
            }
        });
}
