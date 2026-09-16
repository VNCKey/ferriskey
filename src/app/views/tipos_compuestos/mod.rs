pub mod arrays;
pub mod comparativa;
pub mod info;
pub mod slices;
pub mod tuplas;

use crate::app::AppState;
use crate::views::colecciones::info::mostrar_coleccion_info;
use crate::views::colecciones::mostrar_contenido_colecciones;
use eframe::egui;

pub fn retos_tipos_compuestos() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Arreglos e Inicialización por Repetición",
            "Arrays",
            "Memoria contigua en Stack",
            "Los arreglos [T; N] tienen tamaño fijo conocido en compilación y almacenan sus elementos de forma contigua.",
            "Inicializa un buffer de 8 bytes [0; 8] y modifica sus primeros dos elementos.",
            "fn main() {\n    let mut buffer: [u8; 8] = [0; 8];\n    buffer[0] = 0xFF;\n    buffer[1] = 0xFE;\n\n    println!(\"Buffer de memoria en Stack: {:?}\", buffer);\n    println!(\"Tamaño total del buffer: {} bytes\", buffer.len());\n}",
        ),
        (
            "Tuplas y Desestructuración de Retornos",
            "Tuplas",
            "Agrupación heterogénea",
            "Las tuplas (T1, T2, ...) permiten combinar valores de distintos tipos en un solo objeto sin crear un struct.",
            "Crea la función 'calcular_estadisticas' que retorne (min, max, cantidad) y desestructúrala en main.",
            "fn calcular_estadisticas(numeros: &[i32]) -> (i32, i32, usize) {\n    let mut min = numeros[0];\n    let mut max = numeros[0];\n    for &val in numeros {\n        if val < min { min = val; }\n        if val > max { max = val; }\n    }\n    (min, max, numeros.len())\n}\n\nfn main() {\n    let datos = [12, 45, 2, 89, 34];\n    let (minimo, maximo, cantidad) = calcular_estadisticas(&datos);\n    println!(\"Min: {} | Max: {} | Elementos: {}\", minimo, maximo, cantidad);\n}",
        ),
        (
            "Vistas Dinámicas con Slices (&[T])",
            "Slices",
            "Vistas prestadas de memoria",
            "Un slice &[T] es una vista prestada de una secuencia de elementos en memoria contigua sin copiar datos.",
            "Pasa un arreglo completo y un sub-slice &[1..4] a una función de visualización.",
            "fn imprimir_slice(etiqueta: &str, slice: &[i32]) {\n    println!(\"{}: {:?}\", etiqueta, slice);\n}\n\nfn main() {\n    let arreglo = [10, 20, 30, 40, 50, 60];\n    imprimir_slice(\"Arreglo completo\", &arreglo);\n    imprimir_slice(\"Sub-slice [1..4]\", &arreglo[1..4]);\n}",
        ),
    ]
}

pub fn mostrar_tutorial_tipos_compuestos(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.compuestos_tab == 5 {
        let retos = retos_tipos_compuestos();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_tipos_compuestos_codelab",
                separator_id: "tipos_compuestos_editor_toolbar_sep_y",
                terminal_panel_id: "tipos_compuestos_terminal_panel",
                editor_scroll_id: "tipos_compuestos_codelab_editor",
                drawer_key: "tipos_compuestos",
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
                match state.lessons.compuestos_tab {
                    0 => arrays::mostrar_compuesto_array(ui, state, orange, cyan, texto),
                    1 => tuplas::mostrar_compuesto_tupla(ui, state, orange, cyan, texto),
                    2 => mostrar_contenido_colecciones(ui, state),
                    3 => slices::mostrar_compuesto_slice(ui, state, orange, cyan, texto),
                    _ => {
                        info::mostrar_compuesto_info(ui, state, orange, cyan, texto);
                        ui.add_space(24.0);
                        ui.separator();
                        ui.add_space(16.0);
                        mostrar_coleccion_info(ui, state, orange, cyan, texto);
                    }
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Arrays", 0),
        ("Tuplas", 1),
        ("Colecciones", 2),
        ("Slices", 3),
        ("Comparativa e Info", 4),
    ];
    let active = state.lessons.compuestos_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Tipos Compuestos",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.compuestos_tab = idx,
    );
}
