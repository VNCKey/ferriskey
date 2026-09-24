use eframe::egui;
use std::sync::Mutex;
use std::thread;

use crate::app::AppState;
use crate::components::navigation::underline_tab;
use crate::utils::helpers::formatear_tamano_bytes;

use crate::domain::crate_registry::*;

#[derive(Clone, Debug)]
pub struct CratesIoViewState {
    pub search_query: String,
    pub is_search_active: bool,
    pub is_loading: bool,
    pub results: Vec<CrateApiItem>,
    pub error_msg: Option<String>,
    pub selected_category: usize,
    pub copied_feedback: Option<String>,
    pub selected_crate: Option<CrateApiItem>,
    pub full_detail: Option<CrateFullDetailResponse>,
    pub is_loading_detail: bool,
    pub detail_tab: usize,
    pub anim_needs_restart: bool,
    pub anim_start_time: f64,
    pub last_anim_trigger: f64,
    pub summary_data: Option<SummaryApiResponse>,
    pub is_loading_summary: bool,
}

impl Default for CratesIoViewState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            is_search_active: false,
            is_loading: false,
            results: Vec::new(),
            error_msg: None,
            selected_category: 0,
            copied_feedback: None,
            selected_crate: None,
            full_detail: None,
            is_loading_detail: false,
            detail_tab: 0,
            anim_needs_restart: true,
            anim_start_time: 0.0,
            last_anim_trigger: 0.0,
            summary_data: None,
            is_loading_summary: false,
        }
    }
}

static VIEW_STATE: Mutex<Option<CratesIoViewState>> = Mutex::new(None);

pub fn mostrar_tab_crates_io_web(ui: &mut egui::Ui, state: &mut AppState) {
    let mut vs = {
        let mut guard = VIEW_STATE.lock().unwrap();
        guard.get_or_insert_with(CratesIoViewState::default).clone()
    };

    let mut state_changed = false;

    if let Some(selected) = vs.selected_crate.clone() {
        mostrar_detalle_crate(ui, &selected, &mut vs, state, &mut state_changed);
    } else {
        mostrar_lista_crates(ui, &mut vs, state, &mut state_changed);
    }

    if state_changed {
        let mut guard = VIEW_STATE.lock().unwrap();
        if let Some(current) = guard.as_mut() {
            current.search_query = vs.search_query;
            current.is_search_active = vs.is_search_active;
            current.results = vs.results;
            current.selected_category = vs.selected_category;
            current.copied_feedback = vs.copied_feedback;
            current.selected_crate = vs.selected_crate;
            current.full_detail = vs.full_detail;
            current.is_loading_detail = vs.is_loading_detail;
            current.detail_tab = vs.detail_tab;
            current.anim_start_time = vs.anim_start_time;
            current.last_anim_trigger = vs.last_anim_trigger;
            current.summary_data = vs.summary_data;
            current.is_loading_summary = vs.is_loading_summary;
            if !vs.anim_needs_restart {
                current.anim_needs_restart = false;
            }
        }
    }
}

fn mostrar_lista_crates(
    ui: &mut egui::Ui,
    vs: &mut CratesIoViewState,
    state: &mut AppState,
    state_changed: &mut bool,
) {
    let now = ui.input(|i| i.time);
    if vs.anim_needs_restart || vs.last_anim_trigger != state.ui.anim_trigger {
        vs.anim_start_time = now;
        vs.last_anim_trigger = state.ui.anim_trigger;
        vs.anim_needs_restart = false;
        *state_changed = true;
    }

    let elapsed = (now - vs.anim_start_time).max(0.0);
    if elapsed < 2.0 {
        ui.ctx().request_repaint();
    }

    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);
    let text_color = egui::Color32::from_rgb(200, 210, 225);

    let mut card_frame = egui::Frame::new();
    card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    card_frame.inner_margin = egui::Margin::same(12);
    card_frame.corner_radius = egui::CornerRadius::same(8);
    card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let card_w: f32 = 285.0;
            let col_spacing: f32 = 14.0;
            let total_grid_w: f32 = 3.0 * card_w + 2.0 * col_spacing;
            let available_w = ui.available_width();
            let content_w = total_grid_w.min(available_w);
            let left_margin = ((available_w - content_w) / 2.0).max(0.0);

            ui.add_space(8.0);

            // --- BARRA DE BÚSQUEDA (CARD COMPACTO Y CENTRADO) ---
            let is_loading = vs.is_loading || (vs.is_loading_summary && !vs.is_search_active);
            let spinner_w = if is_loading { 18.0 + 8.0 } else { 0.0 };
            let btn_w: f32 = 88.0;
            let input_w: f32 = 300.0;
            let inner_search_w = input_w + 10.0 + btn_w + spinner_w;
            let search_card_w = (inner_search_w + 24.0).min(available_w); // 24.0 = margin de 12px a cada lado
            let search_card_margin = ((available_w - search_card_w) / 2.0).max(0.0);

            ui.horizontal_top(|ui| {
                if search_card_margin > 0.0 {
                    ui.add_space(search_card_margin);
                }
                ui.vertical(|ui| {
                    ui.set_width(search_card_w);

                    card_frame.show(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), 28.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;

                                let search_resp = ui.add_sized(
                                    egui::vec2(input_w, 24.0),
                                    egui::TextEdit::singleline(&mut vs.search_query)
                                        .hint_text("Ingrese nombre del paquete...")
                                        .font(egui::FontId::proportional(14.0))
                                        .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(2, 2))),
                                );

                                let text_rect = search_resp.rect;
                                let bottom_y = text_rect.bottom() + 1.0;
                                let is_focused = search_resp.has_focus();
                                let line_color = if is_focused {
                                    cyan
                                } else {
                                    egui::Color32::from_rgb(55, 125, 155)
                                };
                                let line_width = if is_focused { 2.0 } else { 1.0 };

                                ui.painter().line_segment(
                                    [
                                        egui::pos2(text_rect.left(), bottom_y),
                                        egui::pos2(text_rect.right(), bottom_y),
                                    ],
                                    egui::Stroke::new(line_width, line_color),
                                );

                                let enter_pressed = search_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                                if search_resp.changed() {
                                    *state_changed = true;
                                    if vs.search_query.trim().is_empty() {
                                        vs.is_search_active = false;
                                        vs.results.clear();
                                        vs.anim_start_time = now;
                                        if vs.summary_data.is_none() && !vs.is_loading_summary {
                                            lanzar_resumen_asincrono(ui.ctx().clone());
                                        }
                                    }
                                }

                                ui.add_space(10.0);

                                if boton_custom_cyan_buscar(ui, btn_w).clicked() || enter_pressed {
                                    let clean = vs.search_query.trim().to_string();
                                    if !clean.is_empty() {
                                        vs.is_search_active = true;
                                        vs.results.clear();
                                        vs.anim_start_time = now;
                                        *state_changed = true;
                                        lanzar_busqueda_asincrona(clean, ui.ctx().clone());
                                    } else {
                                        vs.is_search_active = false;
                                        vs.results.clear();
                                        vs.anim_start_time = now;
                                        *state_changed = true;
                                        if vs.summary_data.is_none() && !vs.is_loading_summary {
                                            lanzar_resumen_asincrono(ui.ctx().clone());
                                        }
                                    }
                                }

                                if is_loading {
                                    ui.add_space(8.0);
                                    ui.add(egui::Spinner::new().size(18.0).color(cyan));
                                }
                            },
                        );

                        if let Some(err) = &vs.error_msg {
                            ui.add_space(8.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("Error de conexión: {}", err))
                                        .size(13.0)
                                        .color(egui::Color32::from_rgb(255, 100, 100)),
                                );
                            });
                        }
                    });

                    // --- NOTIFICACIÓN DE COPIADO ---
                    if let Some(msg) = &vs.copied_feedback {
                        ui.add_space(8.0);
                        let mut notify_frame = egui::Frame::new();
                        notify_frame.fill = egui::Color32::from_rgb(18, 44, 30);
                        notify_frame.stroke = egui::Stroke::new(1.0, green);
                        notify_frame.corner_radius = egui::CornerRadius::same(6);
                        notify_frame.inner_margin = egui::Margin::symmetric(12, 6);

                        ui.vertical_centered(|ui| {
                            notify_frame.show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(msg)
                                        .size(12.5)
                                        .strong()
                                        .color(green),
                                );
                            });
                        });
                    }
                });
            });

            ui.add_space(10.0);

            // --- DECISIÓN DE VISTA: RESUMEN DE CRATES.IO VS RESULTADOS DE BÚSQUEDA ---
            if vs.is_search_active {
                if !vs.is_loading && vs.results.is_empty() {
                    ui.add_space(16.0);
                    ui.horizontal_top(|ui| {
                        if left_margin > 0.0 {
                            ui.add_space(left_margin);
                        }
                        ui.vertical(|ui| {
                            ui.set_width(content_w);
                            card_frame.show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("No se encontraron paquetes registrados en crates.io para el término ingresado.")
                                        .size(13.5)
                                        .color(text_color),
                                );
                            });
                        });
                    });
                } else if !vs.results.is_empty() {
                    render_search_results_grid(ui, vs, state_changed, elapsed);
                }
            } else {
                // Modo catálogo principal (Dashboard de Crates.io: 2 Grids de 3 Columnas)
                if vs.summary_data.is_none() && !vs.is_loading_summary {
                    lanzar_resumen_asincrono(ui.ctx().clone());
                }

                if let Some(summary) = vs.summary_data.clone() {
                    render_dashboard_summary(ui, &summary, vs, state_changed, elapsed);
                }
            }

            ui.add_space(20.0);
        });
}

