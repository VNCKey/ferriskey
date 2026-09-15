use eframe::egui;

use crate::app::AppState;
use crate::components::educational_table::mostrar_tabla_educativa;

pub fn mostrar_cargo_workflow(ui: &mut egui::Ui, _state: &mut AppState) {
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading(
            egui::RichText::new("Cargo Workflow")
                .size(20.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(
                "Comandos esenciales para crear, comprobar, ejecutar y mantener un proyecto Rust.",
            )
            .size(15.0)
            .color(text),
        );
        ui.add_space(16.0);

        mostrar_tabla_educativa(ui, "cargo_workflow_commands", |ui| {
            ui.label(egui::RichText::new("Comando").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Función").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Momento de uso").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (command, purpose, moment) in [
                ("cargo new", "Crea un paquete nuevo", "Inicio del proyecto"),
                ("cargo init", "Convierte una carpeta en proyecto Cargo", "Proyecto existente"),
                ("cargo check", "Comprueba el código sin generar un binario final", "Durante el desarrollo"),
                ("cargo build", "Compila el proyecto en modo dev", "Desarrollo"),
                ("cargo run", "Compila y ejecuta un binario", "Pruebas manuales"),
                ("cargo test", "Ejecuta los tests", "Verificación"),
                ("cargo fmt", "Formatea el código", "Antes de guardar o publicar"),
                ("cargo clippy", "Analiza posibles mejoras", "Revisión de calidad"),
                ("cargo doc", "Genera documentación HTML", "Documentación"),
            ] {
                ui.label(egui::RichText::new(command).monospace().strong().color(cyan));
                ui.label(purpose);
                ui.label(moment);
                ui.end_row();
            }
        });
    });
}
