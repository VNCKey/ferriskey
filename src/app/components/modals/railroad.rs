use crate::app::AppState;
use eframe::egui;

fn parse_svg_wh(svg: &str) -> Option<(f32, f32)> {
    // 1. Intentar buscar width="X" height="Y"
    if let (Some(w_pos), Some(h_pos)) = (svg.find("width=\""), svg.find("height=\"")) {
        let w_rest = &svg[w_pos + 7..];
        let h_rest = &svg[h_pos + 8..];
        if let (Some(w_end), Some(h_end)) = (w_rest.find('"'), h_rest.find('"'))
            && let (Ok(w), Ok(h)) = (
                w_rest[..w_end].parse::<f32>(),
                h_rest[..h_end].parse::<f32>(),
            )
            && w > 0.0
            && h > 0.0
        {
            return Some((w, h));
        }
    }

    // 2. Intentar buscar viewBox="min-x min-y width height"
    if let Some(vb_pos) = svg.find("viewBox=\"") {
        let vb_rest = &svg[vb_pos + 9..];
        if let Some(vb_end) = vb_rest.find('"') {
            let parts: Vec<&str> = vb_rest[..vb_end].split_whitespace().collect();
            if parts.len() == 4
                && let (Ok(w), Ok(h)) = (parts[2].parse::<f32>(), parts[3].parse::<f32>())
                && w > 0.0
                && h > 0.0
            {
                return Some((w, h));
            }
        }
    }

    None
}

fn railroad_modal_img_size(svg_bytes: &[u8]) -> egui::Vec2 {
    const TARGET_H: f32 = 96.0;
    const MAX_W: f32 = 1100.0;
    const FALLBACK_ASPECT: f32 = 12.0;

    let s = std::str::from_utf8(svg_bytes).unwrap_or("");
    let (native_w, native_h) = parse_svg_wh(s).unwrap_or((FALLBACK_ASPECT * 60.0, 60.0));
    let aspect = (native_w / native_h.max(1.0)).clamp(0.1, 40.0);

    let mut h = TARGET_H;
    let mut w = h * aspect;
    if w > MAX_W {
        w = MAX_W;
        h = w / aspect;
    }
    egui::vec2(w, h)
}

pub fn mostrar_modal_railroad_let(ctx: &egui::Context, state: &mut AppState) {
    let mode = match state.ui.show_railroad_modal {
        Some(m) => m,
        None => return,
    };

    let is_flowchart = mode >= 4;

    let (titulo, bytes_data, uri) = match mode {
        0 => (
            "Inmutable",
            include_bytes!("../../../../assets/diagramas/diagrama_let_immut.svg").as_slice(),
            "bytes://diagrama_let_immut.svg",
        ),
        1 => (
            "Mutable",
            include_bytes!("../../../../assets/diagramas/diagrama_let_mut.svg").as_slice(),
            "bytes://diagrama_let_mut.svg",
        ),
        2 => (
            "fn main()",
            include_bytes!("../../../../assets/diagramas/diagrama_fn_main.svg").as_slice(),
            "bytes://diagrama_fn_main.svg",
        ),
        3 => (
            "Librería (src/lib.rs)",
            include_bytes!("../../../../assets/diagramas/diagrama_lib.svg").as_slice(),
            "bytes://diagrama_lib.svg",
        ),
        4 => (
            "Compile time",
            include_bytes!("../../../../assets/diagramas/compile_time.svg").as_slice(),
            "bytes://compile_time.svg",
        ),
        5 => (
            "Run time",
            include_bytes!("../../../../assets/diagramas/run_time.svg").as_slice(),
            "bytes://run_time.svg",
        ),
        6 => (
            "Arquitectura Memoria Stack",
            include_bytes!("../../../../assets/diagramas/diagrama_stack.svg").as_slice(),
            "bytes://diagrama_stack.svg",
        ),
        _ => (
            "Arquitectura Memoria Heap",
            include_bytes!("../../../../assets/diagramas/diagrama_heap.svg").as_slice(),
            "bytes://diagrama_heap.svg",
        ),
    };

    if is_flowchart {
        mostrar_modal_flowchart(ctx, state, bytes_data, uri, mode);
    } else {
        let mut abierto = true;
        let mut window_frame = egui::Frame::window(&ctx.style_of(egui::Theme::Dark));
        window_frame.inner_margin = egui::Margin::symmetric(20, 16);
        window_frame.fill = egui::Color32::from_rgb(15, 23, 42);

        let window = egui::Window::new(titulo)
            .open(&mut abierto)
            .collapsible(false)
            .frame(window_frame)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .pivot(egui::Align2::CENTER_CENTER);

        let img_size = railroad_modal_img_size(bytes_data);
        window.resizable(false).show(ctx, |ui| {
            ui.add(
                egui::Image::from_bytes(uri, bytes_data)
                    .fit_to_exact_size(img_size)
                    .maintain_aspect_ratio(true),
            );
        });
        if !abierto {
            state.ui.show_railroad_modal = None;
        }
    }
}

