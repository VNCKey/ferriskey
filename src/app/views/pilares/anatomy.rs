use std::path::{Path, PathBuf};

use eframe::egui;

use crate::app::AppState;
use crate::components::code_editor::syntax_layouter;
use crate::components::console_output::formatear_salida_consola;
use crate::execution::ejecutar_codigo_cargo_run;
use crate::routes::AppRoute;
use crate::views::conceptos::{
    buscar_ruta_proyecto, mostrar_selector_proyectos_estandar_con_archivo_activo,
    pintar_icono_badge_tile,
};
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

#[path = "binary_analyzer.rs"]
mod binary_analyzer;
use binary_analyzer::{BinaryCardStyle, BinaryProfile, mostrar_ficha_binario};

pub struct CodeLabConfig {
    pub project_selector_id: &'static str,
    pub separator_id: &'static str,
    pub terminal_panel_id: &'static str,
    pub editor_scroll_id: &'static str,
    pub drawer_key: &'static str,
    pub drawer_open: bool,
    pub drawer_tab: usize,
    pub output_open: bool,
    pub navigation_step: usize,
    pub navigation_total: usize,
}

#[derive(Clone, Copy)]
pub struct CodeLabState {
    pub drawer_open: bool,
    pub drawer_tab: usize,
    pub output_open: bool,
    pub navigation_delta: i8,
}

pub fn mostrar_anatomia_cargo(ui: &mut egui::Ui, state: &mut AppState) {
    let shell_state = mostrar_code_lab_shell(
        ui,
        state,
        CodeLabConfig {
            project_selector_id: "combo_proyectos_anatomy_codelab",
            separator_id: "anatomy_editor_toolbar_sep_y",
            terminal_panel_id: "anatomy_terminal_panel",
            editor_scroll_id: "anatomy_codelab_editor",
            drawer_key: "explorer",
            drawer_open: state.ui.mostrar_explorer_drawer,
            drawer_tab: state.lessons.anatomy_step,
            output_open: state.ui.mostrar_console_drawer,
            navigation_step: state.lessons.codelab_reto_actual,
            navigation_total: 6,
        },
        |ui, state, orange, cyan| {
            mostrar_instrucciones_codelab(ui, state, orange, cyan);
        },
    );
    state.ui.mostrar_explorer_drawer = shell_state.drawer_open;
    state.lessons.anatomy_step = shell_state.drawer_tab;
    state.ui.mostrar_console_drawer = shell_state.output_open;
    aplicar_navegacion_codelab(
        &mut state.lessons.codelab_reto_actual,
        shell_state.navigation_delta,
        6,
    );
}

pub fn mostrar_code_lab_shell<F>(
    ui: &mut egui::Ui,
    state: &mut AppState,
    mut config: CodeLabConfig,
    content: F,
) -> CodeLabState
where
    F: FnOnce(&mut egui::Ui, &mut AppState, egui::Color32, egui::Color32),
{
    let orange = egui::Color32::from_rgb(255, 180, 80);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let mut drawer_open = config.drawer_open;
    let mut drawer_tab = config.drawer_tab.min(1);
    let mut output_open = config.output_open;

    let syntax_set = state.editor.syntax_set.clone();
    let theme = state.editor.theme_set.themes["base16-ocean.dark"].clone();

    // --- CONTENEDOR PRINCIPAL A PANTALLA COMPLETA 100% EDGE-TO-EDGE ---
    let mut main_frame = egui::Frame::new();
    main_frame.fill = egui::Color32::from_rgb(10, 14, 22);
    main_frame.stroke = egui::Stroke::NONE;
    main_frame.corner_radius = egui::CornerRadius::ZERO;
    main_frame.inner_margin = egui::Margin {
        left: 0,  // 0px para que los paneles (terminal) vayan de borde a borde exacto
        right: 0, // Pegado 100% al borde derecho para el scrollbar
        top: 6,
        bottom: 0, // Al ras del footer (status_bar) sin margen inferior
    };

    main_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        let mut run_clicked = false;

        // 1. TOOLBAR SUPERIOR DEL EDITOR (Selector + Ejecutar + Consola + Explorador)
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.spacing_mut().item_spacing.x = 8.0;

            // Selector de Proyecto y Archivos a la izquierda
            let active_file = state.project.selected_file.clone();
            // El shell decide el buffer según la ruta activa. Así todas las
            // sesiones comparten el mismo selector sin compartir contenido.
            match state.ui.ruta_actual {
                AppRoute::Comenzando => {
                    mostrar_selector_proyectos_estandar_con_archivo_activo(
                        ui,
                        &mut state.project.selected_project,
                        active_file.as_deref(),
                        &mut state.terminal.term_cwd,
                        config.project_selector_id,
                        &mut state.lessons.conceptos_code,
                    );
                }
                _ => {
                    mostrar_selector_proyectos_estandar_con_archivo_activo(
                        ui,
                        &mut state.project.selected_project,
                        active_file.as_deref(),
                        &mut state.terminal.term_cwd,
                        config.project_selector_id,
                        &mut state.lessons.estructura_code,
                    );
                }
            }

            // Botón de Archivos / Explorador de Proyecto: SIEMPRE VISIBLE para abrir la barra lateral derecha (Explorador)
            ui.add_space(2.0);
            let explorador_abierto = drawer_open && drawer_tab == 1;
            let img_file = egui::Image::new(egui::include_image!("../../../../assets/icons/file-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
            let btn_file = pintar_icono_badge_tile(
                ui,
                img_file,
                explorador_abierto,
                cyan,
                "Abrir Explorador de Archivos en Panel Lateral",
            );

            if btn_file.clicked() {
                if explorador_abierto {
                    drawer_open = false;
                } else {
                    drawer_open = true;
                    drawer_tab = 1; // Seleccionar pestaña Explorador
                }
            }

            // 1. Botón Ejecutar (play-svgrepo-com.svg con iluminación interactiva Cyan en hover)
            let img_play = egui::Image::new(egui::include_image!("../../../../assets/icons/play-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(14.0, 14.0));
            let cargo_project_available = state.project.selected_project.as_deref().is_some_and(|project| {
                let project_dir = buscar_ruta_proyecto(&state.terminal.term_cwd, project);
                project_dir.is_dir() && project_dir.join("Cargo.toml").is_file()
            });
            let btn_run = boton_icono_toolbar_estado(
                ui,
                img_play,
                false,
                cyan,
                cargo_project_available,
                if cargo_project_available {
                    "Ejecutar Código (F5 / Ctrl+Enter)"
                } else {
                    "Selecciona un proyecto Cargo para ejecutar"
                },
            );

            if cargo_project_available && btn_run.clicked() {
                run_clicked = true;
                output_open = true;
            }

            ui.add_space(2.0);

            // 2. Botón de Salida (terminal-svgrepo-com.svg con iluminación interactiva Cyan en hover)
            let console_abierta = output_open;
            let img_terminal = egui::Image::new(egui::include_image!("../../../../assets/icons/terminal-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
            let btn_console = boton_icono_toolbar(
                ui,
                img_terminal,
                console_abierta,
                cyan,
                if console_abierta { "Ocultar Salida" } else { "Mostrar Salida" },
            );

            if btn_console.clicked() {
                output_open = !output_open;
            }

            // Guardado explícito del archivo que está abierto en el editor.
            ui.add_space(2.0);
            let guardar_habilitado = state.project.selected_project.is_some();
            let guardar_resp = boton_guardar_toolbar(ui, guardar_habilitado, orange)
                .on_hover_text("Guardar archivo activo (Ctrl + S)");
            if guardar_resp.clicked() {
                state.guardar_proyecto_activo();
            }

            if !state.editor.project_editor_status.is_empty() {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(&state.editor.project_editor_status)
                        .size(10.5)
                        .color(egui::Color32::from_rgb(255, 190, 90)),
                );
            }

            // --- Espaciador elástico seguro hacia la derecha ---
            let drawer_activo = drawer_open;
            let available_w = ui.available_width();
            let needed_for_btn = 32.0 + 10.0;
            if available_w > needed_for_btn {
                ui.add_space(available_w - needed_for_btn);
            }

            // Botón único en el extremo derecho para abrir/cerrar el Panel Lateral (con las pestañas de Reto y Explorador)
            let img_panel = egui::Image::new(egui::include_image!("../../../../assets/icons/design-distribution-of-elements-of-an-article-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
            let btn_panel = if drawer_activo {
                boton_icono_toolbar(
                    ui,
                    img_panel,
                    true,
                    cyan,
                    "Ocultar Panel Lateral",
                )
            } else {
                boton_guia_destacado(
                    ui,
                    img_panel,
                    orange,
                    "Abrir Panel Lateral (Instrucciones del Reto y Explorador)",
                )
            };

            if btn_panel.clicked() {
                drawer_open = !drawer_open;
            }
        });

        ui.add_space(5.0);
        let sep_y = ui.cursor().top();
        ui.data_mut(|d| d.insert_temp(egui::Id::new(config.separator_id), sep_y));
        let frame_rect = ui.max_rect();
        // Línea divisoria horizontal de borde a borde exterior exacto
        ui.painter().line_segment(
            [
                egui::pos2(frame_rect.min.x, sep_y),
                egui::pos2(frame_rect.max.x, sep_y),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95)),
        );
        ui.add_space(5.0);

        // 2. EJECUCIÓN DEL CÓDIGO CON CARGO RUN
        let output_arc = state.obtener_output_activo();

        if run_clicked {
            *output_arc.lock().unwrap() = "Compilando y ejecutando con Cargo (cargo run)...".to_string();
            output_open = true;

            let proj_dir = state.project.selected_project.as_deref().map(|proj| {
                crate::views::conceptos::buscar_ruta_proyecto(&state.terminal.term_cwd, proj)
            });

            let code_clone = state.obtener_codigo_activo().to_string();
            let out_clone = std::sync::Arc::clone(&output_arc);
            let out_error = std::sync::Arc::clone(&output_arc);
            let ctx = ui.ctx().clone();
            let task_result = state.terminal.task_manager.spawn("cargo-run-codelab", move || {
                let res = ejecutar_codigo_cargo_run(&code_clone, proj_dir.as_deref());
                *out_clone.lock().unwrap() = res;
                ctx.request_repaint();
            });
            if let Err(error) = task_result
                && let Ok(mut output) = out_error.lock() {
                    *output = format!("No se pudo iniciar cargo run: {error}");
                }
        }

        let editor_extension = state
            .project.selected_file
            .as_deref()
            .and_then(|file| Path::new(file).extension().and_then(|ext| ext.to_str()))
            .unwrap_or("rs")
            .to_string();
        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
            syntax_layouter(
                ui,
                text.as_str(),
                wrap_width,
                &syntax_set,
                &theme,
                &editor_extension,
            )
        };

        let output_text = output_arc.lock().unwrap().clone();
        // La visibilidad depende del usuario, no de si la salida tiene texto:
        // limpiar el resultado no debe cerrar ni encoger el panel.
        let mut console_open = output_open;
        let prev_console_open = console_open;
        let mut cerrar_console = false;

        // 2. PANEL DE SALIDA DEL CODE LAB (ANIMACIÓN NATIVA SHOW_COLLAPSIBLE)
        egui::Panel::bottom(config.terminal_panel_id)
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(9, 12, 17))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 52, 75)))
                    .inner_margin(egui::Margin::ZERO)
                    .corner_radius(egui::CornerRadius::ZERO),
            )
            .default_size(200.0)
            .size_range(100.0..=480.0)
            .resizable(true)
            .show_collapsible(ui, &mut console_open, |ui| {
                let header_h = 30.0;

                // Cabecera integrada tipo terminal estilo VS Code (Centrado vertical flex)
                let header_frame = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(13, 17, 24))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(26, 36, 52)))
                    .inner_margin(egui::Margin { left: 14, right: 8, top: 0, bottom: 0 });

                header_frame.show(ui, |ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), header_h),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            // Título del panel de resultados (centrado vertical exacto)
                            ui.label(
                                egui::RichText::new("SALIDA")
                                    .strong()
                                    .size(11.5)
                                    .color(egui::Color32::from_rgb(190, 205, 225)),
                            );

                            ui.add_space(8.0);

                            // Badge pill de estado integrado
                            let es_ejecutando = output_text.starts_with("Compilando") || output_text == "Ejecutando...";
                            let es_error = output_text.contains("error[E") || output_text.contains("error:") || output_text.contains("Error ejecutando") || output_text.contains("Error creando") || output_text.contains("Error guardando");
                            let mostrar_badge = es_ejecutando || es_error;

                            let (badge_bg, badge_stroke, badge_text, badge_color, dot_core_color, dot_halo_color) = if es_ejecutando {
                                (
                                    egui::Color32::from_rgb(38, 28, 10),
                                    egui::Color32::from_rgb(85, 60, 20),
                                    "Compilando...",
                                    egui::Color32::from_rgb(255, 205, 80),
                                    egui::Color32::from_rgb(255, 210, 90),
                                    egui::Color32::from_rgba_unmultiplied(255, 205, 80, 50),
                                )
                            } else if es_error {
                                (
                                    egui::Color32::from_rgb(42, 16, 18),
                                    egui::Color32::from_rgb(95, 32, 36),
                                    "Salida con errores",
                                    egui::Color32::from_rgb(255, 115, 115),
                                    egui::Color32::from_rgb(255, 110, 110),
                                    egui::Color32::from_rgba_unmultiplied(255, 90, 90, 50),
                                )
                            } else {
                                // FerrisKey Signature Rust Orange Palette
                                (
                                    egui::Color32::from_rgb(34, 18, 8),
                                    egui::Color32::from_rgb(95, 48, 16),
                                    "",
                                    egui::Color32::from_rgb(255, 180, 80),
                                    egui::Color32::from_rgb(255, 155, 45),
                                    egui::Color32::from_rgba_unmultiplied(255, 150, 40, 50),
                                )
                            };

                            let badge_frame = egui::Frame::new()
                                .fill(badge_bg)
                                .stroke(egui::Stroke::new(1.0, badge_stroke))
                                .corner_radius(egui::CornerRadius::same(3))
                                .inner_margin(egui::Margin::symmetric(7, 3));

                            if mostrar_badge {
                                badge_frame.show(ui, |ui| {
                                    ui.allocate_ui_with_layout(
                                        egui::vec2(ui.available_width(), 16.0),
                                        egui::Layout::left_to_right(egui::Align::Center),
                                        |ui| {
                                            ui.spacing_mut().item_spacing.x = 6.0;
                                            let (dot_rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
                                            // Halo suave exterior
                                            ui.painter().circle_filled(dot_rect.center(), 4.0, dot_halo_color);
                                            // Núcleo brillante
                                            ui.painter().circle_filled(dot_rect.center(), 2.8, dot_core_color);

                                            ui.label(
                                                egui::RichText::new(badge_text)
                                                    .size(10.5)
                                                    .strong()
                                                    .color(badge_color),
                                            );
                                        },
                                    );
                                });
                            }

                            // Acciones a la derecha
                            ui.scope_builder(
                                egui::UiBuilder::new()
                                    .id(egui::Id::new("terminal_header_actions"))
                                    .layout(egui::Layout::right_to_left(egui::Align::Center)),
                                |ui| {
                                    ui.add_space(4.0);

                                    // 1. Botón Cerrar (close-bold-svgrepo-com.svg)
                                    let (close_rect, close_resp) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                                    let close_hovered = close_resp.hovered();
                                    if close_hovered {
                                        ui.painter().rect_filled(close_rect, egui::CornerRadius::same(3), egui::Color32::from_rgba_unmultiplied(255, 255, 255, 22));
                                    }
                                    let close_color = if close_hovered { egui::Color32::WHITE } else { egui::Color32::from_rgb(150, 165, 185) };
                                    let img_close = egui::Image::new(egui::include_image!("../../../../assets/icons/close-bold-svgrepo-com.svg"))
                                        .fit_to_exact_size(egui::vec2(12.0, 12.0))
                                        .tint(close_color);
                                    img_close.paint_at(ui, egui::Rect::from_center_size(close_rect.center(), egui::vec2(12.0, 12.0)));
                                    if close_resp.on_hover_text("Cerrar terminal").clicked() {
                                        cerrar_console = true;
                                    }

                                    ui.add_space(2.0);

                                    // 2. Botón Minimizar (semi-select-svgrepo-com.svg)
                                    let (min_rect, min_resp) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                                    let min_hovered = min_resp.hovered();
                                    if min_hovered {
                                        ui.painter().rect_filled(min_rect, egui::CornerRadius::same(3), egui::Color32::from_rgba_unmultiplied(255, 255, 255, 22));
                                    }
                                    let min_color = if min_hovered { egui::Color32::WHITE } else { egui::Color32::from_rgb(150, 165, 185) };
                                    let img_min = egui::Image::new(egui::include_image!("../../../../assets/icons/semi-select-svgrepo-com.svg"))
                                        .fit_to_exact_size(egui::vec2(12.0, 12.0))
                                        .tint(min_color);
                                    img_min.paint_at(ui, egui::Rect::from_center_size(min_rect.center(), egui::vec2(12.0, 12.0)));
                                    if min_resp.on_hover_text("Minimizar terminal con transición").clicked() {
                                        cerrar_console = true;
                                    }

                                    ui.add_space(6.0);

                                    // 3. Botón Copiar salida (copy-document-svgrepo-com.svg)
                                    let (copy_rect, copy_resp) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                                    let copy_hovered = copy_resp.hovered();
                                    if copy_hovered {
                                        ui.painter().rect_filled(copy_rect, egui::CornerRadius::same(3), egui::Color32::from_rgba_unmultiplied(255, 255, 255, 22));
                                    }
                                    let copy_color = if copy_hovered { egui::Color32::WHITE } else { egui::Color32::from_rgb(150, 165, 185) };
                                    let img_copy = egui::Image::new(egui::include_image!("../../../../assets/icons/copy-document-svgrepo-com.svg"))
                                        .fit_to_exact_size(egui::vec2(13.0, 13.0))
                                        .tint(copy_color);
                                    img_copy.paint_at(ui, egui::Rect::from_center_size(copy_rect.center(), egui::vec2(13.0, 13.0)));
                                    if copy_resp.on_hover_text("Copiar salida al portapapeles").clicked() {
                                        ui.ctx().copy_text(output_text.clone());
                                    }

                                    ui.add_space(2.0);

                                    // 4. Botón Limpiar consola (coffee-svgrepo-com.svg)
                                    let (clear_rect, clear_resp) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                                    let clear_hovered = clear_resp.hovered();
                                    if clear_hovered {
                                        ui.painter().rect_filled(clear_rect, egui::CornerRadius::same(3), egui::Color32::from_rgba_unmultiplied(255, 255, 255, 22));
                                    }
                                    let clear_color = if clear_hovered { egui::Color32::WHITE } else { egui::Color32::from_rgb(150, 165, 185) };
                                    let img_coffee = egui::Image::new(egui::include_image!("../../../../assets/icons/coffee-svgrepo-com.svg"))
                                        .fit_to_exact_size(egui::vec2(13.0, 13.0))
                                        .tint(clear_color);
                                    img_coffee.paint_at(ui, egui::Rect::from_center_size(clear_rect.center(), egui::vec2(13.0, 13.0)));
                                    if clear_resp.on_hover_text("Limpiar únicamente la salida").clicked() {
                                        *output_arc.lock().unwrap() = String::new();
                                    }
                                },
                            );
                        },
                    );
                });

                // Cuerpo de la terminal con salida scrolleable
                egui::Frame::new()
                    .inner_margin(egui::Margin { left: 14, right: 10, top: 8, bottom: 8 })
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                if output_text == "Ejecutando..." || output_text == "Compilando..." {
                                    ui.label(
                                        egui::RichText::new(&output_text)
                                            .color(egui::Color32::YELLOW)
                                            .monospace(),
                                    );
                                } else if let Some(idx) = output_text.find("[Errores/Warnings]:\n") {
                                    let (stdout, stderr) = output_text.split_at(idx);
                                    if !stdout.is_empty() {
                                        ui.label(formatear_salida_consola(stdout, false));
                                        ui.add_space(4.0);
                                        ui.separator();
                                        ui.add_space(4.0);
                                    }
                                    let solo_error = stderr
                                        .strip_prefix("[Errores/Warnings]:\n")
                                        .unwrap_or(stderr);
                                    ui.label(formatear_salida_consola(solo_error, true));
                                } else if output_text.starts_with("Error") {
                                    ui.label(formatear_salida_consola(&output_text, true));
                                } else {
                                    ui.label(formatear_salida_consola(&output_text, false));
                                }
                            });
                    });
            });

        if cerrar_console || console_open != prev_console_open {
            output_open = console_open && !cerrar_console;
        }

        let drawer_is_open = drawer_open;

        // 3. CUERPO DEL EDITOR DE CÓDIGO (OCUPA EL ESPACIO DISPONIBLE RESTANTE)
        let (code_ref, _) = state.obtener_editor_activo_mut();
        let code_max_h = ui.available_height();

        // Personalización de la barra de scroll (Pegada al ras en el extremo derecho, 0px margen)
        let old_style = (**ui.style()).clone();
        ui.style_mut().spacing.scroll.floating = false;
        ui.style_mut().spacing.scroll.bar_width = 8.0;
        ui.style_mut().spacing.scroll.bar_inner_margin = 0.0;
        ui.style_mut().spacing.scroll.bar_outer_margin = 0.0;
        ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 60, 85);
        ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
        ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 85, 120);
        ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
        ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 115, 160);
        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
        ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::ZERO;

        let pointer_over_drawer = drawer_is_open && ui.ctx().pointer_latest_pos().is_some_and(|pos| {
            let bounds = ui.max_rect();
            let default_w = 650.0_f32;
            let drawer_width_id = egui::Id::new(format!(
                "{}_drawer_custom_width",
                config.drawer_key
            ));
            let target_drawer_width = ui.data_mut(|d| d.get_temp::<f32>(drawer_width_id).unwrap_or(default_w));
            pos.x >= (bounds.max.x - target_drawer_width)
        });

        egui::ScrollArea::vertical()
            .id_salt(config.editor_scroll_id)
            .vscroll(!pointer_over_drawer)
            .max_height(code_max_h)
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.add_space(10.0);
                    // Generación dinámica de la columna de números de línea
                    let num_lines = code_ref.split('\n').count().max(1);
                    let mut line_numbers = String::new();
                    for i in 1..=num_lines {
                        use std::fmt::Write;
                        let _ = writeln!(line_numbers, "{:>2}", i);
                    }

                    // Columna de numeración (Gutter)
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(line_numbers.trim_end())
                                .font(egui::FontId::monospace(14.0))
                                .color(egui::Color32::from_rgb(85, 105, 135)),
                        )
                        .selectable(false),
                    );

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    ui.add(
                        egui::TextEdit::multiline(code_ref)
                            .frame(egui::Frame::NONE)
                            .layouter(&mut layouter)
                            .code_editor()
                            .desired_width(ui.available_width())
                            .lock_focus(true),
                    );
                });
            });

        ui.set_style(old_style);

        // El atajo se procesa dentro del editor para que funcione aunque el
        // TextEdit tenga el foco y el evento de teclado ya esté siendo usado.
        let guardar_por_atajo = ui.ctx().input_mut(|input| {
            input.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::S,
            ))
        });
        if guardar_por_atajo && state.project.selected_project.is_some() {
            state.guardar_proyecto_activo();
        }
    });

    config.drawer_open = drawer_open;
    config.drawer_tab = drawer_tab;
    let (drawer_open, drawer_tab, navigation_delta) =
        mostrar_drawer_codelab(ui, state, config, content);

    CodeLabState {
        drawer_open,
        drawer_tab,
        output_open,
        navigation_delta,
    }
}

