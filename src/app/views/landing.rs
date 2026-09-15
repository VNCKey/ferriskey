use crate::app::AppState;
use crate::routes::AppRoute;
use eframe::egui;

#[derive(Clone, Copy)]
struct OrbitStyle {
    color: egui::Color32,
    alpha: u8,
    width: f32,
}

pub fn mostrar_landing_page(ui: &mut egui::Ui, state: &mut AppState) {
    let avail_w = ui.available_width();
    let avail_h = ui.available_height();

    ui.allocate_ui(egui::vec2(avail_w, avail_h), |ui| {
        pintar_fondo_hero(ui, ui.max_rect());

        // --- HEADER SUPERIOR FLOTANTE CON BORDER BOTTOM ---

        let header_frame = egui::Frame::new()
            // .fill(egui::Color32::BLACK)
            .inner_margin(egui::Margin::symmetric(20, 15)) // Padding interno
            .outer_margin(egui::Margin { left: 100, right: 100, top: 10, bottom: 0 }) // Márgenes exteriores
            .stroke(egui::Stroke::NONE); // Sin borde nativo

        let response = header_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width()); // Estirar a todo el ancho disponible
            ui.horizontal(|ui| {
                // Logo izquierdo
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(egui::RichText::new("Ferris").size(22.0).strong().color(egui::Color32::from_rgb(255, 160, 50)));
                    ui.label(egui::RichText::new("Key").size(22.0).strong().color(egui::Color32::WHITE));
                });

                // Menú derecho (Comunidad)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let estilo_previo = (**ui.style()).clone();
                    let estilo_mut = ui.style_mut();
                    estilo_mut.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    estilo_mut.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                    estilo_mut.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;

                    estilo_mut.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(25, 30, 40);
                    estilo_mut.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 160, 50));

                    estilo_mut.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(40, 45, 55);

                    ui.menu_button(egui::RichText::new("Comunidad").size(16.0).color(egui::Color32::from_rgb(200, 210, 225)), |ui| {
                        *ui.style_mut() = estilo_previo;
                        ui.set_min_width(120.0);
                        if ui.button("Discord").clicked() { ui.ctx().open_url(egui::OpenUrl::new_tab("https://discord.com")); }
                        if ui.button("Telegram").clicked() { ui.ctx().open_url(egui::OpenUrl::new_tab("https://telegram.org")); }
                        if ui.button("Página Web").clicked() { ui.ctx().open_url(egui::OpenUrl::new_tab("https://ferriskey.com")); }
                    });
                });
            });
        });

        // HACK DEL BORDER-BOTTOM EN EGUI
        let rect = response.response.rect;
        let start = egui::pos2(rect.left() + 100.0, rect.bottom());
        let end = egui::pos2(rect.right() - 100.0, rect.bottom());

        ui.painter().line_segment(
            [start, end],
            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 160, 50))
        );

        ui.add_space(15.0);
        // ------------------------

        // --- CUERPO PRINCIPAL CON MÁRGENES LATERALES ---
        let max_texto_w = (avail_w * 0.45).clamp(300.0, 700.0);

        let cuerpo_frame = egui::Frame::new()
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE);

        cuerpo_frame.show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // Propósito / Título grande en bloque de ancho limitado
                ui.add_sized(
                    [max_texto_w, 0.0],
                    egui::Label::new(
                        egui::RichText::new("Diviértete conociendo y aprendiendo Rust")
                            .size((avail_w * 0.025).clamp(24.0, 36.0))
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    )
                    .wrap()
                );

                ui.add_space(10.0);

                // Descripción de propósito en bloque de ancho limitado
                ui.add_sized(
                    [max_texto_w, 0.0],
                    egui::Label::new(
                        egui::RichText::new("FerrisKey te guiará paso a paso desde los conceptos más básicos hasta los más avanzados, mediante explicaciones interactivas, ejemplos visuales y desafíos de código prácticos.")
                            .size((avail_w * 0.013).clamp(14.0, 18.0))
                            .color(egui::Color32::from_rgb(180, 200, 230))
                            .line_height(Some(22.0)),
                    )
                    .wrap()
                );

                ui.add_space(20.0);

                /*
                // Imagen de la interfaz / Ilustración
                let max_img_w = (avail_w * 0.50).clamp(280.0, 680.0);
                let max_img_h = (avail_h * 0.40).clamp(160.0, 420.0);

                ui.add(
                    egui::Image::new(egui::include_image!("../../../assets/taller/home2.png"))
                        .max_width(max_img_w)
                        .max_height(max_img_h)
                        .corner_radius(egui::CornerRadius::same(16)),
                );
                */

                ui.add_space(20.0);

                // Botón de acción principal
                let btn_w = (avail_w * 0.20).clamp(180.0, 280.0);
                let btn_h = (avail_h * 0.06).clamp(42.0, 54.0);

                let btn_comenzar = ui.add_sized(
                    [btn_w, btn_h],
                    egui::Button::new(
                        egui::RichText::new("COMENZAR")
                            .size((btn_h * 0.40).clamp(16.0, 22.0))
                            .strong()
                            .color(egui::Color32::BLACK),
                    )
                    .fill(egui::Color32::from_rgb(255, 180, 50))
                    .corner_radius(egui::CornerRadius::same((btn_h * 0.5) as u8)),
                );

                if btn_comenzar.clicked() {
                    state.ui.ruta_actual = AppRoute::TutorialCargo;
                    state.ui.anim_trigger = ui.input(|i| i.time);
                }
            });
        });
    });
}

