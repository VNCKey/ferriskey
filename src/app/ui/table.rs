use eframe::egui::{self, Grid, RichText};
use super::code::table_code_snippet;
use super::theme::{Colors, Spacing, Typography};

/// Centra una tabla completa usando su ancho real del fotograma anterior.
/// No modifica el tamaño de columnas, filas ni widgets internos.
pub fn centered_grid<R>(
    ui: &mut egui::Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    add_grid: impl FnOnce(&mut egui::Ui) -> egui::InnerResponse<R>,
) -> R {
    let id = ui.make_persistent_id(("centered_grid_width", id_salt));
    let available_width = ui.available_width();
    let previous_width = ui
        .ctx()
        .data_mut(|data| data.get_temp::<f32>(id))
        .unwrap_or(0.0);
    let left_padding = if previous_width > 0.0 {
        ((available_width - previous_width) * 0.5).max(0.0)
    } else {
        0.0
    };

    let grid = ui
        .horizontal(|ui| {
            ui.add_space(left_padding);
            add_grid(ui)
        })
        .inner;

    let measured_width = grid.response.rect.width();
    if measured_width > 0.0 && (measured_width - previous_width).abs() > 0.5 {
        ui.ctx()
            .data_mut(|data| data.insert_temp(id, measured_width));
        ui.ctx().request_repaint();
    }

    grid.inner
}

/// Componente universal para tablas educativas y comparativas
pub struct EducationalTable<'a> {
    id: &'a str,
    headers: Vec<&'a str>,
    min_col_width: f32,
    spacing: [f32; 2],
}

impl<'a> EducationalTable<'a> {
    pub fn new(id: &'a str, headers: &[&'a str]) -> Self {
        Self {
            id,
            headers: headers.to_vec(),
            min_col_width: 100.0,
            spacing: [18.0, 6.0],
        }
    }

    pub fn min_col_width(mut self, width: f32) -> Self {
        self.min_col_width = width;
        self
    }

    pub fn spacing(mut self, x: f32, y: f32) -> Self {
        self.spacing = [x, y];
        self
    }

    /// Renderiza la tabla completa dentro de su Frame estándar
    pub fn show<F>(self, ui: &mut egui::Ui, add_rows: F)
    where
        F: FnOnce(&mut TableBody),
    {
        let mut frame = egui::Frame::new();
        frame.fill = Colors::BG_CARD;
        frame.inner_margin = egui::Margin {
            left: 20,
            right: 18,
            top: 10,
            bottom: 10,
        };
        frame.corner_radius = Spacing::card_rounding();
        frame.stroke = Spacing::card_stroke();

        let num_cols = self.headers.len();

        frame.show(ui, |ui| {
            Grid::new(self.id)
                .striped(true)
                .num_columns(num_cols)
                .min_col_width(self.min_col_width)
                .spacing(self.spacing)
                .show(ui, |ui| {
                    // Encabezados con estilo consistente y alineados a la izquierda
                    for header in &self.headers {
                        cell_centered(ui, |ui| {
                            ui.label(
                                RichText::new(*header)
                                    .strong()
                                    .font(Typography::body_small())
                                    .color(Colors::TEXT_WHITE),
                            );
                        });
                    }
                    ui.end_row();

                    let mut body = TableBody { ui };
                    add_rows(&mut body);
                });
        });
    }
}

pub struct TableBody<'a> {
    pub ui: &'a mut egui::Ui,
}

impl<'a> TableBody<'a> {
    /// Añade una fila personalizada a la tabla
    pub fn row(&mut self, add_cells: impl FnOnce(&mut egui::Ui)) {
        add_cells(self.ui);
        self.ui.end_row();
    }
}

/// Alinea el contenido de una celda a la izquierda, manteniendo su altura
/// compacta y evitando saltos de línea en elementos cortos como chips.
pub fn cell_centered<R>(ui: &mut egui::Ui, add_content: impl FnOnce(&mut egui::Ui) -> R) -> R {
    ui.horizontal(|ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
        add_content(ui)
    })
    .inner
}

/// Variante para grupos en fila; conserva la alineación izquierda de la celda.
pub fn cell_centered_horizontal<R>(ui: &mut egui::Ui, add_content: impl FnOnce(&mut egui::Ui) -> R) -> R {
    cell_centered(ui, add_content)
}

/// Renderiza texto explicativo con chips inline, wrapping responsivo y
/// alineación izquierda dentro de la celda.
pub fn texto_con_chips_inline(
    ui: &mut egui::Ui,
    texto: &str,
    text_color: egui::Color32,
    chip_color: egui::Color32,
) {
    let avail = ui.available_width();
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = avail.max(100.0);
    job.halign = egui::Align::LEFT;

    let font_text = egui::FontId::proportional(12.5);
    let font_code = egui::FontId::monospace(12.0);

    let format_text = egui::TextFormat {
        font_id: font_text,
        color: text_color,
        valign: egui::Align::Center,
        ..Default::default()
    };

    let format_code = egui::TextFormat {
        font_id: font_code,
        color: chip_color,
        background: Colors::BG_CODE_INLINE,
        valign: egui::Align::Center,
        ..Default::default()
    };

    let mut in_code = false;
    for part in texto.split('`') {
        if part.is_empty() {
            in_code = !in_code;
            continue;
        }
        if in_code {
            job.append(&format!(" {} ", part), 0.0, format_code.clone());
        } else {
            job.append(part, 0.0, format_text.clone());
        }
        in_code = !in_code;
    }

    let galley = ui.painter().layout_job(job);
    ui.add(egui::Label::new(galley));
}

/// Helper para tablas de métodos estándar de Rust (Método, Descripción, Ejemplo)
pub fn tabla_metodos(
    ui: &mut egui::Ui,
    id: &str,
    titulo: &str,
    subtitulo: &str,
    syntax_set: &syntect::parsing::SyntaxSet,
    theme: &syntect::highlighting::Theme,
    filas: &[(&str, &str, &str)],
) {
    let mut frame = egui::Frame::new();
    frame.fill = Colors::BG_CARD;
    frame.inner_margin = egui::Margin {
        left: 20,
        right: 18,
        top: 10,
        bottom: 10,
    };
    frame.corner_radius = Spacing::card_rounding();
    frame.stroke = Spacing::card_stroke();

        frame.show(ui, |ui| {
        ui.label(
            RichText::new(titulo)
                .strong()
                .font(Typography::card_title())
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(2.0);
        ui.label(
            RichText::new(subtitulo)
                .font(Typography::body_small())
                .color(Colors::TEXT_MUTED),
        );
        ui.add_space(10.0);

        Grid::new(id)
            .striped(true)
            .num_columns(3)
            .min_col_width(120.0)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                cell_centered(ui, |ui| ui.label(RichText::new("Método").strong().color(Colors::TEXT_WHITE)));
                cell_centered(ui, |ui| ui.label(RichText::new("Descripción / Comportamiento").strong().color(Colors::TEXT_WHITE)));
                cell_centered(ui, |ui| ui.label(RichText::new("Ejemplo de uso").strong().color(Colors::TEXT_WHITE)));
                ui.end_row();

                for (metodo, desc, ejemplo) in filas {
                    cell_centered(ui, |ui| {
                        ui.label(
                            RichText::new(*metodo)
                                .monospace()
                                .strong()
                                .color(Colors::CYAN_ACCENT),
                        )
                    });
                    texto_con_chips_inline(ui, desc, Colors::TEXT_PRIMARY, Colors::CYAN_ACCENT);
                    table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                    ui.end_row();
                }
            });
    });
}