fn render_dashboard_summary(
    ui: &mut egui::Ui,
    summary: &SummaryApiResponse,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    elapsed: f64,
) {
    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let available_w = ui.available_width();
    let card_w = 285.0;
    let card_h = 72.0;
    let col_spacing = 14.0;
    let total_grid_w = 3.0 * card_w + 2.0 * col_spacing;
    let is_wide = available_w >= total_grid_w;
    let left_margin = ((available_w - total_grid_w) / 2.0).max(0.0);

    // --- GRID 1: 3 Columnas (New Crates, Most Downloaded, Just Updated) ---
    if is_wide {
        ui.horizontal_top(|ui| {
            if left_margin > 0.0 {
                ui.add_space(left_margin);
            }

            // Columna 0: New Crates
            ui.vertical(|col| {
                col.set_width(card_w);
                col.label(
                    egui::RichText::new("New Crates")
                        .size(16.0)
                        .strong()
                        .color(title_color),
                );
                col.add_space(6.0);

                for (idx, item) in summary.new_crates.iter().take(10).enumerate() {
                    let delay = idx as f64 * 0.04;
                    let local_time = (elapsed - delay).max(0.0);
                    let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                    let ease_out = 1.0 - (1.0 - progress).powi(4);
                    let alpha = ease_out;
                    let y_offset = (1.0 - ease_out) * 12.0;

                    render_crate_version_card(col, item, vs, state_changed, card_w, card_h, alpha, y_offset);
                    col.add_space(5.0);
                }
            });

            ui.add_space(col_spacing);

            // Columna 1: Most Downloaded
            ui.vertical(|col| {
                col.set_width(card_w);
                col.label(
                    egui::RichText::new("Most Downloaded")
                        .size(16.0)
                        .strong()
                        .color(title_color),
                );
                col.add_space(6.0);

                for (idx, item) in summary.most_downloaded.iter().take(10).enumerate() {
                    let delay = idx as f64 * 0.04;
                    let local_time = (elapsed - delay).max(0.0);
                    let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                    let ease_out = 1.0 - (1.0 - progress).powi(4);
                    let alpha = ease_out;
                    let y_offset = (1.0 - ease_out) * 12.0;

                    render_crate_download_card(col, item, vs, state_changed, card_w, card_h, alpha, y_offset, false);
                    col.add_space(5.0);
                }
            });

            ui.add_space(col_spacing);

            // Columna 2: Just Updated
            ui.vertical(|col| {
                col.set_width(card_w);
                col.label(
                    egui::RichText::new("Just Updated")
                        .size(16.0)
                        .strong()
                        .color(title_color),
                );
                col.add_space(6.0);

                for (idx, item) in summary.just_updated.iter().take(10).enumerate() {
                    let delay = idx as f64 * 0.04;
                    let local_time = (elapsed - delay).max(0.0);
                    let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                    let ease_out = 1.0 - (1.0 - progress).powi(4);
                    let alpha = ease_out;
                    let y_offset = (1.0 - ease_out) * 12.0;

                    render_crate_version_card(col, item, vs, state_changed, card_w, card_h, alpha, y_offset);
                    col.add_space(5.0);
                }
            });
        });
    } else {
        render_single_summary_column_version_crates(ui, "New Crates", &summary.new_crates, vs, state_changed, elapsed, 0.0, card_w, card_h);
        ui.add_space(12.0);
        render_single_summary_column_download_crates(ui, "Most Downloaded", &summary.most_downloaded, vs, state_changed, elapsed, 0.1, card_w, card_h, false);
        ui.add_space(12.0);
        render_single_summary_column_version_crates(ui, "Just Updated", &summary.just_updated, vs, state_changed, elapsed, 0.2, card_w, card_h);
    }

    ui.add_space(16.0);

    // --- GRID 2: 3 Columnas (Most Recent Downloads, Popular Keywords, Popular Categories) ---
    if is_wide {
        ui.horizontal_top(|ui| {
            if left_margin > 0.0 {
                ui.add_space(left_margin);
            }

            // Columna 0: Most Recent Downloads
            ui.vertical(|col| {
                col.set_width(card_w);
                col.label(
                    egui::RichText::new("Most Recent Downloads")
                        .size(16.0)
                        .strong()
                        .color(title_color),
                );
                col.add_space(6.0);

                for (idx, item) in summary.most_recently_downloaded.iter().take(10).enumerate() {
                    let delay = 0.15 + idx as f64 * 0.04;
                    let local_time = (elapsed - delay).max(0.0);
                    let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                    let ease_out = 1.0 - (1.0 - progress).powi(4);
                    let alpha = ease_out;
                    let y_offset = (1.0 - ease_out) * 12.0;

                    render_crate_download_card(col, item, vs, state_changed, card_w, card_h, alpha, y_offset, true);
                    col.add_space(5.0);
                }
            });

            ui.add_space(col_spacing);

            // Columna 1: Popular Keywords
            ui.vertical(|col| {
                col.set_width(card_w);
                col.label(
                    egui::RichText::new("Popular Keywords")
                        .size(16.0)
                        .strong()
                        .color(title_color),
                );
                col.add_space(6.0);

                for (idx, item) in summary.popular_keywords.iter().take(10).enumerate() {
                    let delay = 0.15 + idx as f64 * 0.04;
                    let local_time = (elapsed - delay).max(0.0);
                    let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                    let ease_out = 1.0 - (1.0 - progress).powi(4);
                    let alpha = ease_out;
                    let y_offset = (1.0 - ease_out) * 12.0;

                    render_keyword_compact_card(col, item, vs, state_changed, card_w, card_h, alpha, y_offset);
                    col.add_space(5.0);
                }
            });

            ui.add_space(col_spacing);

            // Columna 2: Popular Categories
            ui.vertical(|col| {
                col.set_width(card_w);
                col.label(
                    egui::RichText::new("Popular Categories")
                        .size(16.0)
                        .strong()
                        .color(title_color),
                );
                col.add_space(6.0);

                for (idx, item) in summary.popular_categories.iter().take(10).enumerate() {
                    let delay = 0.15 + idx as f64 * 0.04;
                    let local_time = (elapsed - delay).max(0.0);
                    let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                    let ease_out = 1.0 - (1.0 - progress).powi(4);
                    let alpha = ease_out;
                    let y_offset = (1.0 - ease_out) * 12.0;

                    render_category_compact_card(col, item, vs, state_changed, card_w, card_h, alpha, y_offset);
                    col.add_space(5.0);
                }
            });
        });
    } else {
        render_single_summary_column_download_crates(ui, "Most Recent Downloads", &summary.most_recently_downloaded, vs, state_changed, elapsed, 0.3, card_w, card_h, true);
        ui.add_space(12.0);
        render_single_summary_column_keywords(ui, "Popular Keywords", &summary.popular_keywords, vs, state_changed, elapsed, 0.4, card_w, card_h);
        ui.add_space(12.0);
        render_single_summary_column_categories(ui, "Popular Categories", &summary.popular_categories, vs, state_changed, elapsed, 0.5, card_w, card_h);
    }
}

fn render_single_summary_column_version_crates(
    ui: &mut egui::Ui,
    title: &str,
    items: &[CrateApiItem],
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    elapsed: f64,
    base_delay: f64,
    card_w: f32,
    card_h: f32,
) {
    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let left_margin = ((ui.available_width() - card_w) / 2.0).max(0.0);
    ui.horizontal_top(|ui| {
        if left_margin > 0.0 {
            ui.add_space(left_margin);
        }
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(title).size(16.0).strong().color(title_color));
            ui.add_space(6.0);
            for (idx, item) in items.iter().take(10).enumerate() {
                let delay = base_delay + idx as f64 * 0.03;
                let local_time = (elapsed - delay).max(0.0);
                let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                let ease_out = 1.0 - (1.0 - progress).powi(4);
                render_crate_version_card(ui, item, vs, state_changed, card_w, card_h, ease_out, (1.0 - ease_out) * 12.0);
                ui.add_space(5.0);
            }
        });
    });
}

