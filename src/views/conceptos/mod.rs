use crate::app::AppState;
use crate::application::project_service::ProjectService;
use crate::components::navigation::{
    separador_vertical_centrado, underline_tab, underline_tab_destacado,
};
use crate::infrastructure::process::{captured_text, run_cargo};
#[allow(unused_imports)]
use crate::routes::AppRoute;
use crate::views::pilares::anatomy::{
    codigo_inline_chip, codigo_resaltado_bloque, punto_lista, titulo_seccion,
};
pub mod primitivos;
use self::primitivos::{
    mostrar_categoria_booleanos, mostrar_categoria_caracteres, mostrar_categoria_casting,
    mostrar_categoria_enteros_interactiva, mostrar_categoria_flotantes,
    mostrar_enteros_interactivo,
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
            "../../../assets/icons/folder-off-svgrepo-com.svg"
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
                "../../../assets/icons/file-svgrepo-com.svg"
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
    // Selector de Categoría (Enteros, Decimales, Bool, Char, Casting)
    ui.horizontal(|ui| {
        for (cat_idx, (cat_label, cat_color)) in [
            ("Enteros", egui::Color32::from_rgb(255, 160, 50)),
            ("Decimales", egui::Color32::from_rgb(255, 160, 50)),
            ("Booleanos", egui::Color32::from_rgb(255, 160, 50)),
            ("Caracteres", egui::Color32::from_rgb(255, 160, 50)),
            ("Casting (as)", egui::Color32::from_rgb(255, 160, 50)),
        ]
        .iter()
        .enumerate()
        {
            let es_sel = state.lessons.tipo_primitivo_categoria == cat_idx;
            let text_rich = egui::RichText::new(*cat_label).strong().color(if es_sel {
                *cat_color
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            });
            if ui.add(egui::Button::new(text_rich).frame(es_sel)).clicked() {
                state.lessons.tipo_primitivo_categoria = cat_idx;
            }
            ui.add_space(4.0);
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
            1 => mostrar_categoria_flotantes(ui),
            2 => mostrar_categoria_booleanos(ui, state),
            3 => mostrar_categoria_caracteres(ui),
            _ => mostrar_categoria_casting(ui),
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

pub fn mostrar_contenido_macros(ui: &mut egui::Ui) {
    ui.heading(
        egui::RichText::new("Categorías de Macros en Rust")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
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
            .color(egui::Color32::WHITE),
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
            .color(egui::Color32::WHITE),
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
                        "../../../assets/icons/book-line.svg"
                    ))
                    .fit_to_exact_size(egui::Vec2::new(18.0, 18.0));
                    let (_id, book_rect) = ui.allocate_space(egui::Vec2::new(18.0, 18.0));
                    img_book.paint_at(ui, book_rect);
                    ui.add_space(4.0);

                    let tabs_teoria = [
                        (7, "Core Mechanics"),
                        (4, "Data Types"),
                        (6, "Doc & Comentarios"),
                        (8, "Functions"),
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
                                "../../../assets/icons/monitor-code-line.svg"
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
    if state.lessons.conceptos_tab == 0 {
        mostrar_conceptos_codelab(ui, state);
        return;
    }

    match state.lessons.conceptos_tab {
        0 => funciones::mostrar(ui, state),
        4 => mostrar_contenido_tipos_primitivos(ui, state),
        5 => mostrar_contenido_macros(ui),
        6 => mostrar_seccion_documentacion(ui),
        8 => funciones::mostrar(ui, state),
        7 => {
            ui.heading(
                egui::RichText::new("Mecánicas Centrales de Rust")
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.add_space(20.0);

            // Fusión de las 3 vistas teóricas
            mutabilidad::mostrar(ui, state);

            ui.add_space(30.0);
            ui.separator();
            ui.add_space(30.0);

            scopes::mostrar(ui, state);

            ui.add_space(30.0);
            ui.separator();
            ui.add_space(30.0);

            statements::mostrar(ui, state);
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
            navigation_total: 5,
        },
        |ui, state, orange, cyan| {
            mostrar_retos_conceptos_drawer(ui, state, orange, cyan);
        },
    );
    state.ui.mostrar_conceptos_drawer = shell_state.drawer_open;
    state.ui.conceptos_drawer_tab = shell_state.drawer_tab;
    state.ui.conceptos_salida_abierta = shell_state.output_open;
    crate::views::pilares::anatomy::aplicar_navegacion_codelab(
        &mut state.lessons.conceptos_codelab_reto_actual,
        shell_state.navigation_delta,
        5,
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
            "Declaración y Shadowing",
            "Crea una variable, cambia su valor y prueba Shadowing.",
        ),
        1 => (
            "Blocks & Scope",
            "Crea un Block y observa qué variables puedes utilizar dentro de su Scope.",
        ),
        2 => (
            "Statements & Expressions",
            "Distingue qué líneas ejecutan una acción y qué expresiones producen un valor.",
        ),
        3 => (
            "Data Types: enteros",
            "Elige entre enteros con signo i o sin signo u para comparar sus bits y rangos.",
        ),
        _ => (
            "Comentarios y documentación",
            "Añade un comentario normal y documenta una función con ///.",
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
        let mut tag_topic = egui::Frame::new();
        tag_topic.fill = egui::Color32::from_rgb(22, 28, 38);
        tag_topic.inner_margin = egui::Margin::symmetric(8, 2);
        tag_topic.corner_radius = egui::CornerRadius::same(10);
        tag_topic.show(ui, |ui| {
            ui.label(
                egui::RichText::new(subtitle)
                    .size(11.5)
                    .color(egui::Color32::from_rgb(160, 185, 220)),
            );
        });
    });

    ui.add_space(12.0);
    if current == 0 {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new("Una variable se declara con")
                    .size(13.5)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "let");
            ui.label(
                egui::RichText::new(
                    "y recibe un valor inicial. Las variables son inmutables por defecto; cuando necesites cambiar su valor, usa",
                )
                .size(13.5)
                .color(text_col),
            );
            codigo_inline_chip(ui, "let mut");
            ui.label(
                egui::RichText::new(
                    ". Si vuelves a declarar el mismo nombre, haces Shadowing.",
                )
                    .size(13.5)
                    .color(text_col),
            );
        });
    } else if current == 1 {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new("Un bloque es un grupo de instrucciones rodeado por")
                    .size(13.5)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "{ }");
            ui.label(egui::RichText::new(". El").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Scope");
            ui.label(
                egui::RichText::new(
                    "es el alcance: la zona donde una variable existe y puede utilizarse.",
                )
                .size(13.5)
                .color(text_col),
            );
        });
    } else if current == 2 {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Un").size(13.5).color(text_col));
            codigo_inline_chip(ui, "Statement");
            ui.label(
                egui::RichText::new("ejecuta una acción; una")
                    .size(13.5)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "Expression");
            ui.label(
                egui::RichText::new("se evalúa y produce un valor.")
                    .size(13.5)
                    .color(text_col),
            );
        });
    } else if current == 3 {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(
                    "Los enteros no tienen parte decimal. Rust ofrece tipos con signo, como",
                )
                .size(13.5)
                .color(text_col),
            );
            codigo_inline_chip(ui, "i32");
            ui.label(
                egui::RichText::new("o sin signo, como")
                    .size(13.5)
                    .color(text_col),
            );
            codigo_inline_chip(ui, "u32");
            ui.label(
                egui::RichText::new(". Elige una familia para ver su tabla.")
                    .size(13.5)
                    .color(text_col),
            );
        });
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
        titulo_seccion(ui, "Declaración de una variable", cyan);
        ui.indent("conceptos_variable_let", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                ui.label(egui::RichText::new("Usa").size(13.0).color(text_col));
                codigo_inline_chip(ui, "let");
                ui.label(
                    egui::RichText::new("para crear una variable y darle un valor inicial.")
                        .size(13.0)
                        .color(text_col),
                );
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(ui, "let x = 5;", &state.editor.syntax_set, &theme, "rs");
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "Mutabilidad y reasignación", cyan);
        ui.indent("conceptos_variable_mut", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
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

        titulo_seccion(ui, "Nueva declaración con el mismo nombre", cyan);
        ui.indent("conceptos_shadowing_nueva_declaracion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
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

        titulo_seccion(ui, "Shadowing vs reasignación", cyan);
        ui.indent("conceptos_shadowing_comparacion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
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
                punto_lista(ui, bullet_col);
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
    } else if current == 1 {
        titulo_seccion(ui, "Blocks: un grupo de instrucciones", cyan);
        ui.indent("conceptos_blocks_definicion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                ui.label(
                    egui::RichText::new("Un bloque agrupa instrucciones entre las llaves")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "{ }");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
            ui.add_space(6.0);
            codigo_resaltado_bloque(ui, code, &state.editor.syntax_set, &theme, "rs");
            ui.add_space(12.0);
        });

        titulo_seccion(ui, "Scope: el alcance de una variable", cyan);
        ui.indent("conceptos_scope_alcance", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                codigo_inline_chip(ui, "exterior");
                ui.label(
                    egui::RichText::new(
                        "puede utilizarse dentro del bloque porque fue creada afuera.",
                    )
                    .size(13.0)
                    .color(text_col),
                );
            });
            ui.add_space(5.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                codigo_inline_chip(ui, "interior");
                ui.label(
                    egui::RichText::new(
                        "solo existe dentro del bloque; al cerrarse, termina su Scope.",
                    )
                    .size(13.0)
                    .color(text_col),
                );
            });
            ui.add_space(12.0);
        });
    } else if current == 2 {
        titulo_seccion(ui, "Statement: una instrucción", cyan);
        ui.indent("conceptos_statements_definicion", |ui| {
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                punto_lista(ui, bullet_col);
                ui.label(
                    egui::RichText::new(
                        "Un Statement realiza una acción, como declarar una variable. Normalmente termina con",
                    )
                    .size(13.0)
                    .color(text_col),
                );
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
                punto_lista(ui, bullet_col);
                ui.label(
                    egui::RichText::new(
                        "Una Expression se evalúa y produce un valor. La última expresión de un bloque puede devolver ese resultado si no lleva",
                    )
                    .size(13.0)
                    .color(text_col),
                );
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
    } else if current == 3 {
        titulo_seccion(ui, section_title, cyan);
        ui.indent("conceptos_data_types_enteros", |ui| {
            mostrar_enteros_interactivo(ui, state, false);
        });
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

    titulo_seccion(ui, "Tu práctica", orange);
    ui.indent(format!("conceptos_tarea_{current}"), |ui| {
        ui.add_space(6.0);
        if current == 0 {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Crea una variable y cambia su valor usando")
                        .size(13.0)
                        .color(text_col),
                );
                codigo_inline_chip(ui, "let mut");
                ui.label(egui::RichText::new(".").size(13.0).color(text_col));
            });
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

fn retos_conceptos() -> [(&'static str, &'static str, &'static str, &'static str); 5] {
    [
        (
            "Variables y Shadowing",
            "let · mut · shadowing",
            "Una variable se declara con let y recibe un valor inicial. Las variables son inmutables por defecto; usa let mut cuando necesites cambiar su valor. Si vuelves a declarar el mismo nombre, haces Shadowing.",
            "let x = 5;",
        ),
        (
            "Blocks & Scope",
            "{ } · duración y visibilidad",
            "Un Block agrupa instrucciones entre llaves. Su Scope es el alcance donde sus variables existen y pueden utilizarse: desde su declaración hasta la llave de cierre. Los bloques interiores pueden leer variables creadas en bloques exteriores.",
            "let exterior = 10;\n{\n    let interior = 20;\n    let resultado = exterior + interior;\n}\n",
        ),
        (
            "Statements & Expressions",
            "acción · valor · punto y coma",
            "Un Statement ejecuta una acción, mientras una Expression se evalúa y produce un valor. La última expresión de un bloque puede convertirse en su resultado si no termina en punto y coma.",
            "let base = 6;\nlet total = {\n    base * 2\n};",
        ),
        (
            "Data Types",
            "enteros · i · u · bits",
            "Los enteros no tienen parte decimal. La familia i admite valores negativos y la familia u solo admite cero y positivos. Selecciona una familia para revisar sus bits, rangos y ejemplos.",
            "let x = 5;",
        ),
        (
            "Comments & Docs",
            "// · /// · cargo doc",
            "Los comentarios normales ayudan a leer el código. Los Doc Comments describen APIs y Cargo puede convertirlos en documentación HTML con Markdown.",
            "// Nota interna\n/// Calcula el doble de un número.\nfn doble(x: i32) -> i32 { x * 2 }\n\n// cargo doc --open",
        ),
    ]
}
