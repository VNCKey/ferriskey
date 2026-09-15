use std::path::{Path, PathBuf};

use eframe::egui;

use crate::app::AppState;
use crate::binary_analysis::{self, AnalysisFocus};

#[derive(Clone, Copy)]
pub(super) struct BinaryProfile<'a> {
    pub(super) name: &'a str,
    pub(super) folder: &'a str,
    pub(super) available: bool,
}

#[derive(Clone, Copy)]
pub(super) struct BinaryCardStyle {
    pub(super) accent: egui::Color32,
    pub(super) text: egui::Color32,
}

pub(super) fn mostrar_ficha_binario(
    ui: &mut egui::Ui,
    state: &mut AppState,
    project_dir: &Path,
    project_name: &str,
    profile: BinaryProfile<'_>,
    style: BinaryCardStyle,
) {
    let binario = crate::platform::cargo_binary_path(project_dir, profile.folder, project_name);
    let metadata = std::fs::metadata(&binario).ok();
    // La disponibilidad usa el ámbar de FerrisKey, no verde: el color verde
    // no forma parte de la identidad visual de este analizador.
    let estado_color = if profile.available {
        egui::Color32::from_rgb(255, 190, 90)
    } else {
        egui::Color32::from_rgb(120, 135, 155)
    };
    let estado_texto = if profile.available {
        "Disponible"
    } else {
        "Pendiente"
    };
    // Las acciones tienen un único lenguaje visual para Debug y Release.
    let action_accent = egui::Color32::from_rgb(255, 160, 50);
    let tamano = metadata
        .as_ref()
        .filter(|metadata| metadata.is_file())
        .map(|metadata| formatear_tamano_binario(metadata.len()));

    egui::Frame::new()
        .fill(egui::Color32::from_rgb(16, 23, 34))
        .stroke(egui::Stroke::new(
            1.0,
            if profile.available {
                egui::Color32::from_rgba_unmultiplied(
                    style.accent.r(),
                    style.accent.g(),
                    style.accent.b(),
                    85,
                )
            } else {
                egui::Color32::from_rgb(35, 46, 62)
            },
        ))
        .corner_radius(egui::CornerRadius::same(5))
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.painter().circle_filled(
                    egui::pos2(ui.cursor().left() + 4.0, ui.cursor().top() + 9.0),
                    3.0,
                    estado_color,
                );
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new(profile.name)
                        .strong()
                        .size(13.0)
                        .color(style.accent),
                );
                ui.add_space(8.0);

                // El estado conserva el verde semántico, pero queda aislado
                // en un chip para no teñir también el tamaño del binario.
                egui::Frame::new()
                    .fill(egui::Color32::from_rgba_unmultiplied(
                        estado_color.r(),
                        estado_color.g(),
                        estado_color.b(),
                        18,
                    ))
                    .stroke(egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(
                            estado_color.r(),
                            estado_color.g(),
                            estado_color.b(),
                            72,
                        ),
                    ))
                    .corner_radius(egui::CornerRadius::same(4))
                    .inner_margin(egui::Margin::symmetric(6, 2))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            let (dot_rect, _) =
                                ui.allocate_exact_size(egui::vec2(6.0, 6.0), egui::Sense::hover());
                            ui.painter()
                                .circle_filled(dot_rect.center(), 2.5, estado_color);
                            ui.label(
                                egui::RichText::new(estado_texto)
                                    .size(10.5)
                                    .strong()
                                    .color(estado_color),
                            );
                        });
                    });

                if let Some(tamano) = tamano.as_ref() {
                    ui.add_space(5.0);
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgba_unmultiplied(
                            style.accent.r(),
                            style.accent.g(),
                            style.accent.b(),
                            14,
                        ))
                        .stroke(egui::Stroke::new(
                            1.0,
                            egui::Color32::from_rgba_unmultiplied(
                                style.accent.r(),
                                style.accent.g(),
                                style.accent.b(),
                                58,
                            ),
                        ))
                        .corner_radius(egui::CornerRadius::same(4))
                        .inner_margin(egui::Margin::symmetric(6, 2))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(tamano)
                                    .size(10.5)
                                    .strong()
                                    .color(egui::Color32::from_rgb(220, 230, 242)),
                            );
                        });
                }
            });

            ui.add_space(3.0);
            ui.label(
                egui::RichText::new(binario.display().to_string())
                    .monospace()
                    .size(10.5)
                    .color(egui::Color32::from_rgb(125, 150, 180)),
            );

            if profile.available {
                ui.add_space(5.0);
                ui.horizontal_wrapped(|ui| {
                    for (accion, descripcion, focus, ancho) in [
                        (
                            "Formato",
                            "formato y arquitectura",
                            AnalysisFocus::Format,
                            58.0,
                        ),
                        ("Secciones", "código y datos", AnalysisFocus::Sections, 68.0),
                        (
                            "Cabecera",
                            "metadatos del ejecutable",
                            AnalysisFocus::Header,
                            62.0,
                        ),
                        (
                            "Dependencias",
                            "bibliotecas importadas",
                            AnalysisFocus::Dependencies,
                            88.0,
                        ),
                    ] {
                        let respuesta =
                            boton_accion_binario(ui, accion, true, action_accent, ancho)
                                .on_hover_text(format!("{descripcion}. Analizar con FerrisKey."));
                        if respuesta.clicked() {
                            mostrar_analisis_binario_en_terminal(
                                state,
                                project_dir.to_path_buf(),
                                binario.clone(),
                                profile.name,
                                focus,
                            );
                        }
                        ui.add_space(3.0);
                    }
                });
            } else {
                ui.label(
                    egui::RichText::new(if profile.folder == "debug" {
                        "Ejecuta cargo build para generar este binario."
                    } else {
                        "Ejecuta cargo build --release para generar este binario."
                    })
                    .size(11.5)
                    .color(style.text),
                );
            }
        });
}