/// Drawer lateral compartido por los Code Labs. El shell visual pertenece a
/// esta capa y cada sesión solamente aporta el contenido de sus instrucciones.
pub fn mostrar_drawer_codelab<F>(
    ui: &mut egui::Ui,
    state: &mut AppState,
    config: CodeLabConfig,
    content: F,
) -> (bool, usize, i8)
where
    F: FnOnce(&mut egui::Ui, &mut AppState, egui::Color32, egui::Color32),
{
    let orange = egui::Color32::from_rgb(255, 180, 80);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let mut drawer_open = config.drawer_open;
    let mut active_tab = config.drawer_tab.min(1);
    let mut navigation_delta = 0;
    let anim_id = egui::Id::new(format!("{}_drawer_anim", config.drawer_key));
    let drawer_width_id = egui::Id::new(format!("{}_drawer_custom_width", config.drawer_key));
    let overlay_id = egui::Id::new(format!("{}_right_drawer_overlay", config.drawer_key));
    let resize_id = egui::Id::new(format!("{}_drawer_resizer", config.drawer_key));
    let scroll_id = egui::Id::new(format!("{}_drawer_scroll", config.drawer_key));

    let anim_factor = ui.ctx().animate_bool(anim_id, drawer_open);
    if anim_factor > 0.001 {
        // AppState crea el Panel::bottom("status_bar") antes del CentralPanel.
        // Por eso este rectángulo ya excluye el footer global con los toggles
        // y "FerrisKey v0.1"; el drawer nunca debe calcularse contra toda la ventana.
        let mut bounds = ui.max_rect();
        let status_bar_top = ui.ctx().data_mut(|data| {
            data.get_temp::<Option<f32>>(egui::Id::new("ferriskey_status_bar_top"))
                .flatten()
        });
        if let Some(status_bar_top) = status_bar_top {
            bounds.max.y = bounds.max.y.min(status_bar_top);
        }
        let min_w = 320.0_f32;
        let max_w = (bounds.width() * 0.85).max(min_w);
        let default_w = 650.0_f32.clamp(min_w, max_w);
        let mut target_drawer_width =
            ui.data_mut(|data| data.get_temp::<f32>(drawer_width_id).unwrap_or(default_w));
        target_drawer_width = target_drawer_width.clamp(min_w, max_w);

        let slide_offset = target_drawer_width * (1.0 - anim_factor);
        let top_y = bounds.min.y;
        let drawer_height = bounds.height();
        let left_edge_x = bounds.max.x - target_drawer_width + slide_offset;

        if drawer_open && anim_factor > 0.95 {
            let resize_rect = egui::Rect::from_min_max(
                egui::pos2(left_edge_x - 4.0, top_y),
                egui::pos2(left_edge_x + 4.0, top_y + drawer_height),
            );
            let resize_response = ui.interact(resize_rect, resize_id, egui::Sense::drag());
            if resize_response.hovered() || resize_response.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }
            if resize_response.dragged() {
                target_drawer_width =
                    (target_drawer_width - resize_response.drag_delta().x).clamp(min_w, max_w);
                ui.data_mut(|data| data.insert_temp(drawer_width_id, target_drawer_width));
            }
        }

        egui::Area::new(overlay_id)
            .order(egui::Order::Middle)
            .fixed_pos(egui::pos2(left_edge_x, top_y))
            .show(ui.ctx(), |ui| {
                let visible_min_x = left_edge_x.max(bounds.min.x);
                let drawer_rect = egui::Rect::from_min_max(
                    egui::pos2(visible_min_x, top_y),
                    egui::pos2(bounds.max.x, top_y + drawer_height),
                );
                ui.set_clip_rect(drawer_rect);
                ui.allocate_rect(drawer_rect, egui::Sense::hover());

                let separator_id = egui::Id::new(config.separator_id);
                let separator_y = ui
                    .data_mut(|data| data.get_temp::<f32>(separator_id))
                    .unwrap_or(top_y + 37.0);
                let header_rect = egui::Rect::from_min_max(
                    egui::pos2(left_edge_x, top_y),
                    egui::pos2(left_edge_x + target_drawer_width, separator_y),
                );
                ui.painter().rect_filled(
                    header_rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_rgb(11, 14, 20),
                );

                let mut header_ui =
                    ui.new_child(egui::UiBuilder::new().max_rect(egui::Rect::from_min_max(
                        egui::pos2(left_edge_x + 14.0, top_y + 6.0),
                        egui::pos2(left_edge_x + target_drawer_width - 14.0, separator_y),
                    )));
                header_ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;

                    let img_flag = egui::Image::new(egui::include_image!(
                        "../../../../assets/icons/flag-pin-navigation-direction-svgrepo-com.svg"
                    ))
                    .fit_to_exact_size(egui::vec2(15.0, 15.0));
                    let flag_response = boton_icono_toolbar(
                        ui,
                        img_flag,
                        active_tab == 0,
                        cyan,
                        "Instrucciones del reto",
                    );
                    if flag_response.clicked() {
                        active_tab = 0;
                    }

                    if tab_chip_toolbar(ui, "Explorador", active_tab == 1, cyan).clicked() {
                        active_tab = 1;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (close_rect, close_response) =
                            ui.allocate_exact_size(egui::vec2(28.0, 26.0), egui::Sense::click());
                        let hovered = close_response.hovered();
                        let pressed = close_response.is_pointer_button_down_on();
                        let fill = if pressed {
                            egui::Color32::from_rgb(34, 46, 68)
                        } else if hovered {
                            egui::Color32::from_rgb(26, 36, 54)
                        } else {
                            egui::Color32::from_rgb(16, 22, 32)
                        };
                        let stroke = if hovered {
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 120, 120))
                        } else {
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 54, 80))
                        };
                        ui.painter().rect(
                            close_rect,
                            egui::CornerRadius::same(4),
                            fill,
                            stroke,
                            egui::StrokeKind::Inside,
                        );
                        let img_close = egui::Image::new(egui::include_image!(
                            "../../../../assets/icons/close-circle-svgrepo-com.svg"
                        ))
                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                        .tint(if hovered {
                            egui::Color32::from_rgb(255, 140, 140)
                        } else {
                            egui::Color32::from_rgb(160, 175, 195)
                        });
                        img_close.paint_at(
                            ui,
                            egui::Rect::from_center_size(
                                close_rect.center(),
                                egui::vec2(16.0, 16.0),
                            ),
                        );
                        if close_response.clicked() {
                            drawer_open = false;
                        }
                        close_response.on_hover_text("Cerrar Panel Lateral");
                    });
                });

                ui.painter().line_segment(
                    [
                        egui::pos2(left_edge_x, separator_y),
                        egui::pos2(left_edge_x + target_drawer_width, separator_y),
                    ],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95)),
                );

                let footer_height = if active_tab == 0 {
                    48.0_f32.min(drawer_height)
                } else {
                    0.0
                };
                let footer_top = top_y + drawer_height - footer_height;
                let content_rect = egui::Rect::from_min_max(
                    egui::pos2(left_edge_x, separator_y + 1.0),
                    egui::pos2(left_edge_x + target_drawer_width, footer_top),
                );
                ui.painter().rect_filled(
                    content_rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_rgb(11, 14, 20),
                );

                let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
                content_ui.set_clip_rect(content_rect);
                content_ui.style_mut().spacing.scroll.floating = false;
                content_ui.style_mut().spacing.scroll.bar_width = 8.0;
                content_ui.style_mut().spacing.scroll.bar_inner_margin = 0.0;
                content_ui.style_mut().spacing.scroll.bar_outer_margin = 0.0;
                content_ui.style_mut().visuals.widgets.inactive.bg_fill =
                    egui::Color32::from_rgb(45, 60, 85);
                content_ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                content_ui
                    .style_mut()
                    .visuals
                    .widgets
                    .inactive
                    .corner_radius = egui::CornerRadius::ZERO;
                content_ui.style_mut().visuals.widgets.hovered.bg_fill =
                    egui::Color32::from_rgb(70, 95, 135);
                content_ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                content_ui.style_mut().visuals.widgets.hovered.corner_radius =
                    egui::CornerRadius::ZERO;
                content_ui.style_mut().visuals.widgets.active.bg_fill =
                    egui::Color32::from_rgb(255, 150, 45);
                content_ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
                content_ui.style_mut().visuals.widgets.active.corner_radius =
                    egui::CornerRadius::ZERO;

                egui::ScrollArea::vertical()
                    .id_salt(scroll_id)
                    .max_height(content_rect.height())
                    .auto_shrink([false, false])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .show(&mut content_ui, |ui| {
                        egui::Frame::new()
                            .inner_margin(egui::Margin {
                                left: 16,
                                right: 18,
                                top: 10,
                                bottom: 24,
                            })
                            .fill(egui::Color32::TRANSPARENT)
                            .show(ui, |ui| {
                                if active_tab == 0 {
                                    content(ui, state, orange, cyan);
                                } else {
                                    mostrar_project_explorer_drawer(ui, state, orange, cyan);
                                }
                            });
                    });

                if active_tab == 0 {
                    let footer_rect = egui::Rect::from_min_max(
                        egui::pos2(left_edge_x, footer_top),
                        egui::pos2(left_edge_x + target_drawer_width, top_y + drawer_height),
                    );
                    ui.painter().rect_filled(
                        footer_rect,
                        egui::CornerRadius::ZERO,
                        egui::Color32::from_rgb(11, 14, 20),
                    );
                    ui.painter().line_segment(
                        [
                            egui::pos2(left_edge_x, footer_top),
                            egui::pos2(left_edge_x + target_drawer_width, footer_top),
                        ],
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95)),
                    );

                    let footer_ui_rect = egui::Rect::from_min_max(
                        egui::pos2(left_edge_x + 16.0, footer_top + 10.0),
                        egui::pos2(
                            left_edge_x + target_drawer_width - 16.0,
                            top_y + drawer_height - 10.0,
                        ),
                    );
                    let mut footer_ui =
                        ui.new_child(egui::UiBuilder::new().max_rect(footer_ui_rect));
                    footer_ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        let anterior_habilitado = config.navigation_step > 0;
                        let siguiente_habilitado = config.navigation_total > 0
                            && config.navigation_step < config.navigation_total.saturating_sub(1);

                        if boton_navegacion_codelab(
                            ui,
                            "Anterior",
                            anterior_habilitado,
                            false,
                            orange,
                        )
                        .clicked()
                        {
                            navigation_delta = -1;
                        }

                        let gap = (ui.available_width() - 78.0).max(8.0);
                        ui.add_space(gap);

                        if boton_navegacion_codelab(
                            ui,
                            "Siguiente",
                            siguiente_habilitado,
                            true,
                            orange,
                        )
                        .clicked()
                        {
                            navigation_delta = 1;
                        }
                    });
                }

                let resize_active = ui
                    .ctx()
                    .data_mut(|data| data.get_temp::<f32>(drawer_width_id).is_some());
                let resize_bounds = egui::Rect::from_min_max(
                    egui::pos2(left_edge_x - 4.0, top_y),
                    egui::pos2(left_edge_x + 4.0, top_y + drawer_height),
                );
                let line_color = if resize_active && ui.rect_contains_pointer(resize_bounds) {
                    cyan
                } else {
                    egui::Color32::from_rgb(45, 65, 95)
                };
                ui.painter().line_segment(
                    [
                        egui::pos2(left_edge_x, top_y),
                        egui::pos2(left_edge_x, top_y + drawer_height),
                    ],
                    egui::Stroke::new(1.5, line_color),
                );
            });
    }

    (drawer_open, active_tab, navigation_delta)
}

