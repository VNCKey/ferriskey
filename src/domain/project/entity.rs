use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use super::file::ProjectFile;

/// Tipo o propósito del proyecto Rust en el espacio de trabajo.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProjectKind {
    #[default]
    Binary,
    Library,
    Custom,
}

impl ProjectKind {
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary)
    }

    pub fn is_library(&self) -> bool {
        matches!(self, Self::Library)
    }
}

/// Entidad central del Dominio que representa un Proyecto Rust en el espacio de trabajo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub root_path: PathBuf,
    pub kind: ProjectKind,
    pub files: Vec<ProjectFile>,
}

impl Project {
    pub fn new(name: impl Into<String>, root_path: impl Into<PathBuf>, kind: ProjectKind) -> Self {
        Self {
            name: name.into(),
            root_path: root_path.into(),
            kind,
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: ProjectFile) {
        if !self.files.iter().any(|f| f.relative_path == file.relative_path) {
            self.files.push(file);
        }
    }

    pub fn find_entrypoint(&self) -> Option<&ProjectFile> {
        self.files.iter().find(|f| f.is_entrypoint)
    }

    pub fn get_file(&self, relative_path: &str) -> Option<&ProjectFile> {
        self.files.iter().find(|f| f.relative_path == relative_path)
    }

    pub fn get_file_mut(&mut self, relative_path: &str) -> Option<&mut ProjectFile> {
        self.files.iter_mut().find(|f| f.relative_path == relative_path)
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.root_path.join("Cargo.toml")
    }

    pub fn has_manifest(&self) -> bool {
        self.manifest_path().is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_project_with_correct_kind() {
        let mut proj = Project::new("ferris_demo", "/tmp/ferris_demo", ProjectKind::Binary);
        assert_eq!(proj.name, "ferris_demo");
        assert!(proj.kind.is_binary());
        assert_eq!(proj.manifest_path(), PathBuf::from("/tmp/ferris_demo/Cargo.toml"));

        proj.add_file(ProjectFile::new_entrypoint("src/main.rs", "/tmp/ferris_demo/src/main.rs"));
        assert!(proj.find_entrypoint().is_some());
    }
}
