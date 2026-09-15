use crate::app::AppState;
use crate::components::navigation::nav_item_sidebar;
use crate::routes::AppRoute;
use eframe::egui;
use std::sync::atomic::Ordering;

pub fn mostrar_sidebar(ui: &mut egui::Ui, state: &mut AppState) {
    let mut is_expanded = state.ui.mostrar_sidebar;

    let color_sidebar = egui::Color32::from_rgb(13, 15, 19); // Aún más oscuro para dar profundidad

    egui::Panel::left("sidebar")
        .frame(egui::Frame::default().fill(color_sidebar).inner_margin(4.0))
        .resizable(false)
        .show_collapsible(ui, &mut is_expanded, |ui| {
            ui.set_min_width(220.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    let mut job = egui::text::LayoutJob::default();
                    job.append(
                        "Ferris",
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(24.0, egui::FontFamily::Proportional),
                            color: egui::Color32::from_rgb(255, 160, 50),
                            ..Default::default()
                        },
                    );
                    job.append(
                        "Key",
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(24.0, egui::FontFamily::Proportional),
                            color: egui::Color32::WHITE,
                            ..Default::default()
                        },
                    );

                    let logo_response = ui
                        .add(egui::Label::new(job).sense(egui::Sense::click()))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .on_hover_text("Volver a la pantalla de inicio (FerrisKey)");

                    if logo_response.clicked() {
                        state.ui.ruta_actual = AppRoute::LandingPage;
                    }

                    ui.label(
                        egui::RichText::new("Aprende Rust Jugando")
                            .size(12.0)
                            .italics()
                            .color(egui::Color32::GRAY),
                    );
                });

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(15.0);

                ui.label(
                    egui::RichText::new("CURSO RUST COMPLETO")
                        .strong()
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(10.0);

                // 1. Rust Foundations
                if nav_item_sidebar(
                    ui,
                    "Rust Foundations",
                    state.ui.ruta_actual == AppRoute::TutorialCargo,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialCargo;
                    state.ui.anim_trigger = ui.input(|i| i.time);
                }
                ui.add_space(2.0);

                // 2. Conceptos
                if nav_item_sidebar(
                    ui,
                    "Conceptos",
                    state.ui.ruta_actual == AppRoute::Comenzando,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::Comenzando;
                    state.ui.anim_trigger = ui.input(|i| i.time);
                }
                ui.add_space(2.0);

                // 3. Memoria (reglas de memoria + String/&str)
                let es_memoria = state.ui.ruta_actual == AppRoute::TutorialOwnership
                    || state.ui.ruta_actual == AppRoute::TutorialStrings;
                if nav_item_sidebar(ui, "Memoria", es_memoria).clicked() {
                    state.ui.ruta_actual = AppRoute::TutorialOwnership;
                }
                ui.add_space(2.0);

                // 4. Módulos
                if nav_item_sidebar(
                    ui,
                    "Módulos",
                    state.ui.ruta_actual == AppRoute::TutorialModulos,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialModulos;
                }
                ui.add_space(2.0);

                // 5. Tipos Compuestos
                if nav_item_sidebar(
                    ui,
                    "Tipos Compuestos",
                    state.ui.ruta_actual == AppRoute::TutorialTiposDatos,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialTiposDatos;
                }
                ui.add_space(2.0);

                // 6. Control de Flujo
                if nav_item_sidebar(
                    ui,
                    "Control de Flujo",
                    state.ui.ruta_actual == AppRoute::TutorialControlFlujo,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialControlFlujo;
                }
                ui.add_space(2.0);

                // 7. Funciones & Closures
                if nav_item_sidebar(
                    ui,
                    "Closures",
                    state.ui.ruta_actual == AppRoute::TutorialFunciones,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialFunciones;
                }
                ui.add_space(2.0);

                // 8. Iteradores
                if nav_item_sidebar(
                    ui,
                    "Iteradores",
                    state.ui.ruta_actual == AppRoute::TutorialIteradores,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialIteradores;
                }
                ui.add_space(2.0);

                // 9. Structs & impl
                if nav_item_sidebar(
                    ui,
                    "Custom Types",
                    state.ui.ruta_actual == AppRoute::TutorialStructs,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialStructs;
                }
                ui.add_space(2.0);

                // 10. Error Handling
                if nav_item_sidebar(
                    ui,
                    "Error Handling",
                    state.ui.ruta_actual == AppRoute::TutorialEnums,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialEnums;
                }
                ui.add_space(2.0);

                // 11. Generics
                if nav_item_sidebar(
                    ui,
                    "Generics",
                    state.ui.ruta_actual == AppRoute::TutorialGenericos,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialGenericos;
                }
                ui.add_space(2.0);

                // 13. Traits & Genéricos
                if nav_item_sidebar(
                    ui,
                    "Traits",
                    state.ui.ruta_actual == AppRoute::TutorialTraits,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::TutorialTraits;
                }
                ui.add_space(2.0);

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("LIB")
                        .strong()
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(8.0);

                if nav_item_sidebar(
                    ui,
                    "Tipos y métodos",
                    state.ui.ruta_actual == AppRoute::LibTiposDatos,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::LibTiposDatos;
                }

                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new("PROYECTOS TÉCNICOS")
                        .strong()
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(8.0);
                if nav_item_sidebar(
                    ui,
                    "Visualización de Datos",
                    state.ui.ruta_actual == AppRoute::DashboardGraficos,
                )
                .clicked()
                {
                    state.ui.ruta_actual = AppRoute::DashboardGraficos;
                }

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                // Botones de Utilidades: Salida/Logs, Terminal y Configuración
                ui.horizontal(|ui| {
                    let is_cargo_open = state.ui.show_cargo_output_modal.load(Ordering::Relaxed);
                    let cargo_text_color = if is_cargo_open {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .button(egui::RichText::new("ℹ️").size(18.0).color(cargo_text_color))
                        .on_hover_text("Información / Salida de compilación y macros")
                        .clicked()
                    {
                        state
                            .ui
                            .show_cargo_output_modal
                            .store(!is_cargo_open, Ordering::Relaxed);
                    }

                    ui.add_space(8.0);

                    let term_text_color = if state.ui.show_terminal_modal {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .button(egui::RichText::new("💻").size(18.0).color(term_text_color))
                        .on_hover_text("Terminal del sistema interactiva")
                        .clicked()
                    {
                        let abrir_terminal = !state.ui.show_terminal_modal;
                        state.ui.show_terminal_modal = abrir_terminal;
                    }

                    ui.add_space(8.0);

                    let config_icon_color = if state.ui.show_settings_modal {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    let config_img = egui::Image::from_bytes(
                        "bytes://config.svg",
                        include_bytes!("../../../assets/diagramas/config.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(20.0, 20.0))
                    .tint(config_icon_color);

                    if ui
                        .add(egui::Button::image(config_img))
                        .on_hover_text("Configuración y Atajos de Teclado")
                        .clicked()
                    {
                        state.ui.show_settings_modal = !state.ui.show_settings_modal;
                    }
                });
            });
        });

    state.ui.mostrar_sidebar = is_expanded;
}