fn mostrar_instrucciones_codelab(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let bullet_col = egui::Color32::from_rgb(140, 160, 190);

    ui.add_space(4.0);

    // --- CONTENIDO DEL RETO ACTIVO ---
    match state.lessons.codelab_reto_actual {
        0 => mostrar_reto_1_verificacion(ui, state, orange, cyan, text_col, bullet_col),
        1 => mostrar_reto_2_cargotoml(ui, state, orange, cyan, text_col, bullet_col),
        2 => mostrar_reto_3_debug_release(ui, state, orange, cyan, text_col, bullet_col),
        3 => mostrar_reto_4_optimizacion(ui, state, orange, cyan, text_col, bullet_col),
        4 => mostrar_reto_5_toolchain(ui, state, orange, cyan, text_col, bullet_col),
        _ => mostrar_reto_6_resumen(ui, state, orange, cyan, text_col, bullet_col),
    }
}

pub(crate) fn titulo_seccion(ui: &mut egui::Ui, titulo: &str, accent: egui::Color32) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(19, 27, 40))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(31, 46, 68)))
        .corner_radius(egui::CornerRadius::same(5))
        .inner_margin(egui::Margin {
            left: 6,
            right: 8,
            top: 5,
            bottom: 5,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (accent_rect, _) =
                    ui.allocate_exact_size(egui::vec2(3.0, 18.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(accent_rect, egui::CornerRadius::same(2), accent);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(titulo)
                        .strong()
                        .size(14.5)
                        .color(egui::Color32::WHITE),
                );
            });
        });
}

pub(crate) fn aplicar_navegacion_codelab(step: &mut usize, delta: i8, total: usize) {
    if total == 0 {
        return;
    }

    *step = match delta {
        -1 => step.saturating_sub(1),
        1 => step.saturating_add(1).min(total - 1),
        _ => (*step).min(total - 1),
    };
}

pub(crate) fn punto_lista(ui: &mut egui::Ui, color: egui::Color32) {
    let (dot_rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 18.0), egui::Sense::hover());
    ui.painter().circle_filled(
        egui::pos2(dot_rect.left() + 3.0, dot_rect.center().y),
        2.5,
        color,
    );
}

pub fn boton_navegacion_codelab(
    ui: &mut egui::Ui,
    texto: &str,
    habilitado: bool,
    es_siguiente: bool,
    orange: egui::Color32,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(78.0, 26.0),
        if habilitado {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    let fill = if !habilitado {
        egui::Color32::from_rgb(18, 22, 30)
    } else if es_siguiente {
        if pressed {
            egui::Color32::from_rgb(205, 125, 35)
        } else if hovered {
            egui::Color32::from_rgb(255, 180, 80)
        } else {
            egui::Color32::from_rgb(126, 68, 24)
        }
    } else if pressed {
        egui::Color32::from_rgb(38, 52, 72)
    } else if hovered {
        egui::Color32::from_rgb(30, 43, 62)
    } else {
        egui::Color32::from_rgb(22, 28, 40)
    };

    let stroke_color = if !habilitado {
        egui::Color32::from_rgb(31, 39, 53)
    } else if es_siguiente {
        if hovered || pressed {
            orange
        } else {
            egui::Color32::from_rgb(255, 180, 80)
        }
    } else if hovered || pressed {
        egui::Color32::from_rgb(100, 200, 255)
    } else {
        egui::Color32::from_rgb(45, 60, 85)
    };

    let text_color = if !habilitado {
        egui::Color32::from_rgb(75, 88, 108)
    } else if es_siguiente {
        if hovered || pressed {
            egui::Color32::from_rgb(45, 28, 12)
        } else {
            egui::Color32::from_rgb(255, 225, 175)
        }
    } else if hovered || pressed {
        egui::Color32::from_rgb(235, 245, 255)
    } else {
        egui::Color32::from_rgb(180, 195, 215)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(5),
        fill,
        egui::Stroke::new(1.0, stroke_color),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        texto,
        egui::FontId::proportional(11.5),
        text_color,
    );

    response
}

fn mostrar_acceso_terminal(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(16, 22, 32))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 54, 80)))
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(egui::Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (button_rect, response) =
                    ui.allocate_exact_size(egui::vec2(128.0, 28.0), egui::Sense::click());
                let hovered = response.hovered();
                let pressed = response.is_pointer_button_down_on();

                let button_fill = if pressed {
                    egui::Color32::from_rgb(24, 67, 88)
                } else if hovered {
                    egui::Color32::from_rgb(27, 82, 108)
                } else {
                    egui::Color32::from_rgb(20, 58, 78)
                };
                let button_stroke = if hovered || pressed {
                    cyan
                } else {
                    egui::Color32::from_rgb(55, 125, 155)
                };
                let icon_color = if hovered || pressed {
                    egui::Color32::WHITE
                } else {
                    cyan
                };

                ui.painter().rect(
                    button_rect,
                    egui::CornerRadius::same(5),
                    button_fill,
                    egui::Stroke::new(1.0, button_stroke),
                    egui::StrokeKind::Inside,
                );

                let icon_rect = egui::Rect::from_center_size(
                    egui::pos2(button_rect.left() + 18.0, button_rect.center().y),
                    egui::vec2(15.0, 15.0),
                );
                let img_terminal = egui::Image::new(egui::include_image!(
                    "../../../../assets/icons/terminal-svgrepo-com.svg"
                ))
                .fit_to_exact_size(egui::vec2(15.0, 15.0))
                .tint(icon_color);
                img_terminal.paint_at(ui, icon_rect);

                ui.painter().text(
                    egui::pos2(button_rect.left() + 34.0, button_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    "Abrir Terminal",
                    egui::FontId::proportional(12.0),
                    if hovered || pressed {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::from_rgb(220, 235, 245)
                    },
                );

                if response.clicked() {
                    state.ui.show_terminal_modal = true;
                }

                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new("Atajo")
                        .size(11.5)
                        .color(egui::Color32::from_rgb(140, 155, 175)),
                );

                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(27, 34, 48))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(52, 68, 94)))
                    .corner_radius(egui::CornerRadius::same(4))
                    .inner_margin(egui::Margin::symmetric(6, 3))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Ctrl + T")
                                .monospace()
                                .strong()
                                .size(11.0)
                                .color(orange),
                        );
                    });
            });
        });
}

pub(crate) fn codigo_terminal_bloque(ui: &mut egui::Ui, codigo: &str) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(12, 18, 27))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 160, 50, 72),
        ))
        .corner_radius(egui::CornerRadius::same(5))
        .inner_margin(egui::Margin {
            left: 12,
            right: 12,
            top: 9,
            bottom: 9,
        })
        .show(ui, |ui| {
            // Forzar que el bloque se extienda por toda la columna del
            // Codelab, aunque el texto sea corto.
            ui.set_min_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(codigo)
                        .monospace()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(255, 190, 100)),
                )
                .wrap(),
            );
        });
}

pub(crate) fn codigo_resaltado_bloque(
    ui: &mut egui::Ui,
    codigo: &str,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(12, 18, 27))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 160, 50, 72),
        ))
        .corner_radius(egui::CornerRadius::same(5))
        .inner_margin(egui::Margin {
            left: 12,
            right: 12,
            top: 9,
            bottom: 9,
        })
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            let code_width = ui.available_width();
            let galley = syntax_layouter(ui, codigo, code_width, syntax_set, theme, extension);
            ui.add(egui::Label::new(galley).selectable(true));
        });
}

fn codigo_toml_bloque(ui: &mut egui::Ui, codigo: &str, syntax_set: &SyntaxSet, theme: &Theme) {
    codigo_resaltado_bloque(ui, codigo, syntax_set, theme, "toml");
}

fn mostrar_reto_1_verificacion(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
    text_col: egui::Color32,
    bullet_col: egui::Color32,
) {
    // Encabezado estilo LeetCode (Ej: "1. Verificación e Inicialización")
    ui.heading(
        egui::RichText::new("1. Verificación e Inicialización")
            .size(19.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    // Dificultad / Tag estilo LeetCode
    ui.horizontal(|ui| {
        let mut tag_frame = egui::Frame::new();
        tag_frame.fill = egui::Color32::from_rgb(20, 38, 28);
        tag_frame.inner_margin = egui::Margin::symmetric(8, 2);
        tag_frame.corner_radius = egui::CornerRadius::same(10);
        tag_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Fácil")
                    .size(11.5)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 200, 120)),
            );
        });

        ui.add_space(4.0);

        let mut tag_topic = egui::Frame::new();
        tag_topic.fill = egui::Color32::from_rgb(22, 28, 38);
        tag_topic.inner_margin = egui::Margin::symmetric(8, 2);
        tag_topic.corner_radius = egui::CornerRadius::same(10);
        tag_topic.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Toolchain & Cargo")
                    .size(11.5)
                    .color(egui::Color32::from_rgb(160, 185, 220)),
            );
        });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("Comprueba la instalación del toolchain de Rust, diferencia entre crates binarios y librerías y descubre qué ocurre durante Compile time y Run time antes de crear tu primer proyecto.")
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
    );
    ui.add_space(14.0);

    // Sección 1: Instalación del toolchain
    titulo_seccion(ui, "Instalación de Rust con rustup", cyan);
    ui.indent("reto_1_instalacion", |ui| {
        let instalador_rust = if cfg!(target_os = "windows") {
            "winget install --id Rustlang.Rustup -e"
        } else {
            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        };
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Rustup es la forma recomendada de instalar y mantener Rust. También instala rustc y Cargo. El comando cambia según el sistema operativo.",
            )
            .size(13.0)
            .color(text_col),
        );
        ui.add_space(6.0);

        codigo_terminal_bloque(ui, instalador_rust);
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(
                if cfg!(target_os = "windows") {
                    "En Windows, ejecútalo desde PowerShell y vuelve a abrir la terminal para actualizar el PATH."
                } else {
                    "Sigue las instrucciones del instalador y reinicia la terminal para que el PATH incluya ~/.cargo/bin."
                },
            )
            .size(12.5)
            .color(egui::Color32::from_rgb(160, 180, 205)),
        );
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(
                egui::RichText::new("Después verifica:")
                    .size(13.0)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "rustc --version");
            codigo_inline_chip(ui, "cargo --version");
        });
        ui.add_space(14.0);
    });

    // Sección 1: Verificación del Toolchain
    titulo_seccion(ui, "Verificación del Entorno", cyan);
    ui.indent("reto_1_entorno", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Abre la terminal integrada de FerrisKey para verificar que el compilador y el gestor de paquetes estén disponibles:")
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(6.0);

        mostrar_acceso_terminal(ui, state, orange, cyan);
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Ejecuta en la terminal").size(13.0).color(text_col));
            codigo_inline_chip(ui, "rustc --version");
            ui.label(egui::RichText::new("y").size(13.0).color(text_col));
            codigo_inline_chip(ui, "cargo --version");
            ui.label(egui::RichText::new("para comprobar las herramientas.").size(13.0).color(text_col));
        });
        ui.add_space(14.0);
    });

    // Sección 2: Binary Crate vs Library Crate
    titulo_seccion(ui, "Binary Crate vs Library Crate", cyan);
    ui.indent("reto_1_crates", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "En Rust, un Crate es la unidad mínima de código que compila rustc:",
            )
            .size(13.0)
            .color(text_col),
        );
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(egui::RichText::new("Binary Crate:").strong().color(orange));
            ui.label(egui::RichText::new("Su raíz es").size(13.0).color(text_col));
            codigo_inline_chip(ui, "src/main.rs");
            ui.label(egui::RichText::new("y contiene").size(13.0).color(text_col));
            codigo_inline_chip(ui, "fn main() {}");
            ui.label(
                egui::RichText::new(". Produce un binario ejecutable.")
                    .size(13.0)
                    .color(text_col),
            );
        });
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(egui::RichText::new("Library Crate:").strong().color(orange));
            ui.label(egui::RichText::new("Su raíz es").size(13.0).color(text_col));
            codigo_inline_chip(ui, "src/lib.rs");
            ui.label(
                egui::RichText::new(". Expone funciones y structs públicos con")
                    .size(13.0)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "pub");
            ui.label(
                egui::RichText::new("para ser reutilizados por otros paquetes.")
                    .size(13.0)
                    .color(text_col),
            );
        });
        ui.add_space(14.0);
    });

    // Sección 3: Compile time vs Run time
    mostrar_fases_compile_run(ui, state, orange, cyan, text_col);

    // Sección 4: Comandos de Creación
    titulo_seccion(ui, "Comandos de Creación en Cargo", cyan);
    ui.indent("reto_1_comandos", |ui| {
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(
                egui::RichText::new("Para crear un ejecutable:")
                    .size(13.0)
                    .color(text_col),
            );
            ui.add_space(4.0);
            codigo_inline_chip(ui, "cargo new mi_aplicacion");
        });
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(
                egui::RichText::new("Para crear una librería:")
                    .size(13.0)
                    .color(text_col),
            );
            ui.add_space(4.0);
            codigo_inline_chip(ui, "cargo new mi_libreria --lib");
        });
        ui.add_space(14.0);
    });

    // Sección 5: Instrucciones del Editor
    titulo_seccion(ui, "Tu Tarea", orange);
    ui.indent("reto_1_tarea", |ui| {
        ui.add_space(6.0);

        let tareas = [
            "1. Abre la terminal con el botón superior o presionando Ctrl + T.",
            "2. Crea un proyecto con cargo new mi_proyecto.",
            "3. Selecciónalo en el combo de la barra superior del editor para cargar sus archivos.",
            "4. O escribe código directamente en el editor y presiona Ejecutar (F5 / Ctrl + Enter).",
        ];

        for tarea in tareas {
            ui.label(
                egui::RichText::new(tarea)
                    .size(13.0)
                    .color(text_col),
            );
            ui.add_space(4.0);
        }
        ui.add_space(10.0);
    });
}

