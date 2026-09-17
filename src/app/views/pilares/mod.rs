pub mod anatomy;
pub mod crates_io;
pub mod entorno;
pub mod estructura_tiempos;
pub mod foundations;
pub mod pipeline;

use crate::app::AppState;
use crate::components::navigation::{
    separador_vertical_centrado, underline_tab, underline_tab_destacado,
};
use eframe::egui;

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let mut is_expanded = state.ui.mostrar_nav_superior;

    let color_header = egui::Color32::from_rgb(13, 15, 19);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let orange = egui::Color32::from_rgb(255, 160, 50);

    egui::Panel::top("app_nav_top_header")
        .frame(egui::Frame::default().fill(color_header).inner_margin(4.0))
        .resizable(false)
        .show_collapsible(ui, &mut is_expanded, |ui| {
            ui.add_space(2.0);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 30.0),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.add_space(5.0);

                    // --- LADO IZQUIERDO: Título y Teoría (Centrado vertical flex) ---
                    ui.label(
                        egui::RichText::new("Rust Foundations")
                            .size(14.5)
                            .strong()
                            .color(orange),
                    );

                    separador_vertical_centrado(ui, 14.0);

                    let img_book = egui::Image::new(egui::include_image!(
                        "../../../../assets/icons/book-line.svg"
                    ))
                    .fit_to_exact_size(egui::Vec2::new(18.0, 18.0));
                    let (_id, book_rect) = ui.allocate_space(egui::Vec2::new(18.0, 18.0));
                    img_book.paint_at(ui, book_rect);
                    ui.add_space(4.0);

                    let tabs_teoria = [(0, "Rust Ecosystem"), (2, "Crates.io Explorer")];
                    for (indice, texto) in tabs_teoria {
                        let es_activo = state.lessons.pilares_step == indice;
                        if underline_tab(ui, texto, es_activo, cyan).clicked() {
                            state.lessons.pilares_step = indice;
                            state.ui.anim_trigger = ui.input(|i| i.time); // Reiniciar animaciones al cambiar de pestaña
                        }
                    }

                    // --- LADO DERECHO: Práctica ---
                    ui.scope_builder(
                        egui::UiBuilder::new()
                            .id(egui::Id::new("nav_right_section"))
                            .layout(egui::Layout::right_to_left(egui::Align::Center)),
                        |ui| {
                            ui.add_space(5.0);

                            let es_code_lab_activo = state.lessons.pilares_step == 1;
                            let tabs_practica = [(1, "Code Lab")];
                            // Iteramos al revés para que se dibujen correctamente de derecha a izquierda
                            for (indice, texto) in tabs_practica.iter().rev() {
                                let es_activo = state.lessons.pilares_step == *indice;
                                // Destacamos con baliza pulsante e intuición visual hacia Code Lab cuando no está seleccionado
                                if underline_tab_destacado(ui, texto, es_activo, orange, true)
                                    .clicked()
                                {
                                    state.lessons.pilares_step = *indice;
                                }
                            }

                            let img_code = egui::Image::new(egui::include_image!(
                                "../../../../assets/icons/monitor-code-line.svg"
                            ))
                            .fit_to_exact_size(egui::Vec2::new(18.0, 18.0));
                            let (_id, code_rect) = ui.allocate_space(egui::Vec2::new(18.0, 18.0));

                            let time = ui.input(|i| i.time);
                            let pulse = ((time * 1.8).sin() * 0.5 + 0.5) as f32;
                            let tint = if es_code_lab_activo {
                                orange
                            } else {
                                egui::Color32::from_rgb(
                                    (205.0 + 35.0 * pulse) as u8,
                                    (140.0 + 25.0 * pulse) as u8,
                                    (65.0 + 20.0 * pulse) as u8,
                                )
                            };
                            img_code.tint(tint).paint_at(ui, code_rect);

                            separador_vertical_centrado(ui, 14.0);
                        },
                    );
                },
            );
            ui.add_space(2.0);
        });

    state.ui.mostrar_nav_superior = is_expanded;
}

pub fn mostrar_tutorial_cargo(ui: &mut egui::Ui, state: &mut AppState) {
    // El encabezado y tabs se movieron al panel derecho global

    match state.lessons.pilares_step {
        0 => foundations::mostrar_ecosystem(ui, state),
        1 => anatomy::mostrar_anatomia_cargo(ui, state),
        2 => crates_io::mostrar_tab_crates_io_web(ui, state),
        _ => foundations::mostrar_ecosystem(ui, state),
    }
}
