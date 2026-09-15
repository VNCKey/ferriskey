use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Devuelve la carpeta visible donde FerrisKey guardará los proyectos creados
/// desde su terminal. `directories` elige Documentos y el separador correcto
/// para Linux, macOS y Windows.
pub fn projects_dir() -> PathBuf {
    let root = directories::UserDirs::new()
        .and_then(|dirs| dirs.document_dir().map(Path::to_path_buf))
        .or_else(|| directories::BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf()))
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));

    root.join("FerrisKey").join("Projects")
}

/// Crea la carpeta de proyectos si es posible y devuelve su ruta.
pub fn ensure_projects_dir() -> PathBuf {
    let projects = projects_dir();
    let _ = std::fs::create_dir_all(&projects);
    projects
}

/// Devuelve el directorio personal del usuario con el convenio del sistema.
pub fn home_dir() -> PathBuf {
    directories::BaseDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Selecciona el directorio inicial de la terminal usando la misma ruta en
/// desarrollo y producción: `Documentos/FerrisKey/Projects`.
pub fn initial_terminal_dir() -> PathBuf {
    let projects = ensure_projects_dir();
    if projects.is_dir() {
        projects
    } else {
        env::current_dir().unwrap_or(projects)
    }
}

/// Construye el shell apropiado para el sistema operativo actual.
///
/// En sistemas Unix respeta el shell configurado por el usuario (`$SHELL`) y
/// usa `sh` como respaldo. En Windows usa el PowerShell incluido en el sistema.
pub fn shell_command(command: &str) -> Command {
    #[cfg(target_os = "windows")]
    {
        let mut process = Command::new("powershell.exe");
        process.args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            command,
        ]);
        process
    }

    #[cfg(not(target_os = "windows"))]
    {
        let shell = env::var_os("SHELL").unwrap_or_else(|| OsString::from("sh"));
        let mut process = Command::new(shell);
        process.args(["-c", command]);
        process
    }
}

/// Construye un prompt que representa al usuario, equipo y ruta reales.
pub fn terminal_prompt(cwd: &Path) -> String {
    let user = env::var("USERNAME")
        .or_else(|_| env::var("USER"))
        .unwrap_or_else(|_| "usuario".to_owned());
    let host = env::var("COMPUTERNAME")
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "equipo".to_owned());
    let path = display_path(cwd);
    let prompt_end = if cfg!(target_os = "windows") {
        ">"
    } else {
        "$"
    };

    format!("{user}@{host}:{path}{prompt_end}")
}

/// Devuelve la ruta del ejecutable que Cargo genera para un paquete binario.
/// Cargo añade `.exe` en Windows y no añade extensión en Unix/macOS.
pub fn cargo_binary_path(project_dir: &Path, profile: &str, package_name: &str) -> PathBuf {
    let executable_name = if cfg!(target_os = "windows") {
        format!("{package_name}.exe")
    } else {
        package_name.to_owned()
    };

    project_dir
        .join("target")
        .join(profile)
        .join(executable_name)
}

fn display_path(path: &Path) -> String {
    let home = home_dir();

    if path == home {
        return "~".to_owned();
    }

    if let Ok(relative) = path.strip_prefix(&home) {
        if relative.as_os_str().is_empty() {
            "~".to_owned()
        } else {
            format!("~{}{}", std::path::MAIN_SEPARATOR, relative.display())
        }
    } else {
        path.display().to_string()
    }
}
