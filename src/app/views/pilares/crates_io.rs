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
    pub selected_crate: Option<CrateApiItem>,
    pub detail_tab: usize,
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
            selected_crate: None,
            detail_tab: 0,
        }
    }
}

static VIEW_STATE: Mutex<Option<CratesIoViewState>> = Mutex::new(None);

pub fn mostrar_tab_crates_io_web(ui: &mut egui::Ui, state: &mut AppState) {
    let mut vs_guard = VIEW_STATE.lock().unwrap();
    let vs = vs_guard.get_or_insert_with(CratesIoViewState::default);

    if let Some(selected) = vs.selected_crate.clone() {
        mostrar_detalle_crate(ui, &selected, vs, state);
    } else {
        mostrar_lista_crates(ui, vs, state);
    }
}

fn mostrar_lista_crates(ui: &mut egui::Ui, vs: &mut CratesIoViewState, _state: &mut AppState) {
    let cargo_orange = egui::Color32::from_rgb(235, 120, 35);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);
    let bg_card = egui::Color32::from_rgb(16, 22, 34);

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
                                egui::RichText::new("🔗 Abrir crates.io en Web")
                                    .size(12.0)
                                    .color(cyan),
                            )
                            .on_hover_text("Abrir https://crates.io en navegador del sistema")
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
                            .hint_text("Escribe el nombre de la librería (serde, tokio, axum, clap, egui)...")
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

            // Si los resultados están vacíos y no se está cargando, lanzar búsqueda inicial por defecto
            if vs.results.is_empty() && !vs.is_loading && vs.error_msg.is_none() {
                vs.search_query = "tokio".to_string();
                ejecutar_busqueda_crates(&mut vs.search_query, &mut vs.results, &mut vs.is_loading, &mut vs.error_msg);
            }

            // --- LISTADO DE CRATES ---
            for item in vs.results.clone() {
                let mut card = egui::Frame::new();
                card.fill = bg_card;
                card.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(34, 46, 68));
                card.corner_radius = egui::CornerRadius::same(10);
                card.inner_margin = egui::Margin::same(14);

                card.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .button(
                                egui::RichText::new(&item.name)
                                    .size(17.0)
                                    .strong()
                                    .color(egui::Color32::WHITE),
                            )
                            .on_hover_text("Ver detalles completos e integración")
                            .clicked()
                        {
                            vs.selected_crate = Some(item.clone());
                        }

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
                            if ui
                                .button(
                                    egui::RichText::new("📖 Ver Detalle")
                                        .size(11.5)
                                        .strong()
                                        .color(cyan),
                                )
                                .clicked()
                            {
                                vs.selected_crate = Some(item.clone());
                            }

                            ui.add_space(6.0);

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
                                egui::RichText::new("📁 Repositorio Source")
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