fn render_single_summary_column_download_crates(
    ui: &mut egui::Ui,
    title: &str,
    items: &[CrateApiItem],
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    elapsed: f64,
    base_delay: f64,
    card_w: f32,
    card_h: f32,
    use_recent: bool,
) {
    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let left_margin = ((ui.available_width() - card_w) / 2.0).max(0.0);
    ui.horizontal_top(|ui| {
        if left_margin > 0.0 {
            ui.add_space(left_margin);
        }
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(title).size(16.0).strong().color(title_color));
            ui.add_space(6.0);
            for (idx, item) in items.iter().take(10).enumerate() {
                let delay = base_delay + idx as f64 * 0.03;
                let local_time = (elapsed - delay).max(0.0);
                let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                let ease_out = 1.0 - (1.0 - progress).powi(4);
                render_crate_download_card(ui, item, vs, state_changed, card_w, card_h, ease_out, (1.0 - ease_out) * 12.0, use_recent);
                ui.add_space(5.0);
            }
        });
    });
}

fn render_single_summary_column_keywords(
    ui: &mut egui::Ui,
    title: &str,
    items: &[KeywordItem],
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    elapsed: f64,
    base_delay: f64,
    card_w: f32,
    card_h: f32,
) {
    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let left_margin = ((ui.available_width() - card_w) / 2.0).max(0.0);
    ui.horizontal_top(|ui| {
        if left_margin > 0.0 {
            ui.add_space(left_margin);
        }
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(title).size(16.0).strong().color(title_color));
            ui.add_space(6.0);
            for (idx, item) in items.iter().take(10).enumerate() {
                let delay = base_delay + idx as f64 * 0.03;
                let local_time = (elapsed - delay).max(0.0);
                let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                let ease_out = 1.0 - (1.0 - progress).powi(4);
                render_keyword_compact_card(ui, item, vs, state_changed, card_w, card_h, ease_out, (1.0 - ease_out) * 12.0);
                ui.add_space(5.0);
            }
        });
    });
}

fn render_single_summary_column_categories(
    ui: &mut egui::Ui,
    title: &str,
    items: &[CategoryItem],
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    elapsed: f64,
    base_delay: f64,
    card_w: f32,
    card_h: f32,
) {
    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let left_margin = ((ui.available_width() - card_w) / 2.0).max(0.0);
    ui.horizontal_top(|ui| {
        if left_margin > 0.0 {
            ui.add_space(left_margin);
        }
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(title).size(16.0).strong().color(title_color));
            ui.add_space(6.0);
            for (idx, item) in items.iter().take(10).enumerate() {
                let delay = base_delay + idx as f64 * 0.03;
                let local_time = (elapsed - delay).max(0.0);
                let progress = (local_time / 0.35).clamp(0.0, 1.0) as f32;
                let ease_out = 1.0 - (1.0 - progress).powi(4);
                render_category_compact_card(ui, item, vs, state_changed, card_w, card_h, ease_out, (1.0 - ease_out) * 12.0);
                ui.add_space(5.0);
            }
        });
    });
}

fn render_crate_version_card(
    ui: &mut egui::Ui,
    item: &CrateApiItem,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    card_w: f32,
    card_h: f32,
    alpha: f32,
    y_offset: f32,
) {
    if crate::app::ui::crate_item_card(
        ui,
        &item.name,
        item.description.as_deref(),
        crate::app::ui::CrateCardBadge::Version(&item.max_version),
        card_w,
        card_h,
        alpha,
        y_offset,
    ) {
        vs.selected_crate = Some(item.clone());
        vs.full_detail = None;
        vs.detail_tab = 0;
        *state_changed = true;
    }
}

fn render_crate_download_card(
    ui: &mut egui::Ui,
    item: &CrateApiItem,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    card_w: f32,
    card_h: f32,
    alpha: f32,
    y_offset: f32,
    use_recent: bool,
) {
    let dl_count = if use_recent {
        item.recent_downloads.unwrap_or(item.downloads)
    } else {
        item.downloads
    };

    if crate::app::ui::crate_item_card(
        ui,
        &item.name,
        item.description.as_deref(),
        crate::app::ui::CrateCardBadge::Downloads(dl_count),
        card_w,
        card_h,
        alpha,
        y_offset,
    ) {
        vs.selected_crate = Some(item.clone());
        vs.full_detail = None;
        vs.detail_tab = 0;
        *state_changed = true;
    }
}

fn render_keyword_compact_card(
    ui: &mut egui::Ui,
    item: &KeywordItem,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    card_w: f32,
    card_h: f32,
    alpha: f32,
    y_offset: f32,
) {
    let subtitle = format!("{} paquetes", formatear_numero(item.crates_cnt));
    if crate::app::ui::taxonomy_item_card(
        ui,
        &item.keyword,
        &subtitle,
        card_w,
        card_h,
        alpha,
        y_offset,
    ) {
        let now = ui.input(|i| i.time);
        vs.search_query = item.keyword.clone();
        vs.is_search_active = true;
        vs.results.clear();
        vs.anim_start_time = now;
        *state_changed = true;
        lanzar_busqueda_asincrona(item.keyword.clone(), ui.ctx().clone());
    }
}

fn render_category_compact_card(
    ui: &mut egui::Ui,
    item: &CategoryItem,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    card_w: f32,
    card_h: f32,
    alpha: f32,
    y_offset: f32,
) {
    let desc_raw = item.description.as_deref().unwrap_or("");
    let subtitle = if !desc_raw.trim().is_empty() {
        desc_raw.to_string()
    } else {
        format!("{} paquetes registrados", formatear_numero(item.crates_cnt))
    };

    if crate::app::ui::taxonomy_item_card(
        ui,
        &item.category,
        &subtitle,
        card_w,
        card_h,
        alpha,
        y_offset,
    ) {
        let now = ui.input(|i| i.time);
        vs.search_query = item.category.clone();
        vs.is_search_active = true;
        vs.results.clear();
        vs.anim_start_time = now;
        *state_changed = true;
        lanzar_busqueda_asincrona(item.category.clone(), ui.ctx().clone());
    }
}


