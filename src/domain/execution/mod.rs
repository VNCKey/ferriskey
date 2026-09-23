#![allow(unused_imports, dead_code)]

pub mod diagnostic;
pub mod result;

pub use diagnostic::{CompilerDiagnostic, DiagnosticLevel};
pub use result::{ExecutionOutcome, ExecutionResult};