/// Fondo animado de la landing: una galaxia abstracta construida con
/// espirales logarítmicas aproximadas, trazas de polvo y una lente central.
fn pintar_fondo_hero(ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    let time = ui.input(|i| i.time) as f32;
    let pointer = ui.input(|i| i.pointer.hover_pos()).unwrap_or(rect.center());

    let pointer_x = ((pointer.x - rect.center().x) / rect.width().max(1.0)).clamp(-0.5, 0.5);
    let pointer_y = ((pointer.y - rect.center().y) / rect.height().max(1.0)).clamp(-0.5, 0.5);
    let scale = (rect.width().min(rect.height()) / 560.0).clamp(0.70, 1.25);
    let ice = egui::Color32::from_rgb(220, 235, 255);

    painter.rect_filled(
        rect,
        egui::CornerRadius::ZERO,
        egui::Color32::from_rgb(5, 8, 15),
    );

    // Estrellas en forma de pequeñas trazas lineales.
    for star in 0..170 {
        let seed = star as f32;
        let x = ((seed * 12.9898 + 4.1414).sin() * 43_758.547).fract().abs();
        let y = ((seed * 78.233 + 9.713).sin() * 43_758.547).fract().abs();
        let shimmer = (time * (0.42 + (seed % 4.0) * 0.12) + seed).sin() * 0.5 + 0.5;
        let position = egui::pos2(
            rect.left() + x * rect.width(),
            rect.top() + y * rect.height(),
        );
        let length = (1.4 + (seed % 3.0) * 0.9) * scale;
        painter.line_segment(
            [position, position + egui::vec2(length, length * 0.22)],
            egui::Stroke::new(
                (0.35 + shimmer * 0.3) * scale,
                egui::Color32::from_rgba_unmultiplied(
                    ice.r(),
                    ice.g(),
                    ice.b(),
                    (8.0 + shimmer * 26.0) as u8,
                ),
            ),
        );
    }

    // El cristal queda centrado en el espacio visual inferior, dejando aire
    // para el título y la descripción de la landing.
    let center = egui::pos2(
        rect.center().x + pointer_x * 13.0,
        rect.top() + rect.height() * 0.61 + pointer_y * 9.0,
    );
    pintar_galaxia(painter, rect, center, time, pointer_x, pointer_y);
    ui.ctx().request_repaint();
}