fn render_search_results_grid(
    ui: &mut egui::Ui,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    elapsed: f64,
) {
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text_color = egui::Color32::from_rgb(200, 210, 225);

    let available_w = ui.available_width();
    let col_spacing: f32 = 14.0;
    let card_w: f32 = 285.0;
    let total_grid_w: f32 = 3.0 * card_w + 2.0 * col_spacing;
    let content_w = total_grid_w.min(available_w);
    let left_margin = ((available_w - content_w) / 2.0).max(0.0);
    let is_two_col = content_w >= 600.0;

    if is_two_col {
        let col_w = ((content_w - col_spacing) / 2.0).max(280.0);

        for (idx, pair) in vs.results.chunks(2).enumerate() {
            let card_delay = idx as f64 * 0.08;
            let local_time = (elapsed - card_delay).max(0.0);
            let progress = (local_time / 0.40).clamp(0.0, 1.0) as f32;
            let ease_out = 1.0 - (1.0 - progress).powi(4);

            let alpha = ease_out;
            let alpha_u8 = (alpha.clamp(0.0, 1.0) * 255.0) as u8;
            let y_offset = (1.0 - ease_out) * 16.0;

            ui.horizontal(|ui| {
                if left_margin > 0.0 {
                    ui.add_space(left_margin);
                }
                for item in pair {
                    let item_clone = item.clone();

                    let (rect, _response) = ui.allocate_exact_size(
                        egui::vec2(col_w, 134.0),
                        egui::Sense::hover(),
                    );

                    let card_rect = egui::Rect::from_min_size(
                        egui::pos2(rect.left(), rect.top() + y_offset),
                        rect.size(),
                    );

                    let bg_card = egui::Color32::from_rgba_unmultiplied(14, 18, 26, alpha_u8);
                    let stroke_card = egui::Color32::from_rgba_unmultiplied(45, 60, 90, alpha_u8);

                    // 1. Dibujar el fondo y borde de la tarjeta animada
                    ui.painter().rect(
                        card_rect,
                        egui::CornerRadius::same(8),
                        bg_card,
                        egui::Stroke::new(1.0, stroke_card),
                        egui::StrokeKind::Inside,
                    );

                    // 2. Zona Superior (Header y Descripción dentro de la tarjeta)
                    let top_rect = egui::Rect::from_min_max(
                        egui::pos2(card_rect.left() + 12.0, card_rect.top() + 10.0),
                        egui::pos2(card_rect.right() - 12.0, card_rect.bottom() - 34.0),
                    );

                    let mut top_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(top_rect)
                            .layout(egui::Layout::top_down(egui::Align::Min)),
                    );

                    top_ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(&item_clone.name)
                                .size(15.0)
                                .strong()
                                .color(cyan),
                        );

                        ui.add_space(4.0);

                        crate::app::ui::tag_chip(ui, &format!("v{}", item_clone.max_version));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if boton_custom_cyan(ui, "Ver Detalle", 82.0).clicked() {
                                vs.selected_crate = Some(item_clone.clone());
                                vs.full_detail = None;
                                vs.detail_tab = 0;
                                *state_changed = true;
                            }

                            let cmd = format!("cargo add {}", item_clone.name);
                            if boton_custom_orange(ui, "Copiar", 55.0).clicked() {
                                ui.ctx().copy_text(cmd.clone());
                                vs.copied_feedback = Some(format!("Comando `{}` copiado", cmd));
                                *state_changed = true;
                            }
                        });
                    });

                    top_ui.add_space(5.0);

                    let desc_raw = item_clone.description.as_deref().unwrap_or("Sin descripción proporcionada.");
                    let desc_clean = desc_raw.replace('\n', " ").replace('\r', "");
                    let desc_trunc = if desc_clean.chars().count() > 110 {
                        let s: String = desc_clean.chars().take(107).collect();
                        format!("{}...", s)
                    } else {
                        desc_clean
                    };

                    top_ui.label(
                        egui::RichText::new(desc_trunc)
                            .size(12.0)
                            .color(text_color)
                            .line_height(Some(16.0)),
                    );

                    // 3. Línea Separadora dentro de la tarjeta
                    let sep_y = card_rect.bottom() - 30.0;
                    ui.painter().line_segment(
                        [
                            egui::pos2(card_rect.left() + 10.0, sep_y),
                            egui::pos2(card_rect.right() - 10.0, sep_y),
                        ],
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 42, 62)),
                    );

                    // 4. Zona Inferior (Pie de tarjeta: Descargas y Repositorio estrictamente dentro)
                    let footer_rect = egui::Rect::from_min_max(
                        egui::pos2(card_rect.left() + 12.0, sep_y + 3.0),
                        egui::pos2(card_rect.right() - 12.0, card_rect.bottom() - 5.0),
                    );

                    let mut footer_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(footer_rect)
                            .layout(egui::Layout::left_to_right(egui::Align::Center)),
                    );

                    let img_dl = egui::Image::new(egui::include_image!("../../../../assets/icons/download-svgrepo-com.svg"))
                        .fit_to_exact_size(egui::vec2(13.0, 13.0))
                        .tint(egui::Color32::from_rgb(130, 150, 175));
                    let (_id, dl_rect) = footer_ui.allocate_space(egui::vec2(13.0, 13.0));
                    img_dl.paint_at(&footer_ui, dl_rect);
                    footer_ui.add_space(2.0);

                    footer_ui.label(
                        egui::RichText::new(format!("{} descargas", formatear_numero_compacto(item_clone.downloads)))
                            .size(11.5)
                            .color(egui::Color32::from_rgb(130, 150, 175)),
                    );

                    if let Some(repo) = &item_clone.repository {
                        let repo_url = repo.clone();
                        footer_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.link(egui::RichText::new("Repositorio").size(11.5).color(cyan)).clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab(repo_url));
                            }
                        });
                    }

                    ui.add_space(col_spacing);
                }
            });

            ui.add_space(10.0);
        }
    } else {
        for (idx, item) in vs.results.iter().enumerate() {
            let card_delay = idx as f64 * 0.05;
            let local_time = (elapsed - card_delay).max(0.0);
            let progress = (local_time / 0.40).clamp(0.0, 1.0) as f32;
            let ease_out = 1.0 - (1.0 - progress).powi(4);

            let alpha = ease_out;
            let alpha_u8 = (alpha.clamp(0.0, 1.0) * 255.0) as u8;
            let y_offset = (1.0 - ease_out) * 16.0;
            let item_clone = item.clone();

            ui.horizontal(|ui| {
                if left_margin > 0.0 {
                    ui.add_space(left_margin);
                }

                let (rect, _response) = ui.allocate_exact_size(
                    egui::vec2(content_w, 134.0),
                    egui::Sense::hover(),
                );

            let card_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left(), rect.top() + y_offset),
                rect.size(),
            );

            let bg_card = egui::Color32::from_rgba_unmultiplied(14, 18, 26, alpha_u8);
            let stroke_card = egui::Color32::from_rgba_unmultiplied(45, 60, 90, alpha_u8);

            ui.painter().rect(
                card_rect,
                egui::CornerRadius::same(8),
                bg_card,
                egui::Stroke::new(1.0, stroke_card),
                egui::StrokeKind::Inside,
            );

            let top_rect = egui::Rect::from_min_max(
                egui::pos2(card_rect.left() + 12.0, card_rect.top() + 10.0),
                egui::pos2(card_rect.right() - 12.0, card_rect.bottom() - 34.0),
            );

            let mut top_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(top_rect)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );

            top_ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&item_clone.name)
                        .size(15.0)
                        .strong()
                        .color(cyan),
                );

                ui.add_space(4.0);

                crate::app::ui::tag_chip(ui, &format!("v{}", item_clone.max_version));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if boton_custom_cyan(ui, "Ver Detalle", 82.0).clicked() {
                        vs.selected_crate = Some(item_clone.clone());
                        vs.full_detail = None;
                        vs.detail_tab = 0;
                        *state_changed = true;
                    }

                    let cmd = format!("cargo add {}", item_clone.name);
                    if boton_custom_orange(ui, "Copiar", 55.0).clicked() {
                        ui.ctx().copy_text(cmd.clone());
                        vs.copied_feedback = Some(format!("Comando `{}` copiado", cmd));
                        *state_changed = true;
                    }
                });
            });

            top_ui.add_space(5.0);

            let desc_raw = item_clone.description.as_deref().unwrap_or("Sin descripción proporcionada.");
            let desc_clean = desc_raw.replace('\n', " ").replace('\r', "");
            let desc_trunc = if desc_clean.chars().count() > 110 {
                let s: String = desc_clean.chars().take(107).collect();
                format!("{}...", s)
            } else {
                desc_clean
            };

            top_ui.label(
                egui::RichText::new(desc_trunc)
                    .size(12.0)
                    .color(text_color)
                    .line_height(Some(16.0)),
            );

            let sep_y = card_rect.bottom() - 30.0;
            ui.painter().line_segment(
                [
                    egui::pos2(card_rect.left() + 10.0, sep_y),
                    egui::pos2(card_rect.right() - 10.0, sep_y),
                ],
                egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 42, 62)),
            );

            let footer_rect = egui::Rect::from_min_max(
                egui::pos2(card_rect.left() + 12.0, sep_y + 3.0),
                egui::pos2(card_rect.right() - 12.0, card_rect.bottom() - 5.0),
            );

            let mut footer_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(footer_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );

            let img_dl = egui::Image::new(egui::include_image!("../../../../assets/icons/download-svgrepo-com.svg"))
                .fit_to_exact_size(egui::vec2(13.0, 13.0))
                .tint(egui::Color32::from_rgb(130, 150, 175));
            let (_id, dl_rect) = footer_ui.allocate_space(egui::vec2(13.0, 13.0));
            img_dl.paint_at(&footer_ui, dl_rect);
            footer_ui.add_space(2.0);

            footer_ui.label(
                egui::RichText::new(format!("{} descargas", formatear_numero_compacto(item_clone.downloads)))
                    .size(11.5)
                    .color(egui::Color32::from_rgb(130, 150, 175)),
            );

            if let Some(repo) = &item_clone.repository {
                let repo_url = repo.clone();
                footer_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.link(egui::RichText::new("Repositorio").size(11.5).color(cyan)).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(repo_url));
                    }
                });
            }
            });

            ui.add_space(10.0);
        }
    }
}

// --- VISTA DETALLADA DEL CRATE SELECCIONADO (100% DATOS EN VIVO DE LA API) ---

