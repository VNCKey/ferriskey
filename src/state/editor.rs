use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use crate::components::code_editor::cargar_syntax_set;

pub struct EditorState {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub project_editor_path: Option<String>,
    pub project_editor_code: String,
    pub project_editor_status: String,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            syntax_set: cargar_syntax_set(),
            theme_set: ThemeSet::load_defaults(),
            project_editor_path: None,
            project_editor_code: String::new(),
            project_editor_status: String::new(),
        }
    }
}
