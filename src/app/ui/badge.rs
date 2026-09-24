use eframe::egui::{self, Color32, CornerRadius, Margin, RichText, Stroke};
use super::theme::{Colors, Spacing, Typography};

/// Tipos de insignias de categoría
pub enum BadgeStyle {
    Orange,
    Cyan,
    Green,
    Muted,
}

impl BadgeStyle {
    pub fn colors(&self) -> (Color32, Color32) {
        match self {
            Self::Orange => (
                Color32::from_rgba_unmultiplied(255, 160, 50, 30),
                Colors::ORANGE_RUST,
            ),
            Self::Cyan => (
                Color32::from_rgba_unmultiplied(100, 200, 255, 30),
                Colors::CYAN_ACCENT,
            ),
            Self::Green => (
                Color32::from_rgba_unmultiplied(34, 197, 94, 30),
                Colors::SUCCESS,
            ),
            Self::Muted => (
                Color32::from_rgba_unmultiplied(160, 175, 195, 20),
                Colors::TEXT_MUTED,
            ),
        }
    }
}

/// Renderiza una píldora o etiqueta de categoría
pub fn badge(ui: &mut egui::Ui, text: &str, style: BadgeStyle) {
    let (bg, border) = style.colors();

    let mut frame = egui::Frame::new();
    frame.fill = bg;
    frame.inner_margin = Margin::symmetric(8, 3);
    frame.corner_radius = CornerRadius::same(Spacing::ROUND_SM);
    frame.stroke = Stroke::new(1.0, border);

    frame.show(ui, |ui| {
        ui.label(
            RichText::new(text)
                .font(Typography::code_small())
                .strong()
                .color(border),
        );
    });
}

/// Etiqueta / tag técnico estandarizado y uniforme (estilo sutil oscuro con borde de diseño)
pub fn tag_chip(ui: &mut egui::Ui, text: &str) {
    let mut frame = egui::Frame::new();
    frame.fill = Color32::from_rgb(20, 28, 42);
    frame.corner_radius = CornerRadius::same(4);
    frame.stroke = Stroke::new(1.0, Color32::from_rgb(45, 65, 95));
    frame.inner_margin = Margin::symmetric(7, 2);

    frame.show(ui, |ui| {
        ui.label(
            RichText::new(text)
                .font(Typography::code_small())
                .strong()
                .color(Color32::from_rgb(160, 180, 205)),
        );
    });
}
