use crate::app::AppState;
use crate::application::project_service::ProjectService;
use crate::components::navigation::{
    separador_vertical_centrado, underline_tab, underline_tab_destacado,
};
use crate::infrastructure::process::{captured_text, run_cargo};
#[allow(unused_imports)]
use crate::routes::AppRoute;
use crate::views::pilares::anatomy::{
    codigo_inline_chip, codigo_resaltado_bloque, codigo_terminal_bloque, punto_lista,
    titulo_seccion,
};
use crate::app::ui::{cell_centered, inline_code_chip_color, table_code_snippet, EducationalTable};
pub mod primitivos;
use self::primitivos::{
    mostrar_categoria_booleanos, mostrar_categoria_caracteres, mostrar_categoria_casting,
    mostrar_categoria_enteros_interactiva, mostrar_categoria_flotantes,
};
pub mod funciones;
pub mod globales;
pub mod mutabilidad;
pub mod scopes;
pub mod statements;
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub fn mostrar_comenzando(ui: &mut egui::Ui, state: &mut AppState) {
    mostrar_tutorial_conceptos_basicos(ui, state);
}

pub mod terminal;
pub use terminal::mostrar_componente_terminal_3_modos;

fn obtener_repos_base_dir(term_cwd: &std::path::Path) -> std::path::PathBuf {
    ProjectService::repositories_dir(term_cwd)
}

pub fn buscar_ruta_proyecto(base_path: &std::path::Path, proj_name: &str) -> std::path::PathBuf {
    ProjectService::resolve_project_dir(base_path, proj_name)
}

pub fn listar_proyectos_cargo(base_path: &std::path::Path) -> Vec<String> {
    ProjectService::list_projects(base_path)
}

pub fn listar_archivos_proyecto(proj_dir: &std::path::Path) -> Vec<String> {
    ProjectService::list_files(proj_dir)
}

mod project_explorer;
use project_explorer::{FileTreeContext, build_file_tree, render_file_tree};

#[allow(dead_code)]
pub fn mostrar_selector_proyectos_estandar(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    mostrar_selector_proyectos_base(
        ui,
        selected_project,
        None,
        None,
        term_cwd,
        combo_id,
        code_target,
    );
}

/// Variante del selector que conserva el archivo activo del editor sin mostrar
/// un segundo selector de archivos en la barra de herramientas.
pub fn mostrar_selector_proyectos_estandar_con_archivo_activo(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    active_file: Option<&str>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    mostrar_selector_proyectos_base(
        ui,
        selected_project,
        None,
        active_file,
        term_cwd,
        combo_id,
        code_target,
    );
}

pub fn pintar_icono_badge_tile(
    ui: &mut egui::Ui,
    img: egui::Image,
    activo: bool,
    color_hover: egui::Color32,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(32.0, 26.0), egui::Sense::click());
    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    // Fondo interactivo idéntico al resto de botones
    let bg_color = if is_down {
        egui::Color32::from_rgb(34, 46, 68)
    } else if activo {
        egui::Color32::from_rgb(26, 36, 54)
    } else if is_hovered {
        egui::Color32::from_rgb(24, 32, 48)
    } else {
        egui::Color32::from_rgb(16, 22, 32)
    };

    // Borde interactivo idéntico al resto de botones
    let border_stroke = if activo {
        egui::Stroke::new(1.0, color_hover)
    } else if is_hovered {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 95, 135))
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 54, 80))
    };

    // Color del icono SVG (tenue en reposo, se ilumina en Cyan con hover o activo)
    let icon_tint = if activo || is_hovered {
        color_hover
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

pub fn mostrar_selector_proyectos_estandar_con_archivos(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    selected_file: &mut Option<String>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    mostrar_selector_proyectos_base(
        ui,
        selected_project,
        Some(selected_file),
        None,
        term_cwd,
        combo_id,
        code_target,
    );
}

fn mostrar_selector_proyectos_base(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    mut selected_file: Option<&mut Option<String>>,
    active_file: Option<&str>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    let proyectos_disponibles = listar_proyectos_cargo(term_cwd);

    let mut proj_dir_opt = None;
    let mut archivos_disponibles = Vec::new();

    if let Some(proj) = selected_project.as_ref() {
        let proj_dir = buscar_ruta_proyecto(term_cwd, proj);
        if selected_file.is_some() {
            archivos_disponibles = listar_archivos_proyecto(&proj_dir);
        }
        proj_dir_opt = Some(proj_dir);
    }

    // Asegurar que selected_file tenga un valor válido si hay archivos y se pasó selected_file
    if let Some(ref mut sf) = selected_file
        && sf.is_none()
        && !archivos_disponibles.is_empty()
    {
        if archivos_disponibles.contains(&"src/main.rs".to_string()) {
            **sf = Some("src/main.rs".to_string());
        } else {
            **sf = Some(archivos_disponibles[0].clone());
        }
    }

    // Cargar contenido inicial del proyecto si el búfer compartido está vacío
    if let Some(proj_dir) = proj_dir_opt.as_ref()
        && code_target.is_empty()
    {
        let target_file = if let Some(ref sf) = selected_file {
            let file_rel = sf.as_deref().unwrap_or("src/main.rs");
            proj_dir.join(file_rel)
        } else if let Some(file_rel) = active_file {
            proj_dir.join(file_rel)
        } else {
            let main_rs = proj_dir.join("src/main.rs");
            let lib_rs = proj_dir.join("src/lib.rs");
            if main_rs.exists() {
                main_rs
            } else if lib_rs.exists() {
                lib_rs
            } else {
                main_rs
            }
        };
        if target_file.exists()
            && let Ok(real_content) = std::fs::read_to_string(&target_file)
        {
            *code_target = real_content;
        }
    }

    ui.horizontal(|ui| {
        let cyan = egui::Color32::from_rgb(100, 200, 255);

        // 1. Botón Tile de Proyecto (Icono puro)
        let proj_popup_id = ui.make_persistent_id(format!("{}_proj_popup_menu", combo_id));
        let mut proj_popup_open =
            ui.data_mut(|d| d.get_temp::<bool>(proj_popup_id).unwrap_or(false));
        let proj_anim = ui.ctx().animate_bool(proj_popup_id, proj_popup_open);

        let img_folder = egui::Image::new(egui::include_image!(
            "../../../../assets/icons/folder-off-svgrepo-com.svg"
        ))
        .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
        let btn_proj = pintar_icono_badge_tile(
            ui,
            img_folder,
            proj_popup_open,
            cyan,
            "Seleccionar Proyecto Cargo",
        );

        if btn_proj.clicked() {
            proj_popup_open = !proj_popup_open;
            ui.data_mut(|d| d.insert_temp(proj_popup_id, proj_popup_open));
        }

        // Cierre al hacer clic fuera o presionar Escape
        let proj_rect_id = proj_popup_id.with("rect");
        let last_proj_rect = ui.data_mut(|d| {
            d.get_temp::<egui::Rect>(proj_rect_id)
                .unwrap_or(egui::Rect::NOTHING)
        });
        if proj_popup_open {
            let escape = ui.input(|i| i.key_pressed(egui::Key::Escape));
            let click_outside = ui.input(|i| {
                if i.pointer.any_click() {
                    if let Some(pos) = i.pointer.interact_pos() {
                        !btn_proj.rect.contains(pos) && !last_proj_rect.contains(pos)
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            if escape || click_outside {
                ui.data_mut(|d| d.insert_temp(proj_popup_id, false));
            }
        }

        // Popup Dropdown de Proyectos animado con Area
        if proj_anim > 0.001 {
            let slide_y = (1.0 - proj_anim) * -6.0;
            let popup_pos = btn_proj.rect.left_bottom() + egui::vec2(0.0, 4.0 + slide_y);
            let mut close_popup = false;

            let alpha = (proj_anim * 255.0).clamp(0.0, 255.0) as u8;
            let bg_color = egui::Color32::from_rgba_unmultiplied(16, 22, 32, alpha);
            let border_color = egui::Color32::from_rgba_unmultiplied(45, 65, 95, alpha);

            egui::Area::new(proj_popup_id)
                .fixed_pos(popup_pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    // Sincronizar opacidad en todos los estilos visuales (textos, hovers, separadores)
                    ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                        egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha);
                    ui.style_mut().visuals.widgets.hovered.fg_stroke.color =
                        egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);
                    ui.style_mut().visuals.widgets.hovered.bg_fill =
                        egui::Color32::from_rgba_unmultiplied(30, 42, 60, alpha);
                    ui.style_mut().visuals.widgets.active.fg_stroke.color =
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                    ui.style_mut().visuals.widgets.active.bg_fill =
                        egui::Color32::from_rgba_unmultiplied(40, 58, 85, alpha);
                    ui.style_mut()
                        .visuals
                        .widgets
                        .noninteractive
                        .bg_stroke
                        .color = egui::Color32::from_rgba_unmultiplied(38, 54, 80, alpha);
                    ui.style_mut().visuals.selection.bg_fill =
                        egui::Color32::from_rgba_unmultiplied(30, 46, 70, alpha);
                    ui.style_mut().visuals.selection.stroke.color =
                        egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);

                    egui::Frame::popup(ui.style())
                        .fill(bg_color)
                        .stroke(egui::Stroke::new(1.0, border_color))
                        .corner_radius(egui::CornerRadius::same(6))
                        .inner_margin(egui::Margin::same(6))
                        .show(ui, |ui| {
                            ui.data_mut(|d| d.insert_temp(proj_rect_id, ui.min_rect()));
                            ui.set_width(256.0);
                            ui.label(
                                egui::RichText::new("PROYECTOS CARGO")
                                    .size(9.5)
                                    .strong()
                                    .color(egui::Color32::from_rgba_unmultiplied(
                                        120, 145, 175, alpha,
                                    )),
                            );
                            ui.separator();

                            egui::ScrollArea::vertical()
                                .max_height(150.0)
                                .auto_shrink([false, true])
                                .show(ui, |ui| {
                                    ui.style_mut().spacing.item_spacing.y = 2.0;

                                    let txt_libre = egui::RichText::new("Libre").color(
                                        if selected_project.is_none() {
                                            egui::Color32::from_rgba_unmultiplied(
                                                100, 200, 255, alpha,
                                            )
                                        } else {
                                            egui::Color32::from_rgba_unmultiplied(
                                                190, 205, 225, alpha,
                                            )
                                        },
                                    );

                                    if ui
                                        .selectable_label(selected_project.is_none(), txt_libre)
                                        .clicked()
                                    {
                                        *selected_project = None;
                                        if let Some(ref mut sf) = selected_file {
                                            **sf = None;
                                        }
                                        close_popup = true;
                                    }

                                    for proj in &proyectos_disponibles {
                                        let es_sel = selected_project.as_ref() == Some(proj);
                                        let txt_proj = egui::RichText::new(proj).color(if es_sel {
                                            egui::Color32::from_rgba_unmultiplied(
                                                100, 200, 255, alpha,
                                            )
                                        } else {
                                            egui::Color32::from_rgba_unmultiplied(
                                                190, 205, 225, alpha,
                                            )
                                        });

                                        if ui.selectable_label(es_sel, txt_proj).clicked() {
                                            *selected_project = Some(proj.clone());
                                            let proj_dir = buscar_ruta_proyecto(term_cwd, proj);
                                            *term_cwd = proj_dir.clone();

                                            let target = if let Some(ref mut sf) = selected_file {
                                                let nuevos_archivos =
                                                    listar_archivos_proyecto(&proj_dir);
                                                let main_rel = "src/main.rs".to_string();
                                                if nuevos_archivos.contains(&main_rel) {
                                                    **sf = Some(main_rel);
                                                } else if !nuevos_archivos.is_empty() {
                                                    **sf = Some(nuevos_archivos[0].clone());
                                                }

                                                if let Some(rel) = sf.as_ref() {
                                                    proj_dir.join(rel)
                                                } else {
                                                    proj_dir.join("src/main.rs")
                                                }
                                            } else {
                                                let main_rs = proj_dir.join("src/main.rs");
                                                let lib_rs = proj_dir.join("src/lib.rs");
                                                if main_rs.exists() {
                                                    main_rs
                                                } else if lib_rs.exists() {
                                                    lib_rs
                                                } else {
                                                    main_rs
                                                }
                                            };

                                            if let Ok(content) = std::fs::read_to_string(target) {
                                                *code_target = content;
                                            }
                                            close_popup = true;
                                        }
                                    }
                                });
                        });
                });

            if close_popup {
                ui.data_mut(|d| d.insert_temp(proj_popup_id, false));
            }
        }

        // 2. Botón Tile de Archivo (Icono puro al lado del proyecto) - SOLO si se solicitó con_archivos (selected_file != None)
        if let Some(selected_file) = selected_file
            && selected_project.is_some()
            && !archivos_disponibles.is_empty()
        {
            ui.add_space(2.0);

            let file_popup_id = ui.make_persistent_id(format!("{}_file_popup_menu", combo_id));
            let mut file_popup_open =
                ui.data_mut(|d| d.get_temp::<bool>(file_popup_id).unwrap_or(false));
            let file_anim = ui.ctx().animate_bool(file_popup_id, file_popup_open);

            let img_file = egui::Image::new(egui::include_image!(
                "../../../../assets/icons/file-svgrepo-com.svg"
            ))
            .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
            let btn_file = pintar_icono_badge_tile(
                ui,
                img_file,
                file_popup_open,
                cyan,
                "Seleccionar Archivo del Proyecto",
            );

            if btn_file.clicked() {
                file_popup_open = !file_popup_open;
                ui.data_mut(|d| d.insert_temp(file_popup_id, file_popup_open));
            }

            // Cierre al hacer clic fuera o presionar Escape
            let file_rect_id = file_popup_id.with("rect");
            let last_file_rect = ui.data_mut(|d| {
                d.get_temp::<egui::Rect>(file_rect_id)
                    .unwrap_or(egui::Rect::NOTHING)
            });
            if file_popup_open {
                let escape = ui.input(|i| i.key_pressed(egui::Key::Escape));
                let click_outside = ui.input(|i| {
                    if i.pointer.any_click() {
                        if let Some(pos) = i.pointer.interact_pos() {
                            !btn_file.rect.contains(pos) && !last_file_rect.contains(pos)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
                if escape || click_outside {
                    ui.data_mut(|d| d.insert_temp(file_popup_id, false));
                }
            }

            // Popup Dropdown de Archivos animado con Area
            if file_anim > 0.001 {
                let slide_y = (1.0 - file_anim) * -6.0;
                let popup_pos = btn_file.rect.left_bottom() + egui::vec2(0.0, 4.0 + slide_y);
                let mut close_file_popup = false;

                let alpha = (file_anim * 255.0).clamp(0.0, 255.0) as u8;
                let bg_color = egui::Color32::from_rgba_unmultiplied(16, 22, 32, alpha);
                let border_color = egui::Color32::from_rgba_unmultiplied(45, 65, 95, alpha);

                egui::Area::new(file_popup_id)
                    .fixed_pos(popup_pos)
                    .order(egui::Order::Foreground)
                    .show(ui.ctx(), |ui| {
                        // Sincronizar opacidad en todos los estilos visuales (textos, hovers, separadores)
                        ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                            egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha);
                        ui.style_mut().visuals.widgets.hovered.fg_stroke.color =
                            egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);
                        ui.style_mut().visuals.widgets.hovered.bg_fill =
                            egui::Color32::from_rgba_unmultiplied(30, 42, 60, alpha);
                        ui.style_mut().visuals.widgets.active.fg_stroke.color =
                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                        ui.style_mut().visuals.widgets.active.bg_fill =
                            egui::Color32::from_rgba_unmultiplied(40, 58, 85, alpha);
                        ui.style_mut()
                            .visuals
                            .widgets
                            .noninteractive
                            .bg_stroke
                            .color = egui::Color32::from_rgba_unmultiplied(38, 54, 80, alpha);
                        ui.style_mut().visuals.selection.bg_fill =
                            egui::Color32::from_rgba_unmultiplied(30, 46, 70, alpha);
                        ui.style_mut().visuals.selection.stroke.color =
                            egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);

                        egui::Frame::popup(ui.style())
                            .fill(bg_color)
                            .stroke(egui::Stroke::new(1.0, border_color))
                            .corner_radius(egui::CornerRadius::same(6))
                            .inner_margin(egui::Margin::same(6))
                            .show(ui, |ui| {
                                ui.data_mut(|d| d.insert_temp(file_rect_id, ui.min_rect()));
                                ui.set_width(256.0);
                                ui.label(
                                    egui::RichText::new("ARCHIVOS DEL PROYECTO")
                                        .size(9.5)
                                        .strong()
                                        .color(egui::Color32::from_rgba_unmultiplied(
                                            120, 145, 175, alpha,
                                        )),
                                );
                                ui.separator();

                                egui::ScrollArea::vertical()
                                    .max_height(150.0)
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        ui.style_mut().spacing.item_spacing.y = 2.0;

                                        let tree = build_file_tree(&archivos_disponibles);
                                        let mut tree_context = FileTreeContext {
                                            selected_file,
                                            project_dir: proj_dir_opt.as_deref(),
                                            code_target,
                                            close_popup: &mut close_file_popup,
                                            alpha,
                                            combo_id,
                                        };
                                        render_file_tree(ui, &tree, &mut tree_context, 0);
                                    });
                            });
                    });

                if close_file_popup {
                    ui.data_mut(|d| d.insert_temp(file_popup_id, false));
                }
            }
        }
    });
}