fn mostrar_detalle_crate(
    ui: &mut egui::Ui,
    item: &CrateApiItem,
    vs: &mut CratesIoViewState,
    _state: &mut AppState,
    state_changed: &mut bool,
) {
    lanzar_detalle_asincrono(item.id.clone(), ui.ctx().clone());

    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);
    let text_color = egui::Color32::from_rgb(200, 210, 225);

    let mut card_frame = egui::Frame::new();
    card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    card_frame.inner_margin = egui::Margin::same(12);
    card_frame.corner_radius = egui::CornerRadius::same(8);
    card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(6.0);

            // --- BOTÓN DE REGRESO Y BREADCRUMB ---
            ui.horizontal(|ui| {
                if boton_custom_cyan(ui, "Volver a la lista de crates", 175.0).clicked() {
                    vs.selected_crate = None;
                    vs.full_detail = None;
                    vs.anim_needs_restart = true;
                    *state_changed = true;
                }

                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format!("crates.io / {}", item.name))
                        .size(13.0)
                        .color(egui::Color32::from_rgb(120, 140, 165)),
                );
            });

            if vs.selected_crate.is_none() {
                return;
            }

            ui.add_space(8.0);

            let latest_ver = vs.full_detail.as_ref().and_then(|d| d.versions.first());
            let lic_str = latest_ver
                .and_then(|v| v.license.as_deref())
                .unwrap_or("Consultando licencia...");
            let edition_str = latest_ver
                .and_then(|v| v.edition.as_deref())
                .map(|e| format!("Rust Edition {}", e))
                .unwrap_or_else(|| "Ecosistema Rust".to_string());

            // --- HERO HEADER CARD ---
            card_frame.show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.heading(
                        egui::RichText::new(&item.name)
                            .size(24.0)
                            .strong()
                            .color(cyan),
                    );

                    ui.add_space(8.0);

                    // Badge de Versión
                    crate::app::ui::tag_chip(ui, &format!("v{}", item.max_version));

                    ui.add_space(6.0);

                    // Badge Oficial
                    crate::app::ui::tag_chip(ui, "crates.io oficial");

                    ui.add_space(6.0);

                    // Badge Licencia (Real de la API)
                    crate::app::ui::tag_chip(ui, &format!("Licencia: {}", lic_str));
                });

                ui.add_space(8.0);

                // Descripción real del autor
                let desc_texto = item
                    .description
                    .as_deref()
                    .unwrap_or("Paquete publicado en el registro oficial de Rust crates.io.");
                ui.label(
                    egui::RichText::new(desc_texto)
                        .size(14.0)
                        .color(text_color)
                        .line_height(Some(20.0)),
                );

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // Quick stats strip
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{} descargas acumuladas", formatear_numero(item.downloads)))
                            .size(12.0)
                            .strong()
                            .color(egui::Color32::from_rgb(150, 175, 205)),
                    );

                    ui.add_space(14.0);

                    ui.label(
                        egui::RichText::new(format!("Versión: v{}", item.max_version))
                            .size(12.0)
                            .color(egui::Color32::from_rgb(150, 175, 205)),
                    );

                    ui.add_space(14.0);

                    ui.label(
                        egui::RichText::new(&edition_str)
                            .size(12.0)
                            .color(egui::Color32::from_rgb(150, 175, 205)),
                    );
                });
            });

            ui.add_space(12.0);

            // --- NOTIFICACIÓN DE COPIADO ---
            if let Some(msg) = &vs.copied_feedback {
                let mut notify_frame = egui::Frame::new();
                notify_frame.fill = egui::Color32::from_rgb(18, 44, 30);
                notify_frame.stroke = egui::Stroke::new(1.0, green);
                notify_frame.corner_radius = egui::CornerRadius::same(6);
                notify_frame.inner_margin = egui::Margin::symmetric(12, 6);

                notify_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(msg)
                            .size(12.5)
                            .strong()
                            .color(green),
                    );
                });
                ui.add_space(10.0);
            }

            // --- CONTENIDO PRINCIPAL DE LA PESTAÑA SELECCIONADA (100% DE ANCHO) ---
            render_detalle_contenido_principal(ui, item, vs, state_changed, &card_frame);

            ui.add_space(14.0);

            // --- SECCIÓN DE UTILIDADES (INSTALACIÓN, ENLACES Y METADATOS UBICADOS ABAJO) ---
            render_detalle_utilidades_inferiores(ui, item, vs, state_changed, &card_frame);

            ui.add_space(20.0);
        });
}

