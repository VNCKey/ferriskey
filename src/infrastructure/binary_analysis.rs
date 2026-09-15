use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use object::{Object, ObjectSection};

#[derive(Clone, Copy, Debug)]
pub enum AnalysisFocus {
    Format,
    Sections,
    Header,
    Dependencies,
}

#[derive(Debug)]
struct SectionInfo {
    name: String,
    memory_size: u64,
    file_size: u64,
}

#[derive(Debug)]
struct BinaryAnalysis {
    path: PathBuf,
    file_size: u64,
    format: String,
    kind: String,
    architecture: String,
    address_width: String,
    endianness: String,
    entry: u64,
    has_debug_symbols: bool,
    sections: Vec<SectionInfo>,
    dependencies: Vec<String>,
}

/// Analiza un ejecutable sin invocar herramientas propias de un solo sistema.
/// `object` unifica ELF (Linux), Mach-O (macOS) y PE/COFF (Windows).
pub fn render_report(path: &Path, focus: AnalysisFocus) -> Result<String, String> {
    let analysis = analyze(path)?;
    Ok(analysis.render(focus))
}

fn analyze(path: &Path) -> Result<BinaryAnalysis, String> {
    let data = fs::read(path).map_err(|error| format!("No se pudo leer el binario: {error}"))?;
    let file_size = data.len() as u64;
    let file = object::File::parse(&*data)
        .map_err(|error| format!("El archivo no parece un ejecutable compatible: {error}"))?;

    let mut sections = Vec::new();
    for section in file.sections() {
        let name = section.name().unwrap_or("<sección sin nombre>").to_owned();
        let memory_size = section.size();
        let file_size = section.file_range().map(|(_, size)| size).unwrap_or(0);

        if memory_size > 0 || file_size > 0 {
            sections.push(SectionInfo {
                name,
                memory_size,
                file_size,
            });
        }
    }

    let mut dependencies = BTreeSet::new();
    if let Ok(imports) = file.imports() {
        for import in imports {
            let library = String::from_utf8_lossy(import.library()).into_owned();
            if !library.is_empty() {
                dependencies.insert(library);
            }
        }
    }

    Ok(BinaryAnalysis {
        path: path.to_path_buf(),
        file_size,
        format: format!("{:?}", file.format()),
        kind: format!("{:?}", file.kind()),
        architecture: format!("{:?}", file.architecture()),
        address_width: if file.is_64() { "64-bit" } else { "32-bit" }.to_owned(),
        endianness: if file.is_little_endian() {
            "Little-endian"
        } else {
            "Big-endian"
        }
        .to_owned(),
        entry: file.entry(),
        has_debug_symbols: file.has_debug_symbols(),
        sections,
        dependencies: dependencies.into_iter().collect(),
    })
}

impl BinaryAnalysis {
    fn render(&self, focus: AnalysisFocus) -> String {
        let mut output = vec![
            format!("Binario: {}", self.path.display()),
            format!("Tamaño en disco: {}", format_size(self.file_size)),
        ];

        match focus {
            AnalysisFocus::Format => {
                output.extend([
                    format!("Formato: {}", self.format),
                    format!("Tipo: {}", self.kind),
                    format!(
                        "Arquitectura: {} ({})",
                        self.architecture, self.address_width
                    ),
                    format!("Endianess: {}", self.endianness),
                ]);
            }
            AnalysisFocus::Sections => {
                output.push(format!("Secciones con contenido: {}", self.sections.len()));
                output.push("Nombre                         Memoria       Archivo".to_owned());
                output.push("────────────────────────────────────────────────────".to_owned());
                for section in self.sections.iter().take(24) {
                    output.push(format!(
                        "{:<28} {:>10} {:>13}",
                        truncate_name(&section.name, 28),
                        format_size(section.memory_size),
                        format_size(section.file_size),
                    ));
                }
                if self.sections.len() > 24 {
                    output.push(format!("… y {} secciones más", self.sections.len() - 24));
                }
            }
            AnalysisFocus::Header => {
                output.extend([
                    format!("Cabecera: {} {}", self.format, self.address_width),
                    format!("Arquitectura: {}", self.architecture),
                    format!("Punto de entrada: 0x{:x}", self.entry),
                    format!("Endianess: {}", self.endianness),
                    format!(
                        "Símbolos de depuración: {}",
                        if self.has_debug_symbols { "sí" } else { "no" }
                    ),
                ]);
            }
            AnalysisFocus::Dependencies => {
                output.push(format!(
                    "Bibliotecas importadas: {}",
                    self.dependencies.len()
                ));
                if self.dependencies.is_empty() {
                    output.push("No se encontraron bibliotecas dinámicas declaradas.".to_owned());
                } else {
                    output.extend(
                        self.dependencies
                            .iter()
                            .map(|library| format!("  {library}")),
                    );
                }
            }
        }

        output.join("\n")
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KiB", "MiB", "GiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:>8.2} {}", UNITS[unit])
}

fn truncate_name(name: &str, max_chars: usize) -> String {
    if name.chars().count() <= max_chars {
        return name.to_owned();
    }

    let mut truncated = name
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    truncated.push('…');
    truncated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_long_section_names() {
        assert_eq!(truncate_name("abcdefgh", 5), "abcd…");
        assert_eq!(truncate_name("rust", 5), "rust");
    }

    #[test]
    fn formats_binary_sizes() {
        assert_eq!(format_size(1024), "    1.00 KiB");
    }

    #[test]
    fn analyzes_the_current_test_executable() {
        let path = std::env::current_exe().expect("se obtiene el ejecutable de pruebas");
        let report = render_report(&path, AnalysisFocus::Format).expect("se analiza el ejecutable");
        assert!(report.contains("Formato:"));
        assert!(report.contains("Arquitectura:"));
    }
}
