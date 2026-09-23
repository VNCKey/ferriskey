use serde::{Deserialize, Serialize};

use super::category::LessonCategory;
use super::exercise::Exercise;

/// Entidad central del Dominio que representa una Lección Interactiva de Rust.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub category: LessonCategory,
    pub initial_code: String,
    pub exercises: Vec<Exercise>,
    pub documentation_url: Option<String>,
}

impl Lesson {
    pub fn new(
        id: impl Into<String>,
        slug: impl Into<String>,
        title: impl Into<String>,
        category: LessonCategory,
        initial_code: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            slug: slug.into(),
            title: title.into(),
            summary: String::new(),
            category,
            initial_code: initial_code.into(),
            exercises: Vec::new(),
            documentation_url: None,
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = summary.into();
        self
    }

    pub fn with_doc_url(mut self, url: impl Into<String>) -> Self {
        self.documentation_url = Some(url.into());
        self
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::lesson::exercise::DifficultyLevel;

    #[test]
    fn creates_lesson_with_exercises() {
        let mut lesson = Lesson::new(
            "primitives_integers",
            "enteros",
            "Tipos de Datos Enteros",
            LessonCategory::Basics,
            "fn main() { let x: i32 = 42; }",
        );
        lesson.add_exercise(Exercise::new(
            "Crea una variable i64 con valor 100",
            "fn main() { // Tu codigo aqui\n}",
            DifficultyLevel::Beginner,
        ));

        assert_eq!(lesson.title, "Tipos de Datos Enteros");
        assert_eq!(lesson.exercises.len(), 1);
        assert_eq!(lesson.exercises[0].difficulty, DifficultyLevel::Beginner);
    }
}