fn mostrar_fases_compile_run(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
    text_col: egui::Color32,
) {
    titulo_seccion(ui, "Compile time vs Run time", cyan);
    ui.indent("reto_1_fases_compile_run", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Un programa Rust pasa por dos momentos: primero se revisa y se construye; después el binario se ejecuta.",
            )
            .size(13.0)
            .color(text_col)
            .line_height(Some(19.0)),
        );
        ui.add_space(8.0);

        ui.columns(2, |cols| {
            let phase_colors = PhaseColors {
                cyan,
                orange,
                text: text_col,
            };
            tarjeta_fase_compile_run(
                &mut cols[0],
                state,
                PhaseContent {
                    title: "Compile time",
                    moment: "Antes de ejecutar",
                    description: "Cargo + rustc construyen el binario.",
                    modal: 4,
                },
                phase_colors,
            );
            tarjeta_fase_compile_run(
                &mut cols[1],
                state,
                PhaseContent {
                    title: "Run time",
                    moment: "Programa en marcha",
                    description: "La CPU ejecuta el binario.",
                    modal: 5,
                },
                phase_colors,
            );
        });

        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new("cargo run conecta ambas fases:")
                    .size(12.5)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "cargo run");
            ui.label(
                egui::RichText::new("compila si hace falta y luego ejecuta.")
                    .size(12.5)
                    .color(text_col),
            );
        });
        ui.add_space(10.0);
    });
}

#[derive(Clone, Copy)]
struct PhaseContent<'a> {
    title: &'a str,
    moment: &'a str,
    description: &'a str,
    modal: usize,
}

#[derive(Clone, Copy)]
struct PhaseColors {
    cyan: egui::Color32,
    orange: egui::Color32,
    text: egui::Color32,
}

fn tarjeta_fase_compile_run(
    ui: &mut egui::Ui,
    state: &mut AppState,
    content: PhaseContent<'_>,
    colors: PhaseColors,
) {
    let abierto = state.ui.show_railroad_modal == Some(content.modal);
    let es_compile_time = content.title == "Compile time";
    let fase_color = if es_compile_time {
        colors.cyan
    } else {
        colors.orange
    };
    // Ambas fases comparten una altura fija para que el bloque se perciba como
    // una sola unidad y no como dos tarjetas de tamaños distintos.
    const ALTURA_FASE: f32 = 100.0;
    let (card_rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ALTURA_FASE),
        egui::Sense::hover(),
    );

    ui.painter().rect(
        card_rect,
        egui::CornerRadius::same(6),
        egui::Color32::from_rgb(17, 23, 34),
        egui::Stroke::new(
            1.0,
            if abierto {
                egui::Color32::from_rgba_unmultiplied(255, 180, 80, 150)
            } else {
                egui::Color32::from_rgb(38, 54, 78)
            },
        ),
        egui::StrokeKind::Inside,
    );

    let content_rect = card_rect.shrink(8.0);
    let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
    // Conservar también el recorte del ScrollArea. Usar solamente card_rect
    // permitiría que la tarjeta se pintara por encima del footer global al
    // llegar al límite inferior del drawer.
    content_ui.set_clip_rect(card_rect.intersect(ui.clip_rect()));

    content_ui.horizontal(|ui| {
        let (dot_rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 18.0), egui::Sense::hover());
        ui.painter().circle_filled(
            egui::pos2(dot_rect.center().x, dot_rect.center().y),
            3.0,
            fase_color,
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(content.title)
                .strong()
                .color(egui::Color32::WHITE),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Control de vista minimalista: el marco solo aparece durante
            // hover y conserva las esquinas redondeadas de FerrisKey.
            let (icon_button_rect, respuesta) =
                ui.allocate_exact_size(egui::vec2(24.0, 22.0), egui::Sense::click());
            let hovered = respuesta.hovered();
            let pressed = respuesta.is_pointer_button_down_on();

            if hovered || abierto {
                let hover_fill = if pressed {
                    egui::Color32::from_rgba_unmultiplied(255, 160, 50, 48)
                } else if abierto {
                    egui::Color32::from_rgba_unmultiplied(255, 160, 50, 28)
                } else {
                    egui::Color32::from_rgba_unmultiplied(255, 160, 50, 22)
                };
                let hover_stroke = if pressed {
                    colors.orange
                } else if abierto {
                    egui::Color32::from_rgba_unmultiplied(255, 180, 80, 125)
                } else {
                    egui::Color32::from_rgba_unmultiplied(255, 180, 80, 150)
                };
                ui.painter().rect(
                    icon_button_rect,
                    egui::CornerRadius::same(5),
                    hover_fill,
                    egui::Stroke::new(1.0, hover_stroke),
                    egui::StrokeKind::Inside,
                );
            }

            let icon_color = if abierto || hovered {
                colors.orange
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            };
            egui::Image::from_bytes(
                "bytes://view.svg",
                include_bytes!("../../../../assets/diagramas/view.svg"),
            )
            .fit_to_exact_size(egui::vec2(16.0, 16.0))
            .tint(icon_color)
            .paint_at(
                ui,
                egui::Rect::from_center_size(icon_button_rect.center(), egui::vec2(16.0, 16.0)),
            );

            if respuesta.clicked() {
                state.ui.show_railroad_modal = if abierto { None } else { Some(content.modal) };
            }
            respuesta.on_hover_text(format!("Abrir diagrama de {}", content.title));
        });
    });
    content_ui.add_space(2.0);
    content_ui.label(
        egui::RichText::new(content.moment)
            .size(11.0)
            .strong()
            .color(fase_color),
    );
    content_ui.add_space(5.0);
    content_ui.label(
        egui::RichText::new(content.description)
            .size(12.0)
            .color(colors.text)
            .line_height(Some(17.0)),
    );
    content_ui.add_space(5.0);
}

fn mostrar_reto_2_cargotoml(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
    text_col: egui::Color32,
    bullet_col: egui::Color32,
) {
    ui.heading(
        egui::RichText::new("2. Manifiesto Cargo.toml y Dependencias")
            .size(19.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    // Dificultad / Tags
    ui.horizontal(|ui| {
        let tag_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgb(38, 30, 16))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Intermedio")
                    .size(11.5)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 180, 70)),
            );
        });

        ui.add_space(4.0);

        let tag_topic = egui::Frame::new()
            .fill(egui::Color32::from_rgb(22, 28, 38))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_topic.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Cargo.toml & Crates")
                    .size(11.5)
                    .color(egui::Color32::from_rgb(160, 185, 220)),
            );
        });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("Domina la configuración de tu paquete en Rust y aprende a incorporar crates externos del ecosistema con el comando 'cargo add'.")
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
    );
    ui.add_space(14.0);

    // Sección 1: Anatomía de Cargo.toml
    titulo_seccion(ui, "Anatomía de Cargo.toml", cyan);
    ui.indent("reto_2_anatomia", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("El manifiesto se escribe en formato TOML y se divide en tablas de configuración esenciales:")
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(6.0);

        let cargo_toml_disponible = state
            .project.selected_project
            .as_deref()
            .map(|project| buscar_ruta_proyecto(&state.terminal.term_cwd, project).join("Cargo.toml").is_file())
            .unwrap_or(false);
        let cargo_toml_activo = state.project.selected_file.as_deref() == Some("Cargo.toml");

        ui.horizontal_wrapped(|ui| {
            let abrir_cargo = boton_archivo_codelab(
                ui,
                cargo_toml_disponible,
                cargo_toml_activo,
                cyan,
                "Cargo.toml",
            );
            if abrir_cargo.clicked() {
                state.project.selected_file = Some("Cargo.toml".to_string());
                state.cargar_archivo_proyecto_activo();
            }

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(if cargo_toml_disponible {
                    "Edita el manifiesto real del proyecto seleccionado."
                } else {
                    "Selecciona primero un proyecto Cargo en la barra superior."
                })
                .size(11.5)
                .color(egui::Color32::from_rgb(160, 180, 205)),
            );
        });
        ui.add_space(10.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            codigo_inline_chip(ui, "[package]");
            ui.label(egui::RichText::new("Contiene los metadatos de identidad de tu paquete:").size(13.0).color(text_col));
        });
        ui.add_space(4.0);

        let metadatos = [
            ("name", "Nombre del paquete; normalmente en minúsculas. Los guiones se convierten en '_' al importarlo."),
            ("version", "Versión según SemVer: MAJOR.MINOR.PATCH (ej: 0.1.0)."),
            ("edition", "Edición del compilador de Rust (ej: 2021 o 2024)."),
            ("authors", "Lista opcional de autores; está deprecated, pero se conserva por compatibilidad."),
            ("description", "Resumen breve para búsqueda en crates.io."),
            ("license", "Identificador SPDX de licencia (ej: MIT OR Apache-2.0)."),
            ("rust-version", "Versión mínima de Rust compatible con el paquete."),
            ("readme / repository", "Enlaces o rutas para documentar y localizar el proyecto."),
            ("keywords / categories", "Ayudan a clasificar y encontrar el paquete en crates.io."),
        ];

        for (campo, desc) in metadatos {
            ui.horizontal_wrapped(|ui| {
                ui.add_space(16.0);
                ui.label(egui::RichText::new(format!("- {}:", campo)).monospace().strong().color(orange));
                ui.label(egui::RichText::new(desc).size(12.5).color(text_col));
            });
            ui.add_space(2.0);
        }
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Pero Cargo.toml no es solo metadata. También puede declarar dependencias, features, perfiles de compilación, targets (binarios, librerías y tests), workspaces, lints y configuración para herramientas externas.")
                .size(12.5)
                .color(egui::Color32::from_rgb(160, 180, 205))
                .line_height(Some(18.0)),
        );
        ui.add_space(14.0);
    });

    // Sección 2: Añadir librerías con cargo add
    titulo_seccion(ui, "Añadir Dependencias con cargo add", orange);
    ui.indent("reto_2_dependencias", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("La forma moderna y segura de incorporar crates a tu proyecto es con el comando integrado:")
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Ejecuta en la terminal:").size(13.0).color(text_col));
            ui.add_space(4.0);
            codigo_inline_chip(ui, "cargo add rand");
        });
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Cargo buscará rand en crates.io, añadirá su versión compatible bajo la tabla [dependencies] de Cargo.toml y actualizará Cargo.lock.")
                .size(12.5)
                .color(egui::Color32::from_rgb(160, 180, 205)),
        );
        mostrar_dependencias_en_vivo(ui, state, orange);
        ui.add_space(14.0);
    });

    // Sección 3: Tu Tarea
    titulo_seccion(ui, "Tu Tarea", egui::Color32::from_rgb(78, 205, 132));
    ui.indent("reto_2_tarea", |ui| {
        ui.add_space(6.0);

        let tareas = [
            "1. Usa la terminal integrada para añadir la dependencia rand con cargo add rand.",
            "2. Pulsa 'Mostrar archivo Cargo.toml' para abrir el manifiesto real y ver la tabla [dependencies].",
            "3. Comprueba que rand aparece en [dependencies] y que Cargo.lock fue creado o actualizado.",
        ];

        for tarea in tareas {
            ui.label(
                egui::RichText::new(tarea)
                    .size(13.0)
                    .color(text_col),
            );
            ui.add_space(4.0);
        }
        ui.add_space(10.0);
    });
}

