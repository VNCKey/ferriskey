use crate::app::AppState;
use crate::components::navigation::nav_item_sidebar;
use crate::routes::AppRoute;
use eframe::egui;
use std::sync::atomic::Ordering;

pub fn mostrar_sidebar(ui: &mut egui::Ui, state: &mut AppState) {
    let mut is_expanded = state.ui.mostrar_sidebar;

    let color_sidebar = egui::Color32::from_rgb(13, 15, 19); // Aún más oscuro para dar profundidad

    let sidebar_response = egui::Panel::left("sidebar")
        .frame(
            egui::Frame::default()
                .fill(color_sidebar)
                .inner_margin(4.0)
                .stroke(egui::Stroke::NONE),
        )
        .resizable(false)
        .show_collapsible(ui, &mut is_expanded, |ui| {
            ui.set_min_width(220.0);
            ui.vertical(|ui| {
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

                let utility_area_height = 68.0;
                let sessions_area_height = (ui.available_height() - utility_area_height).max(0.0);
                ui.allocate_ui(
                    egui::vec2(ui.available_width(), sessions_area_height),
                    |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("sidebar_course_sessions_scroll")
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                            )
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
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
                                let es_memoria = state.ui.ruta_actual
                                    == AppRoute::TutorialOwnership
                                    || state.ui.ruta_actual == AppRoute::TutorialStrings;
                                if nav_item_sidebar(ui, "Memory & Ownership", es_memoria).clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialOwnership;
                                }
                                ui.add_space(2.0);

                                // 4. Modules & Visibility
                                if nav_item_sidebar(
                                    ui,
                                    "Modules & Visibility",
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
                                    "Functions & Closures",
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
                                    state.ui.ruta_actual == AppRoute::TutorialErrores,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialErrores;
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

                                // 12. Traits
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

                                // 13. Lifetimes
                                if nav_item_sidebar(
                                    ui,
                                    "Lifetimes",
                                    state.ui.ruta_actual == AppRoute::TutorialLifetimes,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialLifetimes;
                                }
                                ui.add_space(2.0);

                                // 14. Testing
                                if nav_item_sidebar(
                                    ui,
                                    "Testing",
                                    state.ui.ruta_actual == AppRoute::TutorialTesting,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialTesting;
                                }
                                ui.add_space(2.0);

                                // 15. Cargo & Workspaces
                                if nav_item_sidebar(
                                    ui,
                                    "Cargo & Workspaces",
                                    state.ui.ruta_actual == AppRoute::TutorialCargoWorkspaces,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialCargoWorkspaces;
                                }
                                ui.add_space(2.0);

                                // 16. Smart Pointers
                                if nav_item_sidebar(
                                    ui,
                                    "Smart Pointers",
                                    state.ui.ruta_actual == AppRoute::TutorialSmartPointers,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialSmartPointers;
                                }
                                ui.add_space(2.0);

                                // 17. Concurrency
                                if nav_item_sidebar(
                                    ui,
                                    "Concurrency",
                                    state.ui.ruta_actual == AppRoute::TutorialConcurrencia,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialConcurrencia;
                                }
                                ui.add_space(2.0);

                                // 18. Async / Await
                                if nav_item_sidebar(
                                    ui,
                                    "Async / Await",
                                    state.ui.ruta_actual == AppRoute::TutorialAsync,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::TutorialAsync;
                                }
                                ui.add_space(2.0);

                                // Ruta futura: mantiene separados los contenidos que todavía
                                // no forman parte del curso principal.
                                if nav_item_sidebar(
                                    ui,
                                    "Próximamente",
                                    state.ui.ruta_actual == AppRoute::Proximamente,
                                )
                                .clicked()
                                {
                                    state.ui.ruta_actual = AppRoute::Proximamente;
                                }
                                ui.add_space(2.0);
                            });
                    },
                );

                // Zona fija de utilidades, separada del scroll de sesiones.
                let (_, utility_rect) =
                    ui.allocate_space(egui::vec2(ui.available_width(), utility_area_height));
                let line_y = utility_rect.top() + 10.0;
                ui.painter().line_segment(
                    [
                        egui::pos2(utility_rect.left(), line_y),
                        egui::pos2(utility_rect.right(), line_y),
                    ],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 50, 65)),
                );

                // Tamaño compacto: egui lo adapta a la escala de pantalla.
                let button_size = egui::vec2(34.0, 30.0);
                let button_gap = 8.0;
                let buttons_width = button_size.x * 3.0 + button_gap * 2.0;
                let buttons_rect = egui::Rect::from_center_size(
                    egui::pos2(utility_rect.center().x, utility_rect.top() + 45.0),
                    egui::vec2(buttons_width, button_size.y),
                );
                let mut buttons_ui = ui.new_child(egui::UiBuilder::new().max_rect(buttons_rect));

                // Botones de Utilidades: Salida/Logs, Terminal y Configuración.
                buttons_ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let is_cargo_open = state.ui.show_cargo_output_modal.load(Ordering::Relaxed);
                    let cargo_text_color = if is_cargo_open {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(
                                egui::RichText::new("ℹ️").size(18.0).color(cargo_text_color),
                            ),
                        )
                        .on_hover_text("Información / Salida de compilación y macros")
                        .clicked()
                    {
                        state
                            .ui
                            .show_cargo_output_modal
                        .store(!is_cargo_open, Ordering::Relaxed);
                    }

                    ui.add_space(button_gap);

                    let term_text_color = if state.ui.show_terminal_modal {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(
                                egui::RichText::new("💻").size(18.0).color(term_text_color),
                            ),
                        )
                        .on_hover_text("Terminal del sistema interactiva")
                        .clicked()
                    {
                        let abrir_terminal = !state.ui.show_terminal_modal;
                        state.ui.show_terminal_modal = abrir_terminal;
                    }

                    ui.add_space(button_gap);

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
                        .add_sized(button_size, egui::Button::image(config_img))
                        .on_hover_text("Configuración y Atajos de Teclado")
                        .clicked()
                    {
                        state.ui.show_settings_modal = !state.ui.show_settings_modal;
                    }
                });
            });
        });

    // La línea divisoria termina antes del footer global. El borde automático
    // del Panel ocuparía toda la ventana y quedaría por encima del footer.
    if let Some(response) = sidebar_response {
        let sidebar_rect = response.response.rect;
        let footer_top = ui
            .ctx()
            .data(|data| {
                data.get_temp::<Option<f32>>(egui::Id::new("ferriskey_status_bar_top"))
                    .flatten()
            })
            .unwrap_or(sidebar_rect.bottom());
        let divider_bottom = footer_top.min(sidebar_rect.bottom());

        ui.painter().line_segment(
            [
                egui::pos2(sidebar_rect.right(), sidebar_rect.top()),
                egui::pos2(sidebar_rect.right(), divider_bottom),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 50, 65)),
        );
    }

    state.ui.mostrar_sidebar = is_expanded;
}
