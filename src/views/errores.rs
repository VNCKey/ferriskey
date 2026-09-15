use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use eframe::egui;
use std::sync::Arc;

pub fn mostrar_tutorial_errores(ui: &mut egui::Ui, state: &mut AppState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("🚨 Lección 10: Manejo de Errores & Operador ?")
                .size(30.0)
                .strong()
                .color(egui::Color32::from_rgb(240, 90, 90)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("Gestión explícita de fallos sin excepciones irrecuperables")
                .size(15.0)
                .italics(),
        );
    });

    ui.add_space(25.0);

    let mut concept_frame = egui::Frame::new();
    concept_frame.fill = egui::Color32::from_rgb(22, 24, 32);
    concept_frame.inner_margin = egui::Margin::same(15);
    concept_frame.corner_radius = egui::CornerRadius::same(8);
    concept_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 50, 65));
    concept_frame.show(ui, |ui| {
        ui.heading("💡 Excepciones vs Result en Rust");
        ui.add_space(8.0);
        ui.label("• `panic!`: Para condiciones catastróficas e irrecuperables (ej. desbordamiento de índice en array).");
        ui.label("• `Result<T, E>`: Para fallos esperados (ej. archivo no encontrado, error de red).");
        ui.label("• El Operador `?`: Retorna `Err` inmediatamente si la función interna falla, o desenvuelve `Ok(v)`.");
    });

    ui.add_space(20.0);

    ui.heading("📊 Simulador de Tubería de Propagación de Errores (?)");
    ui.add_space(10.0);

    ui.checkbox(
        &mut state.lessons.err_pipeline_fail,
        "Simular fallo en la función interna (dividir por cero)",
    );

    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 160.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let f1_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 100.0, rect.center().y),
        egui::vec2(120.0, 50.0),
    );
    let f2_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 300.0, rect.center().y),
        egui::vec2(120.0, 50.0),
    );
    let f3_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 500.0, rect.center().y),
        egui::vec2(120.0, 50.0),
    );

    ui.painter()
        .rect_filled(f1_rect, 6.0, egui::Color32::from_rgb(40, 50, 70));
    ui.painter().text(
        f1_rect.center(),
        egui::Align2::CENTER_CENTER,
        "dividir(a, b)",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );

    ui.painter()
        .rect_filled(f2_rect, 6.0, egui::Color32::from_rgb(40, 50, 70));
    ui.painter().text(
        f2_rect.center(),
        egui::Align2::CENTER_CENTER,
        "calcular() ?",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );

    ui.painter()
        .rect_filled(f3_rect, 6.0, egui::Color32::from_rgb(40, 50, 70));
    ui.painter().text(
        f3_rect.center(),
        egui::Align2::CENTER_CENTER,
        "main()",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );

    if state.lessons.err_pipeline_fail {
        ui.painter().line_segment(
            [f1_rect.right_center(), f2_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::RED),
        );
        ui.painter().line_segment(
            [f2_rect.right_center(), f3_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::RED),
        );
        ui.painter().text(
            rect.center() + egui::vec2(0.0, 50.0),
            egui::Align2::CENTER_CENTER,
            "❌ Err(\"No se puede dividir entre cero\") propogado hasta main()",
            egui::FontId::proportional(13.0),
            egui::Color32::LIGHT_RED,
        );
    } else {
        ui.painter().line_segment(
            [f1_rect.right_center(), f2_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::GREEN),
        );
        ui.painter().line_segment(
            [f2_rect.right_center(), f3_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::GREEN),
        );
        ui.painter().text(
            rect.center() + egui::vec2(0.0, 50.0),
            egui::Align2::CENTER_CENTER,
            "✅ Ok(10.0) retornado exitosamente",
            egui::FontId::proportional(13.0),
            egui::Color32::GREEN,
        );
    }

    ui.add_space(20.0);

    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.lessons.errores_code,
        Arc::clone(&state.lessons.errores_output),
        "▶ Ejecutar Manejo de Errores",
        ejecutar_codigo_rust,
        &state.editor.syntax_set,
        theme,
    );
}
