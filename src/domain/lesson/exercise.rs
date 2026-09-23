use serde::{Deserialize, Serialize};

/// Nivel de dificultad pedagógica de un ejercicio o lección.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum DifficultyLevel {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl DifficultyLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Beginner => "Principiante",
            Self::Intermediate => "Intermedio",
            Self::Advanced => "Avanzado",
            Self::Expert => "Experto",
        }
    }
}

/// Ejercicio o reto interactivo asociado a una lección.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    pub prompt: String,
    pub starter_code: String,
    pub solution_code: Option<String>,
    pub hints: Vec<String>,
    pub difficulty: DifficultyLevel,
}

impl Exercise {
    pub fn new(
        prompt: impl Into<String>,
        starter_code: impl Into<String>,
        difficulty: DifficultyLevel,
    ) -> Self {
        Self {
            prompt: prompt.into(),
            starter_code: starter_code.into(),
            solution_code: None,
            hints: Vec::new(),
            difficulty,
        }
    }

    pub fn with_solution(mut self, solution: impl Into<String>) -> Self {
        self.solution_code = Some(solution.into());
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }
}
