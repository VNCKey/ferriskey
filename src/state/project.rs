use std::sync::{Arc, Mutex};

pub struct ProjectState {
    pub shared_project_code: String,
    pub shared_project_output: Arc<Mutex<String>>,
    pub created_project_name: Option<String>,
    pub selected_project: Option<String>,
    pub selected_file: Option<String>,
}

impl Default for ProjectState {
    fn default() -> Self {
        Self {
            shared_project_code: String::new(),
            shared_project_output: Arc::new(Mutex::new(String::new())),
            created_project_name: None,
            selected_project: None,
            selected_file: None,
        }
    }
}