fn mostrar_detalle_crate(
    ui: &mut egui::Ui,
    item: &CrateApiItem,
    vs: &mut CratesIoViewState,
    _state: &mut AppState,
) {
    let cargo_orange = egui::Color32::from_rgb(235, 120, 35);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(8.0);

            // --- BOTÓN DE REGRESO ---
            if ui
                .button(
                    egui::RichText::new("⬅ Volver a la Lista de Crates")
                        .size(13.0)
                        .strong()
                        .color(cyan),
                )
                .clicked()
            {
                vs.selected_crate = None;
                return;
            }

            ui.add_space(10.0);

            // --- CABECERA DE DETALLE DEL CRATE ---
            let mut header_frame = egui::Frame::new();
            header_frame.fill = egui::Color32::from_rgb(20, 26, 38);
            header_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 62, 90));
            header_frame.corner_radius = egui::CornerRadius::same(12);
            header_frame.inner_margin = egui::Margin::same(16);

            header_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(
                        egui::RichText::new(format!("📦 {}", item.name))
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );

                    ui.add_space(8.0);

                    let mut tag_v = egui::Frame::new();
                    tag_v.fill = egui::Color32::from_rgb(20, 48, 32);
                    tag_v.corner_radius = egui::CornerRadius::same(6);
                    tag_v.inner_margin = egui::Margin::symmetric(8, 3);
                    tag_v.show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!("v{}", item.max_version))
                                .size(13.0)
                                .strong()
                                .color(green),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let cargo_add_cmd = format!("cargo add {}", item.name);
                        if ui
                            .button(
                                egui::RichText::new("📋 Copiar `cargo add`")
                                    .size(12.0)
                                    .strong()
                                    .color(cargo_orange),
                            )
                            .clicked()
                        {
                            ui.ctx().copy_text(cargo_add_cmd.clone());
                            vs.copied_feedback = Some(format!("Comando `{}` copiado", cargo_add_cmd));
                        }

                        ui.add_space(8.0);

                        if ui
                            .button(
                                egui::RichText::new("📚 Abrir Docs.rs")
                                    .size(12.0)
                                    .color(cyan),
                            )
                            .clicked()
                        {
                            let docs_url = item
                                .documentation
                                .clone()
                                .unwrap_or_else(|| format!("https://docs.rs/{}", item.id));
                            ui.ctx().open_url(egui::OpenUrl::new_tab(docs_url));
                        }

                        ui.add_space(8.0);

                        if ui
                            .button(
                                egui::RichText::new("🔗 Ver en Crates.io")
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(180, 200, 225)),
                            )
                            .clicked()
                        {
                            let url = format!("https://crates.io/crates/{}", item.id);
                            ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                        }
                    });
                });

                ui.add_space(8.0);

                if let Some(desc) = &item.description {
                    ui.label(
                        egui::RichText::new(desc)
                            .size(13.5)
                            .color(egui::Color32::from_rgb(210, 225, 245)),
                    );
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("📥 {} descargas acumuladas", formatear_numero(item.downloads)))
                            .size(11.5)
                            .color(egui::Color32::from_rgb(150, 170, 195)),
                    );

                    if let Some(repo) = &item.repository {
                        ui.add_space(16.0);
                        ui.hyperlink_to(
                            egui::RichText::new("📁 Repositorio en GitHub / GitLab")
                                .size(11.5)
                                .color(cyan),
                            repo,
                        );
                    }
                });
            });

            ui.add_space(14.0);

            // --- NOTIFICACIÓN FEEDBACK ---
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

            // --- PESTAÑAS DE CONTENIDO DEL DETALLE ---
            ui.horizontal(|ui| {
                let tabs = [
                    (0, "📖 Integración en Cargo.toml"),
                    (1, "💻 Ejemplo de Código de Uso"),
                    (2, "⚙️ Características (Features)"),
                ];

                for (idx, label) in tabs {
                    let es_activo = vs.detail_tab == idx;
                    if ui.selectable_label(es_activo, label).clicked() {
                        vs.detail_tab = idx;
                    }
                }
            });

            ui.add_space(10.0);

            // --- SECCIÓN 0: CARGO.TOML INTEGRATION ---
            if vs.detail_tab == 0 {
                let mut cargo_frame = egui::Frame::new();
                cargo_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                cargo_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(32, 45, 68));
                cargo_frame.corner_radius = egui::CornerRadius::same(8);
                cargo_frame.inner_margin = egui::Margin::same(14);

                cargo_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Añadir a tu archivo Cargo.toml:")
                            .size(13.0)
                            .strong()
                            .color(cargo_orange),
                    );
                    ui.add_space(6.0);

                    let toml_snippet = format!("[dependencies]\n{} = \"{}\"", item.name, item.max_version);

                    let mut code_frame = egui::Frame::new();
                    code_frame.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_frame.corner_radius = egui::CornerRadius::same(6);
                    code_frame.inner_margin = egui::Margin::same(10);

                    code_frame.show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&toml_snippet)
                                .font(egui::FontId::monospace(13.0))
                                .color(green),
                        );
                    });

                    ui.add_space(8.0);

                    if ui
                        .button(
                            egui::RichText::new("📋 Copiar Bloque TOML")
                                .size(12.0)
                                .strong()
                                .color(cyan),
                        )
                        .clicked()
                    {
                        ui.ctx().copy_text(toml_snippet.clone());
                        vs.copied_feedback = Some("Bloque TOML copiado al portapapeles".to_string());
                    }
                });
            }

            // --- SECCIÓN 1: EJEMPLO DE CÓDIGO ---
            if vs.detail_tab == 1 {
                let ejemplo = obtener_ejemplo_codigo_crate(&item.name);

                let mut code_box = egui::Frame::new();
                code_box.fill = egui::Color32::from_rgb(10, 14, 22);
                code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 44, 66));
                code_box.corner_radius = egui::CornerRadius::same(8);
                code_box.inner_margin = egui::Margin::same(14);

                code_box.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("Ejemplo de código con {}", item.name))
                                .size(13.0)
                                .strong()
                                .color(cargo_orange),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button(
                                    egui::RichText::new("📋 Copiar Código")
                                        .size(11.5)
                                        .color(cyan),
                                )
                                .clicked()
                            {
                                ui.ctx().copy_text(ejemplo.to_string());
                                vs.copied_feedback = Some("Código fuente copiado al portapapeles".to_string());
                            }
                        });
                    });

                    ui.add_space(8.0);

                    ui.label(
                        egui::RichText::new(ejemplo)
                            .font(egui::FontId::monospace(12.5))
                            .color(egui::Color32::from_rgb(220, 235, 255)),
                    );
                });
            }

            // --- SECCIÓN 2: FEATURES POPULARES ---
            if vs.detail_tab == 2 {
                let features = obtener_features_populares(&item.name);

                let mut feat_frame = egui::Frame::new();
                feat_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                feat_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 42, 62));
                feat_frame.corner_radius = egui::CornerRadius::same(8);
                feat_frame.inner_margin = egui::Margin::same(14);

                feat_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("Características (Features) de {}", item.name))
                            .size(13.0)
                            .strong()
                            .color(cargo_orange),
                    );
                    ui.add_space(8.0);

                    for (feat_name, feat_desc) in features {
                        ui.horizontal(|ui| {
                            let mut feat_tag = egui::Frame::new();
                            feat_tag.fill = egui::Color32::from_rgb(26, 38, 56);
                            feat_tag.corner_radius = egui::CornerRadius::same(4);
                            feat_tag.inner_margin = egui::Margin::symmetric(6, 2);

                            feat_tag.show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(feat_name)
                                        .font(egui::FontId::monospace(11.5))
                                        .strong()
                                        .color(green),
                                );
                            });

                            ui.label(
                                egui::RichText::new(feat_desc)
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(190, 205, 225)),
                            );
                        });
                        ui.add_space(4.0);
                    }
                });
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