/// Galaxia de fondo basada en una espiral logarítmica aproximada. Se dibuja
/// antes del cristal para que la malla siga siendo el foco de la escena.
fn pintar_galaxia(
    painter: &egui::Painter,
    rect: egui::Rect,
    center: egui::Pos2,
    time: f32,
    pointer_x: f32,
    pointer_y: f32,
) {
    let min_dimension = rect.width().min(rect.height()).max(1.0);
    let outer_radius = (min_dimension * 0.48).clamp(220.0, 440.0);
    let tilt = 0.24 + pointer_y * 0.05;
    let drift = time * 0.018 + pointer_x * 0.08;
    let cyan = egui::Color32::from_rgb(93, 198, 255);
    let violet = egui::Color32::from_rgb(147, 126, 255);
    let ice = egui::Color32::from_rgb(212, 235, 255);

    // El disco galáctico: elipses muy finas que sugieren una lente de polvo
    // sin convertir el fondo en una colección de círculos decorativos.
    for ring in 0..5 {
        let progress = ring as f32 / 4.0;
        let radius_x = (34.0 + progress * outer_radius * 0.86).max(1.0);
        let radius_y = radius_x * tilt * (0.86 + progress * 0.10);
        let color = if ring % 2 == 0 { cyan } else { violet };
        let alpha = (12.0 - progress * 5.0) as u8;
        pintar_orbita_eliptica(
            painter,
            center,
            radius_x,
            radius_y,
            drift * 0.35,
            OrbitStyle {
                color,
                alpha,
                width: 2.2,
            },
        );
    }

    // Tres brazos espirales. Cada brazo tiene un resplandor ancho, una línea
    // de polvo intermedia y un filamento central más nítido.
    for arm in 0..3 {
        let arm_offset = arm as f32 * std::f32::consts::TAU / 3.0;
        let mut previous = None;
        let mut previous_dust = None;

        for step in 0..86 {
            let progress = step as f32 / 85.0;
            let radius = (20.0 + progress.powf(0.82) * outer_radius) * (0.92 + tilt * 0.18);
            let angle = arm_offset + progress * std::f32::consts::TAU * 1.55 + drift;
            let position = center + egui::vec2(angle.cos() * radius, angle.sin() * radius * tilt);
            let dust_angle = angle + 0.035 * (step as f32 * 1.7 + time).sin();
            let dust_radius = radius * (0.985 + 0.035 * (step as f32 * 0.6).sin());
            let dust_position = center
                + egui::vec2(
                    dust_angle.cos() * dust_radius,
                    dust_angle.sin() * dust_radius * tilt,
                );

            if let Some(previous_position) = previous {
                let fade = (1.0 - progress * 0.48).clamp(0.0, 1.0);
                let glow = egui::Color32::from_rgba_unmultiplied(
                    cyan.r(),
                    cyan.g(),
                    cyan.b(),
                    (5.0 + fade * 10.0) as u8,
                );
                painter.line_segment([previous_position, position], egui::Stroke::new(8.0, glow));

                let core_color = if arm == 1 { violet } else { cyan };
                painter.line_segment(
                    [previous_position, position],
                    egui::Stroke::new(
                        1.8,
                        egui::Color32::from_rgba_unmultiplied(
                            core_color.r(),
                            core_color.g(),
                            core_color.b(),
                            (10.0 + fade * 29.0) as u8,
                        ),
                    ),
                );
            }

            if let Some(previous_dust_position) = previous_dust {
                let dust_alpha = (8.0 + (1.0 - progress) * 18.0) as u8;
                painter.line_segment(
                    [previous_dust_position, dust_position],
                    egui::Stroke::new(
                        0.65,
                        egui::Color32::from_rgba_unmultiplied(
                            ice.r(),
                            ice.g(),
                            ice.b(),
                            dust_alpha,
                        ),
                    ),
                );
            }

            previous = Some(position);
            previous_dust = Some(dust_position);
        }
    }

    // Rayos finos cerca del núcleo: dan sensación de lente gravitacional y
    // conectan visualmente la galaxia con el centro del cristal.
    for ray in 0..24 {
        let angle = ray as f32 * std::f32::consts::TAU / 24.0 + drift * 0.7;
        let inner = 18.0 + (ray % 4) as f32 * 3.0;
        let outer = 54.0 + (ray % 5) as f32 * 10.0;
        let start = center + egui::vec2(angle.cos() * inner, angle.sin() * inner * tilt);
        let end = center + egui::vec2(angle.cos() * outer, angle.sin() * outer * tilt);
        painter.line_segment(
            [start, end],
            egui::Stroke::new(
                0.55,
                egui::Color32::from_rgba_unmultiplied(ice.r(), ice.g(), ice.b(), 22),
            ),
        );
    }
}

fn pintar_orbita_eliptica(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    style: OrbitStyle,
) {
    let steps = 72;
    let mut previous = None;
    let cos_rotation = rotation.cos();
    let sin_rotation = rotation.sin();

    for step in 0..=steps {
        let angle = step as f32 * std::f32::consts::TAU / steps as f32;
        let x = angle.cos() * radius_x;
        let y = angle.sin() * radius_y;
        let position = center
            + egui::vec2(
                x * cos_rotation - y * sin_rotation,
                x * sin_rotation + y * cos_rotation,
            );

        if let Some(previous_position) = previous {
            painter.line_segment(
                [previous_position, position],
                egui::Stroke::new(
                    style.width,
                    egui::Color32::from_rgba_unmultiplied(
                        style.color.r(),
                        style.color.g(),
                        style.color.b(),
                        style.alpha,
                    ),
                ),
            );
        }
        previous = Some(position);
    }
}
