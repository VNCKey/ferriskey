pub mod bucles;
pub mod condicionales;
pub mod info;
pub mod match_expr;

use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::conceptos::mostrar_selector_proyectos_estandar_con_archivos;
use eframe::egui;

pub fn mostrar_tutorial_control_flujo(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Control de Flujo")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Barra de navegación de pestañas (Estilo unificado con SVG)
    ui.horizontal(|ui| {
        ui.add_space(10.0);

        // --- LADO IZQUIERDO: Teoría ---
        let img_book =
            egui::Image::new(egui::include_image!("../../../assets/icons/book-line.svg"))
                .fit_to_exact_size(egui::Vec2::new(24.0, 24.0));
        ui.add(img_book);

        let tabs_teoria = [
            (0, "Condicionales"),
            (1, "Bucles"),
            (2, "Match"),
            (3, "Info"),
        ];
        for (indice, label) in tabs_teoria {
            let activo =
                !state.lessons.controlflujo_is_practica && state.lessons.controlflujo_tab == indice;
            if ui.selectable_label(activo, label).clicked() {
                state.lessons.controlflujo_is_practica = false;
                state.lessons.controlflujo_tab = indice;
            }
        }

        // --- LADO DERECHO: Práctica ---
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(10.0);

            let activo = state.lessons.controlflujo_is_practica;
            if ui.selectable_label(activo, "Code Lab").clicked() {
                state.lessons.controlflujo_is_practica = true;
            }

            let img_code = egui::Image::new(egui::include_image!(
                "../../../assets/icons/monitor-code-line.svg"
            ))
            .fit_to_exact_size(egui::Vec2::new(24.0, 24.0));
            ui.add(img_code);
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(10.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state.lessons.controlflujo_is_practica {
                // MODO PRÁCTICA (Solo Editor)
                ui.label(egui::RichText::new("Modo Práctica (Code Lab)").strong().size(18.0).color(cyan));
                ui.label(egui::RichText::new("Concéntrate en el código. Escribe, compila y experimenta sin distracciones visuales.").color(texto));
                ui.add_space(15.0);

                let code_target = if state.project.selected_project.is_some() {
                    &mut state.project.shared_project_code
                } else {
                    &mut state.lessons.controlflujo_code
                };

                mostrar_selector_proyectos_estandar_con_archivos(
                    ui,
                    &mut state.project.selected_project,
                    &mut state.project.selected_file,
                    &mut state.terminal.term_cwd,
                    "combo_proyectos_control_flujo",
                    code_target,
                );

                ui.add_space(10.0);

                let syntax_set = state.editor.syntax_set.clone();
                let theme = state.editor.theme_set.themes["base16-ocean.dark"].clone();
                let (code_ref, output_arc) = state.obtener_editor_activo_mut();
                mostrar_editor_interactivo(
                    ui,
                    code_ref,
                    output_arc,
                    "",
                    ejecutar_codigo_rust,
                    &syntax_set,
                    &theme,
                );

                ui.add_space(30.0);
            } else {
                // MODO TEORÍA (Contenido según la pestaña seleccionada)
                match state.lessons.controlflujo_tab {
                    0 => condicionales::mostrar_tab_condicionales(ui, state),
                    1 => bucles::mostrar_tab_bucles(ui, state),
                    2 => match_expr::mostrar_tab_match(ui, state),
                    _ => info::mostrar_control_flujo_info(ui, state, naranja, cyan, texto),
                }
            }
        });
}

pub fn card_frame_tutorial() -> egui::Frame {
    let mut f = egui::Frame::new();
    f.fill = egui::Color32::from_rgb(14, 18, 26);
    f.inner_margin = egui::Margin::same(12);
    f.corner_radius = egui::CornerRadius::same(8);
    f.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));
    f
}
