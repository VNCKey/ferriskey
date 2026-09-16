use eframe::egui;
use serde::Deserialize;
use std::sync::Mutex;

use crate::app::AppState;

#[derive(Clone, Debug, Deserialize)]
pub struct CrateApiItem {
    pub id: String,
    pub name: String,
    pub max_version: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub repository: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct CratesApiResponse {
    crates: Vec<CrateApiItem>,
}

pub struct CratesIoViewState {
    pub search_query: String,
    pub is_loading: bool,
    pub results: Vec<CrateApiItem>,
    pub error_msg: Option<String>,
    pub selected_category: usize,
}

impl Default for CratesIoViewState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            is_loading: false,
            results: Vec::new(),
            error_msg: None,
            selected_category: 0,
        }
    }
}

static VIEW_STATE: Mutex<Option<CratesIoViewState>> = Mutex::new(None);

pub fn mostrar_tab_crates_io_web(ui: &mut egui::Ui, _state: &mut AppState) {
    let orange = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let bg_card = egui::Color32::from_rgb(14, 18, 26);

    let mut state = VIEW_STATE.lock().unwrap();
    let vs = state.get_or_insert_with(CratesIoViewState::default);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);

            // --- HEADER DE NAVEGACIÓN WEB DE CRATES.IO ---
            let mut header_frame = egui::Frame::new();
            header_frame.fill = egui::Color32::from_rgb(18, 24, 36);
            header_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 52, 76));
            header_frame.corner_radius = egui::CornerRadius::same(10);
            header_frame.inner_margin = egui::Margin::same(12);

            header_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;

                    ui.label(
                        egui::RichText::new("🌐 Crates.io - El Registro Oficial de Paquetes de Rust")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(
                                egui::RichText::new("🔗 Abrir crates.io en Navegador Externo")
                                    .size(12.0)
                                    .color(cyan),
                            )
                            .on_hover_text("Abre https://crates.io en tu navegador predeterminado")
                            .clicked()
                        {
                            ui.ctx().open_url(egui::OpenUrl::new_tab("https://crates.io"));
                        }
                    });
                });

                ui.add_space(8.0);

                // --- BARRA DE BÚSQUEDA ---
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("🔍 BUSCAR CRATE:")
                            .size(12.0)
                            .strong()
                            .color(orange),
                    );

                    let search_resp = ui.add(
                        egui::TextEdit::singleline(&mut vs.search_query)
                            .hint_text("Escribe el nombre de la librería (ej: serde, tokio, egui, axum)...")
                            .desired_width(360.0),
                    );

                    let perform_search = search_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if ui.button(egui::RichText::new("Buscar").strong().color(egui::Color32::BLACK)).clicked() || perform_search {
                        ejecutar_busqueda_crates(&mut vs.search_query, &mut vs.results, &mut vs.is_loading, &mut vs.error_msg);
                    }

                    if vs.is_loading {
                        ui.add_space(8.0);
                        ui.spinner();
                        ui.label(egui::RichText::new("Consultando crates.io...").size(12.0).color(cyan));
                    }
                });
            });

            ui.add_space(16.0);

            // --- CATEGORÍAS RÁPIDAS DE CRATES POPULARES ---
            ui.label(
                egui::RichText::new("📦 EXPLORAR CATEGORÍAS POPULARES")
                    .size(13.0)
                    .strong()
                    .color(orange),
            );
            ui.add_space(6.0);

            let categorias = [
                ("🌐 Web & Async", "tokio"),
                ("⚙️ GUI & Desktop", "egui"),
                ("📄 Serialización & Data", "serde"),
                ("🎮 Game Dev & Gráficos", "bevy"),
                ("🛠️ CLI & Utilidades", "clap"),
            ];

            ui.horizontal_wrapped(|ui| {
                for (idx, (cat_label, default_query)) in categorias.iter().enumerate() {
                    let es_seleccionado = vs.selected_category == idx;
                    let bg = if es_seleccionado {
                        egui::Color32::from_rgb(45, 65, 95)
                    } else {
                        egui::Color32::from_rgb(22, 30, 44)
                    };

                    let mut cat_frame = egui::Frame::new();
                    cat_frame.fill = bg;
                    cat_frame.stroke = egui::Stroke::new(1.0, if es_seleccionado { cyan } else { egui::Color32::from_rgb(38, 54, 80) });
                    cat_frame.corner_radius = egui::CornerRadius::same(6);
                    cat_frame.inner_margin = egui::Margin::symmetric(10, 6);

                    cat_frame.show(ui, |ui| {
                        if ui.selectable_label(es_seleccionado, *cat_label).clicked() {
                            vs.selected_category = idx;
                            vs.search_query = default_query.to_string();
                            ejecutar_busqueda_crates(&mut vs.search_query, &mut vs.results, &mut vs.is_loading, &mut vs.error_msg);
                        }
                    });
                }
            });

            ui.add_space(16.0);

            // Mensaje de error si la búsqueda falla
            if let Some(err) = &vs.error_msg {
                let mut err_frame = egui::Frame::new();
                err_frame.fill = egui::Color32::from_rgb(40, 16, 18);
                err_frame.inner_margin = egui::Margin::same(10);
                err_frame.corner_radius = egui::CornerRadius::same(6);
                err_frame.show(ui, |ui| {
                    ui.label(egui::RichText::new(format!("❌ Error de conexión: {}", err)).color(egui::Color32::from_rgb(255, 120, 120)));
                });
                ui.add_space(10.0);
            }

            // Si los resultados están vacíos y no está cargando, ejecutar búsqueda por defecto
            if vs.results.is_empty() && !vs.is_loading && vs.error_msg.is_none() {
                vs.search_query = "serde".to_string();
                ejecutar_busqueda_crates(&mut vs.search_query, &mut vs.results, &mut vs.is_loading, &mut vs.error_msg);
            }

            // --- RESULTADOS DE CRATES ---
            for item in &vs.results {
                let mut card = egui::Frame::new();
                card.fill = bg_card;
                card.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(34, 46, 68));
                card.corner_radius = egui::CornerRadius::same(8);
                card.inner_margin = egui::Margin::same(12);

                card.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(
                            egui::RichText::new(&item.name)
                                .size(16.0)
                                .strong()
                                .color(egui::Color32::WHITE),
                        );

                        ui.add_space(6.0);

                        let mut tag_v = egui::Frame::new();
                        tag_v.fill = egui::Color32::from_rgb(20, 38, 28);
                        tag_v.corner_radius = egui::CornerRadius::same(4);
                        tag_v.inner_margin = egui::Margin::symmetric(6, 2);
                        tag_v.show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("v{}", item.max_version))
                                    .size(11.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(0, 200, 120)),
                            );
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let cargo_add_cmd = format!("cargo add {}", item.name);
                            if ui
                                .button(
                                    egui::RichText::new("📋 Copiar `cargo add`")
                                        .size(11.0)
                                        .color(orange),
                                )
                                .on_hover_text(format!("Copiar al portapapeles: {}", cargo_add_cmd))
                                .clicked()
                            {
                                ui.ctx().copy_text(cargo_add_cmd);
                            }

                            if ui
                                .button(
                                    egui::RichText::new("🔗 Ver en Crates.io")
                                        .size(11.0)
                                        .color(cyan),
                                )
                                .clicked()
                            {
                                let url = format!("https://crates.io/crates/{}", item.id);
                                ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                            }
                        });
                    });

                    ui.add_space(6.0);

                    if let Some(desc) = &item.description {
                        ui.label(
                            egui::RichText::new(desc)
                                .size(12.5)
                                .color(egui::Color32::from_rgb(180, 195, 215)),
                        );
                    }

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("📥 {} descargas totales", formatear_numero(item.downloads)))
                                .size(11.0)
                                .color(egui::Color32::from_rgb(140, 160, 185)),
                        );

                        if let Some(repo) = &item.repository {
                            ui.add_space(10.0);
                            ui.hyperlink_to(
                                egui::RichText::new("📁 Repositorio Source").size(11.0).color(cyan),
                                repo,
                            );
                        }
                    });
                });

                ui.add_space(8.0);
            }

            ui.add_space(20.0);
        });
}

fn ejecutar_busqueda_crates(
    query: &mut String,
    results: &mut Vec<CrateApiItem>,
    is_loading: &mut bool,
    error_msg: &mut Option<String>,
) {
    if query.trim().is_empty() {
        return;
    }
    *is_loading = true;
    *error_msg = None;

    let encoded_query = query.trim().replace(' ', "+");
    let url = format!(
        "https://crates.io/api/v1/crates?q={}&per_page=8",
        encoded_query
    );

    match ureq::get(&url)
        .header(
            "User-Agent",
            "FerrisKey-Desktop/0.1 (learning-rust-platform)",
        )
        .call()
    {
        Ok(mut resp) => {
            if let Ok(data) = resp.body_mut().read_json::<CratesApiResponse>() {
                *results = data.crates;
            } else {
                *error_msg =
                    Some("No se pudo interpretar el formato JSON de respuesta.".to_string());
            }
        }
        Err(e) => {
            *error_msg = Some(format!("{}", e));
        }
    }
    *is_loading = false;
}

fn formatear_numero(n: u64) -> String {
    let s = n.to_string();
    let mut resultado = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            resultado.push(',');
        }
        resultado.push(c);
    }
    resultado
}
