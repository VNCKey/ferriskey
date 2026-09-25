use crate::app::AppState;
use crate::routes::AppRoute;
use eframe::egui;

pub fn mostrar_landing_page(ui: &mut egui::Ui, state: &mut AppState) {
    let avail_w = ui.available_width();
    let avail_h = ui.available_height();

    ui.allocate_ui(egui::vec2(avail_w, avail_h), |ui| {
        let total_rect = ui.max_rect();

        // Renderizado del fondo estelar 3D limpio a velocidad constante
        pintar_escena_estelar(ui, total_rect);

        // --- HEADER SUPERIOR FLOTANTE CON BORDER BOTTOM ---
        let header_frame = egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(20, 15))
            .outer_margin(egui::Margin {
                left: (avail_w * 0.06).clamp(20.0, 90.0) as i8,
                right: (avail_w * 0.06).clamp(20.0, 90.0) as i8,
                top: 10,
                bottom: 0,
            })
            .stroke(egui::Stroke::NONE);

        let response = header_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                // Logo izquierdo limpio
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(
                        egui::RichText::new("Ferris")
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::from_rgb(255, 155, 45)),
                    );
                    ui.label(
                        egui::RichText::new("Key")
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::from_rgb(230, 245, 255)),
                    );
                });

                // Menú derecho (Comunidad)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let estilo_previo = (**ui.style()).clone();
                    let estilo_mut = ui.style_mut();
                    estilo_mut.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    estilo_mut.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                    estilo_mut.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;

                    estilo_mut.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgba_unmultiplied(25, 35, 60, 200);
                    estilo_mut.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 160, 50));

                    estilo_mut.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgba_unmultiplied(40, 55, 90, 240);

                    ui.menu_button(
                        egui::RichText::new("Comunidad ▾")
                            .size(15.0)
                            .color(egui::Color32::from_rgb(205, 220, 240)),
                        |ui| {
                            *ui.style_mut() = estilo_previo;
                            ui.set_min_width(130.0);
                            if ui.button("🌐 Página Web").clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab("https://ferriskey.com"));
                            }
                            if ui.button("💬 Discord").clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab("https://discord.com"));
                            }
                            if ui.button("✈️ Telegram").clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab("https://telegram.org"));
                            }
                        },
                    );
                });
            });
        });

        // Borde inferior con degradado óptico
        let rect = response.response.rect;
        let pad = (avail_w * 0.06).clamp(20.0, 90.0);
        let start = egui::pos2(rect.left() + pad, rect.bottom());
        let end = egui::pos2(rect.right() - pad, rect.bottom());

        ui.painter().line_segment(
            [start, end],
            egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(255, 160, 50, 180)),
        );

        ui.add_space(16.0);

        // --- CUERPO PRINCIPAL HERO ---
        let max_texto_w = (avail_w * 0.52).clamp(340.0, 720.0);

        ui.vertical_centered(|ui| {
            // Título principal con brillo estelar
            ui.add_sized(
                [max_texto_w, 0.0],
                egui::Label::new(
                    egui::RichText::new("Diviértete conociendo y aprendiendo Rust")
                        .size((avail_w * 0.027).clamp(26.0, 38.0))
                        .strong()
                        .color(egui::Color32::from_rgb(255, 175, 55)),
                )
                .wrap(),
            );

            ui.add_space(12.0);

            // Descripción con fondo de lectura mejorado
            ui.add_sized(
                [max_texto_w, 0.0],
                egui::Label::new(
                    egui::RichText::new(
                        "FerrisKey te guiará paso a paso desde los conceptos más básicos hasta los más avanzados, mediante explicaciones interactivas, ejemplos visuales y desafíos de código prácticos.",
                    )
                    .size((avail_w * 0.0135).clamp(14.5, 18.0))
                    .color(egui::Color32::from_rgb(195, 215, 245))
                    .line_height(Some(24.0)),
                )
                .wrap(),
            );

            // Espaciado hacia la parte inferior para que el fondo espacial quede libre y limpio en el centro
            let btn_w = (avail_w * 0.22).clamp(200.0, 300.0);
            let btn_h = (avail_h * 0.065).clamp(46.0, 56.0);
            let margen_inferior = (avail_h * 0.08).clamp(35.0, 70.0);
            let espacio_al_boton = (ui.available_height() - btn_h - margen_inferior).max(40.0);
            ui.add_space(espacio_al_boton);

            // Botón de acción principal "COMENZAR" a velocidad uniforme constante
            let btn_comenzar = ui.add_sized(
                [btn_w, btn_h],
                egui::Button::new(
                    egui::RichText::new("COMENZAR")
                        .size((btn_h * 0.38).clamp(16.0, 21.0))
                        .strong()
                        .color(egui::Color32::BLACK),
                )
                .fill(egui::Color32::from_rgb(255, 175, 45))
                .corner_radius(egui::CornerRadius::same((btn_h * 0.5) as u8)),
            );

            if btn_comenzar.clicked() {
                state.ui.ruta_actual = AppRoute::TutorialCargo;
                state.ui.anim_trigger = ui.input(|i| i.time);
            }
        });
    });
}

