pub mod declaracion;
pub mod estructura;
pub mod facade;
pub mod rutas;
pub mod visibilidad;

use crate::app::AppState;
use eframe::egui;

pub fn retos_modulos() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Declaración de Módulos",
            "Encapsulación",
            "Organización jerárquica de código",
            "Los módulos permiten encapsular código en espacios de nombres para evitar colisiones y estructurar el proyecto.",
            "Declara un módulo utilidades con una función pública saludar() y llámala desde main.",
            "mod utilidades {\n    pub fn saludar() {\n        println!(\"¡Hola desde el módulo utilidades!\");\n    }\n}\n\nfn main() {\n    utilidades::saludar();\n}",
        ),
        (
            "Visibilidad (pub y pub(crate))",
            "Control de Acceso",
            "Privacidad por defecto",
            "En Rust todos los ítems son privados por defecto. La palabra clave 'pub' expone ítems a módulos externos.",
            "Observa el uso de 'pub fn conectar()' dentro del módulo 'red'.",
            "mod red {\n    pub fn conectar() {\n        println!(\"Conexión establecida con éxito\");\n    }\n}\n\nfn main() {\n    red::conectar();\n}",
        ),
        (
            "Importación con use",
            "Rutas de Acceso",
            "Acortamiento de rutas en el Scope",
            "La instrucción 'use' introduce la ruta de un ítem en el scope actual para invocarlo sin prefijos largos.",
            "Importa la función directamente con 'use red::conectar;' antes de ejecutarla.",
            "mod red {\n    pub fn conectar() {\n        println!(\"Conectado vía use!\");\n    }\n}\n\nuse red::conectar;\n\nfn main() {\n    conectar();\n}",
        ),
        (
            "Re-exportación (pub use)",
            "Facade Pattern",
            "API Pública Limpia",
            "Con 'pub use' un módulo re-exporta ítems internos simplificando la interfaz expuesta a los usuarios del crate.",
            "Re-exporta la función 'procesar' desde el módulo 'interno'.",
            "mod interno {\n    pub fn procesar() {\n        println!(\"Procesando datos internos\");\n    }\n}\n\npub use interno::procesar;\n\nfn main() {\n    procesar();\n}",
        ),
    ]
}

pub fn mostrar_tutorial_modulos(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.modulos_tab == 5 {
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
                let current = state.lessons.session_codelab_reto.min(total_retos.saturating_sub(1));
                let (titulo, categoria, subtitulo, explicacion, paso_practico, codigo_ejemplo) = retos[current];
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
                    0 => declaracion::mostrar_tab_declaracion(ui),
                    1 => visibilidad::mostrar_tab_visibilidad(ui),
                    2 => rutas::mostrar_tab_rutas(ui),
                    3 => facade::mostrar_tab_facade(ui),
                    _ => estructura::mostrar_teoria_modulos(ui),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Declaración & Árbol", 0),
        ("Visibilidad (pub)", 1),
        ("Importación & Rutas", 2),
        ("Re-exportación", 3),
        ("Estructura de Archivos", 4),
    ];
    let active = state.lessons.modulos_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Módulos y Visibilidad",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.modulos_tab = idx,
    );
}
