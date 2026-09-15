use std::sync::Arc;
use std::sync::atomic::Ordering;

use eframe::egui;

use crate::app::AppState;
use crate::application::terminal_service::{
    command_updates_manifest, detect_cargo_new, is_cargo_run, run_shell_command,
};
use crate::infrastructure::process::captured_text;
use crate::routes::AppRoute;

pub fn mostrar_componente_terminal_3_modos(
    ui: &mut egui::Ui,
    _cmd_predeterminado: &str,
    state: &mut AppState,
) {
    let mut term_frame = egui::Frame::new();
    term_frame.fill = egui::Color32::from_rgb(11, 15, 22);
    term_frame.inner_margin = egui::Margin::symmetric(14, 12);
    term_frame.corner_radius = egui::CornerRadius::same(4);
    term_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95));
    term_frame.shadow = egui::Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: egui::Color32::from_black_alpha(180),
    };

    term_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        let salida_progreso = ui.ctx().animate_bool(
            egui::Id::new("terminal_salida_expandida"),
            state.terminal.show_terminal_history,
        );
        let comando_en_ejecucion = state.terminal.term_command_running.load(Ordering::Relaxed)
            || state.terminal.task_manager.is_busy();

        // --- CABECERA DE LA TERMINAL FLOTANTE ---
        ui.horizontal(|ui| {
            let history_len = state.terminal.term_history.lock().map(|h| h.len()).unwrap_or(0);
            // Botón Minimizar/mostrar salida (SVG con contador al lado)
            let (hist_rect, hist_resp) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
            let hist_hovered = hist_resp.hovered();
            let hist_tint = if state.terminal.show_terminal_history {
                egui::Color32::from_rgb(100, 200, 255)
            } else if hist_hovered {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_rgb(160, 180, 205)
            };
            let img_hist = egui::Image::new(egui::include_image!("../../../../assets/icons/semi-select-svgrepo-com.svg"))
                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                .tint(hist_tint);
            img_hist.paint_at(ui, hist_rect);
            if hist_resp.clicked() {
                state.terminal.show_terminal_history = !state.terminal.show_terminal_history;
            }
            hist_resp.on_hover_text(format!(
                "{} última salida con transición ({} líneas)",
                if state.terminal.show_terminal_history { "Minimizar" } else { "Mostrar" },
                history_len
            ));

            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(format!("({})", history_len))
                    .size(11.5)
                    .color(egui::Color32::from_rgb(120, 145, 175)),
            );
            if comando_en_ejecucion {
                ui.add_space(8.0);
                ui.spinner();
                ui.label(
                    egui::RichText::new("Ejecutando...")
                        .size(11.5)
                        .color(egui::Color32::from_rgb(255, 205, 80))
                        .monospace(),
                );
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 1. Botón Cerrar Terminal (SVG)
                let (close_rect, close_resp) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
                let close_hovered = close_resp.hovered();
                let close_tint = if close_hovered {
                    egui::Color32::from_rgb(255, 120, 120)
                } else {
                    egui::Color32::from_rgb(160, 175, 195)
                };
                let img_close = egui::Image::new(egui::include_image!("../../../../assets/icons/close-circle-svgrepo-com.svg"))
                    .fit_to_exact_size(egui::vec2(18.0, 18.0))
                    .tint(close_tint);
                img_close.paint_at(ui, close_rect);
                if close_resp.clicked() {
                    state.ui.show_terminal_modal = false;
                }
                close_resp.on_hover_text("Cerrar Terminal (Ctrl + T)");

                ui.add_space(10.0);

                // 2. Botón Limpiar Terminal (clean.svg)
                let (clean_rect, clean_resp) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
                let clean_hovered = clean_resp.hovered();
                let clean_tint = if clean_hovered {
                    egui::Color32::from_rgb(255, 140, 140)
                } else {
                    egui::Color32::from_rgb(160, 175, 195)
                };
                let img_clean = egui::Image::new(egui::include_image!("../../../../assets/icons/clean.svg"))
                    .fit_to_exact_size(egui::vec2(18.0, 18.0))
                    .tint(clean_tint);
                img_clean.paint_at(ui, clean_rect);
                if clean_resp.clicked()
                    && let Ok(mut history) = state.terminal.term_history.lock() {
                        history.clear();
                    }
                clean_resp.on_hover_text("Limpiar terminal");

                ui.add_space(10.0);

                // 3. Botón Copiar Comando (copy-svgrepo-com.svg)
                let (copy_rect, copy_resp) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
                let copy_hovered = copy_resp.hovered();
                let copy_tint = if copy_hovered {
                    egui::Color32::from_rgb(100, 200, 255)
                } else {
                    egui::Color32::from_rgb(160, 175, 195)
                };
                let img_copy = egui::Image::new(egui::include_image!("../../../../assets/icons/copy-svgrepo-com.svg"))
                    .fit_to_exact_size(egui::vec2(18.0, 18.0))
                    .tint(copy_tint);
                img_copy.paint_at(ui, copy_rect);
                if copy_resp.clicked() {
                    let text_to_copy = if !state.terminal.term_input.trim().is_empty() {
                        state.terminal.term_input.trim().to_string()
                    } else {
                        _cmd_predeterminado.to_string()
                    };
                    ui.ctx().output_mut(|o| {
                        o.commands.push(egui::OutputCommand::CopyText(text_to_copy))
                    });
                }
                copy_resp.on_hover_text("Copiar comando al portapapeles");
            });
        });

        ui.add_space(4.0);

        // Línea de entrada limpia sin marcos ni cajas negras flotantes
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(crate::platform::terminal_prompt(&state.terminal.term_cwd))
                    .strong()
                    .color(egui::Color32::from_rgb(255, 160, 50))
                    .monospace()
                    .size(13.0),
            );

            let input_response = ui.add_enabled(
                !comando_en_ejecucion,
                egui::TextEdit::singleline(&mut state.terminal.term_input)
                    .frame(egui::Frame::NONE)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );

            // Mantener el foco automático en la terminal continuamente
            if !comando_en_ejecucion {
                input_response.request_focus();
            }

            // Ejecutar comando al presionar Enter
            if !comando_en_ejecucion && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let cmd_str = state.terminal.term_input.trim().to_string();
                if !cmd_str.is_empty() {
                    state.ui.show_cargo_output_modal.store(false, Ordering::Relaxed);
                    state.terminal.show_terminal_history = true;

                    // En el Code Lab, solo cargo run abre el panel inferior
                    // SALIDA para mostrar el resultado del programa.
                    let es_codelab_conceptos =
                        state.ui.ruta_actual == AppRoute::Comenzando && state.lessons.conceptos_tab == 0;
                    let es_codelab_foundations =
                        state.ui.ruta_actual == AppRoute::TutorialCargo && state.lessons.pilares_step == 1;
                    if is_cargo_run(&cmd_str)
                        && (es_codelab_conceptos || es_codelab_foundations)
                    {
                        state.ui.mostrar_console_drawer = true;
                        state.ui.conceptos_salida_abierta = es_codelab_conceptos;
                        state.ui.show_terminal_modal = false;
                    }

                    // La terminal funciona como una vista de la última
                    // ejecución: el comando actual queda en el prompt y su
                    // nueva salida reemplaza la anterior.
                    if let Ok(mut history) = state.terminal.term_history.lock() {
                        history.clear();
                    }

                    // Manejo especial de comando 'cd' para persistir el directorio de navegación
                    if cmd_str == "cd" || cmd_str.starts_with("cd ") {
                        let target_arg = if cmd_str == "cd" {
                            ""
                        } else {
                            cmd_str[3..].trim()
                        };
                        let new_path = if target_arg.is_empty() || target_arg == "~" {
                            crate::platform::home_dir()
                        } else if target_arg == ".." {
                            state
                                .terminal.term_cwd
                                .parent()
                                .map(|p| p.to_path_buf())
                                .unwrap_or_else(|| state.terminal.term_cwd.clone())
                        } else {
                            let candidate = state.terminal.term_cwd.join(target_arg);
                            candidate.canonicalize().unwrap_or(candidate)
                        };

                        if new_path.is_dir() {
                            state.terminal.term_cwd = new_path;
                            if let Ok(mut history) = state.terminal.term_history.lock() {
                                history.push("────────────────────────────────────────".to_string());
                            }
                        } else {
                            if let Ok(mut history) = state.terminal.term_history.lock() {
                                history.push(format!(
                                    "cd: {}: No existe el directorio",
                                    target_arg
                                ));
                                history.push("────────────────────────────────────────".to_string());
                            }
                        }
                    } else {
                        // Ejecución en hilo secundario asíncrono (evita congelar el GUI)
                        let history_arc = Arc::clone(&state.terminal.term_history);
                        let output_arc = state.obtener_output_activo();
                        let command_running = Arc::clone(&state.terminal.term_command_running);
                        let cwd = state.terminal.term_cwd.clone();
                        let cmd = cmd_str.clone();
                        let ctx = ui.ctx().clone();
                        let cargo_new_event = detect_cargo_new(&cmd, &cwd);
                        let pending_project_arc = Arc::clone(&state.terminal.term_pending_project);
                        let reload_manifest = command_updates_manifest(&cmd);
                        let pending_manifest_arc = Arc::clone(&state.terminal.term_pending_manifest);

                        if cmd.starts_with("cargo ")
                            && let Ok(mut out) = output_arc.lock() {
                                *out = "Compilando con Cargo...".to_string();
                            }
                        command_running.store(true, Ordering::Relaxed);

                        let history_error = Arc::clone(&history_arc);
                        let command_running_error = Arc::clone(&command_running);
                        let task_result = state.terminal.task_manager.spawn("terminal-command", move || {
                            let output = run_shell_command(&cmd, &cwd);

                            match output {
                                Ok(out) => {
                                    if let Some((project_dir, project_name, is_lib)) = cargo_new_event {
                                        let project_dir = std::fs::canonicalize(&project_dir)
                                            .unwrap_or(project_dir);
                                        if out.status.success()
                                            && project_dir.join("Cargo.toml").is_file()
                                            && let Ok(mut pending) = pending_project_arc.lock()
                                        {
                                            *pending = Some((project_dir, project_name, is_lib));
                                        }
                                    }

                                    if reload_manifest && out.status.success() {
                                        let manifest_path = cwd.join("Cargo.toml");
                                        if manifest_path.is_file()
                                            && let Ok(mut pending) = pending_manifest_arc.lock()
                                        {
                                            *pending = Some(manifest_path);
                                        }
                                    }

                                    let stdout = captured_text(&out.stdout);
                                    let stderr = captured_text(&out.stderr);
                                    if let Ok(mut history) = history_arc.lock() {
                                        if !stdout.is_empty() {
                                            for line in stdout.lines() {
                                                history.push(line.to_string());
                                            }
                                        }
                                        if !stderr.is_empty() {
                                            for line in stderr.lines() {
                                                history.push(line.to_string());
                                            }
                                        }
                                        if stdout.trim().is_empty() && stderr.trim().is_empty() {
                                            history.push("El comando terminó sin salidas.".to_string());
                                        }
                                        history.push("────────────────────────────────────────".to_string());
                                    }

                                    // Sincronizar el cuadro de salida dedicado para comandos cargo
                                    if cmd.starts_with("cargo ") {
                                        let mut combined = stdout.clone();
                                        if !stderr.is_empty() {
                                            if !combined.is_empty() {
                                                combined.push_str("\n\n");
                                            }
                                            combined.push_str("[Errores/Warnings]:\n");
                                            combined.push_str(&stderr);
                                            if cmd.starts_with("cargo expand")
                                                && (stderr.contains("no such command")
                                                    || stderr.contains("not found"))
                                            {
                                                combined.push_str(
                                                    "\n\n💡 Nota: 'cargo expand' requiere la herramienta externa. Puedes instalarla ejecutando:\ncargo install cargo-expand",
                                                );
                                            }
                                        }
                                        if combined.is_empty() {
                                            combined = "El comando terminó sin salidas.".to_string();
                                        }
                                        if let Ok(mut out_lock) = output_arc.lock() {
                                            *out_lock = combined;
                                        }
                                    }
                                }
                                Err(err) => {
                                    if let Ok(mut history) = history_arc.lock() {
                                        history.push(format!("Error ejecutando comando: {}", err));
                                        history.push("────────────────────────────────────────".to_string());
                                    }
                                }
                            }
                            command_running.store(false, Ordering::Relaxed);
                            ctx.request_repaint();
                        });
                        if let Err(error) = task_result {
                            command_running_error.store(false, Ordering::Relaxed);
                            if let Ok(mut history) = history_error.lock() {
                                history.push(format!("No se pudo iniciar el comando: {error}"));
                                history.push("────────────────────────────────────────".to_string());
                            }
                        }
                    }
                }
                // Mantener foco en el campo de texto tras presionar Enter
                input_response.request_focus();
            }
        });

        // La línea activa queda arriba; debajo se muestra solamente la salida.
        mostrar_historial_terminal(ui, state, salida_progreso);

        /*
        // --- MODOS RESERVADOS PARA EL FUTURO (MODO 0: ESTÁTICA, MODO 2: PTY REAL) ---
        // Si en el futuro necesitas habilitar el modo PTY nativo o estático:
        //
        // MODO 0 (Estática):
        // ui.label(egui::RichText::new(cmd_predeterminado).strong().color(egui::Color32::WHITE).monospace());
        //
        // MODO 2 (PTY Real Linux con portable-pty):
        // std::thread::spawn(move || { ... portable_pty::NativePtySystem ... });
         */
    });
}

