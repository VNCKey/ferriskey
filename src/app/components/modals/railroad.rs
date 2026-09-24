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
    let mut is_open = state.ui.show_railroad_modal.is_some();
    let dimensions = match mode {
        6 | 7 => Some([860.0, 520.0]),
        _ => None,
    };
    crate::app::ui::mostrar_modal_diagrama(
        ctx,
        &mut is_open,
        "railroad_flowchart_flotante",
        uri,
        bytes_data,
        dimensions,
    );
    if !is_open {
        state.ui.show_railroad_modal = None;
    }
}