/// Renderiza el campo estelar 3D en movimiento constante hacia el observador:
/// Sin ondas concéntricas y a velocidad uniforme constante sin aceleraciones.
fn pintar_escena_estelar(ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    let time = ui.input(|i| i.time) as f32;
    let pointer = ui.input(|i| i.pointer.hover_pos()).unwrap_or(rect.center());

    // Parallax suave según la posición del cursor
    let pointer_x = ((pointer.x - rect.center().x) / rect.width().max(1.0)).clamp(-0.5, 0.5);
    let pointer_y = ((pointer.y - rect.center().y) / rect.height().max(1.0)).clamp(-0.5, 0.5);

    let center = egui::pos2(
        rect.center().x + pointer_x * 24.0,
        rect.top() + rect.height() * 0.50 + pointer_y * 16.0,
    );

    // Fondo cósmico oscuro liso y limpio (sin ondas ni auras)
    painter.rect_filled(
        rect,
        egui::CornerRadius::ZERO,
        egui::Color32::from_rgb(2, 4, 10),
    );

    // Campo estelar 3D a velocidad constante uniforme
    let warp_speed = 340.0;
    pintar_campo_estelar_3d(painter, center, rect, time, warp_speed, pointer_x, pointer_y);

    // Solicitar redibujado continuo a 60 FPS
    ui.ctx().request_repaint();
}

/// Campo estelar 3D con estelas de luz hacia el observador a velocidad constante
fn pintar_campo_estelar_3d(
    painter: &egui::Painter,
    center: egui::Pos2,
    rect: egui::Rect,
    time: f32,
    speed: f32,
    pointer_x: f32,
    pointer_y: f32,
) {
    let num_stars = 380;
    let max_depth = 1600.0;
    let fov = 460.0;
    let spread = 950.0;

    for i in 0..num_stars {
        let seed = i as f32 * 17.371;
        let base_x = ((seed * 12.9898 + 3.1).sin() * 43_758.547).fract() * (spread * 2.0) - spread;
        let base_y = ((seed * 78.233 + 7.5).sin() * 43_758.547).fract() * (spread * 1.5) - (spread * 0.75);
        let base_z = ((seed * 43.123 + 1.9).sin() * 43_758.547).fract().abs() * max_depth;

        let current_z = (base_z - time * speed).rem_euclid(max_depth) + 25.0;

        let depth_now = current_z;
        let scale_now = fov / depth_now;
        let sx_now = center.x + (base_x + pointer_x * 80.0) * scale_now;
        let sy_now = center.y + (base_y + pointer_y * 60.0) * scale_now;

        let trail_dt = (speed * 0.05 + 18.0) * (1.0 - current_z / max_depth).powf(1.35);
        let depth_prev = current_z + trail_dt;
        let scale_prev = fov / depth_prev;
        let sx_prev = center.x + (base_x + pointer_x * 80.0) * scale_prev;
        let sy_prev = center.y + (base_y + pointer_y * 60.0) * scale_prev;

        let p_now = egui::pos2(sx_now, sy_now);
        let p_prev = egui::pos2(sx_prev, sy_prev);

        if !rect.contains(p_now) && !rect.contains(p_prev) {
            continue;
        }

        let closeness = (1.0 - current_z / max_depth).clamp(0.0, 1.0);
        let alpha = (closeness.powf(0.85) * 225.0 + 15.0) as u8;

        let color = if closeness > 0.82 {
            egui::Color32::from_rgba_unmultiplied(255, 240, 220, alpha)
        } else if closeness > 0.42 {
            egui::Color32::from_rgba_unmultiplied(85, 215, 255, alpha)
        } else {
            egui::Color32::from_rgba_unmultiplied(135, 150, 255, alpha)
        };

        let stroke_w = (closeness * 2.2 + 0.6).min(3.2);
        painter.line_segment([p_prev, p_now], egui::Stroke::new(stroke_w, color));

        if closeness > 0.65 {
            painter.circle_filled(
                p_now,
                (closeness * 1.8).min(2.5),
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
            );
        }
    }
}
