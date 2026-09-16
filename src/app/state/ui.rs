use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::routes::AppRoute;

pub struct UiState {
    pub ruta_actual: AppRoute,
    pub mostrar_sidebar: bool,
    pub mostrar_nav_superior: bool,
    pub mostrar_status_bar: bool,
    pub tutorial_time: f64,
    pub anim_trigger: f64,
    pub show_code_modal: Option<(String, String)>,
    pub mostrar_explorer_drawer: bool,
    pub mostrar_console_drawer: bool,
    pub show_commands_modal: bool,
    pub show_macro_expansion: bool,
    pub show_cargo_output_modal: Arc<AtomicBool>,
    pub show_rustc_compilador_modal: bool,
    pub show_terminal_modal: bool,
    pub show_settings_modal: bool,
    pub settings_tab: usize,
    pub mostrar_conceptos_drawer: bool,
    pub conceptos_drawer_tab: usize,
    pub conceptos_salida_abierta: bool,
    pub codelab_drawer_tab: usize,
    pub show_railroad_modal: Option<usize>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            ruta_actual: AppRoute::LandingPage,
            mostrar_sidebar: true,
            mostrar_nav_superior: true,
            mostrar_status_bar: true,
            tutorial_time: 0.0,
            anim_trigger: 0.0,
            show_code_modal: None,
            mostrar_explorer_drawer: false,
            mostrar_console_drawer: false,
            show_commands_modal: false,
            show_macro_expansion: false,
            show_cargo_output_modal: Arc::new(AtomicBool::new(false)),
            show_rustc_compilador_modal: false,
            show_terminal_modal: false,
            show_settings_modal: false,
            settings_tab: 0,
            mostrar_conceptos_drawer: false,
            conceptos_drawer_tab: 0,
            conceptos_salida_abierta: false,
            codelab_drawer_tab: 0,
            show_railroad_modal: None,
        }
    }
}