fn mostrar_reto_3_debug_release(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
    text_col: egui::Color32,
    bullet_col: egui::Color32,
) {
    ui.heading(
        egui::RichText::new("3. Ciclo de Compilación: Debug vs Release")
            .size(19.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    // Dificultad / Tags
    ui.horizontal(|ui| {
        let tag_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgb(38, 22, 16))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Esencial")
                    .size(11.5)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 140, 70)),
            );
        });

        ui.add_space(4.0);

        let tag_topic = egui::Frame::new()
            .fill(egui::Color32::from_rgb(22, 28, 38))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_topic.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Build Profiles & Target")
                    .size(11.5)
                    .color(egui::Color32::from_rgb(160, 185, 220)),
            );
        });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("Comprende a fondo el ciclo de compilación en Rust: comprobación ultrarrápida sin enlazado, modo depuración para desarrollo y optimización extrema para producción.")
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
    );
    ui.add_space(14.0);

    // Sección 1: Comprobación con cargo check
    titulo_seccion(ui, "Comprobación Rápida: cargo check", cyan);
    ui.indent("reto_3_check", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Antes de generar un ejecutable completo, Cargo ofrece una comprobación rápida para validar el proyecto:")
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            codigo_inline_chip(ui, "cargo check");
            ui.label(egui::RichText::new("Valida sintaxis, tipos y las reglas del compilador sin generar el binario final.").size(13.0).color(text_col));
        });
        ui.add_space(14.0);
    });

    // Sección 2: Debug vs Release
    titulo_seccion(ui, "Perfiles: Debug vs Release", cyan);
    ui.indent("reto_3_perfiles", |ui| {
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(egui::RichText::new("Perfil Debug (predeterminado):").strong().color(orange));
        });
        ui.add_space(2.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(16.0);
            ui.label(egui::RichText::new("Comando:").size(12.5).color(text_col));
            codigo_inline_chip(ui, "cargo build");
            ui.label(egui::RichText::new("Genera artefactos en").size(12.5).color(text_col));
            codigo_inline_chip(ui, "target/debug/");
        });
        ui.add_space(2.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(16.0);
            ui.label(egui::RichText::new("Incluye símbolos completos de depuración (gdb/lldb) y no optimiza (-O0) para acelerar el tiempo de compilación.").size(12.5).color(text_col));
        });
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(egui::RichText::new("Perfil Release (Producción):").strong().color(orange));
        });
        ui.add_space(2.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(16.0);
            ui.label(egui::RichText::new("Comando:").size(12.5).color(text_col));
            codigo_inline_chip(ui, "cargo build --release");
            ui.label(egui::RichText::new("Genera artefactos en").size(12.5).color(text_col));
            codigo_inline_chip(ui, "target/release/");
        });
        ui.add_space(2.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(16.0);
            ui.label(egui::RichText::new("Aplica optimizaciones agresivas (-O3), vectorización y LTO. El binario corre entre 10x y 50x más rápido.").size(12.5).color(text_col));
        });
        ui.add_space(14.0);
    });

    // Sección 3: Inspección de los artefactos
    titulo_seccion(
        ui,
        "Analizador del binario: tamaño, sistema y enlazado",
        orange,
    );
    ui.indent("reto_3_inspeccion", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Un binario es el ejecutable que Cargo deja después de compilar. Aquí puedes comprobar cuánto ocupa, para qué sistema y arquitectura fue generado y qué bibliotecas necesita para ejecutarse.")
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new("Cada acción responde una pregunta distinta:")
                .size(12.5)
                .strong()
                .color(cyan),
        );
        for (accion, explicacion) in [
            ("Formato", "¿Qué formato, sistema y arquitectura reconoce?"),
            ("Secciones", "¿Cómo se distribuyen el código y los datos?"),
            ("Cabecera", "¿Cuál es el punto de entrada y contiene símbolos de depuración?"),
            ("Dependencias", "¿Qué bibliotecas dinámicas importa?"),
        ] {
            ui.horizontal_wrapped(|ui| {
                ui.add_space(12.0);
                codigo_inline_chip(ui, accion);
                ui.label(egui::RichText::new(explicacion).size(11.5).color(text_col));
            });
        }
        ui.add_space(8.0);

        let Some(project) = state.project.selected_project.clone() else {
            ui.label(
                egui::RichText::new("Selecciona primero un proyecto Cargo en la barra superior para activar el analizador.")
                    .size(12.0)
                    .color(egui::Color32::from_rgb(160, 180, 205)),
            );
            ui.add_space(10.0);
            return;
        };

        let project_dir = buscar_ruta_proyecto(&state.terminal.term_cwd, &project);
        let debug_binario = crate::platform::cargo_binary_path(&project_dir, "debug", &project);
        let release_binario =
            crate::platform::cargo_binary_path(&project_dir, "release", &project);
        let debug_disponible = std::fs::metadata(&debug_binario)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false);
        let release_disponible = std::fs::metadata(&release_binario)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false);
        let hay_binarios = debug_disponible || release_disponible;
        if !hay_binarios {
            ui.label(
                egui::RichText::new("Todavía no hay un ejecutable para analizar. Ejecuta cargo build o cargo build --release en la terminal.")
                    .size(11.5)
                    .color(egui::Color32::from_rgb(160, 180, 205)),
            );
        }

        ui.add_space(8.0);
        mostrar_ficha_binario(
            ui,
            state,
            &project_dir,
            &project,
            BinaryProfile {
                name: "Debug",
                folder: "debug",
                available: debug_disponible,
            },
            BinaryCardStyle {
                accent: orange,
                text: text_col,
            },
        );
        ui.add_space(6.0);
        mostrar_ficha_binario(
            ui,
            state,
            &project_dir,
            &project,
            BinaryProfile {
                name: "Release",
                folder: "release",
                available: release_disponible,
            },
            BinaryCardStyle {
                accent: orange,
                text: text_col,
            },
        );

        ui.add_space(14.0);
    });

    // Sección 4: Tu Tarea
    titulo_seccion(ui, "Tu Tarea", egui::Color32::from_rgb(255, 160, 50));
    ui.indent("reto_3_tarea", |ui| {
        ui.add_space(6.0);

        let tareas = [
            "1. Ejecuta cargo check en la terminal para ver la rapidez del análisis estático.",
            "2. Ejecuta cargo build y cargo build --release para generar ambos perfiles.",
            "3. Observa qué perfil aparece como Disponible y compara su tamaño.",
            "4. Pulsa Formato, Secciones, Cabecera o Dependencias para investigar el binario real.",
        ];

        for tarea in tareas {
            ui.label(egui::RichText::new(tarea).size(13.0).color(text_col));
            ui.add_space(4.0);
        }
        ui.add_space(10.0);
    });
}

fn mostrar_reto_4_optimizacion(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
    text_col: egui::Color32,
    bullet_col: egui::Color32,
) {
    ui.heading(
        egui::RichText::new("4. Optimización breve con Cargo")
            .size(19.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        let tag_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgb(38, 30, 16))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Intermedio")
                    .size(11.5)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 180, 70)),
            );
        });

        ui.add_space(4.0);

        let tag_topic = egui::Frame::new()
            .fill(egui::Color32::from_rgb(22, 28, 38))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_topic.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Cargo Profiles")
                    .size(11.5)
                    .color(egui::Color32::from_rgb(160, 185, 220)),
            );
        });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("Ya mediste la diferencia entre Debug y Release. Ahora aprenderás a ajustar el perfil de compilación desde Cargo.toml y a comprobar si el cambio realmente ayuda.")
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
    );
    ui.add_space(14.0);

    titulo_seccion(ui, "Configuración de perfiles en Cargo.toml", cyan);
    ui.indent("reto_4_perfiles", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Estas opciones no son metadata de publicación: configuran cómo Cargo construye el programa.")
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(6.0);

        let cargo_toml_disponible = state
            .project.selected_project
            .as_deref()
            .map(|project| buscar_ruta_proyecto(&state.terminal.term_cwd, project).join("Cargo.toml").is_file())
            .unwrap_or(false);
        let cargo_toml_activo = state.project.selected_file.as_deref() == Some("Cargo.toml");
        let abrir_cargo = boton_archivo_codelab(
            ui,
            cargo_toml_disponible,
            cargo_toml_activo,
            cyan,
            "Cargo.toml",
        );
        if abrir_cargo.clicked() {
            state.project.selected_file = Some("Cargo.toml".to_string());
            state.cargar_archivo_proyecto_activo();
        }
        ui.add_space(8.0);

        let theme = state.editor.theme_set.themes["base16-ocean.dark"].clone();
        codigo_toml_bloque(
            ui,
            "[profile.dev]\nopt-level = 0\ndebug = true\n\n[profile.release]\nopt-level = 3\nlto = true\ncodegen-units = 1\nstrip = \"symbols\"",
            &state.editor.syntax_set,
            &theme,
        );
        ui.add_space(6.0);

        for (opcion, descripcion) in [
            ("opt-level = 3", "prioriza el rendimiento del binario release."),
            ("lto = true", "permite optimizaciones entre módulos durante el enlazado."),
            ("codegen-units = 1", "puede mejorar la optimización final, aunque compila más lento."),
            ("strip = \"symbols\"", "reduce símbolos innecesarios y el tamaño distribuido."),
        ] {
            ui.horizontal_wrapped(|ui| {
                ui.add_space(16.0);
                codigo_inline_chip(ui, opcion);
                ui.label(egui::RichText::new(descripcion).size(12.5).color(text_col));
            });
            ui.add_space(2.0);
        }
        ui.add_space(14.0);
    });

    titulo_seccion(ui, "Optimizar no significa activar todo", orange);
    ui.indent("reto_4_medicion", |ui| {
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            codigo_inline_chip(ui, "cargo build --release");
            ui.label(egui::RichText::new("puede tardar más porque busca un binario mejor optimizado.").size(12.5).color(text_col));
        });
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            codigo_inline_chip(ui, "ls -lh");
            ui.label(egui::RichText::new("permite comparar el tamaño antes y después.").size(12.5).color(text_col));
        });
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            ui.label(egui::RichText::new("Una configuración más agresiva puede mejorar tamaño o velocidad, pero siempre debe medirse.").size(12.5).color(text_col));
        });
        ui.add_space(14.0);
    });

    titulo_seccion(ui, "Tu Tarea", egui::Color32::from_rgb(78, 205, 132));
    ui.indent("reto_4_tarea", |ui| {
        ui.add_space(6.0);
        let tareas = [
            "1. Abre el Cargo.toml real con 'Mostrar archivo Cargo.toml'.",
            "2. Añade lto = true dentro de [profile.release].",
            "3. Ejecuta cargo build --release para generar el binario optimizado.",
            "4. Regresa a la sesión 3 y pulsa 'Inspeccionar binarios' para comparar el resultado.",
        ];

        for tarea in tareas {
            ui.label(egui::RichText::new(tarea).size(13.0).color(text_col));
            ui.add_space(4.0);
        }
        ui.add_space(10.0);
    });
}

fn mostrar_reto_5_toolchain(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
    text_col: egui::Color32,
    bullet_col: egui::Color32,
) {
    ui.heading(
        egui::RichText::new("5. Configuración del Toolchain: rust-toolchain.toml")
            .size(19.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        let tag_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgb(38, 22, 16))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Avanzado")
                    .size(11.5)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 140, 70)),
            );
        });

        ui.add_space(4.0);

        let tag_topic = egui::Frame::new()
            .fill(egui::Color32::from_rgb(22, 28, 38))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_topic.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Rustup & Reproducibilidad")
                    .size(11.5)
                    .color(egui::Color32::from_rgb(160, 185, 220)),
            );
        });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("Un proyecto puede declarar qué toolchain necesita para que todo el equipo compile con una configuración coherente.")
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
    );
    ui.add_space(14.0);

    titulo_seccion(ui, "Archivo rust-toolchain.toml", cyan);
    ui.indent("reto_5_archivo_toolchain", |ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Crea este archivo en la raíz del proyecto, junto a Cargo.toml:")
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(6.0);
        let toolchain_disponible = state
            .project.selected_project
            .as_deref()
            .map(|project| {
                buscar_ruta_proyecto(&state.terminal.term_cwd, project)
                    .join("rust-toolchain.toml")
                    .is_file()
            })
            .unwrap_or(false);
        let toolchain_activo = state.project.selected_file.as_deref() == Some("rust-toolchain.toml");
        let abrir_toolchain = boton_archivo_codelab(
            ui,
            toolchain_disponible,
            toolchain_activo,
            cyan,
            "rust-toolchain.toml",
        );
        if abrir_toolchain.clicked() {
            state.project.selected_file = Some("rust-toolchain.toml".to_string());
            state.cargar_archivo_proyecto_activo();
        }
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(if toolchain_disponible {
                "Archivo detectado: puedes editarlo directamente en el editor."
            } else {
                "Crea el archivo con touch rust-toolchain.toml y luego selecciónalo en el explorador."
            })
            .size(12.0)
            .color(egui::Color32::from_rgb(155, 175, 200)),
        );
        ui.add_space(6.0);
        let theme = state.editor.theme_set.themes["base16-ocean.dark"].clone();
        codigo_toml_bloque(
            ui,
            "[toolchain]\nchannel = \"stable\"\ncomponents = [\"rustfmt\", \"clippy\"]\nprofile = \"minimal\"",
            &state.editor.syntax_set,
            &theme,
        );
        ui.add_space(6.0);

        for (campo, descripcion) in [
            ("channel", "Canal o versión de Rust que usará el proyecto."),
            ("components", "Herramientas adicionales como rustfmt y clippy."),
            ("targets", "Plataformas adicionales para compilación cruzada."),
            ("profile", "Cantidad de componentes instalados: minimal, default o complete."),
        ] {
            ui.horizontal_wrapped(|ui| {
                ui.add_space(16.0);
                codigo_inline_chip(ui, campo);
                ui.label(egui::RichText::new(descripcion).size(12.5).color(text_col));
            });
            ui.add_space(2.0);
        }
        ui.add_space(14.0);
    });

    titulo_seccion(ui, "Comandos esenciales de rustup", orange);
    ui.indent("reto_5_comandos_rustup", |ui| {
        ui.add_space(4.0);
        codigo_terminal_bloque(
            ui,
            "rustup show\nrustup toolchain list\nrustup default stable\nrustup update\nrustup override set nightly\nrustup target list",
        );
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            codigo_inline_chip(ui, "rustup show");
            ui.label(egui::RichText::new("muestra el toolchain activo y sus componentes.").size(12.5).color(text_col));
        });
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            codigo_inline_chip(ui, "rustup toolchain list");
            ui.label(egui::RichText::new("lista las versiones instaladas.").size(12.5).color(text_col));
        });
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, bullet_col);
            codigo_inline_chip(ui, "rustup override set nightly");
            ui.label(egui::RichText::new("cambia el canal solo para el proyecto actual.").size(12.5).color(text_col));
        });
        ui.add_space(14.0);
    });

    titulo_seccion(ui, "Versiones que no debes confundir", cyan);
    ui.indent("reto_5_conceptos", |ui| {
        ui.add_space(4.0);
        for (concepto, descripcion) in [
            ("rustc --version", "Versión del compilador instalado."),
            (
                "edition = \"2024\"",
                "Reglas del lenguaje que utiliza el proyecto.",
            ),
            ("rand = \"0.9\"", "Versión de una dependencia externa."),
            ("stable / beta / nightly", "Canal del toolchain de Rust."),
        ] {
            ui.horizontal_wrapped(|ui| {
                ui.add_space(16.0);
                codigo_inline_chip(ui, concepto);
                ui.label(egui::RichText::new(descripcion).size(12.5).color(text_col));
            });
            ui.add_space(2.0);
        }
        ui.add_space(14.0);
    });

    titulo_seccion(ui, "Tu Tarea", egui::Color32::from_rgb(78, 205, 132));
    ui.indent("reto_5_tarea", |ui| {
        ui.add_space(6.0);
        let tareas = [
            "1. Ejecuta rustup show y rustup toolchain list para conocer tu entorno.",
            "2. Crea rust-toolchain.toml en la raíz del proyecto con el bloque mostrado.",
            "3. Ejecuta cargo check y observa que el proyecto utiliza la configuración declarada.",
            "4. Recuerda: el toolchain, la edition y la versión de una crate son conceptos diferentes.",
        ];

        for tarea in tareas {
            ui.label(egui::RichText::new(tarea).size(13.0).color(text_col));
            ui.add_space(4.0);
        }
        ui.add_space(10.0);
    });
}

