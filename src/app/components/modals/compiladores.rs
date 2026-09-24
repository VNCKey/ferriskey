use crate::app::AppState;
use eframe::egui;

pub fn mostrar_modal_comparacion_compiladores(ctx: &egui::Context, state: &mut AppState) {
    crate::app::ui::mostrar_modal_diagrama(
        ctx,
        &mut state.ui.show_rustc_compilador_modal,
        "modal_pipeline_rustc",
        "bytes://compilacion_rustc.svg",
        include_bytes!("../../../../assets/diagramas/compilacion_rustc.svg"),
        Some([480.0, 780.0]),
    );
}