fn render_detalle_contenido_principal(
    ui: &mut egui::Ui,
    item: &CrateApiItem,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    card_frame: &egui::Frame,
) {
    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);
    let text_color = egui::Color32::from_rgb(200, 210, 225);

    // --- UNDERLINE TABS NATIVOS (SIN ICONOS) ---
    ui.horizontal_wrapped(|ui| {
        let tabs = [
            (0, "Resumen"),
            (1, "Cargo.toml"),
            (2, "Características (Features)"),
            (3, "Versiones"),
        ];

        for (idx, label) in tabs {
            let es_activo = vs.detail_tab == idx;
            if underline_tab(ui, label, es_activo, cyan).clicked() {
                vs.detail_tab = idx;
                *state_changed = true;
            }
        }
    });

    ui.add_space(10.0);

    // --- TAB 0: RESUMEN (DATOS REALES DE CRATES.IO) ---
    if vs.detail_tab == 0 {
        card_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!("Acerca de {}", item.name))
                    .size(15.5)
                    .strong()
                    .color(title_color),
            );
            ui.add_space(6.0);

            let desc = item
                .description
                .as_deref()
                .unwrap_or("Sin descripción proporcionada en el registro de crates.io.");
            ui.label(
                egui::RichText::new(desc)
                    .size(13.5)
                    .color(text_color)
                    .line_height(Some(20.0)),
            );

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Ficha Técnica del Paquete")
                    .size(14.0)
                    .strong()
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(6.0);

            let latest_ver = vs.full_detail.as_ref().and_then(|d| d.versions.first());

            let edition_str = latest_ver
                .and_then(|v| v.edition.as_deref())
                .map(|e| format!("Rust Edition {}", e))
                .unwrap_or_else(|| "Rust Edition 2021".to_string());

            let msrv_str = latest_ver
                .and_then(|v| v.rust_version.as_deref())
                .map(|rv| format!("Rust {} o superior (MSRV)", rv))
                .unwrap_or_else(|| "Compatible con versión estándar".to_string());

            let size_str = latest_ver
                .and_then(|v| v.crate_size)
                .map(formatear_tamano_bytes)
                .unwrap_or_else(|| "No informado".to_string());

            let lic_str = latest_ver
                .and_then(|v| v.license.as_deref())
                .unwrap_or("No especificada");

            let pub_date_str = latest_ver
                .and_then(|v| v.created_at.as_deref())
                .map(|d| if d.len() >= 10 { &d[..10] } else { d })
                .unwrap_or("-");

            let tech_specs = [
                ("Edición del Lenguaje", edition_str.as_str()),
                ("Versión Mínima Requerida", msrv_str.as_str()),
                ("Tamaño del Paquete", size_str.as_str()),
                ("Licencia del Código", lic_str),
                ("Fecha de Publicación", pub_date_str),
            ];

            for (titulo, valor) in tech_specs {
                let mut spec_box = egui::Frame::new();
                spec_box.fill = egui::Color32::from_rgb(10, 14, 22);
                spec_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
                spec_box.corner_radius = egui::CornerRadius::same(5);
                spec_box.inner_margin = egui::Margin::symmetric(10, 8);

                spec_box.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(titulo).size(12.5).color(egui::Color32::from_rgb(140, 160, 185)));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(valor).size(12.5).strong().color(cyan));
                        });
                    });
                });
                ui.add_space(4.0);
            }

            // Categorías oficiales de Crates.io
            if let Some(detail) = &vs.full_detail {
                if !detail.categories.is_empty() {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.label(
                        egui::RichText::new("Categorías Oficiales en Crates.io")
                            .size(14.0)
                            .strong()
                            .color(title_color),
                    );
                    ui.add_space(6.0);

                    for cat in &detail.categories {
                        let mut cat_box = egui::Frame::new();
                        cat_box.fill = egui::Color32::from_rgb(10, 14, 22);
                        cat_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
                        cat_box.corner_radius = egui::CornerRadius::same(5);
                        cat_box.inner_margin = egui::Margin::same(10);

                        cat_box.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&cat.category).size(13.0).strong().color(green));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(egui::RichText::new(format!("{} crates", formatear_numero(cat.crates_cnt))).size(11.5).color(egui::Color32::from_rgb(130, 150, 175)));
                                });
                            });

                            if let Some(cat_desc) = &cat.description {
                                if !cat_desc.is_empty() {
                                    ui.add_space(3.0);
                                    ui.label(egui::RichText::new(cat_desc).size(12.0).color(text_color).line_height(Some(16.0)));
                                }
                            }
                        });
                        ui.add_space(4.0);
                    }
                }
            }
        });
    }

    // --- TAB 1: CARGO.TOML (REAL) ---
    if vs.detail_tab == 1 {
        card_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Instalación y Configuración en Cargo.toml")
                    .size(15.5)
                    .strong()
                    .color(title_color),
            );
            ui.add_space(6.0);

            ui.label(
                egui::RichText::new("Añade la siguiente dependencia dentro de la sección [dependencies] de tu archivo Cargo.toml:")
                    .size(13.0)
                    .color(text_color),
            );
            ui.add_space(8.0);

            // 1. Dependencia estándar
            ui.label(egui::RichText::new("1. Instalación estándar:").strong().color(egui::Color32::WHITE).size(13.0));
            ui.add_space(4.0);
            let snippet_base = format!("[dependencies]\n{} = \"{}\"", item.name, item.max_version);

            let mut code_box1 = egui::Frame::new();
            code_box1.fill = egui::Color32::from_rgb(10, 14, 22);
            code_box1.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
            code_box1.corner_radius = egui::CornerRadius::same(6);
            code_box1.inner_margin = egui::Margin::same(10);

            code_box1.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&snippet_base)
                            .font(egui::FontId::monospace(13.0))
                            .color(green),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if boton_custom_cyan(ui, "Copiar", 65.0).clicked() {
                            ui.ctx().copy_text(snippet_base.clone());
                            vs.copied_feedback = Some("Bloque estándar copiado".to_string());
                            *state_changed = true;
                        }
                    });
                });
            });

            ui.add_space(10.0);

            // 2. Línea de comando CLI
            ui.label(egui::RichText::new("2. Línea de comando con Cargo CLI:").strong().color(egui::Color32::WHITE).size(13.0));
            ui.add_space(4.0);
            let cmd_add = format!("cargo add {}", item.name);

            let mut code_box2 = egui::Frame::new();
            code_box2.fill = egui::Color32::from_rgb(10, 14, 22);
            code_box2.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
            code_box2.corner_radius = egui::CornerRadius::same(6);
            code_box2.inner_margin = egui::Margin::same(10);

            code_box2.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&cmd_add)
                            .font(egui::FontId::monospace(13.0))
                            .color(cyan),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if boton_custom_orange(ui, "Copiar", 65.0).clicked() {
                            ui.ctx().copy_text(cmd_add.clone());
                            vs.copied_feedback = Some(format!("Comando `{}` copiado", cmd_add));
                            *state_changed = true;
                        }
                    });
                });
            });
        });
    }

    // --- TAB 2: CARACTERÍSTICAS / FEATURES (100% REAL DE LA API) ---
    if vs.detail_tab == 2 {
        let latest_ver = vs.full_detail.as_ref().and_then(|d| d.versions.first());
        let features_map = latest_ver.map(|v| &v.features);

        card_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!("Características (Features) de {}", item.name))
                    .size(15.5)
                    .strong()
                    .color(title_color),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Lista oficial de banderas de compilación condicional extraídas directamente del manifest Cargo.toml registrado en crates.io:")
                    .size(12.5)
                    .color(text_color),
            );
            ui.add_space(10.0);

            if let Some(features) = features_map {
                if features.is_empty() {
                    ui.label(
                        egui::RichText::new("Este paquete no declara features opcionales adicionales en su manifest Cargo.toml.")
                            .size(13.0)
                            .color(egui::Color32::from_rgb(140, 160, 185)),
                    );
                } else {
                    for (feat_name, sub_deps) in features {
                        let mut feat_box = egui::Frame::new();
                        feat_box.fill = egui::Color32::from_rgb(10, 14, 22);
                        feat_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
                        feat_box.corner_radius = egui::CornerRadius::same(6);
                        feat_box.inner_margin = egui::Margin::symmetric(10, 8);

                        feat_box.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let mut tag_f = egui::Frame::new();
                                tag_f.fill = egui::Color32::from_rgb(18, 40, 28);
                                tag_f.corner_radius = egui::CornerRadius::same(4);
                                tag_f.inner_margin = egui::Margin::symmetric(7, 3);
                                tag_f.show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(feat_name)
                                            .font(egui::FontId::monospace(12.0))
                                            .strong()
                                            .color(green),
                                    );
                                });

                                ui.add_space(6.0);

                                if sub_deps.is_empty() {
                                    ui.label(
                                        egui::RichText::new("Bandera condicional independiente")
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(130, 150, 175)),
                                    );
                                } else {
                                    let deps_str = format!("Habilita: {}", sub_deps.join(", "));
                                    ui.label(
                                        egui::RichText::new(deps_str)
                                            .size(12.0)
                                            .color(text_color),
                                    );
                                }
                            });
                        });
                        ui.add_space(4.0);
                    }
                }
            } else if vs.is_loading_detail {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new().size(28.0).color(cyan));
                });
                ui.add_space(10.0);
            } else {
                ui.label(
                    egui::RichText::new("No se pudieron obtener las características de la API.")
                        .size(13.0)
                        .color(egui::Color32::from_rgb(180, 120, 120)),
                );
            }
        });
    }

    // --- TAB 3: HISTORIAL DE VERSIONES (100% REAL DE LA API) ---
    if vs.detail_tab == 3 {
        let versions_list = vs.full_detail.as_ref().map(|d| &d.versions[..]).unwrap_or(&[]);

        card_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!("Historial de Versiones de {}", item.name))
                    .size(15.5)
                    .strong()
                    .color(title_color),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Registro oficial de todas las versiones publicadas en crates.io:")
                    .size(12.5)
                    .color(text_color),
            );
            ui.add_space(10.0);

            if versions_list.is_empty() {
                if vs.is_loading_detail {
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Spinner::new().size(28.0).color(cyan));
                    });
                    ui.add_space(10.0);
                } else {
                    ui.label(egui::RichText::new("No hay versiones registradas.").size(13.0).color(egui::Color32::from_rgb(140, 160, 185)));
                }
            } else {
                for v in versions_list {
                    let mut v_box = egui::Frame::new();
                    v_box.fill = egui::Color32::from_rgb(10, 14, 22);
                    v_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
                    v_box.corner_radius = egui::CornerRadius::same(5);
                    v_box.inner_margin = egui::Margin::symmetric(10, 8);

                    v_box.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Versión
                            ui.label(
                                egui::RichText::new(format!("v{}", v.num))
                                    .font(egui::FontId::monospace(13.0))
                                    .strong()
                                    .color(green),
                            );

                            if v.yanked == Some(true) {
                                ui.add_space(4.0);
                                let mut yank_tag = egui::Frame::new();
                                yank_tag.fill = egui::Color32::from_rgb(50, 20, 20);
                                yank_tag.corner_radius = egui::CornerRadius::same(3);
                                yank_tag.inner_margin = egui::Margin::symmetric(5, 2);
                                yank_tag.show(ui, |ui| {
                                    ui.label(egui::RichText::new("YANKED").size(10.5).strong().color(egui::Color32::from_rgb(255, 100, 100)));
                                });
                            }

                            if let Some(lic) = &v.license {
                                ui.add_space(8.0);
                                ui.label(egui::RichText::new(lic).size(11.5).color(egui::Color32::from_rgb(130, 150, 175)));
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{} descargas", formatear_numero(v.downloads)))
                                        .size(12.0)
                                        .color(egui::Color32::WHITE),
                                );

                                if let Some(date) = &v.created_at {
                                    let date_short = if date.len() >= 10 { &date[..10] } else { date };
                                    ui.label(
                                        egui::RichText::new(date_short)
                                            .size(11.5)
                                            .color(egui::Color32::from_rgb(120, 140, 165)),
                                    );
                                    ui.add_space(10.0);
                                }
                            });
                        });
                    });
                    ui.add_space(4.0);
                }
            }
        });
    }
}