fn mostrar_reto_6_resumen(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
    text_col: egui::Color32,
    bullet_col: egui::Color32,
) {
    ui.heading(
        egui::RichText::new("6. Resumen final")
            .size(19.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        let tag_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgb(38, 30, 16))
            .inner_margin(egui::Margin::symmetric(8, 2))
            .corner_radius(egui::CornerRadius::same(10));
        tag_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Cierre")
                    .size(11.5)
                    .strong()
                    .color(orange),
            );
        });
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Rust Foundations · Cargo")
                .size(11.5)
                .color(egui::Color32::from_rgb(160, 185, 220)),
        );
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(
            "Antes de continuar, comprueba que puedes explicar el recorrido completo de un proyecto Rust.",
        )
        .size(13.5)
        .color(text_col)
        .line_height(Some(19.0)),
    );
    ui.add_space(14.0);

    titulo_seccion(ui, "Lo que aprendiste", cyan);
    ui.indent("reto_6_resumen_conceptos", |ui| {
        ui.add_space(4.0);
        for (concepto, descripcion) in [
            ("rustup", "instala y administra los toolchains de Rust."),
            (
                "cargo new",
                "crea un package con su crate binario o de librería.",
            ),
            (
                "Cargo.toml",
                "define metadata, dependencias y perfiles de compilación.",
            ),
            (
                "Compile time / Run time",
                "distingue entre construir el programa y ejecutarlo.",
            ),
            (
                "cargo check / build / run",
                "comprueba, compila y ejecuta el proyecto.",
            ),
            (
                "dev / release",
                "elige entre iterar rápido o compilar con optimizaciones.",
            ),
            (
                "rust-toolchain.toml",
                "declara el toolchain que debe usar el proyecto.",
            ),
        ] {
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                codigo_inline_chip(ui, concepto);
                ui.label(egui::RichText::new(descripcion).size(12.5).color(text_col));
            });
            ui.add_space(3.0);
        }
        ui.add_space(10.0);
    });

    titulo_seccion(ui, "Comprueba lo aprendido", orange);
    ui.indent("reto_6_preguntas", |ui| {
        ui.add_space(4.0);
        let preguntas = [
            (
                "1. ¿Qué herramienta administra los toolchains de Rust?",
                ["Cargo", "rustup", "rustc"],
                1,
            ),
            (
                "2. ¿Qué crea cargo new mi_app por defecto?",
                [
                    "Un package con crate binario",
                    "Solo una librería",
                    "Un archivo Cargo.lock vacío",
                ],
                0,
            ),
            (
                "3. ¿Dónde se declaran las dependencias del proyecto?",
                ["En src/main.rs", "En target/", "En Cargo.toml"],
                2,
            ),
            (
                "4. ¿Qué hace cargo check?",
                [
                    "Comprueba el código sin generar un binario final",
                    "Publica el crate en crates.io",
                    "Cambia el toolchain activo",
                ],
                0,
            ),
            (
                "5. ¿Qué archivo puede fijar el toolchain del proyecto?",
                ["Cargo.lock", "rust-toolchain.toml", "build.rs"],
                1,
            ),
        ];

        for (index, (pregunta, opciones, correcta)) in preguntas.iter().enumerate() {
            ui.label(
                egui::RichText::new(*pregunta)
                    .strong()
                    .size(13.0)
                    .color(text_col),
            );
            ui.add_space(4.0);

            for (opcion_index, opcion) in opciones.iter().enumerate() {
                let seleccionada = state.lessons.codelab_respuestas[index] == Some(opcion_index);
                let (opcion_rect, respuesta) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 25.0),
                    egui::Sense::click(),
                );
                let hovered = respuesta.hovered();
                let opcion_fill = if seleccionada {
                    egui::Color32::from_rgba_unmultiplied(255, 160, 50, 32)
                } else if hovered {
                    egui::Color32::from_rgba_unmultiplied(255, 160, 50, 18)
                } else {
                    egui::Color32::from_rgb(15, 20, 30)
                };
                let opcion_stroke = if seleccionada {
                    orange
                } else if hovered {
                    egui::Color32::from_rgba_unmultiplied(255, 180, 80, 150)
                } else {
                    egui::Color32::from_rgb(35, 48, 70)
                };
                ui.painter().rect(
                    opcion_rect,
                    egui::CornerRadius::same(4),
                    opcion_fill,
                    egui::Stroke::new(if seleccionada { 1.2 } else { 1.0 }, opcion_stroke),
                    egui::StrokeKind::Inside,
                );
                ui.painter().text(
                    egui::pos2(opcion_rect.left() + 10.0, opcion_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    *opcion,
                    egui::FontId::proportional(12.5),
                    if seleccionada { orange } else { text_col },
                );
                if respuesta.clicked() {
                    state.lessons.codelab_respuestas[index] = Some(opcion_index);
                }
                ui.add_space(3.0);
            }

            if let Some(respuesta) = state.lessons.codelab_respuestas[index] {
                let es_correcta = respuesta == *correcta;
                ui.label(
                    egui::RichText::new(if es_correcta {
                        "Correcto"
                    } else {
                        "Revisa este concepto y vuelve a intentarlo"
                    })
                    .size(11.5)
                    .color(if es_correcta {
                        egui::Color32::from_rgb(255, 180, 80)
                    } else {
                        egui::Color32::from_rgb(255, 150, 100)
                    }),
                );
            }
            ui.add_space(10.0);
        }
    });

    let respondidas = state
        .lessons
        .codelab_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = state
        .lessons
        .codelab_respuestas
        .iter()
        .zip([1, 0, 2, 0, 1])
        .filter(|(respuesta, correcta)| respuesta == &&Some(*correcta))
        .count();
    let completado = respondidas == state.lessons.codelab_respuestas.len() && correctas == 5;
    state.lessons.codelab_retos_completados[5] = completado;

    ui.add_space(4.0);
    titulo_seccion(
        ui,
        if completado {
            "Sesión completada"
        } else {
            "Progreso de la sesión"
        },
        orange,
    );
    ui.indent("reto_6_estado", |ui| {
        ui.add_space(4.0);
        let estado = if completado {
            "¡Listo! Ya puedes pasar al siguiente módulo."
        } else {
            "Responde las cinco preguntas correctamente para completar la sesión."
        };
        let estado_texto = if completado {
            estado.to_string()
        } else {
            format!("{estado}  ({correctas}/5 correctas)")
        };
        ui.label(
            egui::RichText::new(estado_texto)
                .size(13.0)
                .color(if completado {
                    egui::Color32::from_rgb(255, 180, 80)
                } else {
                    text_col
                }),
        );
        ui.add_space(8.0);
    });
}

pub(crate) fn codigo_inline_chip(ui: &mut egui::Ui, code: &str) {
    let mut chip = egui::Frame::new();
    chip.fill = egui::Color32::from_rgb(26, 32, 44);
    chip.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 90));
    chip.corner_radius = egui::CornerRadius::same(3);
    chip.inner_margin = egui::Margin::symmetric(6, 2);

    chip.show(ui, |ui| {
        ui.add(
            egui::Label::new(
                egui::RichText::new(code)
                    .monospace()
                    .size(12.0)
                    .color(egui::Color32::from_rgb(255, 180, 80)),
            )
            .extend(),
        );
    });
}

fn boton_archivo_codelab(
    ui: &mut egui::Ui,
    habilitado: bool,
    activo: bool,
    cyan: egui::Color32,
    archivo: &str,
) -> egui::Response {
    let label = format!("Mostrar archivo {archivo}");
    let label_galley = ui.painter().layout_no_wrap(
        label.clone(),
        egui::FontId::proportional(11.5),
        egui::Color32::WHITE,
    );
    let button_width = (label_galley.size().x + 44.0)
        .max(196.0)
        .min(ui.available_width());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(button_width, 28.0),
        if habilitado {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    let fill = if !habilitado {
        egui::Color32::from_rgb(18, 22, 30)
    } else if pressed {
        egui::Color32::from_rgb(25, 64, 78)
    } else if hovered || activo {
        egui::Color32::from_rgb(22, 52, 68)
    } else {
        egui::Color32::from_rgb(16, 28, 40)
    };
    let stroke = if !habilitado {
        egui::Color32::from_rgb(31, 39, 53)
    } else if hovered || activo {
        cyan
    } else {
        egui::Color32::from_rgb(48, 92, 112)
    };
    let text_color = if !habilitado {
        egui::Color32::from_rgb(75, 88, 108)
    } else if hovered || activo {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(205, 225, 235)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(5),
        fill,
        egui::Stroke::new(1.0, stroke),
        egui::StrokeKind::Inside,
    );

    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 17.0, rect.center().y),
        egui::vec2(14.0, 14.0),
    );
    let img_file = egui::Image::new(egui::include_image!(
        "../../../../assets/icons/file-svgrepo-com.svg"
    ))
    .fit_to_exact_size(egui::vec2(14.0, 14.0))
    .tint(if habilitado {
        cyan
    } else {
        egui::Color32::from_rgb(75, 88, 108)
    });
    img_file.paint_at(ui, icon_rect);

    ui.painter().text(
        egui::pos2(rect.left() + 30.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(11.5),
        text_color,
    );

    response.on_hover_text(if habilitado {
        format!("Seleccionar {archivo} y cargarlo en el editor")
    } else {
        "Selecciona un proyecto Cargo para habilitar esta acción".to_string()
    })
}

fn mostrar_dependencias_en_vivo(ui: &mut egui::Ui, state: &AppState, accent: egui::Color32) {
    let dependencias = leer_dependencias_cargo(state);

    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        punto_lista(ui, accent);
        codigo_inline_chip(ui, "[dependencies]");
    });

    if dependencias.is_empty() {
        let mensaje = if state.project.selected_project.is_some() {
            "Todavía no hay dependencias declaradas. Ejecuta cargo add <crate> para añadir una."
        } else {
            "Selecciona un proyecto Cargo para consultar sus dependencias."
        };
        ui.label(
            egui::RichText::new(mensaje)
                .size(12.0)
                .color(egui::Color32::from_rgb(150, 170, 195)),
        );
        return;
    }

    for (nombre, especificacion) in dependencias {
        ui.horizontal_wrapped(|ui| {
            ui.add_space(16.0);
            let dependencia = format!("{nombre} = {especificacion}");
            codigo_inline_chip(ui, &dependencia);
        });
    }
}

fn leer_dependencias_cargo(state: &AppState) -> Vec<(String, String)> {
    let Some(project) = state.project.selected_project.as_deref() else {
        return Vec::new();
    };

    let cargo_path = buscar_ruta_proyecto(&state.terminal.term_cwd, project).join("Cargo.toml");
    let Ok(content) = std::fs::read_to_string(cargo_path) else {
        return Vec::new();
    };

    let mut in_dependencies = false;
    let mut dependencies = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with('[') {
            in_dependencies = trimmed == "[dependencies]";
            continue;
        }

        if !in_dependencies {
            continue;
        }

        let Some((name, value)) = trimmed.split_once('=') else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }

        let value = value.trim();
        if !value.is_empty() {
            dependencies.push((name.to_string(), value.to_string()));
        }
    }

    dependencies
}

struct ItemExplicacionCargo {
    categoria: &'static str,
    resumen: &'static str,
    proposito: &'static str,
    cargo_interaccion: &'static str,
    buenas_practicas: &'static str,
    comando_relacionado: &'static str,
}

