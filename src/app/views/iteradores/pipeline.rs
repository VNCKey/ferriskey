use crate::app::AppState;
use eframe::egui;

#[allow(dead_code)]
pub fn mostrar_simulador_pipeline(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("📊 Simulador de Pipeline Perezoso de Iteradores");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Modo de Iteración:").strong());
        ui.selectable_value(&mut state.lessons.iter_mode, 0, "1. .iter() (&T)");
        ui.selectable_value(&mut state.lessons.iter_mode, 1, "2. .iter_mut() (&mut T)");
        ui.selectable_value(&mut state.lessons.iter_mode, 2, "3. .into_iter() (T)");
        ui.add_space(20.0);
        ui.checkbox(
            &mut state.lessons.iter_filter_even,
            "Filtrar solo Pares (.filter)",
        );
    });

    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 150.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let input_nums = vec![1, 2, 3, 4, 5, 6];
    let y = rect.center().y;

    // 1. Array Origen
    let start_x = rect.left() + 40.0;
    ui.painter().text(
        egui::pos2(start_x + 60.0, y - 45.0),
        egui::Align2::CENTER_CENTER,
        "origen.iter()",
        egui::FontId::proportional(12.0),
        egui::Color32::LIGHT_GRAY,
    );
    for (i, num) in input_nums.iter().enumerate() {
        let box_rect = egui::Rect::from_center_size(
            egui::pos2(start_x + (i as f32 * 25.0), y),
            egui::vec2(22.0, 30.0),
        );
        ui.painter()
            .rect_filled(box_rect, 3.0, egui::Color32::from_rgb(40, 50, 70));
        ui.painter().text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            num.to_string(),
            egui::FontId::proportional(11.0),
            egui::Color32::WHITE,
        );
    }

    // 2. Filtro / Map
    let mid_x = rect.left() + 280.0;
    ui.painter().line_segment(
        [egui::pos2(start_x + 160.0, y), egui::pos2(mid_x - 40.0, y)],
        egui::Stroke::new(2.0, egui::Color32::GRAY),
    );

    let filter_rect = egui::Rect::from_center_size(egui::pos2(mid_x, y), egui::vec2(100.0, 50.0));
    ui.painter()
        .rect_filled(filter_rect, 5.0, egui::Color32::from_rgb(240, 140, 40));
    ui.painter().text(
        filter_rect.center(),
        egui::Align2::CENTER_CENTER,
        "filter & map\n(x * x)",
        egui::FontId::proportional(11.0),
        egui::Color32::BLACK,
    );

    // 3. Resultado .collect()
    let out_x = rect.left() + 480.0;
    ui.painter().line_segment(
        [egui::pos2(mid_x + 60.0, y), egui::pos2(out_x - 40.0, y)],
        egui::Stroke::new(2.0, egui::Color32::GREEN),
    );

    let res_nums: Vec<i32> = input_nums
        .into_iter()
        .filter(|&x| !state.lessons.iter_filter_even || x % 2 == 0)
        .map(|x| x * x)
        .collect();
    ui.painter().text(
        egui::pos2(out_x + (res_nums.len() as f32 * 15.0), y - 45.0),
        egui::Align2::CENTER_CENTER,
        ".collect::<Vec<_>>()",
        egui::FontId::proportional(12.0),
        egui::Color32::GREEN,
    );

    for (i, num) in res_nums.iter().enumerate() {
        let box_rect = egui::Rect::from_center_size(
            egui::pos2(out_x + (i as f32 * 35.0), y),
            egui::vec2(30.0, 32.0),
        );
        ui.painter()
            .rect_filled(box_rect, 4.0, egui::Color32::from_rgb(60, 180, 100));
        ui.painter().text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            num.to_string(),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }
}