pub fn ejecutar_cargo_run_proyecto(state: &mut AppState, ctx: &egui::Context) {
    let proj_dir = if let Some(ref proj) = state.project.selected_project {
        buscar_ruta_proyecto(&state.terminal.term_cwd, proj)
    } else {
        obtener_repos_base_dir(&state.terminal.term_cwd)
    };

    let main_rs = proj_dir.join("src/main.rs");
    let lib_rs = proj_dir.join("src/lib.rs");
    let target_file = if main_rs.exists() {
        main_rs
    } else if lib_rs.exists() {
        lib_rs
    } else {
        main_rs.clone()
    };

    // Obtener el código y el búfer de salida de la lección activa
    let codigo_activo = if state.editor.project_editor_path.as_deref() == Some("src/main.rs") {
        state.editor.project_editor_code.clone()
    } else {
        state.obtener_codigo_activo().to_string()
    };
    let output_arc = state.obtener_output_activo();

    if state.project.selected_project.is_some() && target_file.parent().is_some_and(|p| p.exists())
    {
        let _ = std::fs::write(&target_file, &codigo_activo);
    }

    if let Ok(mut out) = output_arc.lock() {
        *out = "Compilando y ejecutando con Cargo (cargo run)...".to_string();
    }

    // Abrir inmediatamente la ventana modal flotante centrada en la pantalla
    state
        .ui
        .show_cargo_output_modal
        .store(true, Ordering::Relaxed);

    let history_arc = Arc::clone(&state.terminal.term_history);
    let ctx_clone = ctx.clone();
    let is_proj = state.project.selected_project.is_some();
    let output_error = Arc::clone(&output_arc);

    let task_result = state.terminal.task_manager.spawn("cargo-run", move || {
        let run_dir = if is_proj && proj_dir.exists() {
            proj_dir.clone()
        } else {
            obtener_repos_base_dir(&proj_dir)
        };
        let output = run_cargo(&["run"], &run_dir);

        match output {
            Ok(out) => {
                let stdout = captured_text(&out.stdout);
                let stderr = captured_text(&out.stderr);
                let mut combined = stdout;
                if !stderr.is_empty() {
                    if !combined.is_empty() {
                        combined.push_str("\n\n");
                    }
                    combined.push_str("[Compilador / Warnings / Errores]:\n");
                    combined.push_str(&stderr);
                }
                if combined.is_empty() {
                    combined = "El programa terminó exitosamente sin salidas.".to_string();
                }
                if let Ok(mut out_lock) = output_arc.lock() {
                    *out_lock = combined;
                }
                if let Ok(mut history) = history_arc.lock() {
                    history.push(format!("$ cargo run (en {})", proj_dir.display()));
                }
            }
            Err(err) => {
                if let Ok(mut out_lock) = output_arc.lock() {
                    *out_lock = format!("Error al ejecutar cargo run: {}", err);
                }
            }
        }
        ctx_clone.request_repaint();
    });
    if let Err(error) = task_result
        && let Ok(mut output) = output_error.lock()
    {
        *output = format!("No se pudo iniciar cargo run: {error}");
    }
}

pub fn mostrar_contenido_tipos_primitivos(ui: &mut egui::Ui, state: &mut AppState) {
    let orange = crate::app::ui::Colors::ORANGE_RUST;
    let categorias = [
        (0, "Enteros"),
        (1, "Decimales"),
        (2, "Booleanos"),
        (3, "Caracteres"),
        (4, "Casting (as)"),
    ];

    // Centrar horizontalmente los tabs de Data Types
    let font_id = egui::FontId::proportional(13.0);
    let h_padding = 10.0;
    let item_spacing = ui.spacing().item_spacing.x;
    let mut total_w = 0.0;
    for (i, (_, cat_label)) in categorias.iter().enumerate() {
        let galley = ui.painter().layout_no_wrap(
            cat_label.to_string(),
            font_id.clone(),
            egui::Color32::WHITE,
        );
        total_w += galley.size().x + h_padding * 2.0;
        if i > 0 {
            total_w += item_spacing;
        }
    }

    let available_w = ui.available_width();
    let left_margin = ((available_w - total_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(left_margin);
        for (cat_idx, cat_label) in categorias {
            let es_sel = state.lessons.tipo_primitivo_categoria == cat_idx;
            if underline_tab(ui, cat_label, es_sel, orange).clicked() {
                state.lessons.tipo_primitivo_categoria = cat_idx;
            }
        }
    });

    ui.add_space(12.0);

    // Las tablas de Data Types pueden superar la altura disponible del panel
    // central. El selector queda fijo y solo el contenido educativo se
    // desplaza, para que las categorías sigan accesibles mientras se lee.
    egui::ScrollArea::vertical()
        .id_salt("conceptos_data_types_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| match state.lessons.tipo_primitivo_categoria {
            0 => mostrar_categoria_enteros_interactiva(ui, state),
            1 => mostrar_categoria_flotantes(ui, state),
            2 => mostrar_categoria_booleanos(ui, state),
            3 => mostrar_categoria_caracteres(ui, state),
            _ => mostrar_categoria_casting(ui, state),
        });
}

