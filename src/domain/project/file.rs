use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Representación en el dominio de un archivo individual dentro de un proyecto.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectFile {
    pub relative_path: String,
    pub full_path: PathBuf,
    pub is_entrypoint: bool,
    pub content: Option<String>,
}

impl ProjectFile {
    pub fn new(relative_path: impl Into<String>, full_path: impl Into<PathBuf>) -> Self {
        let rel = relative_path.into();
        let is_entry = rel == "src/main.rs" || rel == "src/lib.rs" || rel == "main.rs" || rel == "lib.rs";
        Self {
            relative_path: rel,
            full_path: full_path.into(),
            is_entrypoint: is_entry,
            content: None,
        }
    }

    pub fn new_entrypoint(relative_path: impl Into<String>, full_path: impl Into<PathBuf>) -> Self {
        Self {
            relative_path: relative_path.into(),
            full_path: full_path.into(),
            is_entrypoint: true,
            content: None,
        }
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn is_rust_source(&self) -> bool {
        self.relative_path.ends_with(".rs")
    }

    pub fn is_cargo_manifest(&self) -> bool {
        self.relative_path == "Cargo.toml"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_rust_source_and_entrypoint() {
        let file = ProjectFile::new("src/main.rs", "/tmp/demo/src/main.rs");
        assert!(file.is_entrypoint);
        assert!(file.is_rust_source());
        assert!(!file.is_cargo_manifest());

        let cargo = ProjectFile::new("Cargo.toml", "/tmp/demo/Cargo.toml");
        assert!(!cargo.is_entrypoint);
        assert!(cargo.is_cargo_manifest());
    }
}
