use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::errors::{AppResult, FerrisKeyError};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub selected_project: Option<String>,
    pub selected_file: Option<String>,
}

fn session_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "FerrisKey", "FerrisKey")
        .map(|dirs| dirs.config_dir().join("session.json"))
}

pub fn load_session() -> AppResult<SessionSnapshot> {
    let Some(path) = session_path() else {
        return Ok(SessionSnapshot::default());
    };
    if !path.is_file() {
        return Ok(SessionSnapshot::default());
    }
    let content = std::fs::read_to_string(path)
        .map_err(|source| FerrisKeyError::io("leer la sesión guardada", source))?;
    serde_json::from_str(&content).map_err(|error| FerrisKeyError::Serialization(error.to_string()))
}

pub fn save_session(snapshot: &SessionSnapshot) -> AppResult<()> {
    let Some(path) = session_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|source| FerrisKeyError::io("crear la carpeta de configuración", source))?;
    }
    let content = serde_json::to_string_pretty(snapshot)
        .map_err(|error| FerrisKeyError::Serialization(error.to_string()))?;
    std::fs::write(path, content)
        .map_err(|source| FerrisKeyError::io("guardar la sesión", source))?;
    Ok(())
}