fn obtener_explicacion_item(relative: &str) -> ItemExplicacionCargo {
    let clean = relative.trim_end_matches('/');
    match clean {
        "" | "." => ItemExplicacionCargo {
            categoria: "Raíz del Paquete Cargo (Workspace)",
            resumen: "Directorio raíz que encapsula todo el proyecto y la configuración del workspace de Cargo.",
            proposito: "Alberga el manifiesto 'Cargo.toml', el código fuente en 'src/', los artefactos compilados en 'target/' y las configuraciones de entorno y control de versiones.",
            cargo_interaccion: "Es el directorio de trabajo (working directory) desde el cual se ejecutan todos los comandos de Cargo ('cargo run', 'cargo test', 'cargo check').",
            buenas_practicas: "Mantén la raíz organizada y limpia: solo deben situarse aquí el manifiesto, lockfile, README, LICENSE y directorios estándar de Cargo.",
            comando_relacionado: "cargo build / cargo run",
        },
        "Cargo.toml" => ItemExplicacionCargo {
            categoria: "Manifiesto del Paquete (Configuración)",
            resumen: "El archivo de configuración y metadatos central de todo proyecto Rust.",
            proposito: "Escrito en formato TOML, especifica metadatos del paquete (nombre, versión, edición), dependencias de producción, dependencias de desarrollo y perfiles de optimización.",
            cargo_interaccion: "Cargo lee este archivo antes de cualquier tarea. Resuelve y descarga automáticamente las dependencias declaradas desde crates.io.",
            buenas_practicas: "Aplica versionado semántico (SemVer). Para agregar dependencias de forma segura sin errores de sintaxis, usa 'cargo add <crate>'.",
            comando_relacionado: "cargo add <crate>",
        },
        "Cargo.lock" => ItemExplicacionCargo {
            categoria: "Árbol de Dependencias Bloqueado",
            resumen: "Registro exacto de todas las dependencias y subdependencias resueltas.",
            proposito: "Registra versiones exactas y sumas de verificación criptográficas (hashes) de cada dependencia descargada para garantizar compilaciones 100% idénticas y reproducibles.",
            cargo_interaccion: "Es gestionado automáticamente por Cargo. Al compilar en un nuevo entorno o servidor CI, Cargo garantiza que nadie reciba versiones con cambios incompatibles.",
            buenas_practicas: "¡Nunca lo edites a mano! En aplicaciones ejecutables se debe versionar en Git; en librerías puras se suele omitir en '.gitignore'.",
            comando_relacionado: "cargo update",
        },
        "rust-toolchain.toml" => ItemExplicacionCargo {
            categoria: "Configuración de Rustup (Toolchain)",
            resumen: "Declara el canal y las herramientas de Rust que necesita este proyecto.",
            proposito: "Permite que rustup seleccione de forma reproducible stable, beta, nightly o una versión concreta al trabajar dentro de este directorio.",
            cargo_interaccion: "Cargo ejecuta el proyecto con el toolchain que rustup resuelve a partir de este archivo, antes de invocar rustc.",
            buenas_practicas: "Guarda el archivo en la raíz del proyecto y comparte solo la configuración que el equipo realmente necesita.",
            comando_relacionado: "rustup show",
        },
        "src" => ItemExplicacionCargo {
            categoria: "Directorio de Código Fuente",
            resumen: "Carpeta estándar donde reside todo el código Rust del proyecto.",
            proposito: "Por convención universal de Cargo, todo el código fuente se ubica aquí. El compilador 'rustc' comienza el análisis y resolución de módulos desde esta carpeta.",
            cargo_interaccion: "Cargo monitorea los archivos dentro de 'src/' para invalidar cachés y recompilar selectivamente los módulos modificados de manera incremental.",
            buenas_practicas: "Mantén una jerarquía modular limpia. Evita colocar aquí binarios, scripts externos o datos pesados.",
            comando_relacionado: "cargo check",
        },
        "src/main.rs" => ItemExplicacionCargo {
            categoria: "Crate Binaria (Punto de Entrada)",
            resumen: "Punto de inicio y raíz del binario ejecutable de la aplicación.",
            proposito: "Contiene la función obligatoria 'fn main()', que es invocada por el sistema operativo al arrancar el programa. Es la raíz del árbol de módulos para aplicaciones ejecutables.",
            cargo_interaccion: "Al ejecutar 'cargo run' o 'cargo build', Cargo localiza este archivo por convención, lo compila con 'rustc' y genera el binario ejecutable final.",
            buenas_practicas: "Mantén 'main.rs' lo más limpio posible; delega la lógica de negocio, structs y algoritmos a submódulos o a una librería ('src/lib.rs').",
            comando_relacionado: "cargo run",
        },
        "src/lib.rs" => ItemExplicacionCargo {
            categoria: "Crate de Librería (Root Library)",
            resumen: "Raíz de una librería reutilizable y definición de su API pública.",
            proposito: "Define los tipos, traits, funciones y módulos que pueden ser importados y reutilizados por otros binarios, submódulos o dependencias externas.",
            cargo_interaccion: "Cargo compila este archivo como una librería '.rlib'. Las pruebas unitarias y ejemplos de documentación (doctests) se ejecutan con 'cargo test'.",
            buenas_practicas: "Declara con 'pub' únicamente los elementos que formen parte de tu contrato público estable. Documenta cada función pública con '///'.",
            comando_relacionado: "cargo test / cargo doc --open",
        },
        "target" => ItemExplicacionCargo {
            categoria: "Directorio de Artefactos de Compilación",
            resumen: "Carpeta de salida temporal donde Cargo almacena los binarios y cachés.",
            proposito: "Alberga los binarios compilados en 'target/debug/' o 'target/release/', metadatos de dependencias intermedias y datos de compilación incremental.",
            cargo_interaccion: "Generado y administrado íntegramente por Cargo. Permite que las compilaciones subsecuentes sean ultra rápidas al reutilizar archivos sin cambios.",
            buenas_practicas: "Debe estar SIEMPRE en '.gitignore'. Puede llegar a ocupar varios gigabytes; para liberar espacio en disco usa 'cargo clean'.",
            comando_relacionado: "cargo clean",
        },
        "tests" => ItemExplicacionCargo {
            categoria: "Pruebas de Integración (Black-Box)",
            resumen: "Directorio para tests de integración que prueban la API pública externa.",
            proposito: "Cada archivo '.rs' dentro de esta carpeta se compila como un crate independiente que consume tu biblioteca como un usuario externo real.",
            cargo_interaccion: "Se compilan y ejecutan automáticamente al invocar 'cargo test'. No se empaquetan en los binarios de producción.",
            buenas_practicas: "Prueba casos de uso completos de punta a punta (end-to-end) y valida la ergonomía de la API pública.",
            comando_relacionado: "cargo test",
        },
        "examples" => ItemExplicacionCargo {
            categoria: "Ejemplos Ejecutables de Demostración",
            resumen: "Programas de ejemplo que muestran cómo utilizar las funciones de tu paquete.",
            proposito: "Cada archivo '.rs' en esta carpeta es un binario ejecutable con su propio 'fn main()', diseñado para guiar y enseñar a otros desarrolladores.",
            cargo_interaccion: "Puedes ejecutar cualquier ejemplo usando 'cargo run --example <nombre>'. Además, 'cargo test' verifica que sigan compilando.",
            buenas_practicas: "Escribe ejemplos autocontenidos, claros y con comentarios explicativos; son la mejor documentación viva de tu código.",
            comando_relacionado: "cargo run --example <nombre>",
        },
        "benches" => ItemExplicacionCargo {
            categoria: "Benchmarks de Rendimiento",
            resumen: "Pruebas de rendimiento para medir tiempos de ejecución y latencias.",
            proposito: "Mide nanosegundos y ciclos de CPU por operación para detectar regresiones de rendimiento y cuellos de botella en el código.",
            cargo_interaccion: "Se ejecutan con 'cargo bench'. Por defecto compilan con optimizaciones completas ('opt-level = 3') en modo release.",
            buenas_practicas: "Utiliza bibliotecas como 'criterion' para generar análisis estadísticos confiables sin interferencia de ruido del SO.",
            comando_relacionado: "cargo bench",
        },
        "build.rs" => ItemExplicacionCargo {
            categoria: "Script de Pre-Construcción (Build Script)",
            resumen: "Script en Rust que se compila y ejecuta antes del resto del paquete.",
            proposito: "Permite enlazar librerías de C/C++, generar código Rust en tiempo de compilación o configurar banderas condicionales ('cfg').",
            cargo_interaccion: "Cargo compila este script para la arquitectura host y lo ejecuta antes de invocar 'rustc' sobre el código principal.",
            buenas_practicas: "Solo úsalo si es estrictamente necesario, ya que incrementa los tiempos de compilación limpia de todo el proyecto.",
            comando_relacionado: "cargo build",
        },
        ".gitignore" => ItemExplicacionCargo {
            categoria: "Control de Versiones (Git)",
            resumen: "Reglas de exclusión para evitar que archivos indeseados se suban al repositorio.",
            proposito: "Evita versionar la carpeta '/target' con sus binarios pesados, claves secretas '.env' o archivos de configuración local del editor.",
            cargo_interaccion: "Cargo crea automáticamente este archivo al iniciar un proyecto con 'cargo new', asegurando buenas prácticas desde el primer commit.",
            buenas_practicas: "Verifica periódicamente que ningún archivo temporal o binario se haya escapado al control de versiones.",
            comando_relacionado: "git status",
        },
        p if p.starts_with("target") => ItemExplicacionCargo {
            categoria: "Artefacto de Compilación Interno",
            resumen: "Archivo binario o metadato generado automáticamente por el compilador.",
            proposito: "Almacenado temporalmente para permitir compilaciones incrementales veloces o ejecución directa.",
            cargo_interaccion: "Administrado por Cargo; se regenera automáticamente en cada 'cargo build'.",
            buenas_practicas: "No edites archivos dentro de target. Si experimentas comportamientos anómalos, ejecuta 'cargo clean'.",
            comando_relacionado: "cargo clean",
        },
        p if p.ends_with(".rs") => ItemExplicacionCargo {
            categoria: "Módulo de Código Rust",
            resumen: "Submódulo de código fuente que compone la arquitectura interna del crate.",
            proposito: "En Rust, cada archivo '.rs' representa un módulo con su propio espacio de nombres ('namespace'), promoviendo la encapsulación.",
            cargo_interaccion: "Para ser compilado por Cargo y rustc, debe ser declarado en 'main.rs' o 'lib.rs' (o en su módulo padre) con 'mod <nombre>;'.",
            buenas_practicas: "Exporta únicamente los tipos y funciones indispensables usando 'pub' o 'pub(crate)'. Mantén el resto privado.",
            comando_relacionado: "cargo check",
        },
        _ if relative.ends_with('/') => ItemExplicacionCargo {
            categoria: "Directorio de Proyecto",
            resumen: "Directorio organizativo para estructurar módulos, recursos o pruebas.",
            proposito: "Agrupa lógica relacionada dentro del sistema de archivos del paquete.",
            cargo_interaccion: "Cargo inspecciona directorios según las convenciones estándar de Rust ('src/', 'tests/', 'examples/', etc.).",
            buenas_practicas: "Mantén nombres en minúsculas con snake_case para compatibilidad universal en Linux, macOS y Windows.",
            comando_relacionado: "cargo check",
        },
        _ => ItemExplicacionCargo {
            categoria: "Archivo del Proyecto",
            resumen: "Archivo perteneciente al repositorio o paquete de Cargo.",
            proposito: "Proporciona datos, documentación o configuración complementaria al proyecto.",
            cargo_interaccion: "Puede ser empaquetado o ignorado según las directivas 'include' y 'exclude' en 'Cargo.toml'.",
            buenas_practicas: "Evita almacenar archivos binarios grandes en el árbol de código fuente.",
            comando_relacionado: "cargo build",
        },
    }
}

fn formato_tamano_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn dibujar_fila_zed(
    ui: &mut egui::Ui,
    nombre: &str,
    badge_tamano: Option<&str>,
    depth: usize,
    is_dir: bool,
    is_open: bool,
    is_selected: bool,
) -> egui::Response {
    let row_height = 24.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::click(),
    );

    let is_hovered = response.hovered();

    // Fondo interactivo estilo Zed
    let bg_color = if is_selected {
        egui::Color32::from_rgb(28, 42, 64)
    } else if is_hovered {
        egui::Color32::from_rgb(20, 27, 40)
    } else {
        egui::Color32::TRANSPARENT
    };

    if bg_color != egui::Color32::TRANSPARENT {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(4), bg_color);
    }

    // Acento izquierdo de selección (estilo Zed/VS Code)
    if is_selected {
        let accent_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(1.0, 2.0),
            egui::vec2(2.5, rect.height() - 4.0),
        );
        ui.painter().rect_filled(
            accent_rect,
            egui::CornerRadius::same(1),
            egui::Color32::from_rgb(100, 200, 255),
        );
    }

    // Guías de indentación (líneas verticales tenues)
    for d in 0..depth {
        let guide_x = rect.left() + 10.0 + (d as f32) * 16.0;
        ui.painter().line_segment(
            [
                egui::pos2(guide_x, rect.top()),
                egui::pos2(guide_x, rect.bottom()),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(26, 36, 52)),
        );
    }

    let indent_x = rect.left() + 10.0 + (depth as f32) * 16.0;

    // Chevron dibujado con líneas para evitar glifos que algunas fuentes muestran como cuadrados
    let text_start_x = if is_dir {
        let chevron_center = egui::pos2(indent_x + 4.0, rect.center().y);
        let chevron_color = if is_hovered || is_selected {
            egui::Color32::from_rgb(200, 220, 245)
        } else {
            egui::Color32::from_rgb(115, 130, 150)
        };
        let chevron_stroke = egui::Stroke::new(1.4, chevron_color);

        if is_open {
            ui.painter().line_segment(
                [
                    chevron_center + egui::vec2(-3.0, -1.0),
                    chevron_center + egui::vec2(0.0, 2.0),
                ],
                chevron_stroke,
            );
            ui.painter().line_segment(
                [
                    chevron_center + egui::vec2(0.0, 2.0),
                    chevron_center + egui::vec2(3.0, -1.0),
                ],
                chevron_stroke,
            );
        } else {
            ui.painter().line_segment(
                [
                    chevron_center + egui::vec2(-1.5, -3.0),
                    chevron_center + egui::vec2(2.0, 0.0),
                ],
                chevron_stroke,
            );
            ui.painter().line_segment(
                [
                    chevron_center + egui::vec2(2.0, 0.0),
                    chevron_center + egui::vec2(-1.5, 3.0),
                ],
                chevron_stroke,
            );
        }
        indent_x + 14.0
    } else {
        indent_x + 14.0
    };

    // Color tipográfico limpio y unificado para todo el árbol (sin arcoíris)
    let text_color = if is_selected {
        egui::Color32::WHITE
    } else if is_hovered {
        egui::Color32::from_rgb(235, 245, 255)
    } else if is_dir {
        egui::Color32::from_rgb(215, 228, 245)
    } else {
        egui::Color32::from_rgb(190, 205, 225)
    };

    let display_name = if is_dir {
        format!("{nombre}/")
    } else {
        nombre.to_string()
    };

    let text_pos = egui::pos2(text_start_x, rect.center().y);
    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        display_name,
        egui::FontId::monospace(11.5),
        text_color,
    );

    // Badge o tamaño a la derecha
    if let Some(badge) = badge_tamano {
        let badge_pos = egui::pos2(rect.right() - 8.0, rect.center().y);
        ui.painter().text(
            badge_pos,
            egui::Align2::RIGHT_CENTER,
            badge,
            egui::FontId::monospace(9.5),
            egui::Color32::from_rgb(110, 125, 145),
        );
    }

    response
}

fn mostrar_arbol_zed(
    ui: &mut egui::Ui,
    root: &Path,
    directory: &Path,
    depth: usize,
    state: &mut AppState,
) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(directory)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.flatten().map(|entry| entry.path()))
        .filter(|path| path.file_name().is_none_or(|name| name != ".git"))
        .collect();

    entries.sort_by_key(|path| {
        (
            !path.is_dir(),
            path.file_name().map(|name| name.to_os_string()),
        )
    });

    for path in &entries {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("?");
        let relative_raw = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        if path.is_dir() {
            let rel_dir = format!("{relative_raw}/");
            let dir_state_id = ui.make_persistent_id(format!("zed_dir_open_{}", rel_dir));

            // Colapsar por defecto 'target', 'debug' y cualquier subdirectorio dentro de target/
            let es_target_o_subdirectorio = name == "target"
                || name == "debug"
                || relative_raw == "target"
                || relative_raw.starts_with("target/")
                || relative_raw.starts_with("target\\");

            let default_open = if es_target_o_subdirectorio {
                false
            } else {
                name == "src"
            };

            let mut is_open =
                ui.data_mut(|d| d.get_temp::<bool>(dir_state_id).unwrap_or(default_open));

            let is_selected = state.project.selected_file.as_deref() == Some(&rel_dir)
                || state.project.selected_file.as_deref() == Some(&relative_raw);

            let item_count = std::fs::read_dir(path).map(|r| r.count()).unwrap_or(0);
            let badge = if name == "target" {
                Some("build")
            } else if item_count == 0 {
                Some("vacío")
            } else {
                None
            };

            let response = dibujar_fila_zed(ui, name, badge, depth, true, is_open, is_selected);

            if response.clicked() {
                is_open = !is_open;
                ui.data_mut(|d| d.insert_temp(dir_state_id, is_open));
                state.project.selected_file = Some(rel_dir.clone());
            }

            if is_open {
                mostrar_arbol_zed(ui, root, path, depth + 1, state);
            }
        } else {
            let is_selected = state.project.selected_file.as_deref() == Some(&relative_raw);

            let size_badge = std::fs::metadata(path)
                .ok()
                .map(|m| formato_tamano_bytes(m.len()));

            let response = dibujar_fila_zed(
                ui,
                name,
                size_badge.as_deref(),
                depth,
                false,
                false,
                is_selected,
            );

            if response.clicked() {
                state.project.selected_file = Some(relative_raw.clone());
                state.cargar_archivo_proyecto_activo();
            }
        }
    }
}

