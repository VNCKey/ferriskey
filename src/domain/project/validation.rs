use std::path::{Component, Path, PathBuf};

use crate::errors::{AppResult, FerrisKeyError};

/// Valida y resuelve con seguridad la ruta a un archivo dentro del directorio de un proyecto,
/// previniendo ataques de escalada de directorios (Path Traversal).
pub fn selected_file_path(project_dir: &Path, selected_file: Option<&str>) -> AppResult<PathBuf> {
    if let Some(relative) = selected_file {
        let relative_path = Path::new(relative);
        if relative_path.is_absolute()
            || relative_path
                .components()
                .any(|component| matches!(component, Component::ParentDir))
        {
            return Err(FerrisKeyError::InvalidProjectFile(
                relative_path.to_path_buf(),
            ));
        }
        return Ok(project_dir.join(relative_path));
    }

    let main_rs = project_dir.join("src/main.rs");
    let lib_rs = project_dir.join("src/lib.rs");
    if main_rs.is_file() {
        Ok(main_rs)
    } else if lib_rs.is_file() {
        Ok(lib_rs)
    } else {
        Ok(main_rs)
    }
}

/// Alias idiomático para validación segura de rutas de proyectos.
pub fn validate_safe_file_path(project_dir: &Path, file_path: Option<&str>) -> AppResult<PathBuf> {
    selected_file_path(project_dir, file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_the_main_file_by_default() {
        let path = selected_file_path(Path::new("/tmp/demo"), None).expect("default project file");
        assert_eq!(path, PathBuf::from("/tmp/demo/src/main.rs"));
    }

    #[test]
    fn accepts_valid_relative_paths() {
        let path = selected_file_path(Path::new("/tmp/demo"), Some("src/utils.rs"))
            .expect("valid relative path");
        assert_eq!(path, PathBuf::from("/tmp/demo/src/utils.rs"));
    }

    #[test]
    fn rejects_paths_outside_the_project() {
        let result = selected_file_path(Path::new("/tmp/demo"), Some("../secret"));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_absolute_paths_as_relative_selection() {
        let result = selected_file_path(Path::new("/tmp/demo"), Some("/etc/passwd"));
        assert!(result.is_err());
    }
}
