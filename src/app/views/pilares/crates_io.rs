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
    pub copied_feedback: Option<String>,
}

impl Default for CratesIoViewState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            is_loading: false,
            results: Vec::new(),
            error_msg: None,
            selected_category: 0,
            copied_feedback: None,
        }
    }
}

static VIEW_STATE: Mutex<Option<CratesIoViewState>> = Mutex::new(None);

pub fn mostrar_tab_crates_io_web(ui: &mut egui::Ui, _state: &mut AppState) {
    let cargo_orange = egui::Color32::from_rgb(235, 120, 35);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);
    let bg_card = egui::Color32::from_rgb(16, 22, 34);

    let mut state = VIEW_STATE.lock().unwrap();
    let vs = state.get_or_insert_with(CratesIoViewState::default);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(8.0);

            // --- HERO BANNER ESTILO CRATES.IO OFICIAL ---
            let mut hero_frame = egui::Frame::new();
            hero_frame.fill = egui::Color32::from_rgb(22, 28, 42);
            hero_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));
            hero_frame.corner_radius = egui::CornerRadius::same(12);
            hero_frame.inner_margin = egui::Margin::same(16);

            hero_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;

                    ui.label(
                        egui::RichText::new("📦 crates.io")
                            .size(22.0)
                            .strong()
                            .color(cargo_orange),
                    );

                    ui.label(
                        egui::RichText::new("— The Rust community's crate registry")
                            .size(14.0)
                            .color(egui::Color32::from_rgb(180, 195, 215)),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(
                                egui::RichText::new("🔗 Abrir crates.io")
                                    .size(12.0)
                                    .color(cyan),
                            )
                            .on_hover_text("Abrir https://crates.io en navegador")
                            .clicked()
                        {
                            ui.ctx().open_url(egui::OpenUrl::new_tab("https://crates.io"));
                        }

                        // Badge de estadísticas
                        let mut badge_frame = egui::Frame::new();
                        badge_frame.fill = egui::Color32::from_rgb(12, 16, 24);
                        badge_frame.corner_radius = egui::CornerRadius::same(6);
                        badge_frame.inner_margin = egui::Margin::symmetric(8, 4);
                        badge_frame.show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("160,000+ Crates registrados")
                                    .size(11.0)
                                    .strong()
                                    .color(green),
                            );
                        });
                    });
                });

                ui.add_space(12.0);

                // --- BARRA DE BÚSQUEDA DESTACADA (HERO SEARCH) ---
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("🔍 BUSCAR:")
                            .size(13.0)
                            .strong()
                            .color(cargo_orange),
                    );

                    let search_resp = ui.add(
                        egui::TextEdit::singleline(&mut vs.search_query)
                            .hint_text("Presiona Enter para buscar paquetes (ej: serde, tokio, axum, clap, egui)...")
                            .desired_width(500.0),
                    );

                    let enter_pressed = search_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if ui.button(egui::RichText::new("🔎 Buscar Crate").strong()).clicked() || enter_pressed {
                        ejecutar_busqueda_crates(&mut vs.search_query, &mut vs.results, &mut vs.is_loading, &mut vs.error_msg);
                    }

                    if vs.is_loading {
                        ui.add_space(8.0);
                        ui.spinner();
                        ui.label(egui::RichText::new("Consultando API de Crates.io...").size(12.0).color(cyan));
                    }
                });
            });

            ui.add_space(14.0);

            // --- NOTIFICACIÓN DE FEEDBACK DE COPIADO ---
            if let Some(msg) = &vs.copied_feedback {
                let mut notify_frame = egui::Frame::new();
                notify_frame.fill = egui::Color32::from_rgb(20, 48, 32);
                notify_frame.stroke = egui::Stroke::new(1.0, green);
                notify_frame.corner_radius = egui::CornerRadius::same(6);
                notify_frame.inner_margin = egui::Margin::symmetric(12, 6);

                notify_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("✅ {}", msg))
                            .size(12.0)
                            .strong()
                            .color(green),
                    );
                });
                ui.add_space(10.0);
            }

            // --- CHIPS DE CATEGORÍAS POPULARES ---
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Explorar por Categoría:")
                        .size(12.5)
                        .strong()
                        .color(cargo_orange),
                );

                let categorias = [
                    ("🚀 Asíncrono / I/O", "tokio", 0),
                    ("🌐 Desarrollo Web", "axum", 1),
                    ("📄 Serialización", "serde", 2),
                    ("🛠️ CLI / Utilidades", "clap", 3),
                    ("⚙️ Macros & Sintaxis", "syn", 4),
                    ("🎮 GUI & Gráficos", "egui", 5),
                ];

                for (label, query, idx) in categorias {
                    let es_activo = vs.selected_category == idx;
                    let mut cat_frame = egui::Frame::new();
                    cat_frame.fill = if es_activo {
                        egui::Color32::from_rgb(45, 65, 95)
                    } else {
                        egui::Color32::from_rgb(20, 26, 38)
                    };
                    cat_frame.stroke = egui::Stroke::new(
                        1.0,
                        if es_activo { cyan } else { egui::Color32::from_rgb(38, 52, 76) },
                    );
                    cat_frame.corner_radius = egui::CornerRadius::same(6);
                    cat_frame.inner_margin = egui::Margin::symmetric(8, 4);

                    cat_frame.show(ui, |ui| {
                        if ui.selectable_label(es_activo, label).clicked() {
                            vs.selected_category = idx;
                            vs.search_query = query.to_string();
                            ejecutar_busqueda_crates(&mut vs.search_query, &mut vs.results, &mut vs.is_loading, &mut vs.error_msg);
                        }
                    });
                }
            });

            ui.add_space(14.0);

            // Mensaje de error si la búsqueda falla
            if let Some(err) = &vs.error_msg {
                let mut err_frame = egui::Frame::new();
                err_frame.fill = egui::Color32::from_rgb(40, 16, 18);
                err_frame.inner_margin = egui::Margin::same(10);
                err_frame.corner_radius = egui::CornerRadius::same(6);
                err_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("❌ Error de conexión con Crates.io: {}", err))
                            .color(egui::Color32::from_rgb(255, 120, 120)),
                    );
                });
                ui.add_space(10.0);
            }

            // Si los resultados están vacíos y no se está cargando, lanzar búsqueda inicial
            if vs.results.is_empty() && !vs.is_loading && vs.error_msg.is_none() {
                vs.search_query = "tokio".to_string();
                ejecutar_busqueda_crates(&mut vs.search_query, &mut vs.results, &mut vs.is_loading, &mut vs.error_msg);
            }

            // --- LISTADO DE CRATES ESTILO CRATES.IO ---
            for item in &vs.results {
                let mut card = egui::Frame::new();
                card.fill = bg_card;
                card.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(34, 46, 68));
                card.corner_radius = egui::CornerRadius::same(10);
                card.inner_margin = egui::Margin::same(14);

                card.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(
                            egui::RichText::new(&item.name)
                                .size(17.0)
                                .strong()
                                .color(egui::Color32::WHITE),
                        );

                        ui.add_space(6.0);

                        // Badge de versión
                        let mut tag_v = egui::Frame::new();
                        tag_v.fill = egui::Color32::from_rgb(20, 42, 30);
                        tag_v.corner_radius = egui::CornerRadius::same(4);
                        tag_v.inner_margin = egui::Margin::symmetric(6, 2);
                        tag_v.show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("v{}", item.max_version))
                                    .size(11.0)
                                    .strong()
                                    .color(green),
                            );
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let cargo_add_cmd = format!("cargo add {}", item.name);
                            if ui
                                .button(
                                    egui::RichText::new("📋 Copiar `cargo add`")
                                        .size(11.5)
                                        .strong()
                                        .color(cargo_orange),
                                )
                                .on_hover_text(format!("Copiar comando: {}", cargo_add_cmd))
                                .clicked()
                            {
                                ui.ctx().copy_text(cargo_add_cmd.clone());
                                vs.copied_feedback = Some(format!("Comando `{}` copiado al portapapeles", cargo_add_cmd));
                            }

                            ui.add_space(6.0);

                            if ui
                                .button(
                                    egui::RichText::new("📚 Docs.rs")
                                        .size(11.5)
                                        .color(cyan),
                                )
                                .on_hover_text("Ver documentación oficial en docs.rs")
                                .clicked()
                            {
                                let docs_url = item
                                    .documentation
                                    .clone()
                                    .unwrap_or_else(|| format!("https://docs.rs/{}", item.id));
                                ui.ctx().open_url(egui::OpenUrl::new_tab(docs_url));
                            }

                            ui.add_space(6.0);

                            if ui
                                .button(
                                    egui::RichText::new("🔗 Crates.io")
                                        .size(11.5)
                                        .color(egui::Color32::from_rgb(180, 200, 225)),
                                )
                                .on_hover_text("Abrir paquete en crates.io")
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
                                .color(egui::Color32::from_rgb(200, 215, 235)),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("Sin descripción disponible.")
                                .size(12.0)
                                .italics()
                                .color(egui::Color32::GRAY),
                        );
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("📥 {} descargas totales", formatear_numero(item.downloads)))
                                .size(11.0)
                                .color(egui::Color32::from_rgb(140, 160, 185)),
                        );

                        if let Some(repo) = &item.repository {
                            ui.add_space(14.0);
                            ui.hyperlink_to(
                                egui::RichText::new("📁 Repositorio de Código Source")
                                    .size(11.0)
                                    .color(cyan),
                                repo,
                            );
                        }
                    });
                });

                ui.add_space(10.0);
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
        "https://crates.io/api/v1/crates?q={}&per_page=10",
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
                    Some("No se pudo interpretar la respuesta JSON de Crates.io.".to_string());
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