fn obtener_ejemplo_codigo_crate(crate_name: &str) -> &'static str {
    match crate_name.to_lowercase().as_str() {
        "tokio" => r#"// Ejemplo de uso con Tokio (Runtime Asíncrono)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando runtime asíncrono Tokio...");

    let handle = tokio::spawn(async {
        println!("¡Tarea asíncrona ejecutada en segundo plano!");
    });

    handle.await?;
    Ok(())
}"#,
        "serde" | "serde_json" => r#"// Ejemplo de uso con Serde (Serialización JSON)
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Usuario {
    nombre: String,
    edad: u32,
    es_admin: bool,
}

fn main() {
    let u = Usuario {
        nombre: "Ferris".to_string(),
        edad: 10,
        es_admin: true,
    };

    let json = serde_json::to_string_pretty(&u).unwrap();
    println!("JSON Resultante:\n{}", json);
}"#,
        "axum" => r#"// Ejemplo de servidor web con Axum
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "¡Hola desde Axum!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Servidor escuchando en http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}"#,
        "reqwest" => r#"// Ejemplo de cliente HTTP con Reqwest
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .text()
        .await?;
    println!("Respuesta HTTP:\n{}", resp);
    Ok(())
}"#,
        "clap" => r#"// Ejemplo de CLI con Clap
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("¡Hola {}!", args.name);
}"#,
        "egui" | "eframe" => r#"// Ejemplo de GUI con egui / eframe
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Mi App egui", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("¡Hola desde egui en Rust!");
        });
    })
}"#,
        _ => r#"// Ejemplo de integración genérica en main.rs
fn main() {
    println!("Librería cargada e integrada correctamente.");
}"#,
    }
}

fn obtener_features_populares(crate_name: &str) -> Vec<(&'static str, &'static str)> {
    match crate_name.to_lowercase().as_str() {
        "tokio" => vec![
            ("full", "Habilita todas las funcionalidades del runtime de Tokio."),
            ("rt-multi-thread", "Soporte para runtime multihilo con planificación de tareas."),
            ("macros", "Proporciona las macros #[tokio::main] y #[tokio::test]."),
            ("net", "Soporte para sockets TCP, UDP y Unix."),
            ("time", "Temporizadores asíncronos (sleep, interval, timeout)."),
        ],
        "serde" => vec![
            ("derive", "Habilita #[derive(Serialize, Deserialize)]."),
            ("std", "Soporte para tipos de la biblioteca estándar (String, Vec, HashMap)."),
            ("alloc", "Soporte para asignación de memoria sin std completo."),
        ],
        "reqwest" => vec![
            ("json", "Habilita la des/serialización de payloads JSON automáticamente."),
            ("blocking", "Proporciona API síncrona/bloqueante."),
            ("rustls-tls", "Utiliza Rustls en lugar de OpenSSL del sistema."),
        ],
        _ => vec![
            ("default", "Características activas por defecto definidas por el autor."),
            ("std", "Soporte para la biblioteca estándar de Rust."),
        ],
    }
}
