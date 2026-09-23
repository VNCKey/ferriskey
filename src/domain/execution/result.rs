use serde::{Deserialize, Serialize};

use super::diagnostic::CompilerDiagnostic;

/// Resultado final de la ejecución de código Rust.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ExecutionOutcome {
    #[default]
    Success,
    CompileError,
    RuntimeError,
    TimedOut,
}

/// Modelo de Dominio que encapsula el resultado completo de un proceso de compilación y corrida.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub outcome: ExecutionOutcome,
    pub diagnostics: Vec<CompilerDiagnostic>,
    pub duration_ms: u64,
}

impl ExecutionResult {
    pub fn success(stdout: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            stdout: stdout.into(),
            stderr: String::new(),
            exit_code: Some(0),
            outcome: ExecutionOutcome::Success,
            diagnostics: Vec::new(),
            duration_ms,
        }
    }

    pub fn compile_error(stderr: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            stdout: String::new(),
            stderr: stderr.into(),
            exit_code: Some(1),
            outcome: ExecutionOutcome::CompileError,
            diagnostics: Vec::new(),
            duration_ms,
        }
    }

    pub fn is_successful(&self) -> bool {
        self.outcome == ExecutionOutcome::Success
    }

    pub fn full_output(&self) -> String {
        if self.stderr.is_empty() {
            self.stdout.clone()
        } else if self.stdout.is_empty() {
            self.stderr.clone()
        } else {
            format!("{}\n{}", self.stdout, self.stderr)
        }
    }
}
