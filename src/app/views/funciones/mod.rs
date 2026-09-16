pub mod closures;
pub mod parametros;
pub mod retorno;

use crate::app::AppState;
use eframe::egui;

pub fn retos_funciones() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Captura por Préstamo Inmutable",
            "Closures",
            "Captura de variables del entorno",
            "Un closure puede acceder a variables de su ámbito envolvente prestándolas de forma inmutable.",
            "Crea un closure que multiplique por una variable 'factor' definida en el scope principal.",
            "fn main() {\n    let factor = 10;\n    let multiplicar = |x: i32| x * factor;\n\n    println!(\"Resultado de 5 * 10: {}\", multiplicar(5));\n    println!(\"El factor sigue accesible: {}\", factor);\n}",
        ),
        (
            "Captura con Transferencia de Propiedad (move)",
            "Closures",
            "Ownership & Threads",
            "La palabra clave 'move' obliga al closure a tomar propiedad de las variables capturadas.",
            "Usa 'move ||' para mover un String a un closure.",
            "fn main() {\n    let mensaje = String::from(\"Hola desde FerrisKey\");\n    let imprimir = move || println!(\"{}\", mensaje);\n\n    imprimir();\n    // println!(\"{}\", mensaje); // ❌ Error! 'mensaje' fue movido\n}",
        ),
        (
            "Closures como Parámetros (impl Fn)",
            "Higher-Order Functions",
            "Traits Fn / FnMut / FnOnce",
            "Las funciones pueden recibir closures como argumentos tipados con la sintaxis genérica o 'impl Fn'.",
            "Crea la función 'ejecutar_operacion' que acepte un closure 'Fn(i32) -> i32'.",
            "fn ejecutar_operacion<F>(val: i32, operacion: F) -> i32\nwhere\n    F: Fn(i32) -> i32,\n{\n    operacion(val)\n}\n\nfn main() {\n    let resultado = ejecutar_operacion(7, |n| n * n);\n    println!(\"Cuadrado de 7: {}\", resultado);\n}",
        ),
    ]
}

pub fn mostrar_tutorial_funciones(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.funciones_tab == 3 {
        let retos = retos_funciones();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_funciones_codelab",
                separator_id: "funciones_editor_toolbar_sep_y",
                terminal_panel_id: "funciones_terminal_panel",
                editor_scroll_id: "funciones_codelab_editor",
                drawer_key: "funciones",
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
                match state.lessons.funciones_tab {
                    0 => parametros::mostrar_tab_parametros(ui, state, orange, cyan, texto),
                    1 => retorno::mostrar_tab_retorno(ui, state, orange, cyan, texto),
                    _ => closures::mostrar_tab_closures(ui, state, orange, cyan, texto),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Parámetros & Ownership", 0),
        ("Retornos Múltiples", 1),
        ("Closures & Captura", 2),
    ];
    let active = state.lessons.funciones_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Closures y Funciones",
        &tabs,
        3,
        active,
        |st, idx| st.lessons.funciones_tab = idx,
    );
}
