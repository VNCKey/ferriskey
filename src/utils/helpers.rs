use std::path::Path;

/// Trunca una cadena de texto a una longitud máxima dada, agregando "..." si supera el límite.
#[allow(dead_code)]
pub fn truncar_texto(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let mut truncado: String = s.chars().take(max_len.saturating_sub(3)).collect();
        truncado.push_str("...");
        truncado
    }
}

/// Formatea un número de bytes a una representación legible (B, KB, MB, GB).
#[allow(dead_code)]
pub fn formatear_tamano_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Verifica si la extensión de la ruta corresponde a un archivo ejecutable o fuente de Rust (.rs).
#[allow(dead_code)]
pub fn es_archivo_rust(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
}

/// Limpia secuencias ANSI de escape de una cadena de texto (como la salida formateada de terminal/rustc).
#[allow(dead_code)]
pub fn limpiar_codigos_ansi(texto: &str) -> String {
    let mut resultado = String::with_capacity(texto.len());
    let mut en_escape = false;

    for c in texto.chars() {
        if c == '\x1B' {
            en_escape = true;
        } else if en_escape {
            if c.is_ascii_alphabetic() || c == 'm' {
                en_escape = false;
            }
        } else {
            resultado.push(c);
        }
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncar_texto() {
        assert_eq!(truncar_texto("Hola mundo", 20), "Hola mundo");
        assert_eq!(truncar_texto("Hola mundo desde Rust", 10), "Hola mu...");
    }

    #[test]
    fn test_formatear_tamano_bytes() {
        assert_eq!(formatear_tamano_bytes(500), "500 B");
        assert_eq!(formatear_tamano_bytes(2048), "2.00 KB");
        assert_eq!(formatear_tamano_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_es_archivo_rust() {
        assert!(es_archivo_rust(Path::new("src/main.rs")));
        assert!(!es_archivo_rust(Path::new("Cargo.toml")));
    }
}
