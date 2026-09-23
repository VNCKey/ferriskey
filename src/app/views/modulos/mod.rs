pub mod estructura;
pub mod overview;

use crate::app::AppState;
use eframe::egui;

pub fn retos_modulos() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Declaration & Module Tree",
            "Fundamental",
            "mod",
            "Un módulo agrupa código relacionado dentro de un espacio de nombres. La raíz del crate puede declarar módulos inline o conectarlos con archivos separados.",
            "Añade el módulo al archivo del proyecto y utiliza su función mediante la ruta correspondiente.",
            "mod utilidades {\n    pub fn saludar() {\n        println!(\"Hola desde utilidades\");\n    }\n}\n\nutilidades::saludar();",
        ),
        (
            "Visibility",
            "Fundamental",
            "pub · pub(crate)",
            "Los ítems son privados por defecto. pub permite exponer un ítem y pub(crate) lo hace visible dentro de todo el crate, pero no fuera de él.",
            "Compara qué función puede llamarse desde fuera del módulo y qué función permanece privada.",
            "mod red {\n    pub fn conectar() {}\n    pub(crate) fn diagnostico() {}\n    fn secreto() {}\n}",
        ),
        (
            "Paths & Imports",
            "Navigation",
            "crate · self · super · use",
            "Las rutas indican cómo llegar a un ítem del árbol de módulos. crate comienza en la raíz, self representa el módulo actual y super sube al módulo padre. use permite acortar una ruta.",
            "Escribe una ruta completa y después crea un import con use para utilizar el mismo ítem de forma más clara.",
            "mod red {\n    pub fn conectar() {}\n}\n\nuse crate::red::conectar;\nconectar();",
        ),
        (
            "Re-export & Public API",
            "API Design",
            "pub use",
            "pub use expone un ítem desde otra ruta pública. Así los usuarios pueden importar una API sencilla sin conocer todas las carpetas internas.",
            "Crea una entrada pública en la raíz y deja que la implementación permanezca dentro de un módulo interno.",
            "mod interno {\n    pub fn procesar() {}\n}\n\npub use interno::procesar;\n\nprocesar();",
        ),
        (
            "Module File Structure",
            "Architecture",
            "src/",
            "Un módulo puede vivir en un archivo con su propio nombre o dentro de una carpeta con un archivo raíz. La estructura moderna evita usar mod.rs como archivo principal de cada carpeta nueva.",
            "Abre el explorador del proyecto y relaciona cada declaración mod nombre; con el archivo que Rust busca dentro de src/.",
            "src/\n├── main.rs\n├── red.rs\n└── red/\n    └── http.rs",
        ),
    ]
}

pub fn mostrar_tutorial_modulos(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.modulos_tab == 2 {
        let retos = retos_modulos();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_modulos_codelab",
                separator_id: "modulos_editor_toolbar_sep_y",
                terminal_panel_id: "modulos_terminal_panel",
                editor_scroll_id: "modulos_codelab_editor",
                drawer_key: "modulos",
                drawer_open: state.ui.mostrar_explorer_drawer,
                drawer_tab: state.ui.codelab_drawer_tab,
                output_open: state.ui.mostrar_console_drawer,
                navigation_step: state.lessons.session_codelab_reto,
                navigation_total: total_retos,
            },
            |ui, state, orange, cyan| {
                let current = state
                    .lessons
                    .session_codelab_reto
                    .min(total_retos.saturating_sub(1));
                let (titulo, categoria, subtitulo, explicacion, paso_practico, codigo_ejemplo) =
                    retos[current];
                crate::views::pilares::anatomy::mostrar_reto_codelab_item(
                    ui,
                    state,
                    current + 1,
                    titulo,
                    categoria,
                    subtitulo,
                    explicacion,
                    paso_practico,
                    codigo_ejemplo,
                    orange,
                    cyan,
                );
            },
        );

        state.ui.mostrar_explorer_drawer = shell_state.drawer_open;
        state.ui.codelab_drawer_tab = shell_state.drawer_tab;
        state.ui.mostrar_console_drawer = shell_state.output_open;
        crate::views::pilares::anatomy::aplicar_navegacion_codelab(
            &mut state.lessons.session_codelab_reto,
            shell_state.navigation_delta,
            total_retos,
        );
    } else {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(10.0);
                match state.lessons.modulos_tab {
                    0 => overview::mostrar(ui),
                    1 => estructura::mostrar_teoria_modulos(ui),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("Structure & Best Practices", 1),
    ];
    let active = state.lessons.modulos_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Modules & Visibility",
        &tabs,
        2,
        active,
        |st, idx| st.lessons.modulos_tab = idx,
    );
}