#[allow(dead_code)]
fn centrar_texto_en_rectangulos(raw_svg: &str) -> String {
    let clean_svg = raw_svg.replace(">\n", ">").replace(">\r\n", ">");

    let mut output = String::with_capacity(clean_svg.len());
    let mut search_idx = 0;

    while let Some(g_start) = clean_svg[search_idx..].find("<g class=") {
        let abs_g_start = search_idx + g_start;
        let g_substr = &clean_svg[abs_g_start..];

        if (g_substr.starts_with("<g class=\"terminal\"")
            || g_substr.starts_with("<g class=\"nonterminal\""))
            && let Some(g_end) = g_substr.find("</g>")
        {
            let abs_g_end = abs_g_start + g_end + 4;
            let group_block = &clean_svg[abs_g_start..abs_g_end];

            let mut rx = 0.0f32;
            let mut ry = 0.0f32;
            let mut rw = 0.0f32;
            let mut rh = 0.0f32;

            if let Some(r_pos) = group_block.find("<rect ") {
                let r_sub = &group_block[r_pos..];
                if let Some(r_close) = r_sub.find('>') {
                    let r_tag = &r_sub[..r_close];
                    for attr in r_tag.split_whitespace() {
                        if let Some((k, v)) = attr.split_once('=') {
                            let val = v
                                .trim_matches('"')
                                .trim_matches('\'')
                                .trim_matches('>')
                                .trim_matches('/');
                            match k {
                                "x" => rx = val.parse().unwrap_or(0.0),
                                "y" => ry = val.parse().unwrap_or(0.0),
                                "width" => rw = val.parse().unwrap_or(0.0),
                                "height" => rh = val.parse().unwrap_or(0.0),
                                _ => {}
                            }
                        }
                    }
                }
            }

            if rw > 0.0
                && rh > 0.0
                && let (Some(t_start), Some(t_end)) =
                    (group_block.find("<text "), group_block.find("</text>"))
            {
                let text_sub = &group_block[t_start..t_end + 7];
                if let Some(tag_close) = text_sub.find('>') {
                    let content = text_sub[tag_close + 1..text_sub.len() - 7].trim();

                    let cx = rx + rw / 2.0;
                    let cy = ry + rh / 2.0;

                    let is_terminal = group_block.contains("class=\"terminal\"");
                    let fill_color = if is_terminal { "#ffb347" } else { "#64c8ff" };

                    let new_group = format!(
                        "{}\n<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-family=\"sans-serif\" font-size=\"12\" font-weight=\"normal\" text-anchor=\"middle\" dominant-baseline=\"central\" stroke=\"none\">{}</text>\n</g>",
                        group_block[..t_start].trim_end(),
                        cx,
                        cy,
                        fill_color,
                        content
                    );
                    output.push_str(&clean_svg[search_idx..abs_g_start]);
                    output.push_str(&new_group);
                    search_idx = abs_g_end;
                    continue;
                }
            }
        }

        output.push_str(&clean_svg[search_idx..abs_g_start + 8]);
        search_idx = abs_g_start + 8;
    }

    output.push_str(&clean_svg[search_idx..]);
    output
}

#[allow(dead_code)]
fn generar_railroad_color_image() -> Option<egui::ColorImage> {
    use railroad::*;

    let mut seq = Sequence::default();
    let e1: Box<dyn railroad::Node> = Box::new(Terminal::new("let".to_string()));
    let e2: Box<dyn railroad::Node> = Box::new(Optional::new(Terminal::new("mut".to_string())));
    let e3: Box<dyn railroad::Node> = Box::new(NonTerminal::new("identificador".to_string()));
    let e4_sub1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
    let e4_sub2: Box<dyn railroad::Node> = Box::new(NonTerminal::new("tipo".to_string()));
    let e4: Box<dyn railroad::Node> =
        Box::new(Optional::new(Sequence::new(vec![e4_sub1, e4_sub2])));
    let e5: Box<dyn railroad::Node> = Box::new(Terminal::new("=".to_string()));
    let e6: Box<dyn railroad::Node> = Box::new(NonTerminal::new("expresion".to_string()));
    let e7: Box<dyn railroad::Node> = Box::new(Terminal::new(";".to_string()));

    seq.push(e1);
    seq.push(e2);
    seq.push(e3);
    seq.push(e4);
    seq.push(e5);
    seq.push(e6);
    seq.push(e7);

    let dia = Diagram::new(seq);
    let mut raw_svg = dia.to_string();

    if !raw_svg.contains("width=") {
        raw_svg = raw_svg.replace(
            "<svg ",
            "<svg width=\"626\" height=\"60\" xmlns=\"http://www.w3.org/2000/svg\" ",
        );
    } else if !raw_svg.contains("xmlns=") {
        raw_svg = raw_svg.replace("<svg ", "<svg xmlns=\"http://www.w3.org/2000/svg\" ");
    }

    raw_svg = raw_svg
        .replace(
            "<g class=\"terminal\">",
            "<g class=\"terminal\" fill=\"#1e2638\" stroke=\"#ff9d00\" stroke-width=\"2\">",
        )
        .replace(
            "<g class=\"nonterminal\">",
            "<g class=\"nonterminal\" fill=\"#1a2336\" stroke=\"#64c8ff\" stroke-width=\"2\">",
        )
        .replace(
            "<path ",
            "<path stroke=\"#64c8ff\" stroke-width=\"2.5\" fill=\"none\" ",
        );

    raw_svg = centrar_texto_en_rectangulos(&raw_svg);

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(&raw_svg, &opt).ok()?;
    let width = tree.size().width().ceil() as u32;
    let height = tree.size().height().ceil() as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width.max(1), height.max(1))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    let pixels = pixmap.data();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels,
    ))
}

pub fn mostrar_contenido_macros(ui: &mut egui::Ui, state: &AppState) {
    let orange = egui::Color32::from_rgb(255, 160, 50);
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];

    ui.heading(
        egui::RichText::new("Macros")
            .size(20.0)
            .strong()
            .color(orange),
    );
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(
            "Las macros permiten escribir una sintaxis que Rust transforma en código durante la compilación. Esta tabla reúne las Declarative Macros más utilizadas de la biblioteca estándar.",
        )
        .size(13.5)
        .color(text_col),
    );
    ui.add_space(10.0);

    ui.heading(
        egui::RichText::new("Format Strings")
            .size(18.0)
            .strong()
            .color(orange),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Los Format Strings indican cómo mostrar un valor dentro de macros como println!, print!, format! y write!. El tipo debe ser compatible con el formato elegido.",
        )
        .size(13.5)
        .color(text_col),
    );
    ui.add_space(8.0);

    EducationalTable::new(
        "tabla_format_strings",
        &["Sintaxis", "Nombre", "Ejemplo de Código", "Uso"],
    )
    .min_col_width(90.0)
    .spacing(18.0, 6.0)
    .show(ui, |body| {
        let formatos = [
            ("{}", "Display", "println!(\"{}\", valor);", "Salida legible para usuarios."),
            ("{:?}", "Debug", "println!(\"{:?}\", valor);", "Inspección técnica en una línea."),
            ("{:#?}", "Pretty Debug", "println!(\"{:#?}\", datos);", "Debug con formato expandido."),
            ("{0}", "Positional argument", "println!(\"{0} - {0}\", valor);", "Reutiliza el argumento en una posición."),
            ("{nombre}", "Named argument", "println!(\"Hola, {nombre}\");", "Inserta un argumento por nombre."),
            ("{:b}", "Binary", "println!(\"{:b}\", numero);", "Muestra un entero en base 2."),
            ("{:o}", "Octal", "println!(\"{:o}\", numero);", "Muestra un entero en base 8."),
            ("{:x}", "Hexadecimal", "println!(\"{:x}\", numero);", "Muestra un entero en hexadecimal minúsculo."),
            ("{:X}", "Hexadecimal uppercase", "println!(\"{:X}\", numero);", "Muestra un entero en hexadecimal mayúsculo."),
            ("{:e}", "Scientific", "println!(\"{:e}\", decimal);", "Muestra un decimal en notación científica."),
            ("{:E}", "Scientific uppercase", "println!(\"{:E}\", decimal);", "Usa la notación científica con E mayúscula."),
            ("{:.2}", "Precision", "println!(\"{:.2}\", precio);", "Muestra dos cifras decimales."),
            ("{:>10}", "Right alignment", "println!(\"{:>10}\", texto);", "Alinea el valor a la derecha."),
            ("{:<10}", "Left alignment", "println!(\"{:<10}\", texto);", "Alinea el valor a la izquierda."),
            ("{:^10}", "Center alignment", "println!(\"{:^10}\", texto);", "Centra el valor dentro del ancho."),
            ("{:0>8}", "Fill and width", "println!(\"{:0>8}\", numero);", "Rellena con ceros hasta ocho posiciones."),
            ("{:+}", "Explicit sign", "println!(\"{:+}\", numero);", "Muestra también el signo positivo."),
            ("{:#x}", "Alternate form", "println!(\"{:#x}\", numero);", "Añade el prefijo 0x al hexadecimal."),
            ("{:p}", "Pointer", "println!(\"{:p}\", &valor);", "Muestra la dirección de una referencia."),
            ("{{ }}", "Literal braces", "println!(\"{{{{}}}}\");", "Muestra llaves como texto literal."),
            ("{:width$}", "Dynamic width", "println!(\"{:width$}\", texto, width = 10);", "Recibe el ancho desde otro argumento."),
            ("{:.precision$}", "Dynamic precision", "println!(\"{:.precision$}\", valor, precision = 2);", "Recibe la precisión desde otro argumento."),
        ];

        for (sintaxis, nombre, ejemplo, uso) in formatos {
            body.row(|ui| {
                cell_centered(ui, |ui| inline_code_chip_color(ui, sintaxis, orange));
                ui.label(egui::RichText::new(nombre).size(12.0).color(text_col));
                table_code_snippet(ui, ejemplo, &state.editor.syntax_set, theme, "rs");
                ui.label(egui::RichText::new(uso).size(12.0).color(text_col));
            });
        }
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Declarative Macros")
            .size(18.0)
            .strong()
            .color(orange),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Estas macros utilizan patrones para generar código. Las macros de salida de la tabla anterior usan este mismo sistema de invocación con el signo !.",
        )
        .size(13.5)
        .color(text_col),
    );
    ui.add_space(8.0);

    let declarative_max_col_width = ((ui.available_width() - 120.0) / 4.0).max(140.0);
    EducationalTable::new(
        "tabla_macros_declarativas",
        &["Macro", "Propósito", "Ejemplo de Código", "Comportamiento"],
    )
    .min_col_width(90.0)
    .max_col_width(declarative_max_col_width)
    .spacing(18.0, 6.0)
    .show(ui, |body| {
        let macros = [
            ("print!", "Escritura en stdout", "print!(\"Cargando...\");", "Escribe sin salto de línea."),
            ("println!", "Escritura en stdout", "println!(\"Hola, Rust!\");", "Escribe y añade un salto de línea."),
            ("eprint!", "Escritura en stderr", "eprint!(\"Error\");", "Escribe errores sin salto de línea."),
            ("eprintln!", "Escritura en stderr", "eprintln!(\"Error\");", "Escribe errores con salto de línea."),
            ("format!", "Crear texto formateado", "let s = format!(\"x = {}\", 10);", "Devuelve un String sin imprimirlo."),
            ("write!", "Escribir texto formateado", "write!(salida, \"x = {}\", 10);", "Escribe en un destino que acepta formato."),
            ("writeln!", "Escribir con salto", "writeln!(salida, \"Listo\");", "Escribe texto y añade un salto de línea."),
            ("dbg!", "Inspección de valores", "let y = dbg!(x * 2);", "Muestra información de depuración y devuelve el valor."),
            ("vec!", "Crear un vector", "let valores = vec![1, 2, 3];", "Construye un Vec<T> con los valores indicados."),
            ("assert!", "Comprobar una condición", "assert!(edad >= 18);", "Continúa si es true y genera un panic si es false."),
            ("assert_eq!", "Comparar igualdad", "assert_eq!(actual, esperado);", "Comprueba que dos valores sean iguales."),
            ("assert_ne!", "Comparar diferencia", "assert_ne!(a, b);", "Comprueba que dos valores sean diferentes."),
            ("panic!", "Interrumpir la ejecución", "panic!(\"Fallo crítico\");", "Detiene el hilo actual con un mensaje."),
            ("todo!", "Marcar una tarea pendiente", "todo!(\"Implementar\");", "Indica código pendiente y genera un panic al ejecutarse."),
            ("unimplemented!", "Marcar una función pendiente", "unimplemented!();", "Indica que todavía no existe una implementación."),
            ("unreachable!", "Marcar un camino imposible", "unreachable!();", "Indica que esa parte del código no debería alcanzarse."),
            ("matches!", "Comprobar un patrón", "matches!(valor, Some(_));", "Devuelve true si el valor coincide con el patrón."),
            ("concat!", "Unir literales", "concat!(\"Rust\", \"!\");", "Concatena textos conocidos durante la compilación."),
            ("stringify!", "Convertir código en texto", "stringify!(a + b);", "Convierte los tokens recibidos en un String literal."),
            ("env!", "Leer una variable de entorno", "env!(\"CARGO_PKG_NAME\");", "Obtiene su valor durante la compilación."),
            ("option_env!", "Leer una variable opcional", "option_env!(\"MI_VALOR\");", "Devuelve un Option según exista la variable."),
            ("include_str!", "Incluir texto", "include_str!(\"datos.txt\");", "Incluye un archivo como texto durante la compilación."),
            ("include_bytes!", "Incluir bytes", "include_bytes!(\"datos.bin\");", "Incluye un archivo como bytes durante la compilación."),
            ("line!", "Consultar la línea", "line!();", "Devuelve la línea del código fuente."),
            ("column!", "Consultar la columna", "column!();", "Devuelve la columna del código fuente."),
            ("file!", "Consultar el archivo", "file!();", "Devuelve la ruta del archivo fuente."),
            ("module_path!", "Consultar el módulo", "module_path!();", "Devuelve la ruta del módulo actual."),
            ("cfg!", "Consultar configuración", "cfg!(debug_assertions);", "Evalúa una condición de compilación y devuelve bool."),
            ("compile_error!", "Detener la compilación", "compile_error!(\"Configuración inválida\");", "Genera un error de compilación personalizado."),
            ("macro_rules!", "Definir una macro", "macro_rules! repetir { ($x:expr) => { $x }; }", "Define una Declarative Macro basada en patrones."),
        ];

        for (nombre, proposito, ejemplo, comportamiento) in macros {
            body.row(|ui| {
                cell_centered(ui, |ui| inline_code_chip_color(ui, nombre, orange));
                ui.label(egui::RichText::new(proposito).size(12.0).color(text_col));
                table_code_snippet(ui, ejemplo, &state.editor.syntax_set, theme, "rs");
                ui.label(
                    egui::RichText::new(comportamiento)
                        .size(12.0)
                        .color(text_col),
                );
            });
        }
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Compiler Attributes (Lint Attributes)")
            .size(18.0)
            .strong()
            .color(orange),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Los Compiler Attributes utilizan la sintaxis #[...] para dar instrucciones al compilador. Los Lint Attributes controlan advertencias del código; se escriben encima del elemento al que afectan y no son macros.")
            .size(13.5)
            .color(text_col),
    );
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new("allow: permitir una situación concreta")
            .size(15.0)
            .strong()
            .color(orange),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Evita una advertencia específica cuando sabes que el código es intencional.")
            .size(13.0)
            .color(text_col),
    );
    ui.add_space(6.0);
    codigo_resaltado_bloque(
        ui,
        "#[allow(unused_variables)]\nfn ejemplo() {\n    let resultado = 10;\n}",
        &state.editor.syntax_set,
        theme,
        "rs",
    );

    ui.add_space(14.0);
    ui.label(
        egui::RichText::new("warn: solicitar una advertencia")
            .size(15.0)
            .strong()
            .color(orange),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Activa una advertencia para el elemento indicado cuando se detecta ese lint.")
            .size(13.0)
            .color(text_col),
    );
    ui.add_space(6.0);
    codigo_resaltado_bloque(
        ui,
        "#[warn(dead_code)]\nfn auxiliar() {}",
        &state.editor.syntax_set,
        theme,
        "rs",
    );

    ui.add_space(14.0);
    ui.label(
        egui::RichText::new("deny: convertir una advertencia en error")
            .size(15.0)
            .strong()
            .color(orange),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Impide compilar cuando aparece el lint indicado.")
            .size(13.0)
            .color(text_col),
    );
    ui.add_space(6.0);
    codigo_resaltado_bloque(
        ui,
        "#[deny(unused_imports)]\nuse std::fmt::Debug;",
        &state.editor.syntax_set,
        theme,
        "rs",
    );

    ui.add_space(14.0);
    ui.label(
        egui::RichText::new("forbid: impedir que el lint se relaje")
            .size(15.0)
            .strong()
            .color(orange),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Bloquea ese lint de forma más estricta y no permite cambiarlo posteriormente con allow.")
            .size(13.0)
            .color(text_col),
    );
    ui.add_space(6.0);
    codigo_resaltado_bloque(
        ui,
        "#[forbid(unsafe_code)]\nfn seguro() {\n    println!(\"Código sin unsafe\");\n}",
        &state.editor.syntax_set,
        theme,
        "rs",
    );
}

