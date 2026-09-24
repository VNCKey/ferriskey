use eframe::egui::{self, CornerRadius, Response, Stroke};
use super::theme::{Colors, Spacing, Typography};

/// Renderizador base para botones con dimensiones estables que NUNCA se encojen ni deforman en hover/click
fn render_stable_button(
    ui: &mut egui::Ui,
    text: &str,
    font_id: egui::FontId,
    text_color: egui::Color32,
    base_bg: egui::Color32,
    hover_bg: egui::Color32,
    active_bg: egui::Color32,
    base_stroke: Stroke,
    hover_stroke: Stroke,
    padding: egui::Vec2,
) -> Response {
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, text_color);
    let desired_size = galley.size() + padding * 2.0;

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let is_hovered = response.hovered();
        let is_down = response.is_pointer_button_down_on();

        let bg_color = if is_down {
            active_bg
        } else if is_hovered {
            hover_bg
        } else {
            base_bg
        };

        let stroke = if is_hovered || is_down {
            hover_stroke
        } else {
            base_stroke
        };

        // Dibujar fondo y borde estricto sin deformaciones
        ui.painter().rect(
            rect,
            CornerRadius::same(Spacing::ROUND_SM),
            bg_color,
            stroke,
            egui::StrokeKind::Inside,
        );

        // Centrar el texto perfectamente dentro del botón
        let text_pos = rect.center() - galley.size() / 2.0;
        let final_text_color = if is_hovered {
            egui::Color32::WHITE
        } else {
            text_color
        };
        ui.painter().galley(text_pos, galley, final_text_color);
    }

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Botón pequeño de acción para encabezados de tarjetas (ej. "Ver", "Config")
/// Tamaño 100% estable: no se encoje ni se mueve en hover.
pub fn btn_action_small(ui: &mut egui::Ui, text: &str) -> Response {
    render_stable_button(
        ui,
        text,
        Typography::body_small(),
        Colors::CYAN_ACCENT,
        Colors::BG_CODE_INLINE,
        egui::Color32::from_rgb(25, 36, 52),
        egui::Color32::from_rgb(20, 28, 40),
        Stroke::new(1.0, Colors::BORDER_SUBTLE),
        Stroke::new(1.0, Colors::CYAN_ACCENT),
        egui::vec2(10.0, 4.0),
    )
}

/// Botón pequeño con acento naranja
pub fn btn_action_orange_small(ui: &mut egui::Ui, text: &str) -> Response {
    render_stable_button(
        ui,
        text,
        Typography::body_small(),
        Colors::ORANGE_RUST,
        Colors::BG_CODE_INLINE,
        egui::Color32::from_rgb(45, 30, 20),
        egui::Color32::from_rgb(35, 22, 16),
        Stroke::new(1.0, Colors::BORDER_SUBTLE),
        Stroke::new(1.0, Colors::ORANGE_RUST),
        egui::vec2(10.0, 4.0),
    )
}

/// Botón primario de acción destacada (Rust Orange)
pub fn btn_primary(ui: &mut egui::Ui, text: &str) -> Response {
    render_stable_button(
        ui,
        text,
        Typography::body(),
        Colors::TEXT_WHITE,
        Colors::ORANGE_RUST,
        egui::Color32::from_rgb(255, 175, 75),
        egui::Color32::from_rgb(230, 140, 40),
        Stroke::NONE,
        Stroke::NONE,
        egui::vec2(14.0, 6.0),
    )
}

/// Botón secundario con borde sutil
pub fn btn_secondary(ui: &mut egui::Ui, text: &str) -> Response {
    render_stable_button(
        ui,
        text,
        Typography::body(),
        Colors::TEXT_PRIMARY,
        Colors::BG_CARD,
        egui::Color32::from_rgb(24, 30, 42),
        egui::Color32::from_rgb(18, 22, 32),
        Spacing::card_stroke(),
        Stroke::new(1.0, Colors::TEXT_MUTED),
        egui::vec2(14.0, 6.0),
    )
}
