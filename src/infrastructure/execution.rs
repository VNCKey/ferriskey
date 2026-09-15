use serde_json::{Value, json};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_RUN_ID: AtomicU64 = AtomicU64::new(0);

struct TemporaryRun {
    directory: PathBuf,
}

impl TemporaryRun {
    fn create() -> std::io::Result<Self> {
        let sequence = NEXT_RUN_ID.fetch_add(1, Ordering::Relaxed);
        let directory =
            std::env::temp_dir().join(format!("ferriskey-{}-{sequence}", std::process::id()));
        std::fs::create_dir(&directory)?;
        Ok(Self { directory })
    }

    fn path(&self, name: &str) -> PathBuf {
        self.directory.join(name)
    }
}

impl Drop for TemporaryRun {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

pub(crate) fn ejecutar_codigo_rust(codigo: &str) -> String {
    match ejecutar_codigo_rust_inner(codigo) {
        Ok(output) => output,
        Err(error) => format!("Error preparando la ejecución: {error}"),
    }
}

#[allow(dead_code)]
pub(crate) fn expandir_macros_rust(codigo: &str) -> String {
    // 1. Intentar cargo expand local si existe
    if let Ok(run) = TemporaryRun::create() {
        let _ = Command::new("cargo")
            .args(["init", "--bin", "--quiet"])
            .current_dir(&run.directory)
            .output();

        let main_path = run.directory.join("src/main.rs");
        let _ = std::fs::write(&main_path, codigo);

        if let Ok(output) = Command::new("cargo")
            .arg("expand")
            .current_dir(&run.directory)
            .output()
            && output.status.success()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                return format!("[Expansión de Macros vía cargo expand]:\n\n{}", stdout);
            }
        }
    }

    // 2. Intentar Rust Playground macro-expansion API
    let payload = json!({
        "code": codigo,
        "edition": "2021"
    });

    if let Ok(response) = post_json("https://play.rust-lang.org/macro-expansion", &payload)
        && let Ok(parsed) = serde_json::from_str::<Value>(&response)
        && let Some(expanded) = parsed["stdout"].as_str()
        && !expanded.trim().is_empty()
    {
        return format!("[Expansión de Macros vía Rust Playground]:\n\n{}", expanded);
    }

    // 3. Fallback didáctico offline
    expandir_macros_didactico(codigo)
}

#[allow(dead_code)]
fn expandir_macros_didactico(codigo: &str) -> String {
    let mut resultado = codigo.to_string();

    if resultado.contains("println!") {
        resultado = resultado.replace("println!(\"", "std::io::_print(format_args!(\"");
        resultado = resultado.replace("println!(", "std::io::_print(format_args!(");
    }
    if resultado.contains("eprintln!") {
        resultado = resultado.replace("eprintln!(\"", "std::io::_eprint(format_args!(\"");
        resultado = resultado.replace("eprintln!(", "std::io::_eprint(format_args!(");
    }
    if resultado.contains("format!") {
        resultado = resultado.replace("format!", "alloc::fmt::format");
    }
    if resultado.contains("vec!") {
        resultado = resultado.replace("vec!", "alloc::vec::Vec::from");
    }
    if resultado.contains("panic!") {
        resultado = resultado.replace("panic!", "std::rt::begin_panic");
    }

    format!(
        "[Expansión Didáctica de Macros (Sin cargo-expand instalado)]:\n\n{}\n\n💡 Nota: Para obtener la AST completa desglosada por rustc/LLVM, puedes instalar la herramienta oficial ejecutando en tu terminal:\ncargo install cargo-expand",
        resultado
    )
}

#[allow(dead_code)]
pub(crate) fn ejecutar_codigo_cargo_run(
    codigo: &str,
    project_dir: Option<&std::path::Path>,
) -> String {
    if let Some(dir) = project_dir
        && dir.exists()
    {
        let main_rs = dir.join("src/main.rs");
        let lib_rs = dir.join("src/lib.rs");
        let target_file = if main_rs.exists() {
            main_rs
        } else if lib_rs.exists() {
            lib_rs
        } else {
            main_rs
        };

        if let Some(parent) = target_file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Err(e) = std::fs::write(&target_file, codigo) {
            return format!("Error guardando cambios en disco: {e}");
        }

        let output = match Command::new("cargo").arg("run").current_dir(dir).output() {
            Ok(out) => out,
            Err(e) => return format!("Error ejecutando 'cargo run': {e}"),
        };

        return format_cargo_process_output(
            &output.stdout,
            &output.stderr,
            output.status.success(),
        );
    }

    // Fallback: Si no hay directorio físico válido seleccionado, crear plantilla Cargo temp y ejecutar `cargo run`
    let run = match TemporaryRun::create() {
        Ok(r) => r,
        Err(e) => return format!("Error creando directorio temporal: {e}"),
    };

    let init_res = Command::new("cargo")
        .args(["init", "--bin", "--quiet"])
        .current_dir(&run.directory)
        .output();

    if let Err(e) = init_res {
        return format!("Error inicializando proyecto cargo temporal: {e}");
    }

    let main_path = run.directory.join("src/main.rs");
    if let Err(e) = std::fs::write(&main_path, codigo) {
        return format!("Error escribiendo código fuente: {e}");
    }

    let output = match Command::new("cargo")
        .arg("run")
        .current_dir(&run.directory)
        .output()
    {
        Ok(out) => out,
        Err(e) => return format!("Error ejecutando 'cargo run': {e}"),
    };

    format_cargo_process_output(&output.stdout, &output.stderr, output.status.success())
}

