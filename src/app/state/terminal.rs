use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use crate::application::task_manager::TaskManager;

pub struct TerminalState {
    pub term_input: String,
    pub term_history: Arc<Mutex<Vec<String>>>,
    pub term_cwd: PathBuf,
    pub show_terminal_history: bool,
    pub term_command_running: Arc<AtomicBool>,
    pub term_pending_project: Arc<Mutex<Option<(PathBuf, String, bool)>>>,
    pub term_pending_manifest: Arc<Mutex<Option<PathBuf>>>,
    pub task_manager: TaskManager,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            term_input: String::new(),
            term_history: Arc::new(Mutex::new(Vec::new())),
            term_cwd: crate::platform::initial_terminal_dir(),
            show_terminal_history: false,
            term_command_running: Arc::new(AtomicBool::new(false)),
            term_pending_project: Arc::new(Mutex::new(None)),
            term_pending_manifest: Arc::new(Mutex::new(None)),
            task_manager: TaskManager::default(),
        }
    }
}
