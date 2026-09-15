use std::path::{Path, PathBuf};

use crate::errors::{AppResult, FerrisKeyError};

pub fn selected_file_path(project_dir: &Path, selected_file: Option<&str>) -> AppResult<PathBuf> {
    if let Some(relative) = selected_file {
        let relative_path = Path::new(relative);
        if relative_path.is_absolute()
            || relative_path
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
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

#[cfg(test)]
mod tests {
    use super::selected_file_path;

    #[test]
    fn selects_the_main_file_by_default() {
        let path = selected_file_path(std::path::Path::new("/tmp/demo"), None)
            .expect("default project file");
        assert_eq!(path, std::path::PathBuf::from("/tmp/demo/src/main.rs"));
    }

    #[test]
    fn rejects_paths_outside_the_project() {
        let result = selected_file_path(std::path::Path::new("/tmp/demo"), Some("../secret"));
        assert!(result.is_err());
    }
}
