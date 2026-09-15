use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use crate::errors::FerrisKeyError;

#[derive(Clone, Default)]
pub struct TaskManager {
    active_tasks: Arc<AtomicUsize>,
}

impl TaskManager {
    pub fn spawn<F>(&self, name: &'static str, task: F) -> Result<(), FerrisKeyError>
    where
        F: FnOnce() + Send + 'static,
    {
        let active_tasks = Arc::clone(&self.active_tasks);
        active_tasks.fetch_add(1, Ordering::Relaxed);

        let result = thread::Builder::new().name(name.to_owned()).spawn(move || {
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
            if outcome.is_err() {
                tracing::error!(
                    task = name,
                    "Una tarea en segundo plano terminó por un panic"
                );
            }
            active_tasks.fetch_sub(1, Ordering::Relaxed);
        });

        if let Err(source) = result {
            self.active_tasks.fetch_sub(1, Ordering::Relaxed);
            return Err(FerrisKeyError::TaskStart {
                name: name.to_owned(),
                source,
            });
        }

        Ok(())
    }

    pub fn is_busy(&self) -> bool {
        self.active_tasks.load(Ordering::Relaxed) > 0
    }
}
