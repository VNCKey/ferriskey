use serde::{Deserialize, Serialize};

/// Estadísticas de la sesión de estudio actual.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SessionStats {
    pub executions_count: u32,
    pub successful_runs: u32,
    pub failed_runs: u32,
    pub time_spent_seconds: u64,
}

impl SessionStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_execution(&mut self, success: bool) {
        self.executions_count += 1;
        if success {
            self.successful_runs += 1;
        } else {
            self.failed_runs += 1;
        }
    }

    pub fn add_time(&mut self, seconds: u64) {
        self.time_spent_seconds += seconds;
    }

    pub fn success_rate(&self) -> f32 {
        if self.executions_count == 0 {
            return 0.0;
        }
        (self.successful_runs as f32 / self.executions_count as f32) * 100.0
    }
}
