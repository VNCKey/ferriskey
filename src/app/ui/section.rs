use eframe::egui::{self, RichText};
use super::code::inline_code_chip;
use super::theme::{Colors, Typography};

/// Título principal de la sesión (24pt naranja fuerte)
pub fn session_title(ui: &mut egui::Ui, title: &str) {
    ui.heading(
        RichText::new(title)
            .font(Typography::session_title())
            .strong()
            .color(Colors::ORANGE_RUST),
    );
    ui.add_space(8.0);
}

/// Introducción conceptual general de la sesión
pub fn session_intro(ui: &mut egui::Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY)
            .line_height(Some(21.0)),
    );
    ui.add_space(8.0);
}

/// Mensaje estándar que indica que la práctica interactiva está en el CodeLab
pub fn codelab_notice(ui: &mut egui::Ui) {
    ui.label(
        RichText::new(
            "El Code Lab contiene los ejemplos, ejercicios y comprobaciones prácticas de cada tema. Esta sección funciona como el mapa conceptual general de la sesión.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);
}

/// Título de sección dentro de la teoría (18pt naranja)
pub fn section_heading(ui: &mut egui::Ui, title: &str) {
    ui.heading(
        RichText::new(title)
            .font(Typography::heading())
            .strong()
            .color(Colors::ORANGE_RUST),
    );
    ui.add_space(8.0);
}

/// Invitación visual para ir al CodeLab al final de la teoría
pub fn codelab_cta_banner(ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Para poner en práctica todos estos conceptos con proyectos reales y compilación en vivo, dirígete a la pestaña")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "Code Lab");
        ui.label(
            RichText::new("en la esquina superior derecha.")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
    });
}

/// Divisor horizontal sutil
pub fn divider(ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);
}
