pub mod desempaquetado;
pub mod info;
pub mod option;
pub mod result;

use crate::app::AppState;
use eframe::egui;

pub fn retos_enums() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Familia de Combinadores en Option",
            "Option<T>",
            "Manejo seguro de ausencia de valor",
            "Option<T> evita punteros nulos permitiendo transformaciones fluidas con combinadores como unwrap_or, map y unwrap_or_else.",
            "Utiliza unwrap_or y map para procesar valores opcionales sin desempaquetar manualmente.",
            "fn main() {\n    let config_usuario: Option<String> = None;\n    let tema = config_usuario.clone().unwrap_or(String::from(\"Oscuro\"));\n    println!(\"Tema seleccionado: {}\", tema);\n\n    let puerto = None::<u16>.unwrap_or_else(|| 8080);\n    println!(\"Puerto activo: {}\", puerto);\n\n    let nombre = Some(String::from(\"Ferris\"));\n    let longitud = nombre.map(|n| n.len());\n    println!(\"Longitud opcional: {:?}\", longitud);\n}",
        ),
        (
            "Propagación de Errores con Result y ?",
            "Result<T, E>",
            "Propagación limpia de errores con operador ?",
            "El operador '?' en funciones que retornan Result desempaqueta Ok(v) o retorna Err(e) inmediatamente hacia el llamador.",
            "Implementa 'sumar_numeros_str' propagando el error de parseo ParseIntError.",
            "use std::num::ParseIntError;\n\nfn sumar_numeros_str(a: &str, b: &str) -> Result<i32, ParseIntError> {\n    let num_a = a.parse::<i32>()?;\n    let num_b = b.parse::<i32>()?;\n    Ok(num_a + num_b)\n}\n\nfn main() {\n    match sumar_numeros_str(\"15\", \"25\") {\n        Ok(total) => println!(\"Suma exitosa: {}\", total),\n        Err(e) => println!(\"Error de parseo: {}\", e),\n    }\n}",
        ),
    ]
}

pub fn mostrar_tutorial_enums(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.enums_tab == 4 {
        let retos = retos_enums();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_enums_codelab",
                separator_id: "enums_editor_toolbar_sep_y",
                terminal_panel_id: "enums_terminal_panel",
                editor_scroll_id: "enums_codelab_editor",
                drawer_key: "enums",
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
                match state.lessons.enums_tab {
                    0 => option::mostrar_tab_option(ui, state, orange, cyan, texto),
                    1 => result::mostrar_tab_result(ui, state, orange, cyan, texto),
                    2 => desempaquetado::mostrar_tab_desempaquetado(ui, state, orange, cyan, texto),
                    _ => info::mostrar_errores_info(ui, state, orange, cyan, texto),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Option<T>", 0),
        ("Result<T, E>", 1),
        ("Desempaquetado (& ?)", 2),
        ("Info de Errores", 3),
    ];
    let active = state.lessons.enums_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Manejo de Errores",
        &tabs,
        4,
        active,
        |st, idx| st.lessons.enums_tab = idx,
    );
}
