use eframe::egui::{self, Color32, CornerRadius, Margin, RichText, Stroke};
use super::theme::{Colors, Spacing, Typography};

/// Tipos de alerta didáctica
pub enum CalloutType {
    Tip,
    Ferris,
    Warning,
    Info,
}

impl CalloutType {
    pub fn label_prefix(&self) -> &'static str {
        match self {
            Self::Tip => "[TIP]",
            Self::Ferris => "[FERRIS]",
            Self::Warning => "[ADVERTENCIA]",
            Self::Info => "[NOTA]",
        }
    }

    pub fn colors(&self) -> (Color32, Color32) {
        // (Fondo, Borde/Texto destacador)
        match self {
            Self::Tip => (
                Color32::from_rgb(18, 32, 24),
                Colors::SUCCESS,
            ),
            Self::Ferris => (
                Color32::from_rgb(32, 22, 18),
                Colors::ORANGE_RUST,
            ),
            Self::Warning => (
                Color32::from_rgb(34, 28, 16),
                Colors::WARNING,
            ),
            Self::Info => (
                Color32::from_rgb(16, 26, 38),
                Colors::CYAN_ACCENT,
            ),
        }
    }
}

/// Renderiza una caja de llamada / alerta estilizada
pub fn callout(ui: &mut egui::Ui, callout_type: CalloutType, title: &str, message: &str) {
    let (bg_color, accent_color) = callout_type.colors();
    let prefix = callout_type.label_prefix();

    let mut frame = egui::Frame::new();
    frame.fill = bg_color;
    frame.inner_margin = Margin::symmetric(14, 10);
    frame.corner_radius = CornerRadius::same(Spacing::ROUND_SM);
    frame.stroke = Stroke::new(1.0, accent_color);

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("{prefix} {title}"))
                    .font(Typography::card_title())
                    .strong()
                    .color(accent_color),
            );
        });

        ui.add_space(4.0);

        ui.label(
            RichText::new(message)
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY)
                .line_height(Some(19.0)),
        );
    });
}

/// Helper para consejos de buenas prácticas
pub fn callout_tip(ui: &mut egui::Ui, title: &str, message: &str) {
    callout(ui, CalloutType::Tip, title, message);
}

/// Helper para consejos exclusivos del compilador o reglas de Rust
pub fn callout_ferris(ui: &mut egui::Ui, title: &str, message: &str) {
    callout(ui, CalloutType::Ferris, title, message);
}

/// Helper para advertencias de errores en tiempo de ejecución o panics
pub fn callout_warning(ui: &mut egui::Ui, title: &str, message: &str) {
    callout(ui, CalloutType::Warning, title, message);
}

/// Helper para notas informativas
pub fn callout_info(ui: &mut egui::Ui, title: &str, message: &str) {
    callout(ui, CalloutType::Info, title, message);
}