fn mostrar_modal_flowchart(
    ctx: &egui::Context,
    state: &mut AppState,
    bytes_data: &'static [u8],
    uri: &'static str,
    mode: usize,
) {
    let svg_text = std::str::from_utf8(bytes_data).unwrap_or("");
    let (native_w, native_h) = parse_svg_wh(svg_text).unwrap_or((248.0, 704.0));
    let aspect = native_w / native_h.max(1.0);
    let (default_w, default_h) = if aspect >= 1.0 {
        (840.0, 640.0)
    } else {
        (320.0, 680.0)
    };

    egui::Window::new("railroad_flowchart_flotante")
        .title_bar(false)
        .frame(egui::Frame::NONE)
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .default_size([default_w, default_h])
        .min_size([320.0, 300.0])
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            let mut modal_frame = egui::Frame::new();
            modal_frame.fill = egui::Color32::from_rgb(11, 15, 22);
            modal_frame.inner_margin = egui::Margin::same(2);
            modal_frame.corner_radius = egui::CornerRadius::same(4);
            modal_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95));
            modal_frame.shadow = egui::Shadow {
                offset: [0, 8],
                blur: 24,
                spread: 0,
                color: egui::Color32::from_black_alpha(180),
            };

            modal_frame.show(ui, |ui| {
                ui.set_width(ui.available_width());

                // Visor limpio: solo se conserva el cierre flotante sobre el gráfico.
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 24.0),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        let (close_rect, close_resp) =
                            ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                        let hovered = close_resp.hovered();
                        if hovered {
                            ui.painter().rect_filled(
                                close_rect,
                                egui::CornerRadius::same(5),
                                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 22),
                            );
                        }
                        let close_color = if hovered {
                            egui::Color32::from_rgb(255, 140, 140)
                        } else {
                            egui::Color32::from_rgb(160, 175, 195)
                        };
                        egui::Image::new(egui::include_image!(
                            "../../../../assets/icons/close-circle-svgrepo-com.svg"
                        ))
                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                        .tint(close_color)
                        .paint_at(
                            ui,
                            egui::Rect::from_center_size(
                                close_rect.center(),
                                egui::vec2(16.0, 16.0),
                            ),
                        );
                        if close_resp.clicked() {
                            state.ui.show_railroad_modal = None;
                        }
                        close_resp.on_hover_text("Cerrar diagrama");
                    },
                );

                ui.style_mut().spacing.scroll.floating = false;
                ui.style_mut().spacing.scroll.bar_width = 8.0;
                ui.style_mut().spacing.scroll.bar_inner_margin = 0.0;
                ui.style_mut().spacing.scroll.bar_outer_margin = 0.0;
                ui.style_mut().visuals.widgets.inactive.bg_fill =
                    egui::Color32::from_rgb(45, 60, 85);
                ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
                ui.style_mut().visuals.widgets.hovered.bg_fill =
                    egui::Color32::from_rgb(70, 95, 135);
                ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
                ui.style_mut().visuals.widgets.active.bg_fill =
                    egui::Color32::from_rgb(255, 150, 45);
                ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
                ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::ZERO;

                egui::ScrollArea::both()
                    .id_salt(("railroad_flowchart_scroll", mode))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(0.0);
                        // El diagrama conserva aire alrededor y no ocupa toda
                        // la ventana, especialmente en los flowcharts verticales.
                        let max_diagram_width = if aspect < 1.0 { 280.0 } else { 680.0 };
                        let avail_w = (ui.available_width().min(max_diagram_width)).max(230.0);
                        let target_h = avail_w / aspect;
                        ui.vertical_centered(|ui| {
                            ui.add(
                                egui::Image::from_bytes(uri, bytes_data)
                                    .fit_to_exact_size(egui::vec2(avail_w, target_h))
                                    .maintain_aspect_ratio(true),
                            );
                        });
                        ui.add_space(2.0);
                    });
            });
        });
}