#[allow(dead_code)]
fn mostrar_contenido_macros_legacy(ui: &mut egui::Ui) {
    ui.heading(
        egui::RichText::new("Categorías de Macros en Rust")
            .size(18.0)
            .strong()
            .color(egui::Color32::from_rgb(255, 160, 50)),
    );
    ui.add_space(6.0);
    ui.label(
        "En Rust, las macros son herramientas de metaprogramación que generan código durante la compilación. Se distinguen fácilmente por llevar un signo de exclamación ! al final.",
    );
    ui.add_space(10.0);

    ui.columns(2, |cols| {
        let mut card1 = egui::Frame::new();
        card1.fill = egui::Color32::from_rgb(18, 22, 32);
        card1.inner_margin = egui::Margin::same(12);
        card1.corner_radius = egui::CornerRadius::same(8);
        card1.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

        card1.show(&mut cols[0], |ui| {
            ui.heading(
                egui::RichText::new("1. Macros Declarativas (macro_rules!)")
                    .size(15.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.add_space(4.0);
            ui.label("Basadas en coincidencia de patrones (pattern matching) similares a una sentencia match. Permiten escribir código conciso como vec![] o println!.");
        });

        card1.show(&mut cols[1], |ui| {
            ui.heading(
                egui::RichText::new("2. Macros Procedurales (Derive, Atributos)")
                    .size(15.0)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(4.0);
            ui.label("Operan sobre el Árbol de Sintaxis Abstracta (AST) de Rust como código ejecutable durante la compilación (ej: #[derive(Debug, Serialize)]).");
        });
    });

    ui.add_space(16.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_macros_estandar_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Macro")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Salida / Comportamiento")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                let macros_list = [
                    (
                        "println!",
                        "Impresión a Consola (stdout)",
                        "println!(\"Hola {name}\");",
                        "Escribe en consola con salto de línea al final (\\n).",
                    ),
                    (
                        "print!",
                        "Impresión Continua (stdout)",
                        "print!(\"Cargando...\");",
                        "Escribe en consola sin agregar salto de línea.",
                    ),
                    (
                        "format!",
                        "Creación de String Formateado",
                        "let s = format!(\"x = {}\", 10);",
                        "Devuelve un String dinámico sin imprimir en consola.",
                    ),
                    (
                        "eprintln!",
                        "Impresión de Errores (stderr)",
                        "eprintln!(\"Error: {}\", err);",
                        "Escribe en la salida estándar de errores stderr.",
                    ),
                    (
                        "dbg!",
                        "Macro de Depuración Nativa",
                        "let y = dbg!(x * 2);",
                        "Imprime archivo, línea, expresión y devuelve el valor.",
                    ),
                    (
                        "vec!",
                        "Creación de Vectores Dinámicos",
                        "let v = vec![1, 2, 3];",
                        "Sintaxis conveniente para inicializar un Vec<T>.",
                    ),
                    (
                        "panic!",
                        "Interrupción de Emergencia",
                        "panic!(\"Fallo crítico\");",
                        "Detiene la ejecución del hilo enviando un mensaje de pánico.",
                    ),
                    (
                        "assert_eq!",
                        "Verificación de Pruebas",
                        "assert_eq!(a, b);",
                        "Valida igualdad en tests; entra en pánico si son distintos.",
                    ),
                ];

                for (m_name, m_prop, m_code, m_desc) in macros_list {
                    ui.label(
                        egui::RichText::new(m_name)
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(
                        egui::RichText::new(m_prop).color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(m_code)
                            .monospace()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    ui.label(m_desc);
                    ui.end_row();
                }
            });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Depuración")
            .size(18.0)
            .strong()
            .color(egui::Color32::from_rgb(255, 160, 50)),
    );
    ui.add_space(6.0);
    ui.label(
        "Rust ofrece potentes mecanismos de formateo e inspección de variables. Conocer la diferencia entre Display {}, Debug {:?}, Pretty Debug {:#?} y dbg! es clave para el desarrollo diario.",
    );
    ui.add_space(10.0);

    let mut fmt_frame = egui::Frame::new();
    fmt_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    fmt_frame.inner_margin = egui::Margin::same(12);
    fmt_frame.corner_radius = egui::CornerRadius::same(8);
    fmt_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    fmt_frame.show(ui, |ui| {
        egui::Grid::new("tabla_debug_formato_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Formato / Herramienta")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Especificador")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Uso Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Display {}
                ui.label(
                    egui::RichText::new("Display")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Formato amigable para el usuario final");
                ui.label(
                    egui::RichText::new("println!(\"Score: {}\", puntos);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Debug {:?}
                ui.label(
                    egui::RichText::new("Debug")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{:?}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Inspección técnica de 1 sola línea");
                ui.label(
                    egui::RichText::new("println!(\"{:?}\", arreglo);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Pretty Debug {:#?}
                ui.label(
                    egui::RichText::new("Pretty Debug")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{:#?}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Inspección multilínea indentada (estructuras compuestas)");
                ui.label(
                    egui::RichText::new("println!(\"{:#?}\", persona);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Macro dbg!
                ui.label(
                    egui::RichText::new("Macro dbg!")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("dbg!(exp)")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Imprime archivo, nº línea, expresión y devuelve el valor");
                ui.label(
                    egui::RichText::new("let b = dbg!(a + 5);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    let mut tip_frame = egui::Frame::new();
    tip_frame.fill = egui::Color32::from_rgb(20, 28, 42);
    tip_frame.inner_margin = egui::Margin::same(14);
    tip_frame.corner_radius = egui::CornerRadius::same(8);
    tip_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 90, 140));

    tip_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new("A diferencia de println!, dbg! toma la propiedad de la expresión, imprime su ubicación exacta en el código fuente, el resultado de la expresión, y devuelve el valor evaluado. ¡Eso te permite envolver llamadas intermedias sin romper tu código!")
                .color(egui::Color32::from_rgb(200, 230, 255)),
        );
    });

    ui.add_space(20.0);
    ui.heading(
        egui::RichText::new("Macros y atributos: cómo se relacionan")
            .size(18.0)
            .strong()
            .color(egui::Color32::from_rgb(255, 160, 50)),
    );
    ui.add_space(6.0);
    ui.label("La sintaxis y la implementación son conceptos distintos: ! indica una invocación, mientras que #[...] indica un atributo. Algunos atributos activan macros procedurales.");
    ui.add_space(10.0);

    let mut mapa_frame = egui::Frame::new();
    mapa_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    mapa_frame.inner_margin = egui::Margin::same(12);
    mapa_frame.corner_radius = egui::CornerRadius::same(8);
    mapa_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    mapa_frame.show(ui, |ui| {
        egui::Grid::new("mapa_macros_atributos_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                for (forma, categoria, ejemplo, explicacion) in [
                    (
                        "!",
                        "Macro declarativa",
                        "println!(\"Hola\");",
                        "macro_rules! usa patrones para generar código.",
                    ),
                    (
                        "!",
                        "Macro procedural function-like",
                        "mi_macro!(dato);",
                        "recibe tokens y genera código durante la compilación.",
                    ),
                    (
                        "#[derive(...)]",
                        "Macro procedural derive",
                        "#[derive(Debug, Clone)]",
                        "cada trait, como Debug o Clone, puede ser una macro derive.",
                    ),
                    (
                        "#[atributo]",
                        "Macro procedural attribute",
                        "#[tokio::main]",
                        "transforma la función, struct o módulo al que se aplica.",
                    ),
                    (
                        "#[atributo]",
                        "Atributo del compilador",
                        "#[cfg(test)]",
                        "configura la compilación; no todos los atributos son macros.",
                    ),
                ] {
                    ui.label(
                        egui::RichText::new(forma)
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(
                        egui::RichText::new(categoria)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.label(
                        egui::RichText::new(ejemplo)
                            .monospace()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    ui.label(explicacion);
                    ui.end_row();
                }
            });
    });

    ui.add_space(14.0);
    let mut nota_frame = egui::Frame::new();
    nota_frame.fill = egui::Color32::from_rgb(20, 28, 42);
    nota_frame.inner_margin = egui::Margin::same(14);
    nota_frame.corner_radius = egui::CornerRadius::same(8);
    nota_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 90, 140));
    nota_frame.show(ui, |ui| {
        ui.label(egui::RichText::new("Regla rápida").strong().color(egui::Color32::from_rgb(255, 160, 50)));
        ui.label("println!() es una macro declarativa invocada con !. #[derive(Debug)] usa una macro derive mediante sintaxis de atributo. #[tokio::main] sí es una macro procedural de atributo. #[allow(...)] y #[cfg(...)] son atributos integrados del compilador.");
    });
}

#[allow(dead_code)]
fn generar_railroad_desde_codigo(codigo: &str) -> Option<egui::ColorImage> {
    use railroad::*;

    let mut mut_token: Option<String> = None;
    let mut ident_token = "variable".to_string();
    let mut tipo_token: Option<String> = None;
    let mut expr_token = "expresion".to_string();
    let mut tiene_punto_y_coma = false;

    let mut encontrado = false;
    for orig_line in codigo.lines() {
        let line_without_comment = match orig_line.split_once("//") {
            Some((code, _)) => code,
            None => orig_line,
        };
        let trimmed = line_without_comment.trim();
        if trimmed.starts_with("let ") || trimmed.starts_with("let\t") || trimmed == "let" {
            encontrado = true;
            let mut rest = if trimmed.starts_with("let ") || trimmed.starts_with("let\t") {
                trimmed["let".len()..].trim()
            } else {
                ""
            };

            if rest.contains(';') {
                tiene_punto_y_coma = true;
                rest = rest.trim_end_matches(';').trim();
            }

            let (pat_part, expr_part) = match rest.split_once('=') {
                Some((p, e)) => (p.trim(), e.trim()),
                None => (rest, ""),
            };

            if !expr_part.is_empty() {
                expr_token = expr_part.to_string();
            }

            let mut pat_str = pat_part;
            if pat_str.starts_with("mut ") || pat_str.starts_with("mut\t") || pat_str == "mut" {
                mut_token = Some("mut".to_string());
                if pat_str.len() > 3 {
                    pat_str = pat_str[3..].trim();
                } else {
                    pat_str = "";
                }
            }

            if let Some((ident, ty)) = pat_str.split_once(':') {
                if !ident.trim().is_empty() {
                    ident_token = ident.trim().to_string();
                }
                if !ty.trim().is_empty() {
                    tipo_token = Some(ty.trim().to_string());
                }
            } else if !pat_str.trim().is_empty() {
                ident_token = pat_str.trim().to_string();
            }
            break;
        }
    }

    if !encontrado {
        return None;
    }

    let mut seq = Sequence::default();
    let e1: Box<dyn railroad::Node> = Box::new(Terminal::new("let".to_string()));
    let e2: Box<dyn railroad::Node> = match mut_token {
        Some(m) => Box::new(Terminal::new(m)),
        None => Box::new(Optional::new(Terminal::new("mut".to_string()))),
    };
    let e3: Box<dyn railroad::Node> = Box::new(NonTerminal::new(ident_token));

    let e4: Box<dyn railroad::Node> = match tipo_token {
        Some(ty) => {
            let n1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
            let n2: Box<dyn railroad::Node> = Box::new(NonTerminal::new(ty));
            Box::new(Sequence::new(vec![n1, n2]))
        }
        None => {
            let e4_sub1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
            let e4_sub2: Box<dyn railroad::Node> = Box::new(NonTerminal::new("tipo".to_string()));
            Box::new(Optional::new(Sequence::new(vec![e4_sub1, e4_sub2])))
        }
    };

    let e5: Box<dyn railroad::Node> = Box::new(Terminal::new("=".to_string()));
    let e6: Box<dyn railroad::Node> = Box::new(NonTerminal::new(expr_token));
    let e7: Box<dyn railroad::Node> = if tiene_punto_y_coma {
        Box::new(Terminal::new(";".to_string()))
    } else {
        Box::new(Optional::new(Terminal::new(";".to_string())))
    };

    seq.push(e1);
    seq.push(e2);
    seq.push(e3);
    seq.push(e4);
    seq.push(e5);
    seq.push(e6);
    seq.push(e7);

    let dia = Diagram::new(seq);
    let mut raw_svg = dia.to_string();

    if !raw_svg.contains("width=") {
        raw_svg = raw_svg.replace(
            "<svg ",
            "<svg width=\"650\" height=\"60\" xmlns=\"http://www.w3.org/2000/svg\" ",
        );
    } else if !raw_svg.contains("xmlns=") {
        raw_svg = raw_svg.replace("<svg ", "<svg xmlns=\"http://www.w3.org/2000/svg\" ");
    }

    raw_svg = raw_svg
        .replace(
            "<g class=\"terminal\">",
            "<g class=\"terminal\" fill=\"#1e2638\" stroke=\"#ff9d00\" stroke-width=\"2\">",
        )
        .replace(
            "<g class=\"nonterminal\">",
            "<g class=\"nonterminal\" fill=\"#1a2336\" stroke=\"#64c8ff\" stroke-width=\"2\">",
        )
        .replace(
            "<path ",
            "<path stroke=\"#64c8ff\" stroke-width=\"2.5\" fill=\"none\" ",
        );

    raw_svg = centrar_texto_en_rectangulos(&raw_svg);

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(&raw_svg, &opt).ok()?;
    let width = tree.size().width().ceil() as u32;
    let height = tree.size().height().ceil() as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width.max(1), height.max(1))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    let pixels = pixmap.data();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels,
    ))
}

pub fn mostrar_seccion_documentacion(ui: &mut egui::Ui) {
    ui.label(
        "Rust cuenta con soporte nativo de primera clase para comentarios de código y documentación. La herramienta 'cargo doc' compila automáticamente los Doc Comments con sintaxis Markdown en un sitio web de documentación HTML interactivo.",
    );
    ui.add_space(10.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_doc_comentarios")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("¿Genera HTML?")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Destino / Ámbito")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Uso Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Comentario de Línea
                ui.label(
                    egui::RichText::new("Línea")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("// texto")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("No")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("Ignorado por compilador");
                ui.label("Notas breves e internas de lógica.");
                ui.end_row();

                // Fila 2: Comentario de Bloque
                ui.label(
                    egui::RichText::new("Bloque")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("/* texto */")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("No")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("Ignorado por compilador");
                ui.label("Desactivar temporalmente código.");
                ui.end_row();

                // Fila 3: Doc Comment Externo
                ui.label(
                    egui::RichText::new("Doc Externo")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("/// texto")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Sí (cargo doc)")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Elemento siguiente");
                ui.label("Documentar funciones, structs y enums.");
                ui.end_row();

                // Fila 4: Doc Comment Interno
                ui.label(
                    egui::RichText::new("Doc Interno")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("//! texto")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Sí (cargo doc)")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Módulo contenedor");
                ui.label("Cabecera de crate, lib.rs o main.rs.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Dos Columnas: Comentarios vs Doc Comments con Markdown
    ui.columns(2, |cols| {
        // Columna Izquierda: Comentarios Normales
        let mut card_comm = egui::Frame::new();
        card_comm.fill = egui::Color32::from_rgb(14, 18, 26);
        card_comm.inner_margin = egui::Margin::same(12);
        card_comm.corner_radius = egui::CornerRadius::same(8);
        card_comm.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        card_comm.show(&mut cols[0], |ui| {
            ui.label(
                egui::RichText::new("Comentarios Internos (// y /* */)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("Son notas para ti y tu equipo que el compilador elimina por completo durante el análisis léxico:");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("// Este es un comentario de una sola línea").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.label(egui::RichText::new("let x = 5; // Nota al final de la línea").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("/* Comentario multilínea").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.label(egui::RichText::new("   útil para grandes bloques */").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
            });
        });

        // Columna Derecha: Doc Comments y cargo doc
        let mut card_doc = egui::Frame::new();
        card_doc.fill = egui::Color32::from_rgb(14, 18, 26);
        card_doc.inner_margin = egui::Margin::same(12);
        card_doc.corner_radius = egui::CornerRadius::same(8);
        card_doc.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        card_doc.show(&mut cols[1], |ui| {
            ui.label(
                egui::RichText::new("Doc Comments (/// & cargo doc)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("Admiten Markdown y generan la documentación ejecutando 'cargo doc --open' en la terminal:");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("/// Calcula el área de un rectángulo.").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("/// # Argumentos").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("/// * `base` - Longitud en metros").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("fn area(base: f64, altura: f64) -> f64 {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.indent("doc_code_inner", |ui| {
                    ui.label(egui::RichText::new("base * altura").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                });
                ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });
        });
    });
}

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
                        egui::RichText::new("Conceptos de Rust")
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

                    let tabs_teoria = [
                        (7, "Core"),
                        (4, "Data Types"),
                        (5, "Macros"),
                    ];
                    for (indice, texto) in tabs_teoria {
                        let es_activo = state.lessons.conceptos_tab == indice;
                        if underline_tab(ui, texto, es_activo, cyan).clicked() {
                            state.lessons.conceptos_tab = indice;
                        }
                    }

                    // --- LADO DERECHO: Práctica ---
                    ui.scope_builder(
                        egui::UiBuilder::new()
                            .id(egui::Id::new("nav_right_section"))
                            .layout(egui::Layout::right_to_left(egui::Align::Center)),
                        |ui| {
                            ui.add_space(5.0);

                            let es_code_lab_activo = state.lessons.conceptos_tab == 0;
                            let tabs_practica = [(0, "Code Lab")];
                            // Iteramos al revés por el right_to_left
                            for (indice, texto) in tabs_practica.iter().rev() {
                                let es_activo = state.lessons.conceptos_tab == *indice;
                                // Destacamos con baliza pulsante e intuición visual hacia Code Lab cuando no está seleccionado
                                if underline_tab_destacado(ui, texto, es_activo, orange, true)
                                    .clicked()
                                {
                                    state.lessons.conceptos_tab = *indice;
                                    if *indice == 0 {
                                        state.ui.mostrar_conceptos_drawer = false;
                                        state.ui.conceptos_drawer_tab = 0;
                                    }
                                }
                            }

                            let img_code = egui::Image::new(egui::include_image!(
                                "../../../../assets/icons/monitor-code-line.svg"
                            ))
                            .fit_to_exact_size(egui::Vec2::new(18.0, 18.0));
                            let (_id, code_rect) = ui.allocate_space(egui::Vec2::new(18.0, 18.0));

                            let time = ui.input(|i| i.time);
                            let pulse = ((time * 3.5).sin() * 0.5 + 0.5) as f32;
                            let tint = if es_code_lab_activo {
                                orange
                            } else {
                                egui::Color32::from_rgb(
                                    (170.0 + 65.0 * pulse) as u8,
                                    (125.0 + 45.0 * pulse) as u8,
                                    (60.0 + 35.0 * pulse) as u8,
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

pub fn mostrar_tutorial_conceptos_basicos(ui: &mut egui::Ui, state: &mut AppState) {
    // Estos índices pertenecían a tabs que ahora viven dentro del recorrido del Code Lab.
    if matches!(state.lessons.conceptos_tab, 6 | 8) {
        state.lessons.conceptos_tab = 7;
    }

    if state.lessons.conceptos_tab == 0 {
        mostrar_conceptos_codelab(ui, state);
        return;
    }

    match state.lessons.conceptos_tab {
        0 => funciones::mostrar(ui, state),
        4 => mostrar_contenido_tipos_primitivos(ui, state),
        5 => {
            egui::ScrollArea::vertical()
                .id_salt("conceptos_macros_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| mostrar_contenido_macros(ui, state));
        }
        7 => {
            egui::ScrollArea::vertical()
                .id_salt("conceptos_core_mechanics_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    mutabilidad::mostrar(ui, state);
                });
        }
        _ => {}
    }
}

/// Code Lab de la sesión Conceptos. El editor ocupa el espacio principal y
/// las explicaciones progresivas viven en el drawer derecho, igual que en
/// Rust Foundations.
fn mostrar_conceptos_codelab(ui: &mut egui::Ui, state: &mut AppState) {
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_conceptos_codelab",
            separator_id: "conceptos_editor_toolbar_sep_y",
            terminal_panel_id: "conceptos_terminal_panel",
            editor_scroll_id: "conceptos_codelab_editor",
            drawer_key: "conceptos",
            drawer_open: state.ui.mostrar_conceptos_drawer,
            drawer_tab: state.ui.conceptos_drawer_tab,
            output_open: state.ui.conceptos_salida_abierta,
            navigation_step: state.lessons.conceptos_codelab_reto_actual,
            navigation_total: 8,
        },
        |ui, state, orange, cyan| {
            mostrar_retos_conceptos_drawer(ui, state, orange, cyan);
        },
    );
    state.ui.mostrar_conceptos_drawer = shell_state.drawer_open;
    state.ui.conceptos_drawer_tab = shell_state.drawer_tab;
    state.ui.conceptos_salida_abierta = shell_state.output_open;
    if state.lessons.conceptos_tab != 0 {
        state.ui.mostrar_conceptos_drawer = false;
    }
    crate::views::pilares::anatomy::aplicar_navegacion_codelab(
        &mut state.lessons.conceptos_codelab_reto_actual,
        shell_state.navigation_delta,
        8,
    );
}

fn mostrar_retos_conceptos_drawer(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    cyan: egui::Color32,
) {
    let retos = retos_conceptos();
    let current = state
        .lessons
        .conceptos_codelab_reto_actual
        .min(retos.len().saturating_sub(1));
    state.lessons.conceptos_codelab_reto_actual = current;

    let (title, subtitle, explanation, code) = retos[current];
    let (section_title, task) = match current {
        0 => (
            "Variables paso a paso",
            "Practica el recorrido: let, let mut y shadowing.",
        ),
        1 => (
            "Macros",
            "Entender println! y las macros",
        ),
        2 => (
            "Blocks & Scope",
            "Crea un Block y observa qué variables puedes utilizar dentro y fuera de él.",
        ),
        3 => (
            "Statements & Expressions",
            "Statement y Expression",
        ),
        4 => (
            "Data Types",
            "Consulta y practica Data Types",
        ),
        5 => (
            "Comments & Docs",
            "Escribe comentarios y genera documentación con cargo doc.",
        ),
        6 => (
            "Functions",
            "Declara y utiliza Functions paso a paso.",
        ),
        _ => (
            "Questions",
            "Comprueba lo aprendido en 15 preguntas.",
        ),
    };
    let bullet_col = egui::Color32::from_rgb(140, 160, 190);
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let theme = state.editor.theme_set.themes["base16-ocean.dark"].clone();

    // El encabezado, las etiquetas y la lectura vertical siguen el mismo
    // patrón que Foundations; solo cambia la lección que vive dentro.
    ui.heading(
        egui::RichText::new(format!("{}. {}", current + 1, title))
            .size(19.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        let mut tag_frame = egui::Frame::new();
        tag_frame.fill = egui::Color32::from_rgb(20, 38, 28);
        tag_frame.inner_margin = egui::Margin::symmetric(8, 2);
        tag_frame.corner_radius = egui::CornerRadius::same(10);
        tag_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Fundamental")
                    .size(11.5)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 200, 120)),
            );
        });

        ui.add_space(4.0);
        if current == 0 {
            for tag in ["let", "let mut", "shadowing"] {
                let tag_topic = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(22, 28, 38))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .corner_radius(egui::CornerRadius::same(10));
                tag_topic.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(tag)
                            .size(11.5)
                            .color(egui::Color32::from_rgb(160, 185, 220)),
                    );
                });
                ui.add_space(4.0);
            }
        } else if current == 1 {
            for tag in ["Macros", "println!"] {
                let tag_topic = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(22, 28, 38))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .corner_radius(egui::CornerRadius::same(10));
                tag_topic.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(tag)
                            .size(11.5)
                            .color(egui::Color32::from_rgb(160, 185, 220)),
                    );
                });
                ui.add_space(4.0);
            }
        } else if current == 2 {
            for tag in ["Block", "Scope"] {
                let tag_topic = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(22, 28, 38))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .corner_radius(egui::CornerRadius::same(10));
                tag_topic.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(tag)
                            .size(11.5)
                            .color(egui::Color32::from_rgb(160, 185, 220)),
                    );
                });
                ui.add_space(4.0);
            }
        } else if current == 3 {
            for tag in ["Statement", "Expression"] {
                let tag_topic = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(22, 28, 38))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .corner_radius(egui::CornerRadius::same(10));
                tag_topic.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(tag)
                            .size(11.5)
                            .color(egui::Color32::from_rgb(160, 185, 220)),
                    );
                });
                ui.add_space(4.0);
            }
        } else if current == 5 {
            for tag in ["Comments", "Doc Comments", "cargo doc"] {
                let tag_topic = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(22, 28, 38))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .corner_radius(egui::CornerRadius::same(10));
                tag_topic.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(tag)
                            .size(11.5)
                            .color(egui::Color32::from_rgb(160, 185, 220)),
                    );
                });
                ui.add_space(4.0);
            }
        } else if current == 6 {
            for tag in ["fn", "Parameters", "Return value"] {
                let tag_topic = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(22, 28, 38))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .corner_radius(egui::CornerRadius::same(10));
                tag_topic.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(tag)
                            .size(11.5)
                            .color(egui::Color32::from_rgb(160, 185, 220)),
                    );
                });
                ui.add_space(4.0);
            }
        } else if current == 7 {
            for tag in ["15 Questions", "Review"] {
                let tag_topic = egui::Frame::new()
                    .fill(egui::Color32::from_rgb(22, 28, 38))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .corner_radius(egui::CornerRadius::same(10));
                tag_topic.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(tag)
                            .size(11.5)
                            .color(egui::Color32::from_rgb(160, 185, 220)),
                    );
                });
                ui.add_space(4.0);
            }
        } else {
            let tag_topic = egui::Frame::new()
                .fill(egui::Color32::from_rgb(22, 28, 38))
                .inner_margin(egui::Margin::symmetric(8, 2))
                .corner_radius(egui::CornerRadius::same(10));
            tag_topic.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(subtitle)
                        .size(11.5)
                        .color(egui::Color32::from_rgb(160, 185, 220)),
                );
            });
        }
    });

    ui.add_space(12.0);
    if current == 0 {
        ui.label(
            egui::RichText::new(
                "En esta parte aprenderás qué son las variables, cómo declararlas y cómo utilizarlas. Después verás qué ocurre cuando necesitas modificar un valor y cómo diferenciar varias formas de declarar valores. Avanzaremos paso a paso con ejemplos pequeños.",
            )
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
        );
    } else if current == 1 {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new("En Rust, una macro se reconoce por el signo")
                    .size(13.5)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "!");
            ui.label(
                egui::RichText::new(". ")
                    .size(13.5)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "println!");
            ui.label(
                egui::RichText::new("es una macro que muestra información en la terminal. Primero veremos cómo se usa y después por qué no es una función común.")
                    .size(13.5)
                    .color(text_col),
            );
        });
    } else if current == 2 {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Un").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Block");
            ui.label(egui::RichText::new("agrupa instrucciones entre llaves. Su").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Scope");
            ui.label(egui::RichText::new("indica en qué parte del programa una variable puede utilizarse y cuándo deja de estar disponible.").size(13.5).color(text_col));
        });
    } else if current == 3 {
        ui.label(
            egui::RichText::new(
                "En esta parte aprenderás cómo el código puede realizar acciones y cómo también puede producir resultados. Primero veremos instrucciones sencillas y después fragmentos de código que calculan o devuelven un valor.",
            )
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
        );
    } else if current == 4 {
        ui.horizontal_wrapped(|ui| {
            codigo_inline_chip(ui, "Data Types");
            ui.label(egui::RichText::new("indican qué clase de valor puede guardar una variable y qué operaciones son válidas. Aquí veremos un ejemplo breve; la pestaña").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Data Types");
            ui.label(egui::RichText::new("contiene las tablas y la explicación completa de cada tipo.").size(13.5).color(text_col));
        });
    } else if current == 5 {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("En esta parte aprenderás a dejar").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Comments");
            ui.label(egui::RichText::new("útiles en el código y a convertir explicaciones de tus APIs en").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Doc Comments");
            ui.label(egui::RichText::new("consultables. Terminaremos generando documentación del proyecto.").size(13.5).color(text_col));
        });
    } else if current == 6 {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Una").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Function");
            ui.label(egui::RichText::new("reúne instrucciones bajo un nombre. Avanzaremos desde una").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Function");
            ui.label(egui::RichText::new("sencilla hasta sus").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Parameters");
            ui.label(egui::RichText::new("y el valor que puede").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Return");
            ui.label(egui::RichText::new(".").size(13.5).color(text_col));
        });
    } else if current == 7 {
        ui.label(
            egui::RichText::new(
                "Esta evaluación repasa los conceptos principales de la sesión. Lee cada pregunta, elige una respuesta y revisa el resultado antes de continuar.",
            )
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
        );
    } else {
        ui.label(
            egui::RichText::new(explanation)
                .size(13.5)
                .color(text_col)
                .line_height(Some(19.0)),
        );
    }
    ui.add_space(14.0);

    if current == 0 {
        titulo_seccion(ui, "1. Declarar una variable con let", cyan);
        ui.indent("conceptos_variable_let", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Usa").size(13.0).color(text_col));
                ui.add_space(4.0);
                codigo_inline_chip(ui, "let");
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(
                        "para crear una variable y darle un valor inicial. En Rust, su valor queda protegido por defecto, por lo que no puede cambiar por accidente. Más adelante veremos cómo permitir cambios de manera explícita.",
                    )
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(ui, "let x = 5;", &state.editor.syntax_set, &theme, "rs");
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "2. Cambiar un valor con let mut", cyan);
        ui.indent("conceptos_variable_mut", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Las variables son inmutables por defecto. Usa")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "let mut");
                ui.label(
                    egui::RichText::new("cuando necesites cambiar su valor.")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let mut x = 5;\nx = 10;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "3. Crear una nueva variable con Shadowing", cyan);
        ui.indent("conceptos_shadowing_nueva_declaracion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Con Shadowing vuelves a escribir")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "let");
                ui.label(
                    egui::RichText::new(
                        "con el mismo nombre. La nueva variable oculta la anterior.",
                    )
                    .size(13.0)
                    .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let pasos = 1;\nlet pasos = pasos + 1;\nlet pasos = pasos + 1;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "4. Shadowing y reasignación no son lo mismo", cyan);
        ui.indent("conceptos_shadowing_comparacion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Shadowing crea otra variable usando")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "let");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let pasos = 1;\nlet pasos = pasos + 1;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Para poder reasignar el valor, declara la variable con")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "let mut");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let mut pasos = 1;\npasos = pasos + 1;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "5. Indicar que una variable no se utilizará", cyan);
        ui.indent("conceptos_variable_prefijo_guion_bajo", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Si una variable comienza con")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "_");
                ui.label(
                    egui::RichText::new(", Rust entiende que probablemente no utilizarás su valor y no muestra la advertencia correspondiente.")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let _resultado = 10;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(5.0);
            ui.label(
                egui::RichText::new("El nombre sigue siendo una variable; el prefijo indica tu intención de no utilizarla por ahora.")
                    .size(12.5)
                    .color(text_col),
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "6. const: un valor fijo", cyan);
        ui.indent("conceptos_const_intro", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Una").size(13.0).color(text_col));
                codigo_inline_chip(ui, "const");
                ui.label(egui::RichText::new("representa un valor que no cambia y debe conocerse durante el").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Compile time");
                ui.label(egui::RichText::new(". A diferencia de una variable común, siempre necesita un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Data Type");
                ui.label(egui::RichText::new("explícito.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "const MAX_INTENTOS: u32 = 3;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "7. static: un valor global", cyan);
        ui.indent("conceptos_static_intro", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                codigo_inline_chip(ui, "static");
                ui.label(egui::RichText::new("declara un valor global que permanece disponible durante toda la ejecución del programa. Por ahora basta con reconocer su duración y su ubicación global.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "static PUERTO_SERVICIO: u16 = 8080;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "8. type: un alias legible", cyan);
        ui.indent("conceptos_type_intro", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                codigo_inline_chip(ui, "type");
                ui.label(egui::RichText::new("crea un nombre alternativo para un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Data Type");
                ui.label(egui::RichText::new("existente. No crea un tipo nuevo; ayuda a expresar mejor la intención del código.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "type Puntos = u32;\nlet total: Puntos = 100;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "9. Comparar let, let mut, const y static", orange);
        ui.indent("conceptos_variables_comparacion", |ui| {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Todos pueden guardar valores, pero no significan lo mismo. La siguiente comparación resume su función.")
                    .size(13.0)
                    .color(text_col),
            );
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let intentos = 3;\nlet mut contador = 0;\nconst MAX_INTENTOS: u32 = 3;\nstatic PUERTO_SERVICIO: u16 = 8080;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(6.0);
            for (nombre, descripcion) in [
                ("let", "variable inmutable por defecto"),
                ("let mut", "variable que puede cambiar"),
                ("const", "valor fijo conocido en Compile time"),
                ("static", "valor global disponible durante la ejecución"),
            ] {
                ui.horizontal_wrapped(|ui| {
                    codigo_inline_chip(ui, nombre);
                    ui.label(
                        egui::RichText::new(descripcion)
                            .size(12.5)
                            .color(text_col),
                    );
                });
                ui.add_space(3.0);
            }
            ui.add_space(12.0);
        });
    } else if current == 1 {
        titulo_seccion(ui, "1. Declarative macros", cyan);
        ui.indent("conceptos_macros_definicion", |ui| {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(
                    "También se conocen como macros por reglas. Funcionan buscando patrones y sustituyéndolos por código antes de compilar.",
                )
                .size(13.0)
                .color(text_col),
            );
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Se crean normalmente con")
                        .size(13.0)
                        .color(text_col),
                );
                ui.add_space(4.0);
                codigo_inline_chip(ui, "macro_rules!");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "println!(\"Hola, Rust!\");",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.label(
                egui::RichText::new(
                    "println! es una macro declarativa de la biblioteca estándar. El signo ! indica que no llamamos a una función normal, sino a una macro.",
                )
                .size(13.0)
                .color(text_col),
            );
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "println!(\"Hola\");\nprintln!(\"Resultado: {}\", 2 + 3);",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "2. Compiler Attributes (Lint Attributes)", cyan);
        ui.indent("conceptos_macros_compiler_attributes", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Los Compiler Attributes usan la sintaxis")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "#[...]");
                ui.label(
                    egui::RichText::new("para dar instrucciones al compilador y controlar algunas advertencias.")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            for (atributo, descripcion) in [
                (
                    "#[allow(unused_variables)]",
                    "permite variables que todavía no se utilizan sin mostrar esa advertencia.",
                ),
                (
                    "#[allow(dead_code)]",
                    "permite código declarado que aún no se utiliza.",
                ),
                (
                    "#[allow(unused_imports)]",
                    "permite imports que todavía no se utilizan.",
                ),
            ] {
                ui.horizontal_wrapped(|ui| {
                    codigo_inline_chip(ui, atributo);
                    ui.label(
                        egui::RichText::new(descripcion)
                            .size(12.5)
                            .color(text_col),
                    );
                });
                ui.add_space(3.0);
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("No son macros: son instrucciones integradas del compilador, aunque usan una sintaxis parecida.")
                    .size(12.5)
                    .color(text_col),
            );
            ui.add_space(12.0);
        });
    } else if current == 2 {
        titulo_seccion(ui, "1. Block: agrupar instrucciones", cyan);
        ui.indent("conceptos_blocks_definicion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Block");
                ui.label(egui::RichText::new("es una zona de código delimitada por").size(13.0).color(text_col));
                codigo_inline_chip(ui, "{ }");
                ui.label(
                    egui::RichText::new(". Dentro puede contener variables e instrucciones.")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "{\n    let interior = 20;\n}",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "2. Scope: alcance de una variable", cyan);
        ui.indent("conceptos_scope_alcance", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Scope");
                ui.label(egui::RichText::new("indica dónde puede utilizarse una variable. Es una regla del lenguaje.").size(13.0).color(text_col));
            });
            ui.add_space(7.0);
            codigo_resaltado_bloque(
                ui,
                "let exterior = 10;\n\n{\n    let interior = 20;\n\n    println!(\"{}\", exterior); // válido\n    println!(\"{}\", interior); // válido\n}\n\nprintln!(\"{}\", exterior); // válido\n// println!(\"{}\", interior); // error: está fuera de su Scope",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(
                    "El Block interior puede utilizar exterior porque fue creada afuera. En cambio, interior solo está disponible entre sus llaves; después del Block, su nombre queda fuera de su Scope.",
                )
                .size(13.0)
                .color(text_col),
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "3. RAII: limpiar al salir del Scope", cyan);
        ui.indent("conceptos_raii_scope", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                codigo_inline_chip(ui, "RAII");
                ui.label(egui::RichText::new("indica que un valor adquiere su recurso al crearse y Rust lo limpia automáticamente cuando sale de su").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Scope");
                ui.label(egui::RichText::new(". Más adelante relacionaremos esta regla con").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Ownership");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "{\n    let recurso = String::from(\"Hola\");\n} // Rust limpia el recurso al salir del Scope",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });
    } else if current == 3 {
        titulo_seccion(ui, "Statement: una instrucción", cyan);
        ui.indent("conceptos_statements_definicion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Statement");
                ui.label(egui::RichText::new("realiza una acción, como declarar una variable. Normalmente termina con").size(13.0).color(text_col));
                codigo_inline_chip(ui, ";");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let base = 6;\nlet doble = base * 2;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "Expression: una expresión con valor", cyan);
        ui.indent("conceptos_expressions_definicion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Una").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Expression");
                ui.label(egui::RichText::new("se evalúa y produce un valor. La última expresión de un bloque puede devolver ese resultado si no lleva").size(13.0).color(text_col));
                codigo_inline_chip(ui, ";");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let total = {\n    let base = 6;\n    base * 2\n};",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });
    } else if current == 4 {
        titulo_seccion(ui, "1. Data Types: describir un valor", cyan);
        ui.indent("conceptos_data_types_intro", |ui| {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(
                    "Una variable puede guardar distintos tipos de valores. La anotación de tipo ayuda a Rust a comprobar que el valor y las operaciones son correctos.",
                )
                .size(13.0)
                .color(text_col),
            );
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let edad: u32 = 26;\nlet temperatura: i32 = -5;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(
                    "Para consultar todos los tipos, sus rangos y sus usos, abre la pestaña de referencia Data Types.",
                )
                .size(12.5)
                .color(egui::Color32::from_rgb(160, 180, 205)),
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "2. Type annotation y Type inference", cyan);
        ui.indent("conceptos_data_types_formas", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Puedes indicar el").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Data Type");
                ui.label(egui::RichText::new("de forma explícita o dejar que Rust lo deduzca a partir del valor. Ambas formas pertenecen al lenguaje y el compilador comprueba que sean coherentes.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "let edad: u32 = 26;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(5.0);
            codigo_resaltado_bloque(
                ui,
                "let puntos = 100;",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(
                    "En el primer ejemplo el tipo está escrito; en el segundo, Rust lo infiere automáticamente.",
                )
                .size(12.5)
                .color(egui::Color32::from_rgb(160, 180, 205)),
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "3. Consulta Data Types", orange);
        ui.indent("conceptos_data_types_referencia", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("La pestaña").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Data Types");
                ui.label(egui::RichText::new("contiene las tablas, rangos y ejemplos completos de cada tipo. Úsala como referencia mientras pruebas tus propios valores en el editor.").size(13.0).color(text_col));
            });
            ui.add_space(10.0);
            if boton_abrir_data_types(ui, cyan) {
                state.lessons.conceptos_tab = 4;
                state.ui.mostrar_conceptos_drawer = false;
                state.ui.conceptos_drawer_tab = 0;
            }
            ui.add_space(14.0);
        });
    } else if current == 5 {
        titulo_seccion(ui, "1. Comments: dejar una nota", cyan);
        ui.indent("conceptos_comments", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Comment");
                ui.label(egui::RichText::new("ayuda a explicar una decisión o una parte del código para que otra persona pueda entenderla más fácilmente.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "// Explica qué hace esta parte del código.",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "2. Doc Comments: documentar una API", cyan);
        ui.indent("conceptos_doc_comments", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Doc Comment");
                ui.label(egui::RichText::new("empieza con").size(13.0).color(text_col));
                codigo_inline_chip(ui, "///");
                ui.label(egui::RichText::new("y se coloca justo encima del elemento que quieres documentar. Por ahora observa su forma; en la siguiente sesión lo aplicaremos a").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Functions");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "/// Describe el propósito de este elemento del proyecto.",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "3. cargo doc: generar documentación", orange);
        ui.indent("conceptos_cargo_doc", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Cargo recoge los").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Doc Comments");
                ui.label(egui::RichText::new("y genera una página HTML que puedes consultar en el navegador.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_terminal_bloque(ui, "cargo doc --open");
            ui.add_space(12.0);
        });
    } else if current == 6 {
        titulo_seccion(ui, "1. Declarar una Function", cyan);
        ui.indent("conceptos_functions_declarar", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Usa")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "fn");
                ui.label(
                    egui::RichText::new("para declarar una")
                    .size(13.0)
                    .color(text_col),
                );
                codigo_inline_chip(ui, "Function");
                ui.label(
                    egui::RichText::new(". Su nombre indica qué tarea realiza y sus llaves contienen las instrucciones.")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "fn saludar() {\n    println!(\"Hola, Rust!\");\n}",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "2. Ejecutar una Function", cyan);
        ui.indent("conceptos_functions_llamada", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Declarar una").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Function");
                ui.label(egui::RichText::new("no la ejecuta. Para utilizarla, escribe su nombre seguido de paréntesis.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "saludar();",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "3. Recibir datos con Parameters", cyan);
        ui.indent("conceptos_functions_parameters", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Los").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Parameters");
                ui.label(egui::RichText::new("permiten que una").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Function");
                ui.label(egui::RichText::new("reciba valores. Cada").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Parameter");
                ui.label(egui::RichText::new("tiene un nombre y un").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Data Type");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "fn mostrar_edad(edad: u32) {\n    println!(\"Edad: {}\", edad);\n}",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "4. Return value", cyan);
        ui.indent("conceptos_functions_return", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Una").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Function");
                ui.label(egui::RichText::new("puede devolver un valor. Después de la flecha se escribe el").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Data Type");
                ui.label(egui::RichText::new("del resultado; la última").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Expression");
                ui.label(egui::RichText::new("del").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Block");
                ui.label(egui::RichText::new("se convierte en ese").size(13.0).color(text_col));
                codigo_inline_chip(ui, "Return value");
                ui.label(egui::RichText::new("cuando no termina en punto y coma.").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(
                ui,
                "fn sumar(a: i32, b: i32) -> i32 {\n    a + b\n}",
                &state.editor.syntax_set,
                &theme,
                "rs",
            );
            ui.add_space(12.0);
        });
    } else if current == 7 {
        titulo_seccion(ui, "Comprueba lo aprendido", orange);
        mostrar_preguntas_conceptos(ui, state, orange, text_col);
    } else {
        titulo_seccion(ui, section_title, cyan);
        ui.indent(format!("conceptos_reto_{current}"), |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                ui.label(
                    egui::RichText::new("Ejemplo para experimentar en el editor:")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(ui, code, &state.editor.syntax_set, &theme, "rs");
            ui.add_space(7.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                ui.label(
                    egui::RichText::new("Concepto clave:")
                        .strong()
                        .size(13.0)
                        .color(orange),
                );
                ui.label(egui::RichText::new(subtitle).size(13.0).color(text_col));
            });
            ui.add_space(12.0);
        });
    }

    if current != 7 {
        titulo_seccion(ui, "Tu práctica", orange);
        ui.indent(format!("conceptos_tarea_{current}"), |ui| {
            ui.add_space(6.0);
            if current == 0 {
                for paso in [
                    "1. Declara una variable usando let.",
                    "2. Declara otra variable con let mut y reasigna su valor.",
                    "3. Declara nuevamente un nombre con let para practicar Shadowing.",
                    "4. Declara una const y escribe su Data Type.",
                    "5. Declara un static usando un entero.",
                    "6. Explica con tus palabras la diferencia entre let, let mut, const y static.",
                ] {
                    ui.label(egui::RichText::new(paso).size(13.0).color(text_col));
                    ui.add_space(4.0);
                }
            } else {
                ui.label(egui::RichText::new(task).size(13.0).color(text_col));
            }
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                ui.label(
                    egui::RichText::new("Escribe o adapta el ejemplo en el editor central y ejecútalo cuando tengas un proyecto Cargo seleccionado.")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(10.0);
        });
    }
}

fn mostrar_preguntas_conceptos(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
    text_col: egui::Color32,
) {
    let preguntas = [
        (
            "1. ¿Qué palabra declara una variable en Rust?",
            ["let", "var", "define"],
            0,
        ),
        (
            "2. ¿Qué permite let mut?",
            [
                "Cambiar el valor de una variable",
                "Crear una constante global",
                "Generar documentación",
            ],
            0,
        ),
        (
            "3. ¿Qué sucede con Shadowing?",
            [
                "Se crea una nueva variable con el mismo nombre",
                "Se borra todo el Block",
                "Se cambia el sistema operativo",
            ],
            0,
        ),
        (
            "4. ¿Qué describe mejor una const?",
            [
                "Un valor fijo conocido durante Compile time",
                "Un valor que cambia en cada llamada",
                "Una función sin Parameters",
            ],
            0,
        ),
        (
            "5. ¿Qué caracteriza a static?",
            [
                "Es un valor global disponible durante la ejecución",
                "Solo existe dentro de un Block",
                "Es una macro de impresión",
            ],
            0,
        ),
        (
            "6. ¿Para qué sirve type?",
            [
                "Para crear un alias legible de un Data Type",
                "Para ejecutar una Function",
                "Para cerrar un Scope",
            ],
            0,
        ),
        (
            "7. ¿Qué indica un Scope?",
            [
                "Dónde puede utilizarse una variable",
                "Cuánto pesa el ejecutable",
                "Qué versión de Cargo está instalada",
            ],
            0,
        ),
        (
            "8. ¿Qué explica RAII?",
            [
                "Los recursos se limpian al salir de su Scope",
                "Las macros se ejecutan después del programa",
                "Los Data Types no necesitan comprobación",
            ],
            0,
        ),
        (
            "9. ¿Qué indica el signo ! en println!?",
            [
                "Que se está invocando una macro",
                "Que la línea es un Comment",
                "Que la variable es mutable",
            ],
            0,
        ),
        (
            "10. ¿Qué hace un Statement?",
            [
                "Realiza una acción",
                "Siempre devuelve un String",
                "Solo documenta una API",
            ],
            0,
        ),
        (
            "11. ¿Qué produce una Expression?",
            [
                "Un valor que puede utilizarse",
                "Un archivo Cargo.lock",
                "Un nuevo toolchain",
            ],
            0,
        ),
        (
            "12. ¿Qué es una Type annotation?",
            [
                "Escribir explícitamente el Data Type de un valor",
                "Cambiar el nombre de una Function",
                "Añadir un Comment al final de una línea",
            ],
            0,
        ),
        (
            "13. ¿Qué símbolo inicia un Comment de una línea?",
            ["//", "///", "##"],
            0,
        ),
        (
            "14. ¿Qué permite cargo doc?",
            [
                "Generar documentación HTML desde los Doc Comments",
                "Cambiar una variable a mutable",
                "Crear un Block automáticamente",
            ],
            0,
        ),
        (
            "15. ¿Qué palabra declara una Function?",
            ["fn", "func", "method"],
            0,
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
            let seleccionada =
                state.lessons.conceptos_preguntas_respuestas[index] == Some(opcion_index);
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
                state.lessons.conceptos_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.conceptos_preguntas_respuestas[index] {
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

    let respondidas = state
        .lessons
        .conceptos_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.conceptos_preguntas_respuestas[*index] == Some(*correcta)
        })
        .count();
    let completada = respondidas == preguntas.len() && correctas == preguntas.len();

    titulo_seccion(
        ui,
        if completada {
            "Sesión completada"
        } else {
            "Progreso de la evaluación"
        },
        orange,
    );
    ui.label(
        egui::RichText::new(if completada {
            "¡Excelente! Has respondido correctamente las 15 preguntas."
                .to_owned()
        } else {
            format!("Respuestas correctas: {correctas}/15 · Respondidas: {respondidas}/15")
        })
        .size(13.0)
        .color(if completada {
            orange
        } else {
            text_col
        }),
    );
}

fn boton_abrir_data_types(ui: &mut egui::Ui, cyan: egui::Color32) -> bool {
    let (button_rect, response) =
        ui.allocate_exact_size(egui::vec2(190.0, 30.0), egui::Sense::click());
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    if hovered || pressed {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

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
    let text_color = if hovered || pressed {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(220, 235, 245)
    };

    ui.painter().rect(
        button_rect,
        egui::CornerRadius::same(5),
        button_fill,
        egui::Stroke::new(1.0, button_stroke),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        button_rect.center(),
        egui::Align2::CENTER_CENTER,
        "Abrir pestaña Data Types",
        egui::FontId::proportional(12.0),
        text_color,
    );

    response.clicked()
}

fn retos_conceptos() -> [(&'static str, &'static str, &'static str, &'static str); 8] {
    [
        (
            "Variables paso a paso",
            "Variables",
            "Primero declararás variables con let, después aprenderás a cambiar sus valores con let mut y al final conocerás el Shadowing.",
            "let x = 5;",
        ),
        (
            "Macros",
            "Declarative macros y Compiler Attributes",
            "Una Declarative Macro puede adaptar o generar código antes de compilar. También conocerás los Compiler Attributes, como #[allow(...)], que controlan advertencias del compilador.",
            "println!(\"Hola, Rust!\");",
        ),
        (
            "Blocks & Scope",
            "Block y Scope",
            "Un Block agrupa instrucciones entre llaves. Su Scope indica dónde existen sus variables y pueden utilizarse: desde su declaración hasta la llave de cierre. Los bloques interiores pueden leer variables creadas en bloques exteriores.",
            "let exterior = 10;\n{\n    let interior = 20;\n    let resultado = exterior + interior;\n}\n",
        ),
        (
            "Statements & Expressions",
            "Statement y Expression",
            "Un Statement ejecuta una acción, mientras una Expression se evalúa y produce un valor. La última expresión de un bloque puede convertirse en su resultado si no termina en punto y coma.",
            "let base = 6;\nlet total = {\n    base * 2\n};",
        ),
        (
            "Data Types",
            "Data Types",
            "Los Data Types indican qué clase de valor puede guardar una variable. Consulta la pestaña Data Types para revisar todos los tipos con detalle.",
            "let edad: u32 = 26;",
        ),
        (
            "Comments & Docs",
            "Comments, Doc Comments y cargo doc",
            "Los comentarios normales ayudan a leer el código. Los Doc Comments describen APIs y Cargo puede convertirlos en documentación HTML con Markdown.",
            "// Nota interna\n/// Describe el propósito de este elemento del proyecto.",
        ),
        (
            "Functions",
            "fn, Parameters y Return value",
            "Una Function reúne instrucciones bajo un nombre. Puede recibir Parameters y devolver un Return value para que otra parte del programa utilice el resultado.",
            "fn sumar(a: i32, b: i32) -> i32 {\n    a + b\n}",
        ),
        (
            "Questions",
            "15 Questions",
            "La evaluación repasa los conceptos principales de la sesión y se completa cuando respondes correctamente todas las preguntas.",
            "",
        ),
    ]
}