fn boton_accion_binario(
    ui: &mut egui::Ui,
    texto: &str,
    habilitado: bool,
    accent: egui::Color32,
    ancho: f32,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ancho, 23.0),
        if habilitado {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();
    let fill = if !habilitado {
        egui::Color32::from_rgb(19, 25, 35)
    } else if pressed {
        egui::Color32::from_rgb(105, 62, 25)
    } else if hovered {
        egui::Color32::from_rgb(80, 57, 31)
    } else {
        egui::Color32::from_rgb(27, 35, 48)
    };
    let color = if !habilitado {
        egui::Color32::from_rgb(75, 88, 108)
    } else if hovered || pressed {
        accent
    } else {
        egui::Color32::from_rgb(175, 195, 220)
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(4),
        fill,
        egui::Stroke::new(
            1.0,
            if hovered && habilitado {
                accent
            } else {
                egui::Color32::from_rgb(52, 67, 89)
            },
        ),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        texto,
        egui::FontId::monospace(10.5),
        color,
    );
    response
}

fn mostrar_analisis_binario_en_terminal(
    state: &mut AppState,
    project_dir: PathBuf,
    binario: PathBuf,
    perfil: &str,
    focus: AnalysisFocus,
) {
    let foco = match focus {
        AnalysisFocus::Format => "formato",
        AnalysisFocus::Sections => "secciones",
        AnalysisFocus::Header => "cabecera",
        AnalysisFocus::Dependencies => "dependencias",
    };
    let salida = binary_analysis::render_report(&binario, focus).unwrap_or_else(|error| error);

    let comando_visible = format!("FerrisKey analiza {perfil} · {foco}");
    state.terminal.term_cwd = project_dir;
    state.terminal.term_input.clear();
    if let Ok(mut history) = state.terminal.term_history.lock() {
        history.clear();
        history.push(format!("$ {comando_visible}"));
        history.extend(salida.lines().map(str::to_owned));
        history.push("────────────────────────────────────────".to_owned());
    }
    state.terminal.show_terminal_history = true;
    state.ui.show_terminal_modal = true;
}

fn formatear_tamano_binario(bytes: u64) -> String {
    const UNIDADES: [&str; 4] = ["B", "KiB", "MiB", "GiB"];
    let mut cantidad = bytes as f64;
    let mut unidad = 0;
    while cantidad >= 1024.0 && unidad < UNIDADES.len() - 1 {
        cantidad /= 1024.0;
        unidad += 1;
    }
    format!("{cantidad:.2} {}", UNIDADES[unidad])
}
