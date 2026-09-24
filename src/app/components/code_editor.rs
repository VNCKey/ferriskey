use eframe::egui;
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme};
use syntect::parsing::{SyntaxDefinition, SyntaxSet};
use syntect::util::LinesWithEndings;

use super::console_output::formatear_salida_consola;

const MAX_HIGHLIGHT_CACHE_ENTRIES: usize = 512;

#[derive(Clone)]
struct CachedHighlight {
    source: String,
    extension: String,
    font_size_bits: u32,
    theme_signature: u64,
    job: egui::text::LayoutJob,
}

#[derive(Default)]
struct HighlightCache {
    entries: HashMap<u64, CachedHighlight>,
    computations: usize,
}

static HIGHLIGHT_CACHE: OnceLock<Mutex<HighlightCache>> = OnceLock::new();

fn highlight_cache() -> &'static Mutex<HighlightCache> {
    HIGHLIGHT_CACHE.get_or_init(|| Mutex::new(HighlightCache::default()))
}

fn theme_signature(theme: &Theme) -> u64 {
    let mut hasher = DefaultHasher::new();
    theme.name.hash(&mut hasher);
    theme.author.hash(&mut hasher);
    theme.scopes.len().hash(&mut hasher);
    theme.settings.foreground.hash(&mut hasher);
    theme.settings.background.hash(&mut hasher);
    hasher.finish()
}

fn highlight_key(source: &str, extension: &str, font_size: f32, theme_signature: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    extension.hash(&mut hasher);
    font_size.to_bits().hash(&mut hasher);
    theme_signature.hash(&mut hasher);
    hasher.finish()
}

/// Carga la paleta de sintaxis de la aplicación y añade TOML a los lenguajes
/// predeterminados de syntect, que no lo incluye en su dump estándar.
pub fn cargar_syntax_set() -> SyntaxSet {
    let mut builder = SyntaxSet::load_defaults_newlines().into_builder();

    if let Ok(toml) = SyntaxDefinition::load_from_str(
        include_str!("../../../assets/syntaxes/TOML.sublime-syntax"),
        true,
        None,
    ) {
        builder.add(toml);
    }

    builder.build()
}

pub fn rust_layouter(
    ui: &egui::Ui,
    string: &str,
    wrap_width: f32,
    syntax_set: &SyntaxSet,
    theme: &Theme,
) -> std::sync::Arc<egui::Galley> {
    syntax_layouter(ui, string, wrap_width, syntax_set, theme, "rs")
}

pub fn syntax_layouter(
    ui: &egui::Ui,
    string: &str,
    wrap_width: f32,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
) -> std::sync::Arc<egui::Galley> {
    syntax_layouter_with_font_size(
        ui,
        string,
        wrap_width,
        syntax_set,
        theme,
        extension,
        14.0,
    )
}

pub fn syntax_layouter_with_font_size(
    ui: &egui::Ui,
    string: &str,
    wrap_width: f32,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
    font_size: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = cached_highlight_job(string, syntax_set, theme, extension, font_size);
    job.wrap.max_width = wrap_width;
    ui.painter().layout_job(job)
}

fn cached_highlight_job(
    string: &str,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
    font_size: f32,
) -> egui::text::LayoutJob {
    let theme_signature = theme_signature(theme);
    let key = highlight_key(string, extension, font_size, theme_signature);
    let font_size_bits = font_size.to_bits();
    let mut cache = highlight_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    if let Some(cached) = cache.entries.get(&key)
        && cached.source == string
        && cached.extension == extension
        && cached.font_size_bits == font_size_bits
        && cached.theme_signature == theme_signature
    {
        return cached.job.clone();
    }

    let job = build_highlight_job(string, syntax_set, theme, extension, font_size);
    cache.computations += 1;

    if cache.entries.len() >= MAX_HIGHLIGHT_CACHE_ENTRIES {
        cache.entries.clear();
    }

    cache.entries.insert(
        key,
        CachedHighlight {
            source: string.to_owned(),
            extension: extension.to_owned(),
            font_size_bits,
            theme_signature,
            job: job.clone(),
        },
    );

    job
}

fn build_highlight_job(
    string: &str,
    syntax_set: &SyntaxSet,
    theme: &Theme,
    extension: &str,
    font_size: f32,
) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();

    let syntax = syntax_set
        .find_syntax_by_extension(extension)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, theme);

    for line in LinesWithEndings::from(string) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, syntax_set).unwrap_or_default();
        for (style, text) in ranges {
            let color =
                egui::Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b);
            let font_id = egui::FontId::monospace(font_size);
            job.append(text, 0.0, egui::TextFormat::simple(font_id, color));
        }
    }

    if !string.ends_with('\n') && job.text.ends_with('\n') {
        job.text.pop();
    }

    job
}

