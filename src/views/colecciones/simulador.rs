use crate::app::AppState;
use crate::views::control_flujo::card_frame_tutorial;
use eframe::egui;

pub fn mostrar_simulador_vectores(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Simulador interactivo de reasignación de capacidad de un Vector en la memoria Heap. Experimenta cómo la capacidad se duplica automáticamente cuando la longitud supera el espacio disponible.",
        )
        .color(texto),
    );
    ui.add_space(12.0);

    ui.heading(
        egui::RichText::new("Simulador de Reasignación de Capacidad (Heap)")
            .size(16.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(6.0);

    card_frame_tutorial().show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui
                .button(
                    egui::RichText::new("➕ Push Elemento")
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .clicked()
            {
                state.lessons.vec_sim_len += 1;
                if state.lessons.vec_sim_cap == 0 {
                    state.lessons.vec_sim_cap = 1;
                } else if state.lessons.vec_sim_len > state.lessons.vec_sim_cap {
                    state.lessons.vec_sim_cap *= 2;
                }
            }
            if ui
                .button(
                    egui::RichText::new("➖ Pop Elemento")
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .clicked()
                && state.lessons.vec_sim_len > 0
            {
                state.lessons.vec_sim_len -= 1;
            }
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(format!(
                    "len: {} | cap: {}",
                    state.lessons.vec_sim_len, state.lessons.vec_sim_cap
                ))
                .monospace()
                .strong()
                .color(cyan),
            );
        });

        ui.add_space(10.0);

        let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 80.0));
        ui.painter()
            .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

        let start_x = rect.left() + 20.0;
        let y = rect.center().y;

        for i in 0..state.lessons.vec_sim_cap {
            let box_x = start_x + (i as f32 * 45.0);
            let box_rect =
                egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(38.0, 38.0));
            let filled = i < state.lessons.vec_sim_len;

            ui.painter().rect_filled(
                box_rect,
                4.0,
                if filled {
                    egui::Color32::from_rgb(60, 180, 120)
                } else {
                    egui::Color32::from_rgb(40, 45, 55)
                },
            );
            ui.painter().text(
                box_rect.center(),
                egui::Align2::CENTER_CENTER,
                if filled {
                    format!("[{}]", i)
                } else {
                    "_".to_string()
                },
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
        }
    });
}
