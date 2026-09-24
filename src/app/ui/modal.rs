use eframe::egui::{self, CornerRadius, Margin, Rect, Stroke, Vec2};
use super::theme::{Colors, Spacing};

/// Extrae las dimensiones (ancho, alto) nativas de un texto SVG o su viewBox
pub fn parse_svg_dimensions(svg: &str) -> Option<(f32, f32)> {
    // 1. Buscar width="..." height="..."
    if let (Some(w_pos), Some(h_pos)) = (svg.find("width=\""), svg.find("height=\"")) {
        let w_rest = &svg[w_pos + 7..];
        let h_rest = &svg[h_pos + 8..];
        if let (Some(w_end), Some(h_end)) = (w_rest.find('"'), h_rest.find('"')) {
            let w_str = w_rest[..w_end].trim_end_matches("px").trim_end_matches("pt");
            let h_str = h_rest[..h_end].trim_end_matches("px").trim_end_matches("pt");
            if let (Ok(w), Ok(h)) = (w_str.parse::<f32>(), h_str.parse::<f32>()) {
                if w > 0.0 && h > 0.0 {
                    return Some((w, h));
                }
            }
        }
    }

    // 2. Buscar viewBox="min-x min-y width height"
    if let Some(vb_pos) = svg.find("viewBox=\"") {
        let vb_rest = &svg[vb_pos + 9..];
        if let Some(vb_end) = vb_rest.find('"') {
            let clean_vb = vb_rest[..vb_end].replace(',', " ");
            let parts: Vec<&str> = clean_vb.split_whitespace().collect();
            if parts.len() == 4 {
                if let (Ok(w), Ok(h)) = (parts[2].parse::<f32>(), parts[3].parse::<f32>()) {
                    if w > 0.0 && h > 0.0 {
                        return Some((w, h));
                    }
                }
            }
        }
    }

    None
}

/// Modal visor de diagramas SVG con diseño minimalista, fondo oscuro profundo (#0B0F16)
/// y botón de cierre flotante con icono SVG vectorial.
pub fn mostrar_modal_diagrama(
    ctx: &egui::Context,
    is_open: &mut bool,
    window_id: &'static str,
    uri: &'static str,
    svg_bytes: &'static [u8],
    dimensions: Option<[f32; 2]>,
) {
    if !*is_open {
        return;
    }

    let svg_text = std::str::from_utf8(svg_bytes).unwrap_or("");
    let (native_w, native_h) = parse_svg_dimensions(svg_text).unwrap_or((400.0, 600.0));
    let aspect = (native_w / native_h.max(1.0)).clamp(0.01, 50.0);

    let (default_w, default_h) = match dimensions {
        Some([w, h]) => (w, h),
        None => {
            if aspect >= 1.0 {
                (840.0, 640.0)
            } else {
                (440.0, 740.0)
            }
        }
    };

    egui::Window::new(window_id)
        .title_bar(false)
        .frame(egui::Frame::NONE)
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .default_size([default_w, default_h])
        .min_size([280.0, 240.0])
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            let mut modal_frame = egui::Frame::new();
            modal_frame.fill = egui::Color32::from_rgb(11, 15, 22);
            modal_frame.inner_margin = Margin::same(2);
            modal_frame.corner_radius = CornerRadius::same(Spacing::ROUND_SM);
            modal_frame.stroke = Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95));
            modal_frame.shadow = egui::Shadow {
                offset: [0, 8],
                blur: 24,
                spread: 0,
                color: egui::Color32::from_black_alpha(180),
            };

            modal_frame.show(ui, |ui| {
                ui.set_width(ui.available_width());

                // Botón de cierre flotante superior derecho
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 26.0),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        let (close_rect, close_resp) =
                            ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::click());
                        let hovered = close_resp.hovered();
                        if hovered {
                            ui.painter().rect_filled(
                                close_rect,
                                CornerRadius::same(Spacing::ROUND_SM),
                                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 22),
                            );
                        }
                        let close_color = if hovered {
                            egui::Color32::from_rgb(255, 140, 140)
                        } else {
                            Colors::TEXT_MUTED
                        };

                        egui::Image::new(egui::include_image!(
                            "../../../assets/icons/close-circle-svgrepo-com.svg"
                        ))
                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                        .tint(close_color)
                        .paint_at(
                            ui,
                            Rect::from_center_size(close_rect.center(), Vec2::splat(16.0)),
                        );

                        if close_resp.clicked() {
                            *is_open = false;
                        }
                        close_resp
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .on_hover_text("Cerrar diagrama");
                    },
                );

                // Estilo personalizado de barra de desplazamiento
                ui.style_mut().spacing.scroll.floating = false;
                ui.style_mut().spacing.scroll.bar_width = 8.0;
                ui.style_mut().spacing.scroll.bar_inner_margin = 0.0;
                ui.style_mut().spacing.scroll.bar_outer_margin = 0.0;
                ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 60, 85);
                ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 95, 135);
                ui.style_mut().visuals.widgets.active.bg_fill = Colors::ORANGE_RUST;

                egui::ScrollArea::both()
                    .id_salt((window_id, "diagram_scroll"))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let max_w = match dimensions {
                            Some([w, _]) => (w - 24.0).max(native_w),
                            None => {
                                if aspect < 1.0 {
                                    native_w.max(280.0)
                                } else {
                                    native_w.max(680.0)
                                }
                            }
                        };
                        let avail_w = ui.available_width().min(max_w).max(200.0);
                        let img_h = avail_w / aspect;

                        ui.vertical_centered(|ui| {
                            ui.add(
                                egui::Image::from_bytes(uri, svg_bytes)
                                    .fit_to_exact_size(egui::vec2(avail_w, img_h))
                                    .maintain_aspect_ratio(true),
                            );
                        });
                        ui.add_space(4.0);
                    });
            });
        });
}
