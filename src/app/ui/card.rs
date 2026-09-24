use eframe::egui::{self, CornerRadius, Pos2, Rect, RichText, Stroke, StrokeKind, Vec2};
use super::theme::{Colors, Spacing, Typography};

/// Formatea un número de forma compacta (e.g. 1.2K, 3.4M, 1.0B)
pub fn formatear_numero_compacto(n: u64) -> String {
    if n >= 1_000_000_000_000 {
        format!("{:.1}T", n as f64 / 1_000_000_000_000.0)
    } else if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        format!("{}", n)
    }
}

/// Formatea un número entero con separadores de miles
pub fn formatear_numero(n: u64) -> String {
    let s = n.to_string();
    let mut resultado = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            resultado.push('.');
        }
        resultado.push(c);
    }
    resultado
}

/// Tipo de badge para una tarjeta de crate
pub enum CrateCardBadge<'a> {
    Version(&'a str),
    Downloads(u64),
}

/// Contenedor base de tarjeta con estilo consistente
pub fn card<R>(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let mut frame = egui::Frame::new();
    frame.fill = Colors::BG_CARD;
    frame.inner_margin = Spacing::card_margin();
    frame.corner_radius = Spacing::card_rounding();
    frame.stroke = Spacing::card_stroke();

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        add_contents(ui)
    }).inner
}

/// Tarjeta didáctica con título naranja y descripción explicativa (estándar de Overview)
pub fn card_overview(ui: &mut egui::Ui, title: &str, description: &str) {
    card(ui, |ui| {
        ui.label(
            RichText::new(title)
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);
        ui.label(
            RichText::new(description)
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY)
                .line_height(Some(19.0)),
        );
    });
}

/// Tarjeta para diagramas ASCII, estructuras de archivos o árboles de módulos
pub fn diagram_card(ui: &mut egui::Ui, diagram_text: &str) {
    let mut frame = egui::Frame::new();
    frame.fill = Colors::BG_CARD_DARK;
    frame.inner_margin = Spacing::card_margin();
    frame.corner_radius = Spacing::card_rounding();
    frame.stroke = Spacing::card_stroke_orange();

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(
            RichText::new(diagram_text)
                .monospace()
                .font(Typography::code())
                .color(Colors::CYAN_ACCENT),
        );
    });
}

/// Contenedor base de tarjeta interactiva compacta con soporte de animación de entrada y hover
pub fn compact_interactive_card(
    ui: &mut egui::Ui,
    size: Vec2,
    alpha: f32,
    y_offset: f32,
    add_contents: impl FnOnce(&mut egui::Ui, Rect, bool),
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let card_rect = Rect::from_min_size(
        Pos2::new(rect.left(), rect.top() + y_offset),
        size,
    );

    let is_hovered = ui.input(|i| i.pointer.hover_pos().map_or(false, |pos| card_rect.contains(pos)));
    if is_hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let alpha_u8 = (alpha.clamp(0.0, 1.0) * 255.0) as u8;

    let bg_card = if is_hovered {
        egui::Color32::from_rgba_unmultiplied(24, 32, 46, alpha_u8)
    } else {
        egui::Color32::from_rgba_unmultiplied(14, 18, 26, alpha_u8)
    };

    let stroke_card = if is_hovered {
        egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha_u8)
    } else {
        egui::Color32::from_rgba_unmultiplied(45, 60, 90, alpha_u8)
    };

    ui.painter().rect(
        card_rect,
        CornerRadius::same(Spacing::ROUND_SM),
        bg_card,
        Stroke::new(1.0, stroke_card),
        StrokeKind::Inside,
    );

    add_contents(ui, card_rect, is_hovered);

    response
}