fn mostrar_historial_terminal(ui: &mut egui::Ui, state: &AppState, progreso: f32) {
    if progreso > 0.001
        && let Ok(history) = state.terminal.term_history.lock()
        && !history.is_empty()
    {
        egui::ScrollArea::vertical()
            // La salida corta ocupa solo lo que necesita; las salidas
            // largas siguen teniendo un límite y usan scroll vertical.
            .max_height(260.0 * progreso)
            .auto_shrink([false, true])
            .id_salt("scroll_terminal_history")
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                for (idx, line) in history.iter().enumerate() {
                    ui.push_id(idx, |ui| {
                        if line.starts_with("────────") {
                            // Divisor real y adaptable al ancho de la
                            // terminal, en lugar de una cadena fija de
                            // caracteres que se corta o queda corta.
                            let (separator_rect, _) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), 8.0),
                                egui::Sense::hover(),
                            );
                            let y = separator_rect.center().y;
                            ui.painter().line_segment(
                                [
                                    egui::pos2(separator_rect.left() + 8.0, y),
                                    egui::pos2(separator_rect.right() - 8.0, y),
                                ],
                                egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgba_unmultiplied(255, 160, 50, 135),
                                ),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(line)
                                    .monospace()
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(200, 230, 255)),
                            );
                        }
                    });
                }
            });
        ui.add_space(6.0);
    }
}
