use crate::app::AppState;
use eframe::egui;

pub fn mostrar_modal_comparacion_compiladores(ctx: &egui::Context, state: &mut AppState) {
    if !state.ui.show_rustc_compilador_modal {
        return;
    }

    let mut open = true;
    let mut window_frame = egui::Frame::window(&ctx.style_of(egui::Theme::Dark));
    window_frame.inner_margin = egui::Margin::symmetric(20, 16);
    window_frame.fill = egui::Color32::from_rgb(15, 23, 42);

    egui::Window::new("Pipeline de Compilación de rustc")
        .open(&mut open)
        .frame(window_frame)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(true)
        .default_size([380.0, 720.0])
        .min_width(360.0)
        .min_height(300.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(4.0);

                // Diagrama SVG con texto nativo estándar compatible con egui
                // Proporción actual del viewBox de compilacion_rustc.svg.
                let aspect_ratio = 444.0 / 2789.0;
                let available_width = (ui.available_width() - 12.0).clamp(1.0, 760.0);
                let image_height = available_width / aspect_ratio;

                let img = egui::Image::from_bytes(
                    "bytes://compilacion_rustc.svg",
                    include_bytes!("../../../assets/diagramas/compilacion_rustc.svg"),
                )
                .fit_to_exact_size(egui::vec2(available_width, image_height))
                .maintain_aspect_ratio(true);

                ui.vertical_centered(|ui| {
                    ui.add(img);
                });
                ui.add_space(4.0);
            });
        });

    if !open {
        state.ui.show_rustc_compilador_modal = false;
    }
}