fn mostrar_tarjeta_explicacion(
    ui: &mut egui::Ui,
    _state: &AppState,
    project_dir: &Path,
    relative: &str,
    _orange: egui::Color32,
    cyan: egui::Color32,
) {
    let exp = obtener_explicacion_item(relative);
    let full_path = if relative.is_empty() || relative == "." {
        project_dir.to_path_buf()
    } else {
        project_dir.join(relative.trim_end_matches('/'))
    };

    let display_title = if relative.is_empty() || relative == "." {
        project_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("proyecto")
            .to_string()
            + "/"
    } else {
        relative.to_string()
    };

    let mut card_frame = egui::Frame::new();
    card_frame.fill = egui::Color32::from_rgb(14, 18, 28);
    card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(36, 50, 75));
    card_frame.corner_radius = egui::CornerRadius::same(6);
    card_frame.inner_margin = egui::Margin::same(14);

    card_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        // --- ENCABEZADO DE LA TARJETA (LIMPIO SIN ICONOS) ---
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                // Categoría badge sobrio y profesional
                let mut cat_chip = egui::Frame::new();
                cat_chip.fill =
                    egui::Color32::from_rgba_unmultiplied(cyan.r(), cyan.g(), cyan.b(), 22);
                cat_chip.stroke = egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(cyan.r(), cyan.g(), cyan.b(), 75),
                );
                cat_chip.corner_radius = egui::CornerRadius::same(3);
                cat_chip.inner_margin = egui::Margin::symmetric(6, 2);
                cat_chip.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(exp.categoria.to_uppercase())
                            .size(9.5)
                            .strong()
                            .color(cyan),
                    );
                });

                ui.add_space(3.0);

                // Nombre en negrita
                ui.label(
                    egui::RichText::new(&display_title)
                        .size(14.0)
                        .strong()
                        .monospace()
                        .color(egui::Color32::WHITE),
                );
            });

            // Metadatos a la derecha (tamaño, líneas o elementos)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if full_path.exists() {
                    if full_path.is_file() {
                        if let Ok(meta) = std::fs::metadata(&full_path) {
                            let size_str = formato_tamano_bytes(meta.len());
                            let mut chip_frame = egui::Frame::new();
                            chip_frame.fill = egui::Color32::from_rgb(22, 28, 42);
                            chip_frame.corner_radius = egui::CornerRadius::same(3);
                            chip_frame.inner_margin = egui::Margin::symmetric(6, 2);
                            chip_frame.stroke =
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 85));
                            chip_frame.show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(size_str)
                                        .monospace()
                                        .size(10.5)
                                        .color(egui::Color32::from_rgb(180, 195, 215)),
                                );
                            });
                        }
                        if let Ok(content) = std::fs::read_to_string(&full_path) {
                            let lines = content.lines().count();
                            let mut chip_frame = egui::Frame::new();
                            chip_frame.fill = egui::Color32::from_rgb(22, 28, 42);
                            chip_frame.corner_radius = egui::CornerRadius::same(3);
                            chip_frame.inner_margin = egui::Margin::symmetric(6, 2);
                            chip_frame.stroke =
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 85));
                            chip_frame.show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{lines} líneas"))
                                        .monospace()
                                        .size(10.5)
                                        .color(egui::Color32::from_rgb(180, 195, 215)),
                                );
                            });
                        }
                    } else if full_path.is_dir() {
                        let count = std::fs::read_dir(&full_path)
                            .map(|r| r.count())
                            .unwrap_or(0);
                        let mut chip_frame = egui::Frame::new();
                        chip_frame.fill = egui::Color32::from_rgb(22, 28, 42);
                        chip_frame.corner_radius = egui::CornerRadius::same(3);
                        chip_frame.inner_margin = egui::Margin::symmetric(6, 2);
                        chip_frame.stroke =
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 85));
                        chip_frame.show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{count} elementos"))
                                    .monospace()
                                    .size(10.5)
                                    .color(egui::Color32::from_rgb(180, 195, 215)),
                            );
                        });
                    }
                }
            });
        });

        ui.add_space(10.0);

        // --- BANNER DE RESUMEN DESTACADO ---
        let mut quote_frame = egui::Frame::new();
        quote_frame.fill = egui::Color32::from_rgb(19, 25, 38);
        quote_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 56, 82));
        quote_frame.corner_radius = egui::CornerRadius::same(4);
        quote_frame.inner_margin = egui::Margin::symmetric(10, 8);
        quote_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                let (line_rect, _) =
                    ui.allocate_exact_size(egui::vec2(3.0, 16.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(line_rect, egui::CornerRadius::same(1), cyan);
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(exp.resumen)
                        .size(12.5)
                        .color(egui::Color32::from_rgb(215, 230, 250)),
                );
            });
        });

        ui.add_space(12.0);

        // --- SECCIÓN 1: PROPÓSITO ---
        ui.label(
            egui::RichText::new("Propósito en Rust")
                .size(12.5)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(3.0);
        ui.label(
            egui::RichText::new(exp.proposito)
                .size(12.0)
                .color(egui::Color32::from_rgb(185, 200, 225)),
        );

        ui.add_space(10.0);

        // --- SECCIÓN 2: ROL EN CARGO ---
        ui.label(
            egui::RichText::new("Rol en Cargo & rustc")
                .size(12.5)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(3.0);
        ui.label(
            egui::RichText::new(exp.cargo_interaccion)
                .size(12.0)
                .color(egui::Color32::from_rgb(185, 200, 225)),
        );

        ui.add_space(10.0);

        // --- SECCIÓN 3: BUENAS PRÁCTICAS ---
        ui.label(
            egui::RichText::new("Buenas Prácticas & Convenciones")
                .size(12.5)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(3.0);
        ui.label(
            egui::RichText::new(exp.buenas_practicas)
                .size(12.0)
                .color(egui::Color32::from_rgb(185, 200, 225)),
        );

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);

        // --- FOOTER: COMANDO CLAVE ---
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new("Comando relacionado:")
                    .size(11.5)
                    .color(egui::Color32::from_rgb(130, 150, 175)),
            );
            ui.add_space(4.0);
            codigo_inline_chip(ui, exp.comando_relacionado);
        });
    });
}

fn mostrar_empty_state_creando(ui: &mut egui::Ui, _cyan: egui::Color32) {
    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 28);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(36, 50, 75));
    frame.corner_radius = egui::CornerRadius::same(6);
    frame.inner_margin = egui::Margin::same(20);

    frame.show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("Esperando a que Cargo cree el proyecto...")
                    .strong()
                    .size(14.0)
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Si acabas de ejecutar cargo new, se detectará automáticamente en cuanto finalice.")
                    .size(12.0)
                    .color(egui::Color32::from_rgb(160, 175, 195)),
            );
        });
    });
}

fn mostrar_empty_state_sin_proyecto(ui: &mut egui::Ui, _cyan: egui::Color32) {
    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 28);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(36, 50, 75));
    frame.corner_radius = egui::CornerRadius::same(6);
    frame.inner_margin = egui::Margin::same(20);

    frame.show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("Sin proyecto Cargo seleccionado")
                    .strong()
                    .size(14.5)
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Crea un proyecto en la terminal integrada o selecciónalo en la barra de herramientas.")
                    .size(12.0)
                    .color(egui::Color32::from_rgb(160, 175, 195)),
            );
            ui.add_space(14.0);

            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, egui::Color32::from_rgb(100, 200, 255));
                ui.label(egui::RichText::new("Para crear ejecutable:").size(12.0).color(egui::Color32::from_rgb(180, 195, 215)));
                codigo_inline_chip(ui, "cargo new mi_proyecto");
            });
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, egui::Color32::from_rgb(100, 200, 255));
                ui.label(egui::RichText::new("Para crear librería:").size(12.0).color(egui::Color32::from_rgb(180, 195, 215)));
                codigo_inline_chip(ui, "cargo new mi_libreria --lib");
            });
        });
    });
}

fn mostrar_project_explorer_drawer(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
) {
    let project_dir = state
        .project
        .selected_project
        .as_deref()
        .map(|project| buscar_ruta_proyecto(&state.terminal.term_cwd, project));

    match project_dir.as_ref() {
        Some(dir) if dir.exists() => {
            // Si selected_file es None o no existe o está vacío, preseleccionar src/main.rs o src/lib.rs o Cargo.toml
            if state
                .project
                .selected_file
                .as_deref()
                .is_none_or(|f| f.is_empty())
            {
                if dir.join("src/main.rs").exists() {
                    state.project.selected_file = Some("src/main.rs".to_string());
                } else if dir.join("src/lib.rs").exists() {
                    state.project.selected_file = Some("src/lib.rs".to_string());
                } else if dir.join("Cargo.toml").exists() {
                    state.project.selected_file = Some("Cargo.toml".to_string());
                }
            }

            // 2. CONTENEDOR DEL ÁRBOL DE ARCHIVOS
            let mut tree_frame = egui::Frame::new();
            tree_frame.fill = egui::Color32::from_rgb(12, 15, 22);
            tree_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 42, 60));
            tree_frame.corner_radius = egui::CornerRadius::same(6);
            tree_frame.inner_margin = egui::Margin::symmetric(6, 6);

            tree_frame.show(ui, |ui| {
                ui.set_width(ui.available_width());

                // Árbol interactivo recursivo: muestra directamente el contenido sin fila de raíz redundante
                mostrar_arbol_zed(ui, dir, dir, 0, state);
            });

            ui.add_space(14.0);

            // 3. TARJETA DE EXPLICACIÓN DETALLADA (PERSISTENTE PARA ARCHIVOS Y CARPETAS)
            if let Some(ref selected_ref) = state.project.selected_file
                && !selected_ref.is_empty()
            {
                mostrar_tarjeta_explicacion(ui, state, dir, selected_ref, orange, cyan);
            }
        }
        Some(_) => {
            mostrar_empty_state_creando(ui, cyan);
        }
        None => {
            mostrar_empty_state_sin_proyecto(ui, cyan);
        }
    }
}

pub fn boton_guardar_toolbar(
    ui: &mut egui::Ui,
    habilitado: bool,
    accent: egui::Color32,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(70.0, 26.0),
        if habilitado {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    let fill = if !habilitado {
        egui::Color32::from_rgb(16, 21, 29)
    } else if pressed {
        egui::Color32::from_rgb(105, 62, 25)
    } else if hovered {
        egui::Color32::from_rgb(62, 43, 24)
    } else {
        egui::Color32::from_rgb(32, 27, 21)
    };
    let border = if !habilitado {
        egui::Color32::from_rgb(38, 48, 62)
    } else if hovered || pressed {
        accent
    } else {
        egui::Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 105)
    };
    let text_color = if !habilitado {
        egui::Color32::from_rgb(85, 98, 115)
    } else if hovered || pressed {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(255, 190, 90)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(4),
        fill,
        egui::Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "Guardar",
        egui::FontId::proportional(11.0),
        text_color,
    );

    response
}

pub fn boton_icono_toolbar(
    ui: &mut egui::Ui,
    img: egui::Image,
    activo: bool,
    color_activo: egui::Color32,
    tooltip: &str,
) -> egui::Response {
    boton_icono_toolbar_estado(ui, img, activo, color_activo, true, tooltip)
}

pub fn boton_icono_toolbar_estado(
    ui: &mut egui::Ui,
    img: egui::Image,
    activo: bool,
    color_activo: egui::Color32,
    habilitado: bool,
    tooltip: &str,
) -> egui::Response {
    let sense = if habilitado {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(32.0, 26.0), sense);
    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    // Fondo interactivo
    let bg_color = if !habilitado {
        egui::Color32::from_rgb(14, 19, 28)
    } else if is_down {
        egui::Color32::from_rgb(34, 46, 68)
    } else if activo {
        egui::Color32::from_rgb(26, 36, 54)
    } else if is_hovered {
        egui::Color32::from_rgb(24, 32, 48)
    } else {
        egui::Color32::from_rgb(16, 22, 32)
    };

    // Borde interactivo
    let border_stroke = if !habilitado {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 39, 53))
    } else if activo {
        egui::Stroke::new(1.0, color_activo)
    } else if is_hovered {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 95, 135))
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 54, 80))
    };

    // Color del icono SVG (se ilumina suavemente en hover o activo)
    let icon_tint = if !habilitado {
        egui::Color32::from_rgb(75, 88, 106)
    } else if activo || is_hovered {
        color_activo
    } else {
        egui::Color32::from_rgb(160, 180, 205)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(4),
        bg_color,
        border_stroke,
        egui::StrokeKind::Inside,
    );
    let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(15.0, 15.0));
    img.tint(icon_tint).paint_at(ui, icon_rect);

    response.on_hover_text(tooltip)
}

pub fn boton_guia_destacado(
    ui: &mut egui::Ui,
    img: egui::Image,
    accent_color: egui::Color32,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(32.0, 26.0), egui::Sense::click());
    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    // Pulso de resalte suave basado en el tiempo
    let time = ui.input(|i| i.time);
    let pulse = ((time * 3.0).sin() * 0.5 + 0.5) as f32; // oscila suavemente entre 0.0 y 1.0

    let bg_color = if is_down {
        egui::Color32::from_rgb(45, 35, 20)
    } else if is_hovered {
        egui::Color32::from_rgb(40, 30, 15)
    } else {
        egui::Color32::from_rgb(26, 22, 16)
    };

    // Borde ámbar/dorado con brillo sutil
    let border_color = if is_hovered {
        accent_color
    } else {
        egui::Color32::from_rgb(
            (180.0 + 75.0 * pulse) as u8,
            (120.0 + 50.0 * pulse) as u8,
            (30.0 + 30.0 * pulse) as u8,
        )
    };

    let border_stroke = egui::Stroke::new(if is_hovered { 1.5 } else { 1.2 }, border_color);

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(4),
        bg_color,
        border_stroke,
        egui::StrokeKind::Inside,
    );
    let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(15.0, 15.0));
    img.tint(if is_hovered {
        accent_color
    } else {
        egui::Color32::from_rgb(255, 200, 120)
    })
    .paint_at(ui, icon_rect);

    // Circulito badge posicionado justo en la esquina superior derecha del borde, con efecto de parpadeo (blink)
    let blink_alpha = ((time * 4.5).sin() * 0.5 + 0.5) as f32; // oscilación de parpadeo entre 0.0 y 1.0
    let badge_center = egui::pos2(rect.max.x - 1.0, rect.min.y + 1.0);

    // Halo exterior tenue que se expande al parpadear
    let halo_alpha = (blink_alpha * 90.0) as u8;
    ui.painter().circle_filled(
        badge_center,
        4.5 + 1.5 * blink_alpha,
        egui::Color32::from_rgba_unmultiplied(
            accent_color.r(),
            accent_color.g(),
            accent_color.b(),
            halo_alpha,
        ),
    );

    // Núcleo brillante que parpadea
    let core_alpha = (140.0 + 115.0 * blink_alpha) as u8;
    ui.painter().circle_filled(
        badge_center,
        3.5,
        egui::Color32::from_rgba_unmultiplied(
            accent_color.r(),
            accent_color.g(),
            accent_color.b(),
            core_alpha,
        ),
    );

    // Borde fino oscuro alrededor del punto para que destaque sobre cualquier fondo
    ui.painter().circle_stroke(
        badge_center,
        3.5,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(10, 14, 22)),
    );

    // Forzar repaint mientras este botón esté visible para mantener el parpadeo en vivo
    ui.ctx().request_repaint();

    response.on_hover_text(tooltip)
}

fn tab_chip_toolbar(
    ui: &mut egui::Ui,
    label: &str,
    activo: bool,
    color_activo: egui::Color32,
) -> egui::Response {
    let font_id = egui::FontId::proportional(12.5);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), font_id, egui::Color32::WHITE);
    let text_w = galley.size().x;

    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(text_w + 18.0, 26.0), egui::Sense::click());
    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    let bg_color = if is_down {
        egui::Color32::from_rgb(34, 46, 68)
    } else if activo {
        egui::Color32::from_rgb(26, 36, 54)
    } else if is_hovered {
        egui::Color32::from_rgb(22, 28, 42)
    } else {
        egui::Color32::from_rgb(15, 20, 30)
    };

    let border_stroke = if activo {
        egui::Stroke::new(1.0, color_activo)
    } else if is_hovered {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 85, 120))
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70))
    };

    let text_color = if activo {
        color_activo
    } else if is_hovered {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(175, 190, 215)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(4),
        bg_color,
        border_stroke,
        egui::StrokeKind::Inside,
    );
    let text_pos = egui::pos2(
        rect.center().x - text_w / 2.0,
        rect.center().y - galley.size().y / 2.0,
    );
    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(12.5),
        text_color,
    );

    response
}