/// Tarjeta de catálogo de Crate (285x72 px por defecto)
pub fn crate_item_card(
    ui: &mut egui::Ui,
    name: &str,
    description: Option<&str>,
    badge: CrateCardBadge,
    card_w: f32,
    card_h: f32,
    alpha: f32,
    y_offset: f32,
) -> bool {
    let alpha_u8 = (alpha.clamp(0.0, 1.0) * 255.0) as u8;

    let resp = compact_interactive_card(
        ui,
        Vec2::new(card_w, card_h),
        alpha,
        y_offset,
        |ui, card_rect, is_hovered| {
            // Badge unificado a la derecha (mismo contenedor y paleta estándar)
            let badge_right = card_rect.right() - 8.0;
            let badge_top = card_rect.top() + 6.0;
            let tag_bg = egui::Color32::from_rgba_unmultiplied(20, 28, 42, alpha_u8);
            let tag_stroke = egui::Color32::from_rgba_unmultiplied(45, 65, 95, alpha_u8);
            let tag_text_color = egui::Color32::from_rgba_unmultiplied(160, 180, 205, alpha_u8);

            let badge_w = match badge {
                CrateCardBadge::Version(ver) => {
                    let ver_text = format!("v{}", ver);
                    let tag_font = egui::FontId::proportional(10.5);
                    let galley = ui.painter().layout_no_wrap(ver_text, tag_font, tag_text_color);
                    let bw = galley.size().x + 8.0;
                    let brect = Rect::from_min_size(
                        Pos2::new(badge_right - bw, badge_top),
                        Vec2::new(bw, 18.0),
                    );
                    ui.painter().rect(
                        brect,
                        CornerRadius::same(4),
                        tag_bg,
                        Stroke::new(1.0, tag_stroke),
                        StrokeKind::Inside,
                    );
                    ui.painter().galley(
                        Pos2::new(brect.left() + 4.0, brect.center().y - galley.size().y / 2.0),
                        galley,
                        tag_text_color,
                    );
                    bw
                }
                CrateCardBadge::Downloads(count) => {
                    let dl_text = formatear_numero_compacto(count);
                    let dl_font = egui::FontId::proportional(10.5);
                    let galley = ui.painter().layout_no_wrap(dl_text, dl_font, tag_text_color);
                    let icon_size = 12.0;
                    let bw = icon_size + 3.0 + galley.size().x + 8.0;
                    let brect = Rect::from_min_size(
                        Pos2::new(badge_right - bw, badge_top),
                        Vec2::new(bw, 18.0),
                    );
                    ui.painter().rect(
                        brect,
                        CornerRadius::same(4),
                        tag_bg,
                        Stroke::new(1.0, tag_stroke),
                        StrokeKind::Inside,
                    );
                    let icon_rect = Rect::from_min_size(
                        Pos2::new(brect.left() + 3.0, brect.center().y - icon_size / 2.0),
                        Vec2::splat(icon_size),
                    );
                    egui::Image::new(egui::include_image!("../../../assets/icons/download-svgrepo-com.svg"))
                        .fit_to_exact_size(Vec2::splat(icon_size))
                        .tint(tag_text_color)
                        .paint_at(ui, icon_rect);
                    ui.painter().galley(
                        Pos2::new(icon_rect.right() + 2.0, brect.center().y - galley.size().y / 2.0),
                        galley,
                        tag_text_color,
                    );
                    bw
                }
            };

            // Nombre a la izquierda
            let name_color = if is_hovered {
                egui::Color32::WHITE
            } else {
                Colors::CYAN_ACCENT
            };
            let name_color_alpha = egui::Color32::from_rgba_unmultiplied(
                name_color.r(),
                name_color.g(),
                name_color.b(),
                alpha_u8,
            );
            let name_pos = Pos2::new(card_rect.left() + 8.0, card_rect.top() + 6.0);
            let name_galley = ui.painter().layout_no_wrap(
                name.to_string(),
                egui::FontId::proportional(13.0),
                name_color_alpha,
            );
            let name_clip = Rect::from_min_max(
                Pos2::new(card_rect.left() + 8.0, card_rect.top() + 4.0),
                Pos2::new(badge_right - badge_w - 4.0, card_rect.top() + 25.0),
            );
            ui.painter().with_clip_rect(name_clip).galley(name_pos, name_galley, name_color_alpha);

            // Descripción
            let desc_raw = description.unwrap_or("Sin descripción proporcionada.");
            let desc_clean = desc_raw.replace('\n', " ").replace('\r', "").trim().to_string();
            let desc_text = if desc_clean.chars().count() > 100 {
                let s: String = desc_clean.chars().take(100).collect();
                format!("{}...", s)
            } else {
                desc_clean
            };
            let desc_color = egui::Color32::from_rgba_unmultiplied(
                Colors::TEXT_MUTED.r(),
                Colors::TEXT_MUTED.g(),
                Colors::TEXT_MUTED.b(),
                alpha_u8,
            );
            let desc_max_w = (card_rect.width() - 16.0).max(40.0);
            let desc_clip = Rect::from_min_max(
                Pos2::new(card_rect.left() + 8.0, card_rect.top() + 26.0),
                Pos2::new(card_rect.right() - 8.0, card_rect.bottom() - 4.0),
            );
            let desc_galley = ui.painter().layout(
                desc_text,
                egui::FontId::proportional(10.5),
                desc_color,
                desc_max_w,
            );
            ui.painter().with_clip_rect(desc_clip).galley(
                Pos2::new(card_rect.left() + 8.0, card_rect.top() + 26.0),
                desc_galley,
                desc_color,
            );
        },
    );

    let is_hovered = ui.input(|i| {
        let card_rect = Rect::from_min_size(
            Pos2::new(resp.rect.left(), resp.rect.top() + y_offset),
            Vec2::new(card_w, card_h),
        );
        i.pointer.hover_pos().map_or(false, |pos| card_rect.contains(pos))
    });

    resp.clicked() && is_hovered
}

