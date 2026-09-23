use std::collections::HashSet;
use serde::{Deserialize, Serialize};

/// Estado de progreso y logros del estudiante en la plataforma.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserProgress {
    pub completed_lessons: HashSet<String>,
    pub completed_exercises: HashSet<String>,
    pub bookmarked_lessons: HashSet<String>,
    pub current_streak_days: u32,
    pub total_points: u32,
    pub last_active_date: Option<String>,
}

impl UserProgress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_lesson_completed(&mut self, lesson_id: impl Into<String>) {
        let id = lesson_id.into();
        if self.completed_lessons.insert(id) {
            self.total_points += 10;
        }
    }

    pub fn is_lesson_completed(&self, lesson_id: &str) -> bool {
        self.completed_lessons.contains(lesson_id)
    }

    pub fn mark_exercise_completed(&mut self, exercise_id: impl Into<String>) {
        let id = exercise_id.into();
        if self.completed_exercises.insert(id) {
            self.total_points += 25;
        }
    }

    pub fn toggle_bookmark(&mut self, lesson_id: impl Into<String>) -> bool {
        let id = lesson_id.into();
        if self.bookmarked_lessons.contains(&id) {
            self.bookmarked_lessons.remove(&id);
            false
        } else {
            self.bookmarked_lessons.insert(id);
            true
        }
    }

    pub fn completion_percentage(&self, total_lessons_count: usize) -> f32 {
        if total_lessons_count == 0 {
            return 0.0;
        }
        (self.completed_lessons.len() as f32 / total_lessons_count as f32) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_progress_and_points() {
        let mut progress = UserProgress::new();
        assert_eq!(progress.total_points, 0);

        progress.mark_lesson_completed("datatypes");
        assert!(progress.is_lesson_completed("datatypes"));
        assert_eq!(progress.total_points, 10);

        // No duplicar puntos
        progress.mark_lesson_completed("datatypes");
        assert_eq!(progress.total_points, 10);

        assert_eq!(progress.completion_percentage(2), 50.0);
    }
}
