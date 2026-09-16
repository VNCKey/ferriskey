pub mod bounds;
pub mod funciones;
pub mod info;
pub mod structs;

use crate::app::AppState;
use eframe::egui;

pub fn retos_genericos() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Estructuras Genéricas (Par<T, U>)",
            "Genéricos",
            "Múltiples parámetros de tipo",
            "Los genéricos permiten definir estructuras reutilizables para distintos tipos sin duplicar código ni perder seguridad.",
            "Crea la estructura Par<T, U> instanciando tipos entero y string.",
            "struct Par<T, U> {\n    primero: T,\n    segundo: U,\n}\n\nfn main() {\n    let p = Par { primero: 42, segundo: \"Hola Rust\" };\n    println!(\"Par: {}, {}\", p.primero, p.segundo);\n}",
        ),
        (
            "Funciones Genéricas",
            "Genéricos",
            "Polimorfismo en funciones",
            "Una función genérica opera sobre tipos parametrizados <T> inferidos automáticamente por el compilador.",
            "Define 'obtener_primero<T>' que devuelva una referencia al primer elemento de un slice.",
            "fn obtener_primero<T>(slice: &[T]) -> &T {\n    &slice[0]\n}\n\nfn main() {\n    let nums = [10, 20, 30];\n    println!(\"Primero: {}\", obtener_primero(&nums));\n}",
        ),
        (
            "Trait Bounds (T: PartialOrd + Copy)",
            "Trait Bounds",
            "Restricciones de capacidades",
            "Los Trait Bounds garantizan que los tipos genéricos soporten operaciones específicas como comparación u ordenación.",
            "Crea la función 'obtener_mayor<T: PartialOrd + Copy>' para comparar dos valores.",
            "fn obtener_mayor<T: PartialOrd + Copy>(a: T, b: T) -> T {\n    if a > b { a } else { b }\n}\n\nfn main() {\n    println!(\"Mayor: {}\", obtener_mayor(15, 42));\n}",
        ),
    ]
}

pub fn mostrar_tutorial_genericos(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.genericos_tab == 4 {
        let retos = retos_genericos();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_genericos_codelab",
                separator_id: "genericos_editor_toolbar_sep_y",
                terminal_panel_id: "genericos_terminal_panel",
                editor_scroll_id: "genericos_codelab_editor",
                drawer_key: "genericos",
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
                let orange = egui::Color32::from_rgb(255, 160, 50);
                let cyan = egui::Color32::from_rgb(100, 200, 255);
                let texto = egui::Color32::from_rgb(200, 210, 225);
                ui.add_space(10.0);
                match state.lessons.genericos_tab {
                    0 => funciones::mostrar_tab_funciones(ui, state, orange, cyan, texto),
                    1 => structs::mostrar_tab_structs(ui, state, orange, cyan, texto),
                    2 => bounds::mostrar_tab_bounds(ui, state, orange, cyan, texto),
                    _ => info::mostrar_genericos_info(ui, state, orange, cyan, texto),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Funciones Genéricas", 0),
        ("Estructuras Genéricas", 1),
        ("Trait Bounds", 2),
        ("Info de Genéricos", 3),
    ];
    let active = state.lessons.genericos_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Tipos Genéricos",
        &tabs,
        4,
        active,
        |st, idx| st.lessons.genericos_tab = idx,
    );
}
