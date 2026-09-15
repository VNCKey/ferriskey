use eframe::egui;

/// Item de navegación para la barra lateral (Sidebar - Nivel 1).
/// Posee fondo oscuro sutil, texto nítido y barra indicadora vertical en el lateral izquierdo al estar seleccionado.
pub fn nav_item_sidebar(ui: &mut egui::Ui, text: &str, is_selected: bool) -> egui::Response {
    let height = 28.0;
    let available_w = ui.available_width();
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(available_w, height), egui::Sense::click());

    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    // Color de fondo
    let bg_color = if is_selected {
        egui::Color32::from_rgb(24, 34, 50)
    } else if is_down {
        egui::Color32::from_rgb(28, 38, 56)
    } else if is_hovered {
        egui::Color32::from_rgb(18, 25, 38)
    } else {
        egui::Color32::TRANSPARENT
    };

    if bg_color != egui::Color32::TRANSPARENT {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(4), bg_color);
    }

    // Barra indicadora vertical en el lateral izquierdo al estar seleccionado (Rust Orange)
    // Desplazada un poco más a la derecha con un margen interior (inset)
    if is_selected {
        let bar_width = 3.0;
        let bar_height = rect.height() - 8.0;
        let bar_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + 6.0, rect.center().y - bar_height / 2.0),
            egui::vec2(bar_width, bar_height),
        );
        ui.painter().rect_filled(
            bar_rect,
            egui::CornerRadius::same(1),
            egui::Color32::from_rgb(255, 160, 50),
        );
    }

    // Color y estilo del texto
    let text_color = if is_selected {
        egui::Color32::WHITE
    } else if is_hovered {
        egui::Color32::from_rgb(230, 242, 255)
    } else {
        egui::Color32::from_rgb(170, 185, 208)
    };

    let text_x = rect.left() + if is_selected { 18.0 } else { 16.0 };
    let text_pos = egui::pos2(text_x, rect.center().y);

    let font_id = egui::FontId::proportional(13.0);

    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        text,
        font_id,
        text_color,
    );

    response
}

/// Pestaña superior con línea indicadora inferior (Underline Tab - Nivel 2).
/// Sin cajas celestes ni border-radius; destaca con tipografía nítida y línea inferior recta del color acento.
pub fn underline_tab(
    ui: &mut egui::Ui,
    text: &str,
    is_selected: bool,
    accent_color: egui::Color32,
) -> egui::Response {
    underline_tab_destacado(ui, text, is_selected, accent_color, false)
}

