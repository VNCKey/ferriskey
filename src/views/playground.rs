use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::{ejecutar_codigo_api, ejecutar_codigo_rust};
use eframe::egui;
use std::sync::Arc;

pub fn mostrar_editor(ui: &mut egui::Ui, state: &mut AppState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new("💻 Code Playground").size(32.0).strong().color(egui::Color32::from_rgb(100, 200, 255)));
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Editor 100% funcional. Escribe código Rust, haz clic en compilar y se ejecutará de verdad usando rustc en segundo plano.").size(16.0).italics());
    });

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.lessons.playground_code,
        Arc::clone(&state.lessons.playground_output),
        "▶ Ejecutar Local (rustc)",
        ejecutar_codigo_rust,
        &state.editor.syntax_set,
        theme,
    );
}

pub fn mostrar_editor_nube(ui: &mut egui::Ui, state: &mut AppState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new("☁️ Rust Playground API").size(32.0).strong().color(egui::Color32::from_rgb(255, 150, 100)));
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Envía tu código a los servidores oficiales de Rust para compilarlo. ¡Soporta el Top 100 crates (ej: serde, rand)!").size(16.0).italics());
    });

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.lessons.playground_nube_code,
        Arc::clone(&state.lessons.playground_nube_output),
        "☁️ Compilar en la Nube",
        ejecutar_codigo_api,
        &state.editor.syntax_set,
        theme,
    );
}