pub fn mostrar_editor_interactivo<F>(
    ui: &mut egui::Ui,
    code: &mut String,
    output_mutex: Arc<Mutex<String>>,
    btn_text: &str,
    execute_fn: F,
    syntax_set: &SyntaxSet,
    theme: &Theme,
) where
    F: Fn(&str) -> String + Send + 'static,
{
    let mut editor_frame = egui::Frame::new();
    editor_frame.fill = egui::Color32::from_rgb(10, 14, 22);
    editor_frame.inner_margin = egui::Margin::same(14);
    editor_frame.corner_radius = egui::CornerRadius::same(8);
    editor_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95)); // Borde estilo LeetCode / Exercism

    editor_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
            rust_layouter(ui, text.as_str(), wrap_width, syntax_set, theme)
        };

        egui::ScrollArea::vertical()
            .max_height(260.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    // Generación dinámica de la columna de números de línea
                    let num_lines = code.split('\n').count().max(1);
                    let mut line_numbers = String::new();
                    for i in 1..=num_lines {
                        use std::fmt::Write;
                        let _ = writeln!(line_numbers, "{:>2}", i);
                    }

                    // Columna de numeración (Gutter)
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(line_numbers.trim_end())
                                .font(egui::FontId::monospace(14.0))
                                .color(egui::Color32::from_rgb(85, 105, 135)),
                        )
                        .selectable(false),
                    );

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    ui.add(
                        egui::TextEdit::multiline(code)
                            .frame(egui::Frame::NONE)
                            .layouter(&mut layouter)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .lock_focus(true),
                    );
                });
            });
    });

    if !btn_text.is_empty() {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui
                .button(
                    egui::RichText::new(btn_text)
                        .size(16.0)
                        .color(egui::Color32::LIGHT_GREEN),
                )
                .clicked()
            {
                *output_mutex.lock().unwrap() = "Ejecutando...".to_string();
                let code_clone = code.clone();
                let out_clone = Arc::clone(&output_mutex);
                let ctx = ui.ctx().clone();
                std::thread::spawn(move || {
                    let res = execute_fn(&code_clone);
                    *out_clone.lock().unwrap() = res;
                    ctx.request_repaint();
                });
            }
        });

        let output_text = output_mutex.lock().unwrap().clone();
        if !output_text.is_empty() {
            ui.add_space(10.0);
            let mut out_frame = egui::Frame::new();
            out_frame.fill = egui::Color32::from_rgb(8, 11, 16);
            out_frame.inner_margin = egui::Margin::same(12);
            out_frame.corner_radius = egui::CornerRadius::same(6);
            out_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));
            out_frame.show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                if output_text == "Ejecutando..." || output_text == "Compilando..." {
                    ui.label(
                        egui::RichText::new(output_text)
                            .color(egui::Color32::YELLOW)
                            .monospace(),
                    );
                } else if let Some(idx) = output_text.find("[Errores/Warnings]:\n") {
                    let (stdout, stderr) = output_text.split_at(idx);
                    if !stdout.is_empty() {
                        ui.label(formatear_salida_consola(stdout, false));
                        ui.add_space(5.0);
                        ui.separator();
                        ui.add_space(5.0);
                    }
                    let solo_error = stderr
                        .strip_prefix("[Errores/Warnings]:\n")
                        .unwrap_or(stderr);
                    ui.label(formatear_salida_consola(solo_error, true));
                } else if output_text.starts_with("Error") {
                    ui.label(formatear_salida_consola(&output_text, true));
                } else {
                    ui.label(formatear_salida_consola(&output_text, false));
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntect_reconoce_toml_con_la_sintaxis_de_ferriskey() {
        let syntax_set = cargar_syntax_set();
        let syntax = syntax_set
            .find_syntax_by_extension("toml")
            .expect("FerrisKey debe incluir resaltado TOML");

        assert_eq!(syntax.name, "TOML");
    }

    #[test]
    fn toml_aplica_varios_colores_de_la_paleta_del_editor() {
        use std::collections::HashSet;
        use syntect::easy::HighlightLines;
        use syntect::highlighting::ThemeSet;

        let syntax_set = cargar_syntax_set();
        let syntax = syntax_set
            .find_syntax_by_extension("toml")
            .expect("FerrisKey debe incluir resaltado TOML");
        let theme_set = ThemeSet::load_defaults();
        let theme = &theme_set.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax, theme);
        let ranges = highlighter
            .highlight_line(
                "[profile.release]\nopt-level = 3\nstrip = \"symbols\"\n",
                &syntax_set,
            )
            .expect("el resaltado TOML debe ser válido");
        let colores: HashSet<_> = ranges.iter().map(|(style, _)| style.foreground).collect();

        assert!(colores.len() >= 3, "TOML debe mostrar colores semánticos");
    }

    #[test]
    fn cache_reutiliza_el_resaltado_estatico_entre_frames() {
        let syntax_set = cargar_syntax_set();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = &theme_set.themes["base16-ocean.dark"];
        let source = "let ferriskey_cache_test: u32 = 42;";

        {
            let mut cache = highlight_cache()
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            cache.entries.clear();
            cache.computations = 0;
        }

        for _ in 0..120 {
            let job = cached_highlight_job(source, &syntax_set, theme, "rs", 12.0);
            assert_eq!(job.text, source);
        }

        let cache = highlight_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        assert_eq!(cache.computations, 1);
        assert_eq!(cache.entries.len(), 1);
    }
}