/// Pestaña superior con línea indicadora inferior y un acento animado opcional.
/// Cuando `destacar` es `true` y la pestaña no está seleccionada, una luz cálida
/// recorre discretamente la línea inferior para invitar al usuario a hacer clic.
pub fn underline_tab_destacado(
    ui: &mut egui::Ui,
    text: &str,
    is_selected: bool,
    accent_color: egui::Color32,
    destacar: bool,
) -> egui::Response {
    let animar = destacar && !is_selected;
    if animar {
        ui.ctx().request_repaint();
    }

    let time = if animar { ui.input(|i| i.time) } else { 0.0 };
    let pulse = if animar {
        ((time * 1.8).sin() * 0.5 + 0.5) as f32
    } else {
        0.0
    };

    ui.push_id(text, |ui| {
        let font_id = egui::FontId::proportional(13.0);
        let galley =
            ui.painter()
                .layout_no_wrap(text.to_string(), font_id.clone(), egui::Color32::WHITE);
        let text_width = galley.size().x;

        // El efecto no cambia el ancho de la pestaña para evitar pequeños saltos
        // de layout mientras la animación está activa.
        let h_padding = 10.0;
        let total_w = text_width + h_padding * 2.0;
        let total_h = 28.0;

        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(total_w, total_h), egui::Sense::click());
        let is_hovered = response.hovered();

        // Hover cálido con una pequeña elevación visual, respetando el color del tab.
        if is_hovered && !is_selected {
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(4),
                egui::Color32::from_rgba_unmultiplied(
                    accent_color.r(),
                    accent_color.g(),
                    accent_color.b(),
                    18,
                ),
            );
        }

        // Acento refinado: solo la línea inferior tiene movimiento. Así se
        // conserva la lectura limpia del tab y no aparece una caja ni una
        // línea inesperada en la parte superior.
        if animar {
            let line_y = rect.bottom() - 1.0;
            let line_start = rect.left() + 4.0;
            let line_end = rect.right() - 4.0;
            let line_width = line_end - line_start;
            let base_line = egui::Color32::from_rgba_unmultiplied(
                accent_color.r(),
                accent_color.g(),
                accent_color.b(),
                (24.0 + 18.0 * pulse) as u8,
            );
            ui.painter().line_segment(
                [egui::pos2(line_start, line_y), egui::pos2(line_end, line_y)],
                egui::Stroke::new(1.0, base_line),
            );

            // Un pequeño brillo recorre la línea y desaparece suavemente en
            // los extremos, en lugar de perseguir todo el contorno.
            let highlight_width = (line_width * 0.30).max(18.0);
            let travel = line_width + highlight_width;
            let head = (time as f32 * 22.0).rem_euclid(travel);
            let highlight_start = line_start + head - highlight_width;
            let visible_start = highlight_start.max(line_start);
            let visible_end = (highlight_start + highlight_width).min(line_end);

            if visible_end > visible_start {
                let highlight = egui::Color32::from_rgba_unmultiplied(
                    accent_color.r(),
                    accent_color.g(),
                    accent_color.b(),
                    (90.0 + 55.0 * pulse) as u8,
                );
                let glow = egui::Color32::from_rgba_unmultiplied(
                    accent_color.r(),
                    accent_color.g(),
                    accent_color.b(),
                    (18.0 + 14.0 * pulse) as u8,
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(visible_start, line_y),
                        egui::pos2(visible_end, line_y),
                    ],
                    egui::Stroke::new(4.0, glow),
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(visible_start, line_y),
                        egui::pos2(visible_end, line_y),
                    ],
                    egui::Stroke::new(1.2, highlight),
                );
            }
        }

        // Color del texto
        let text_color = if is_selected {
            egui::Color32::WHITE
        } else if is_hovered {
            accent_color
        } else if animar {
            // Tono ámbar estable; la animación queda reservada para la línea.
            egui::Color32::from_rgb(220, 165, 92)
        } else {
            egui::Color32::from_rgb(150, 168, 192)
        };

        // Dibujar texto centrado
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font_id,
            text_color,
        );

        // Línea indicadora inferior (Underline plano, recto y nítido)
        if is_selected {
            let line_y = rect.bottom() - 1.0;
            let line_p1 = egui::pos2(rect.left() + 2.0, line_y);
            let line_p2 = egui::pos2(rect.right() - 2.0, line_y);
            ui.painter()
                .line_segment([line_p1, line_p2], egui::Stroke::new(2.0, accent_color));
        } else if is_hovered {
            let line_y = rect.bottom() - 1.0;
            let line_p1 = egui::pos2(rect.left() + 4.0, line_y);
            let line_p2 = egui::pos2(rect.right() - 4.0, line_y);
            ui.painter()
                .line_segment([line_p1, line_p2], egui::Stroke::new(1.5, accent_color));
        } else if animar {
            // Línea inferior sutil que respira
            let line_y = rect.bottom() - 1.0;
            let line_p1 = egui::pos2(rect.left() + 4.0, line_y);
            let line_p2 = egui::pos2(rect.right() - 4.0, line_y);
            let alpha = (30.0 + 60.0 * pulse) as u8;
            ui.painter().line_segment(
                [line_p1, line_p2],
                egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(
                        accent_color.r(),
                        accent_color.g(),
                        accent_color.b(),
                        alpha,
                    ),
                ),
            );
        }

        response
    })
    .inner
}

/// Separador vertical delgado y centrado verticalmente para barras de navegación o toolbars.
pub fn separador_vertical_centrado(ui: &mut egui::Ui, altura: f32) {
    let (_id, rect) = ui.allocate_space(egui::vec2(16.0, 24.0));
    let center_x = rect.center().x;
    let center_y = rect.center().y;
    ui.painter().line_segment(
        [
            egui::pos2(center_x, center_y - altura / 2.0),
            egui::pos2(center_x, center_y + altura / 2.0),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 85)),
    );
}
