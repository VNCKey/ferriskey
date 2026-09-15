use std::path::{Path, PathBuf};

use crate::domain::project::selected_file_path;
use crate::errors::{AppResult, FerrisKeyError};

pub struct ProjectService;

impl ProjectService {
    pub fn resolve_project_dir(base_path: &Path, project_name: &str) -> PathBuf {
        if base_path.file_name().and_then(|name| name.to_str()) == Some(project_name)
            && base_path.join("Cargo.toml").is_file()
        {
            return base_path.to_path_buf();
        }

        let candidate = base_path.join(project_name);
        if candidate.exists() {
            return candidate;
        }

        let repositories = repositories_dir(base_path);
        let candidate = repositories.join(project_name);
        if candidate.exists() {
            return candidate;
        }

        if let Ok(entries) = std::fs::read_dir(&repositories) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                if path.file_name().is_some_and(|name| name == project_name) {
                    return path;
                }
                let nested = path.join(project_name);
                if nested.exists() {
                    return nested;
                }
            }
        }

        base_path.join(project_name)
    }

    pub fn read_file(path: &Path) -> AppResult<String> {
        std::fs::read_to_string(path)
            .map_err(|source| FerrisKeyError::io("leer el archivo", source))
    }

    pub fn load_selected_file(
        base_path: &Path,
        project_name: &str,
        selected_file: Option<&str>,
    ) -> AppResult<(PathBuf, String)> {
        let project_dir = Self::resolve_project_dir(base_path, project_name);
        if !project_dir.is_dir() {
            return Err(FerrisKeyError::ProjectNotFound(project_dir));
        }

        let file_path = selected_file_path(&project_dir, selected_file)?;
        let content = Self::read_file(&file_path)?;
        Ok((file_path, content))
    }

    pub fn save_selected_file(
        base_path: &Path,
        project_name: &str,
        selected_file: Option<&str>,
        content: &str,
    ) -> AppResult<PathBuf> {
        let project_dir = Self::resolve_project_dir(base_path, project_name);
        if !project_dir.is_dir() {
            return Err(FerrisKeyError::ProjectNotFound(project_dir));
        }

        let file_path = selected_file_path(&project_dir, selected_file)?;
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|source| FerrisKeyError::io("crear la carpeta del archivo", source))?;
        }
        std::fs::write(&file_path, content)
            .map_err(|source| FerrisKeyError::io("guardar el archivo", source))?;
        Ok(file_path)
    }

    pub fn repositories_dir(base_path: &Path) -> PathBuf {
        repositories_dir(base_path)
    }

    pub fn list_projects(base_path: &Path) -> Vec<String> {
        let mut projects = Vec::new();
        let mut directories = vec![repositories_dir(base_path)];
        if base_path.is_dir() && !directories.contains(&base_path.to_path_buf()) {
            directories.push(base_path.to_path_buf());
        }
        if let Some(parent) = base_path.parent()
            && !directories.contains(&parent.to_path_buf())
        {
            directories.push(parent.to_path_buf());
        }

        for directory in directories {
            let Ok(entries) = std::fs::read_dir(directory) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir()
                    && is_cargo_project(&path)
                    && let Some(name) = path.file_name().and_then(|name| name.to_str())
                    && !projects.iter().any(|project| project == name)
                {
                    projects.push(name.to_owned());
                }
            }
        }
        projects.sort();
        projects
    }

    pub fn list_files(project_dir: &Path) -> Vec<String> {
        let mut files = Vec::new();
        scan_files(project_dir, project_dir, &mut files);
        files.sort();
        files
    }
}

fn repositories_dir(base_path: &Path) -> PathBuf {
    let projects_dir = crate::platform::projects_dir();
    if base_path == projects_dir || base_path.starts_with(&projects_dir) {
        return projects_dir;
    }
    if base_path.join("Cargo.toml").is_file() {
        return base_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| base_path.to_path_buf());
    }
    if base_path.is_dir() {
        base_path.to_path_buf()
    } else {
        crate::platform::ensure_projects_dir()
    }
}

fn is_cargo_project(path: &Path) -> bool {
    path.join("Cargo.toml").is_file()
        || path.join("src/main.rs").is_file()
        || path.join("src/lib.rs").is_file()
}

fn scan_files(root: &Path, current: &Path, results: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(current) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if name == ".git" || name == "target" || name == "Cargo.lock" {
            continue;
        }
        if path.is_dir() {
            scan_files(root, &path, results);
        } else if let Ok(relative) = path.strip_prefix(root) {
            results.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}
