use eframe::egui;
use std::cell::RefCell;
use wry::WebView;

use crate::app::AppState;

pub struct CratesWebViewState {
    pub current_url: String,
    pub webview: Option<WebView>,
    pub is_initialized: bool,
    pub init_error: Option<String>,
}

impl Default for CratesWebViewState {
    fn default() -> Self {
        Self {
            current_url: "https://crates.io".to_string(),
            webview: None,
            is_initialized: false,
            init_error: None,
        }
    }
}

thread_local! {
    static WEBVIEW_STATE: RefCell<CratesWebViewState> = RefCell::new(CratesWebViewState::default());
}

pub fn mostrar_tab_crates_io_html_wry(ui: &mut egui::Ui, _state: &mut AppState) {
    WEBVIEW_STATE.with(|state_cell| {
        let mut vs = state_cell.borrow_mut();

        let orange = egui::Color32::from_rgb(255, 160, 50);
        let cyan = egui::Color32::from_rgb(100, 200, 255);

        ui.add_space(10.0);

        // --- HEADER CON BARRA DE DIRECCIÓN Y CONTROLES DEL NAVEGADOR HTML REAL ---
        let mut header_frame = egui::Frame::new();
        header_frame.fill = egui::Color32::from_rgb(18, 24, 36);
        header_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 52, 76));
        header_frame.corner_radius = egui::CornerRadius::same(10);
        header_frame.inner_margin = egui::Margin::same(12);

        header_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("🌐 Navegador HTML Real (wry / WebKit Engine)")
                        .size(15.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(
                            egui::RichText::new("🔗 Abrir en Navegador Externo")
                                .size(12.0)
                                .color(cyan),
                        )
                        .clicked()
                    {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(&vs.current_url));
                    }
                });
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("URL:")
                        .size(12.0)
                        .strong()
                        .color(orange),
                );

                ui.add(
                    egui::TextEdit::singleline(&mut vs.current_url)
                        .desired_width(450.0),
                );

                if ui.button(egui::RichText::new("Ir a URL").strong()).clicked() {
                    if let Some(wv) = &vs.webview {
                        let _ = wv.load_url(&vs.current_url);
                    }
                }
            });
        });

        ui.add_space(12.0);

        // Banner explicativo sobre el Enfoque 2 (wry HTML WebView)
        let mut banner_frame = egui::Frame::new();
        banner_frame.fill = egui::Color32::from_rgb(24, 32, 48);
        banner_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95));
        banner_frame.corner_radius = egui::CornerRadius::same(8);
        banner_frame.inner_margin = egui::Margin::same(10);

        banner_frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new("ℹ️ Enfoque 2 (wry Native WebView): Esta pestaña integra la ventana webview de wry / WebKit cargando directamente el sitio HTML5 real de https://crates.io. Puedes comparar el rendimiento y la estética visual contra la pestaña 'Crates.io API (Enfoque 1)'.")
                    .size(12.0)
                    .color(egui::Color32::from_rgb(190, 210, 235)),
            );
        });

        ui.add_space(12.0);

        // Contenedor principal donde se aloja la vista del navegador
        let available_rect = ui.available_rect_before_wrap();
        let webview_rect = egui::Rect::from_min_max(
            available_rect.min,
            egui::pos2(available_rect.max.x, available_rect.max.y.max(available_rect.min.y + 450.0)),
        );

        ui.allocate_rect(webview_rect, egui::Sense::hover());

        if let Some(err) = &vs.init_error {
            ui.label(
                egui::RichText::new(format!("⚠️ No se pudo iniciar el WebView nativo: {}", err))
                    .color(egui::Color32::from_rgb(255, 130, 130)),
            );
        }
    });
}
