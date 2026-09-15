use crate::app::AppState;
use crate::views::conceptos::mostrar_componente_terminal_3_modos;
use eframe::egui;

pub fn mostrar_modal_terminal(ctx: &egui::Context, state: &mut AppState) {
    if !state.ui.show_terminal_modal {
        return;
    }

    egui::Window::new("terminal_flotante_moderna")
        .title_bar(false)
        .frame(egui::Frame::NONE)
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -18.0))
        .default_size([820.0, 360.0])
        .min_size([620.0, 270.0])
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            mostrar_componente_terminal_3_modos(ui, "cargo run", state);
        });
}
