use std::fmt;
use std::io;
use std::path::PathBuf;

pub type AppResult<T> = Result<T, FerrisKeyError>;

#[derive(Debug)]
pub enum FerrisKeyError {
    Io {
        operation: &'static str,
        source: io::Error,
    },
    ProjectNotFound(PathBuf),
    InvalidProjectFile(PathBuf),
    TaskStart {
        name: String,
        source: io::Error,
    },
    Serialization(String),
}

impl FerrisKeyError {
    pub fn io(operation: &'static str, source: io::Error) -> Self {
        Self::Io { operation, source }
    }
}

impl fmt::Display for FerrisKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { operation, source } => {
                write!(formatter, "No se pudo {operation}: {source}")
            }
            Self::ProjectNotFound(path) => {
                write!(formatter, "No se encontró el proyecto: {}", path.display())
            }
            Self::InvalidProjectFile(path) => write!(
                formatter,
                "El archivo seleccionado no es válido para este proyecto: {}",
                path.display()
            ),
            Self::TaskStart { name, source } => {
                write!(formatter, "No se pudo iniciar la tarea {name}: {source}")
            }
            Self::Serialization(message) => {
                write!(formatter, "No se pudo guardar la sesión: {message}")
            }
        }
    }
}

impl std::error::Error for FerrisKeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } | Self::TaskStart { source, .. } => Some(source),
            _ => None,
        }
    }
}