/// Tarjeta compacta para palabras clave o categorías taxonómicas
pub fn taxonomy_item_card(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    card_w: f32,
    card_h: f32,
    alpha: f32,
    y_offset: f32,
) -> bool {
    let alpha_u8 = (alpha.clamp(0.0, 1.0) * 255.0) as u8;

    let resp = compact_interactive_card(
        ui,
        Vec2::new(card_w, card_h),
        alpha,
        y_offset,
        |ui, card_rect, is_hovered| {
            // Flecha a la derecha
            let arrow_size = 14.0;
            let arrow_rect = Rect::from_center_size(
                Pos2::new(card_rect.right() - 14.0, card_rect.center().y),
                Vec2::splat(arrow_size),
            );
            let arrow_color = if is_hovered {
                Colors::ORANGE_RUST
            } else {
                Colors::CYAN_ACCENT
            };
            let arrow_color_alpha = egui::Color32::from_rgba_unmultiplied(
                arrow_color.r(),
                arrow_color.g(),
                arrow_color.b(),
                alpha_u8,
            );
            egui::Image::new(egui::include_image!("../../../assets/icons/arrow-up-svgrepo-com.svg"))
                .fit_to_exact_size(Vec2::splat(arrow_size))
                .tint(arrow_color_alpha)
                .paint_at(ui, arrow_rect);

            // Título a la izquierda
            let title_color = if is_hovered {
                egui::Color32::WHITE
            } else {
                Colors::CYAN_ACCENT
            };
            let title_color_alpha = egui::Color32::from_rgba_unmultiplied(
                title_color.r(),
                title_color.g(),
                title_color.b(),
                alpha_u8,
            );
            let title_pos = Pos2::new(card_rect.left() + 10.0, card_rect.top() + 8.0);
            let title_galley = ui.painter().layout_no_wrap(
                title.to_string(),
                egui::FontId::proportional(13.0),
                title_color_alpha,
            );
            let title_clip = Rect::from_min_max(
                Pos2::new(card_rect.left() + 10.0, card_rect.top() + 4.0),
                Pos2::new(arrow_rect.left() - 4.0, card_rect.top() + 26.0),
            );
            ui.painter().with_clip_rect(title_clip).galley(title_pos, title_galley, title_color_alpha);

            // Subtítulo
            let sub_color = egui::Color32::from_rgba_unmultiplied(
                Colors::TEXT_MUTED.r(),
                Colors::TEXT_MUTED.g(),
                Colors::TEXT_MUTED.b(),
                alpha_u8,
            );
            let sub_pos = Pos2::new(card_rect.left() + 10.0, card_rect.top() + 32.0);
            let sub_max_w = (arrow_rect.left() - card_rect.left() - 10.0).max(40.0);
            let sub_galley = ui.painter().layout(
                subtitle.to_string(),
                egui::FontId::proportional(10.5),
                sub_color,
                sub_max_w,
            );
            let sub_clip = Rect::from_min_max(
                Pos2::new(card_rect.left() + 10.0, card_rect.top() + 26.0),
                Pos2::new(arrow_rect.left() - 4.0, card_rect.bottom() - 4.0),
            );
            ui.painter().with_clip_rect(sub_clip).galley(sub_pos, sub_galley, sub_color);
        },
    );

    let is_hovered = ui.input(|i| {
        let card_rect = Rect::from_min_size(
            Pos2::new(resp.rect.left(), resp.rect.top() + y_offset),
            Vec2::new(card_w, card_h),
        );
        i.pointer.hover_pos().map_or(false, |pos| card_rect.contains(pos))
    });

    resp.clicked() && is_hovered
}