fn render_detalle_utilidades_inferiores(
    ui: &mut egui::Ui,
    item: &CrateApiItem,
    vs: &mut CratesIoViewState,
    state_changed: &mut bool,
    card_frame: &egui::Frame,
) {
    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);

    let total_w = ui.available_width();
    let es_pantalla_amplia = total_w >= 650.0;

    let latest_ver = vs.full_detail.as_ref().and_then(|d| d.versions.first());
    let lic_str = latest_ver.and_then(|v| v.license.as_deref()).unwrap_or("No especificada");
    let keywords_list = vs.full_detail.as_ref().map(|d| &d.keywords[..]).unwrap_or(&[]);

    // --- CARD 1: INSTALACIÓN RÁPIDA ---
    card_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new("Instalación")
                .size(14.5)
                .strong()
                .color(title_color),
        );
        ui.add_space(8.0);

        let cmd = format!("cargo add {}", item.name);
        let toml_line = format!("{} = \"{}\"", item.name, item.max_version);

        if es_pantalla_amplia {
            let col_w = (ui.available_width() - 16.0) / 2.0;

            ui.horizontal(|ui| {
                // Columna 1: CLI
                ui.allocate_ui_with_layout(
                    egui::vec2(col_w, 64.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.label(
                            egui::RichText::new("Línea de comando (CLI):")
                                .size(12.0)
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.add_space(3.0);

                        let mut box_cli = egui::Frame::new();
                        box_cli.fill = egui::Color32::from_rgb(10, 14, 22);
                        box_cli.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
                        box_cli.corner_radius = egui::CornerRadius::same(5);
                        box_cli.inner_margin = egui::Margin::symmetric(8, 6);

                        box_cli.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&cmd)
                                        .font(egui::FontId::monospace(12.0))
                                        .color(cyan),
                                );

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if boton_custom_orange(ui, "Copiar", 55.0).clicked() {
                                        ui.ctx().copy_text(cmd.clone());
                                        vs.copied_feedback = Some(format!("Comando `{}` copiado", cmd));
                                        *state_changed = true;
                                    }
                                });
                            });
                        });
                    },
                );

                ui.add_space(16.0);

                // Columna 2: Cargo.toml
                ui.allocate_ui_with_layout(
                    egui::vec2(col_w, 64.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.label(
                            egui::RichText::new("En Cargo.toml:")
                                .size(12.0)
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.add_space(3.0);

                        let mut box_toml = egui::Frame::new();
                        box_toml.fill = egui::Color32::from_rgb(10, 14, 22);
                        box_toml.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
                        box_toml.corner_radius = egui::CornerRadius::same(5);
                        box_toml.inner_margin = egui::Margin::symmetric(8, 6);

                        box_toml.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&toml_line)
                                        .font(egui::FontId::monospace(12.0))
                                        .color(green),
                                );

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if boton_custom_cyan(ui, "Copiar", 55.0).clicked() {
                                        ui.ctx().copy_text(toml_line.clone());
                                        vs.copied_feedback = Some(format!("Línea `{}` copiada", toml_line));
                                        *state_changed = true;
                                    }
                                });
                            });
                        });
                    },
                );
            });
        } else {
            // Modo pantalla angosta: apilado
            ui.label(
                egui::RichText::new("Línea de comando (CLI):")
                    .size(12.0)
                    .strong()
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(3.0);

            let mut box_cli = egui::Frame::new();
            box_cli.fill = egui::Color32::from_rgb(10, 14, 22);
            box_cli.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
            box_cli.corner_radius = egui::CornerRadius::same(5);
            box_cli.inner_margin = egui::Margin::symmetric(8, 6);

            box_cli.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&cmd)
                            .font(egui::FontId::monospace(12.0))
                            .color(cyan),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if boton_custom_orange(ui, "Copiar", 55.0).clicked() {
                            ui.ctx().copy_text(cmd.clone());
                            vs.copied_feedback = Some(format!("Comando `{}` copiado", cmd));
                            *state_changed = true;
                        }
                    });
                });
            });

            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("En Cargo.toml:")
                    .size(12.0)
                    .strong()
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(3.0);

            let mut box_toml = egui::Frame::new();
            box_toml.fill = egui::Color32::from_rgb(10, 14, 22);
            box_toml.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 48, 70));
            box_toml.corner_radius = egui::CornerRadius::same(5);
            box_toml.inner_margin = egui::Margin::symmetric(8, 6);

            box_toml.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&toml_line)
                            .font(egui::FontId::monospace(12.0))
                            .color(green),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if boton_custom_cyan(ui, "Copiar", 55.0).clicked() {
                            ui.ctx().copy_text(toml_line.clone());
                            vs.copied_feedback = Some(format!("Línea `{}` copiada", toml_line));
                            *state_changed = true;
                        }
                    });
                });
            });
        }
    });

    ui.add_space(10.0);

    // --- CARD 2 & 3: ENLACES OFICIALES Y METADATOS ---
    if es_pantalla_amplia {
        let col_w = (ui.available_width() - 16.0) / 2.0;

        ui.horizontal(|ui| {
            // Columna Izquierda: Enlaces Oficiales
            ui.allocate_ui_with_layout(
                egui::vec2(col_w, 180.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    card_frame.show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Enlaces Oficiales")
                                .size(14.5)
                                .strong()
                                .color(title_color),
                        );
                        ui.add_space(8.0);

                        let btn_w = ui.available_width();

                        let docs_url = item
                            .documentation
                            .clone()
                            .unwrap_or_else(|| format!("https://docs.rs/{}", item.id));
                        if boton_custom_cyan(ui, "Abrir Docs.rs", btn_w).clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(docs_url));
                        }

                        ui.add_space(6.0);

                        if let Some(repo) = &item.repository {
                            if boton_custom_cyan(ui, "Repositorio de Código", btn_w).clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab(repo.clone()));
                            }
                            ui.add_space(6.0);
                        }

                        let crates_url = format!("https://crates.io/crates/{}", item.id);
                        if boton_custom_cyan(ui, "Ver en Crates.io", btn_w).clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(crates_url));
                        }
                    });
                },
            );

            ui.add_space(16.0);

            // Columna Derecha: Metadatos del Paquete
            ui.allocate_ui_with_layout(
                egui::vec2(col_w, 180.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    card_frame.show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Metadatos del Paquete")
                                .size(14.5)
                                .strong()
                                .color(title_color),
                        );
                        ui.add_space(8.0);

                        // Descargas
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Descargas totales:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new(formatear_numero(item.downloads)).size(12.0).strong().color(egui::Color32::WHITE));
                            });
                        });
                        ui.add_space(4.0);

                        // Versión
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Última versión:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new(format!("v{}", item.max_version)).size(12.0).strong().color(green));
                            });
                        });
                        ui.add_space(4.0);

                        // Licencia
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Licencia:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new(lic_str).size(12.0).color(egui::Color32::WHITE));
                            });
                        });
                        ui.add_space(4.0);

                        // Registro
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Registro:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("crates.io").size(12.0).color(cyan));
                            });
                        });

                        if !keywords_list.is_empty() {
                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(6.0);

                            ui.label(egui::RichText::new("Palabras Clave (Keywords):").size(12.0).strong().color(egui::Color32::WHITE));
                            ui.add_space(4.0);

                            ui.horizontal_wrapped(|ui| {
                                for kw in keywords_list {
                                    let mut tag_kw = egui::Frame::new();
                                    tag_kw.fill = egui::Color32::from_rgb(18, 24, 36);
                                    tag_kw.corner_radius = egui::CornerRadius::same(3);
                                    tag_kw.inner_margin = egui::Margin::symmetric(5, 2);
                                    tag_kw.show(ui, |ui| {
                                        ui.label(egui::RichText::new(format!("#{}", kw.keyword)).size(11.0).color(cyan));
                                    });
                                    ui.add_space(3.0);
                                }
                            });
                        }
                    });
                },
            );
        });
    } else {
        // Enlaces Oficiales (Apilado vertical)
        card_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Enlaces Oficiales")
                    .size(14.5)
                    .strong()
                    .color(title_color),
            );
            ui.add_space(8.0);

            let btn_w = ui.available_width();

            let docs_url = item
                .documentation
                .clone()
                .unwrap_or_else(|| format!("https://docs.rs/{}", item.id));
            if boton_custom_cyan(ui, "Abrir Docs.rs", btn_w).clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(docs_url));
            }

            ui.add_space(6.0);

            if let Some(repo) = &item.repository {
                if boton_custom_cyan(ui, "Repositorio de Código", btn_w).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(repo.clone()));
                }
                ui.add_space(6.0);
            }

            let crates_url = format!("https://crates.io/crates/{}", item.id);
            if boton_custom_cyan(ui, "Ver en Crates.io", btn_w).clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(crates_url));
            }
        });

        ui.add_space(10.0);

        // Metadatos (Apilado vertical)
        card_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Metadatos del Paquete")
                    .size(14.5)
                    .strong()
                    .color(title_color),
            );
            ui.add_space(8.0);

            // Descargas
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Descargas totales:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(formatear_numero(item.downloads)).size(12.0).strong().color(egui::Color32::WHITE));
                });
            });
            ui.add_space(4.0);

            // Versión
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Última versión:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("v{}", item.max_version)).size(12.0).strong().color(green));
                });
            });
            ui.add_space(4.0);

            // Licencia
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Licencia:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(lic_str).size(12.0).color(egui::Color32::WHITE));
                });
            });
            ui.add_space(4.0);

            // Registro
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Registro:").size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("crates.io").size(12.0).color(cyan));
                });
            });

            if !keywords_list.is_empty() {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                ui.label(egui::RichText::new("Palabras Clave (Keywords):").size(12.0).strong().color(egui::Color32::WHITE));
                ui.add_space(4.0);

                ui.horizontal_wrapped(|ui| {
                    for kw in keywords_list {
                        let mut tag_kw = egui::Frame::new();
                        tag_kw.fill = egui::Color32::from_rgb(18, 24, 36);
                        tag_kw.corner_radius = egui::CornerRadius::same(3);
                        tag_kw.inner_margin = egui::Margin::symmetric(5, 2);
                        tag_kw.show(ui, |ui| {
                            ui.label(egui::RichText::new(format!("#{}", kw.keyword)).size(11.0).color(cyan));
                        });
                        ui.add_space(3.0);
                    }
                });
            }
        });
    }
}

