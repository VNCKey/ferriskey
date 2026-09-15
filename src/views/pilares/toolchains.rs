use eframe::egui;

use crate::app::AppState;
use crate::components::educational_table::mostrar_tabla_educativa;

#[allow(dead_code)]
pub fn mostrar_toolchains(ui: &mut egui::Ui, _state: &mut AppState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        mostrar_toolchains_contenido(ui);
    });
}

pub fn mostrar_toolchains_contenido(ui: &mut egui::Ui) {
    let orange = egui::Color32::from_rgb(255, 180, 80);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

        ui.heading(
            egui::RichText::new("Rust Toolchains and Versions")
                .size(20.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(
                "Rustup permite instalar, seleccionar y mantener las versiones y herramientas utilizadas por un proyecto.",
            )
            .size(15.0)
            .color(text),
        );
        ui.add_space(16.0);

        mostrar_tabla_educativa(ui, "rust_toolchains_channels", |ui| {
            ui.label(egui::RichText::new("Canal").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Propósito").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (channel, purpose) in [
                ("stable", "Versión recomendada para proyectos normales"),
                ("beta", "Próxima versión que se prepara para stable"),
                ("nightly", "Características experimentales y no estabilizadas"),
            ] {
                ui.label(egui::RichText::new(channel).monospace().strong().color(cyan));
                ui.label(purpose);
                ui.end_row();
            }
        });

        ui.add_space(18.0);
        ui.label(egui::RichText::new("Comandos de rustup").strong().color(orange));
        ui.add_space(6.0);
        mostrar_codigo(
            ui,
            "rustup show\nrustup toolchain list\nrustup default stable\nrustup update\nrustup override set nightly",
        );

        ui.add_space(16.0);
        ui.label(egui::RichText::new("Declarar el toolchain del proyecto").strong().color(orange));
        ui.add_space(6.0);
        mostrar_codigo(
            ui,
            "# rust-toolchain.toml\n\n[toolchain]\nchannel = \"stable\"\ncomponents = [\"rustfmt\", \"clippy\"]",
        );

        ui.add_space(14.0);
        ui.label(
            "La versión del compilador, la edición del lenguaje y la versión de una crate son conceptos diferentes.",
        );
}

fn mostrar_codigo(ui: &mut egui::Ui, code: &str) {
    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(13, 17, 23);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(6);
    frame.show(ui, |ui| {
        ui.label(egui::RichText::new(code).monospace().color(egui::Color32::LIGHT_GREEN));
    });
}
