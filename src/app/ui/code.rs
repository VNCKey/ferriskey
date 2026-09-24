use eframe::egui::{self, CornerRadius, Margin, RichText, Stroke};
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

use crate::components::code_editor::syntax_layouter_with_font_size;
use super::theme::{Colors, Spacing, Typography};

/// Chip para palabras clave, tipos o expresiones cortas (ej. `String`, `&str`, `mod`)
pub fn inline_code_chip(ui: &mut egui::Ui, code: &str) {
    let mut frame = egui::Frame::new();
    frame.fill = Colors::BG_CODE_INLINE;
    frame.inner_margin = Margin::symmetric(6, 2);
    frame.corner_radius = CornerRadius::same(Spacing::ROUND_SM);
    frame.stroke = Stroke::new(1.0, Colors::BORDER_SUBTLE);

    frame.show(ui, |ui| {
        ui.add(
            egui::Label::new(
                RichText::new(code)
                    .font(Typography::code())
                    .color(Colors::CYAN_ACCENT),
            )
            .wrap_mode(egui::TextWrapMode::Extend),
        );
    });
}

/// Chip con color personalizado (por ejemplo naranja, verde o blanco)
pub fn inline_code_chip_color(ui: &mut egui::Ui, code: &str, color: egui::Color32) {
    let mut frame = egui::Frame::new();
    frame.fill = Colors::BG_CODE_INLINE;
    frame.inner_margin = Margin::symmetric(6, 2);
    frame.corner_radius = CornerRadius::same(Spacing::ROUND_SM);
    frame.stroke = Stroke::new(1.0, Colors::BORDER_SUBTLE);

    frame.show(ui, |ui| {
        ui.add(
            egui::Label::new(
                RichText::new(code)
                    .font(Typography::code())
                    .color(color),
            )
            .wrap_mode(egui::TextWrapMode::Extend),
        );
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodePresentation {
    Block,
    Compact,
}

/// Renderizador universal de código con el mismo resaltado de sintaxis y la
/// misma superficie visual en Code Lab y tablas.
pub fn highlighted_code(
    ui: &mut egui::Ui,
    code: &str,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
    presentation: CodePresentation,
) {
    let margin = match presentation {
        CodePresentation::Block => Margin {
            left: 12,
            right: 12,
            top: 9,
            bottom: 9,
        },
        CodePresentation::Compact => Margin::symmetric(7, 4),
    };

    let render_code = |ui: &mut egui::Ui| {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(12, 18, 27))
            .stroke(Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(255, 160, 50, 72),
            ))
            .corner_radius(CornerRadius::same(5))
            .inner_margin(margin)
            .show(ui, |ui| {
            if presentation == CodePresentation::Block {
                ui.set_min_width(ui.available_width());
            }

            let code_width = match presentation {
                CodePresentation::Block => ui.available_width(),
                CodePresentation::Compact => f32::INFINITY,
            };
            let font_size = match presentation {
                CodePresentation::Block => 14.0,
                CodePresentation::Compact => 12.0,
            };
            let galley = syntax_layouter_with_font_size(
                ui,
                code,
                code_width,
                syntax_set,
                theme,
                extension,
                font_size,
            );
            ui.add(egui::Label::new(galley).selectable(true));
        });
    };

    if presentation == CodePresentation::Compact {
        // Espacio exterior al borde para que el bloque no toque los límites
        // superior e inferior de la franja de la tabla.
        egui::Frame::new()
            .inner_margin(Margin::symmetric(0, 3))
            .show(ui, render_code);
    } else {
        render_code(ui);
    }
}

pub fn highlighted_code_block(
    ui: &mut egui::Ui,
    code: &str,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
) {
    highlighted_code(
        ui,
        code,
        syntax_set,
        theme,
        extension,
        CodePresentation::Block,
    );
}

pub fn table_code_snippet(
    ui: &mut egui::Ui,
    code: &str,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
) {
    highlighted_code(
        ui,
        code,
        syntax_set,
        theme,
        extension,
        CodePresentation::Compact,
    );
}

/// Bloque de código con encabezado, nombre de archivo o título y botón de copiar al portapapeles
pub fn code_block(ui: &mut egui::Ui, title: &str, code: &str) {
    let mut frame = egui::Frame::new();
    frame.fill = Colors::BG_CARD_DARK;
    frame.inner_margin = Spacing::card_margin();
    frame.corner_radius = CornerRadius::same(Spacing::ROUND_MD);
    frame.stroke = Spacing::card_stroke_orange();

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(title)
                    .font(Typography::body_small())
                    .strong()
                    .color(Colors::ORANGE_RUST),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn = egui::Button::new(
                    RichText::new("Copiar")
                        .font(Typography::body_small())
                        .color(Colors::TEXT_MUTED),
                )
                .fill(Colors::BG_CODE_INLINE)
                .corner_radius(CornerRadius::same(Spacing::ROUND_SM));

                if ui.add(btn).on_hover_text("Copiar este código al portapapeles").clicked() {
                    ui.ctx().copy_text(code.to_string());
                }
            });
        });

        ui.add_space(8.0);

        ui.add(
            egui::Label::new(
                RichText::new(code)
                    .font(Typography::code())
                    .color(Colors::TEXT_PRIMARY),
            )
            .selectable(true)
            .wrap_mode(egui::TextWrapMode::Wrap),
        );
    });
}

/// Contenedor simple para salidas de consola o snippets de código monospace
pub fn code_box(ui: &mut egui::Ui, text: &str) {
    let mut frame = egui::Frame::new();
    frame.fill = Colors::BG_CARD_DARK;
    frame.inner_margin = Margin::symmetric(12, 8);
    frame.corner_radius = CornerRadius::same(Spacing::ROUND_SM);
    frame.stroke = Spacing::card_stroke();

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.add(
            egui::Label::new(
                RichText::new(text)
                    .font(Typography::code())
                    .color(Colors::TEXT_PRIMARY),
            )
            .selectable(true)
            .wrap_mode(egui::TextWrapMode::Wrap),
        );
    });
}
