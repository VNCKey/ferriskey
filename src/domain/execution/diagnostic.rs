use serde::{Deserialize, Serialize};

/// Nivel de severidad de un diagnóstico emitido por el compilador `rustc` o clippy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DiagnosticLevel {
    #[default]
    Error,
    Warning,
    Note,
    Help,
}

impl DiagnosticLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
        }
    }
}

/// Diagnóstico estructurado de compilación.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompilerDiagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub code_suggestion: Option<String>,
}

impl CompilerDiagnostic {
    pub fn new(level: DiagnosticLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            line: None,
            column: None,
            code_suggestion: None,
        }
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.code_suggestion = Some(suggestion.into());
        self
    }
}