// --- BOTONES PERSONALIZADOS PINTADOS CON ILUMINACIÓN HOVER ESTILO FERRISKEY ---

fn boton_custom_cyan_buscar(ui: &mut egui::Ui, width: f32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 28.0), egui::Sense::click());

    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    let cyan = egui::Color32::from_rgb(100, 200, 255);

    let button_fill = if pressed {
        egui::Color32::from_rgb(22, 60, 80)
    } else if hovered {
        egui::Color32::from_rgb(28, 75, 100)
    } else {
        egui::Color32::from_rgb(18, 48, 65)
    };
    let button_stroke = if hovered || pressed {
        cyan
    } else {
        egui::Color32::from_rgb(45, 110, 140)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(5),
        button_fill,
        egui::Stroke::new(1.0, button_stroke),
        egui::StrokeKind::Inside,
    );

    let icon_color = if hovered || pressed {
        egui::Color32::WHITE
    } else {
        cyan
    };
    let text_color = if hovered || pressed {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(190, 225, 255)
    };

    let text = "Buscar";
    let font_id = egui::FontId::proportional(12.0);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id.clone(), text_color);
    let text_w = galley.size().x;
    let icon_size = 14.0;
    let gap = 5.0;
    let total_content_w = icon_size + gap + text_w;

    let start_x = rect.center().x - total_content_w / 2.0;
    let icon_rect = egui::Rect::from_min_size(
        egui::pos2(start_x, rect.center().y - icon_size / 2.0),
        egui::vec2(icon_size, icon_size),
    );

    let img_search = egui::Image::new(egui::include_image!(
        "../../../../assets/icons/search-alt-2-svgrepo-com.svg"
    ))
    .fit_to_exact_size(egui::vec2(icon_size, icon_size))
    .tint(icon_color);
    img_search.paint_at(ui, icon_rect);

    let text_pos = egui::pos2(start_x + icon_size + gap, rect.center().y);
    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        text,
        font_id,
        text_color,
    );

    response
}

fn boton_custom_cyan(ui: &mut egui::Ui, texto: &str, width: f32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 28.0), egui::Sense::click());

    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    let cyan = egui::Color32::from_rgb(100, 200, 255);

    let button_fill = if pressed {
        egui::Color32::from_rgb(22, 60, 80)
    } else if hovered {
        egui::Color32::from_rgb(28, 75, 100)
    } else {
        egui::Color32::from_rgb(18, 48, 65)
    };
    let button_stroke = if hovered || pressed {
        cyan
    } else {
        egui::Color32::from_rgb(45, 110, 140)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(5),
        button_fill,
        egui::Stroke::new(1.0, button_stroke),
        egui::StrokeKind::Inside,
    );

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        texto,
        egui::FontId::proportional(12.0),
        if hovered || pressed {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_rgb(190, 225, 255)
        },
    );

    response
}

fn boton_custom_orange(ui: &mut egui::Ui, texto: &str, width: f32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 28.0), egui::Sense::click());

    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    let orange = egui::Color32::from_rgb(255, 160, 50);

    let button_fill = if pressed {
        egui::Color32::from_rgb(60, 38, 18)
    } else if hovered {
        egui::Color32::from_rgb(72, 46, 20)
    } else {
        egui::Color32::from_rgb(50, 32, 16)
    };
    let button_stroke = if hovered || pressed {
        orange
    } else {
        egui::Color32::from_rgb(140, 85, 35)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(5),
        button_fill,
        egui::Stroke::new(1.0, button_stroke),
        egui::StrokeKind::Inside,
    );

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        texto,
        egui::FontId::proportional(12.0),
        if hovered || pressed {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_rgb(255, 225, 195)
        },
    );

    response
}

fn lanzar_resumen_asincrono(ctx: egui::Context) {
    {
        let mut state = VIEW_STATE.lock().unwrap();
        let vs = state.get_or_insert_with(CratesIoViewState::default);
        if vs.is_loading_summary || vs.summary_data.is_some() {
            return;
        }
        vs.is_loading_summary = true;
    }

    thread::spawn(move || {
        let url = "https://crates.io/api/v1/summary";
        let res = ureq::get(url)
            .header(
                "User-Agent",
                "FerrisKey-Desktop/0.1 (learning-rust-platform)",
            )
            .call();

        {
            let mut state = VIEW_STATE.lock().unwrap();
            let vs = state.get_or_insert_with(CratesIoViewState::default);
            match res {
                Ok(mut resp) => {
                    if let Ok(data) = resp.body_mut().read_json::<SummaryApiResponse>() {
                        vs.summary_data = Some(data);
                        vs.anim_needs_restart = true;
                    }
                }
                Err(_) => {}
            }
            vs.is_loading_summary = false;
        }
        ctx.request_repaint();
    });
}

fn lanzar_busqueda_asincrona(query: String, ctx: egui::Context) {
    let clean_query = query.trim().to_string();
    if clean_query.is_empty() {
        return;
    }
    {
        let mut state = VIEW_STATE.lock().unwrap();
        let vs = state.get_or_insert_with(CratesIoViewState::default);
        if vs.is_loading {
            return;
        }
        vs.is_search_active = true;
        vs.is_loading = true;
        vs.error_msg = None;
    }

    thread::spawn(move || {
        let encoded_query = clean_query.replace(' ', "+");
        let url = format!(
            "https://crates.io/api/v1/crates?q={}&per_page=10",
            encoded_query
        );

        let res = ureq::get(&url)
            .header(
                "User-Agent",
                "FerrisKey-Desktop/0.1 (learning-rust-platform)",
            )
            .call();

        {
            let mut state = VIEW_STATE.lock().unwrap();
            let vs = state.get_or_insert_with(CratesIoViewState::default);
            match res {
                Ok(mut resp) => {
                    if let Ok(data) = resp.body_mut().read_json::<CratesApiResponse>() {
                        vs.results = data.crates;
                        vs.error_msg = None;
                        vs.anim_needs_restart = true;
                    } else {
                        vs.error_msg = Some("No se pudo interpretar la respuesta JSON de Crates.io.".to_string());
                    }
                }
                Err(e) => {
                    vs.error_msg = Some(format!("{}", e));
                }
            }
            vs.is_loading = false;
        }
        ctx.request_repaint();
    });
}

fn lanzar_detalle_asincrono(crate_id: String, ctx: egui::Context) {
    {
        let mut state = VIEW_STATE.lock().unwrap();
        let vs = state.get_or_insert_with(CratesIoViewState::default);
        if vs.is_loading_detail {
            return;
        }
        if let Some(detail) = &vs.full_detail {
            if detail.krate.id == crate_id {
                return;
            }
        }
        vs.is_loading_detail = true;
    }

    thread::spawn(move || {
        let url = format!("https://crates.io/api/v1/crates/{}", crate_id);
        let res = ureq::get(&url)
            .header("User-Agent", "FerrisKey-Desktop/0.1 (learning-rust-platform)")
            .call();

        {
            let mut state = VIEW_STATE.lock().unwrap();
            let vs = state.get_or_insert_with(CratesIoViewState::default);
            match res {
                Ok(mut resp) => {
                    if let Ok(data) = resp.body_mut().read_json::<CrateFullDetailResponse>() {
                        vs.full_detail = Some(data);
                    }
                }
                Err(_) => {}
            }
            vs.is_loading_detail = false;
        }
        ctx.request_repaint();
    });
}

fn formatear_numero(n: u64) -> String {
    let s = n.to_string();
    let mut resultado = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            resultado.push('.');
        }
        resultado.push(c);
    }
    resultado
}

fn formatear_numero_compacto(n: u64) -> String {
    if n >= 1_000_000_000_000 {
        format!("{:.1}T", n as f64 / 1_000_000_000_000.0)
    } else if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        format!("{}", n)
    }
}