fn format_cargo_process_output(stdout: &[u8], stderr: &[u8], success: bool) -> String {
    let stdout_str = String::from_utf8_lossy(stdout);
    let stderr_str = String::from_utf8_lossy(stderr);
    let mut result = String::new();

    if !stderr_str.is_empty() {
        result.push_str(&stderr_str);
        if !result.ends_with('\n') {
            result.push('\n');
        }
    }

    if !stdout_str.is_empty() {
        result.push_str(&stdout_str);
    }

    if result.trim().is_empty() {
        if success {
            "El programa terminó exitosamente sin salidas.".to_owned()
        } else {
            "El proceso terminó con error sin salidas adicionales.".to_owned()
        }
    } else {
        result
    }
}

fn ejecutar_codigo_rust_inner(codigo: &str) -> std::io::Result<String> {
    let run = TemporaryRun::create()?;
    let source_path = run.path("main.rs");
    let executable_path = run.path(if cfg!(target_os = "windows") {
        "programa.exe"
    } else {
        "programa"
    });
    std::fs::write(&source_path, codigo)?;

    let compilation = Command::new("rustc")
        .arg(&source_path)
        .arg("-o")
        .arg(&executable_path)
        .output()?;

    if !compilation.status.success() {
        return Ok(format!(
            "[Errores/Warnings]:\n{}",
            String::from_utf8_lossy(&compilation.stderr)
        ));
    }

    let output = Command::new(&executable_path).output()?;
    Ok(format_process_output(&output.stdout, &output.stderr))
}

fn format_process_output(stdout: &[u8], stderr: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(stdout).into_owned();
    let stderr = String::from_utf8_lossy(stderr);

    if !stderr.is_empty() {
        if !result.is_empty() {
            result.push_str("\n\n");
        }
        result.push_str("[Errores/Warnings]:\n");
        result.push_str(&stderr);
    }

    if result.is_empty() {
        "El programa terminó sin salidas.".to_owned()
    } else {
        result
    }
}

pub(crate) fn ejecutar_codigo_api(codigo: &str) -> String {
    let payload = json!({
        "channel": "stable",
        "mode": "debug",
        "edition": "2021",
        "crateType": "bin",
        "tests": false,
        "code": codigo,
        "backtrace": false,
    });

    match post_json("https://play.rust-lang.org/execute", &payload) {
        Ok(response) => parse_playground_response(response.as_bytes()),
        Err(error) => format!("Error comunicando con Rust Playground: {error}"),
    }
}

fn post_json(endpoint: &str, payload: &Value) -> Result<String, String> {
    let mut response = ureq::post(endpoint)
        .send_json(payload)
        .map_err(|error| error.to_string())?;

    response
        .body_mut()
        .read_to_string()
        .map_err(|error| error.to_string())
}

fn parse_playground_response(response: &[u8]) -> String {
    let parsed: Value = match serde_json::from_slice(response) {
        Ok(parsed) => parsed,
        Err(error) => {
            return format!(
                "Error interpretando la respuesta del servidor: {error}\n{}",
                String::from_utf8_lossy(response)
            );
        }
    };

    let stdout = parsed["stdout"].as_str().unwrap_or_default();
    let stderr = parsed["stderr"].as_str().unwrap_or_default();
    format_process_output(stdout.as_bytes(), stderr.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_stdout_and_stderr() {
        assert_eq!(
            format_process_output(b"hola\n", b"warning: ejemplo\n"),
            "hola\n\n\n[Errores/Warnings]:\nwarning: ejemplo\n"
        );
    }

    #[test]
    fn reports_programs_without_output() {
        assert_eq!(
            format_process_output(b"", b""),
            "El programa terminó sin salidas."
        );
    }

    #[test]
    fn parses_escaped_playground_output() {
        let response = br#"{"success":true,"stdout":"hola \"Rust\"\n","stderr":""}"#;
        assert_eq!(parse_playground_response(response), "hola \"Rust\"\n");
    }

    #[test]
    fn temporary_runs_use_distinct_directories_and_clean_up() {
        let first = TemporaryRun::create().expect("se crea el primer directorio");
        let second = TemporaryRun::create().expect("se crea el segundo directorio");
        assert_ne!(first.directory, second.directory);

        let first_path = first.directory.clone();
        assert!(first_path.is_dir());
        drop(first);
        assert!(!first_path.exists());
    }
}
