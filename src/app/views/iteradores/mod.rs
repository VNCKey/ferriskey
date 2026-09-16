pub mod adaptadores;
pub mod consumidores;
pub mod info;
pub mod modos;
pub mod pipeline;

use crate::app::AppState;
use eframe::egui;

pub fn retos_iteradores() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Comparación de Modos de Iteración",
            "Iteradores",
            "Borrowing vs Move (.iter vs .into_iter)",
            ".iter() presta referencias inmutables dejando la colección intacta, mientras que .into_iter() consume la colección.",
            "Itera sobre una lista primero prestando con .iter() y luego consumiéndola con .into_iter().",
            "fn main() {\n    let numeros = vec![1, 2, 3];\n    for num in numeros.iter() {\n        println!(\"Referencia: {}\", num);\n    }\n    let suma: i32 = numeros.into_iter().sum();\n    println!(\"Suma total consumida: {}\", suma);\n}",
        ),
        (
            "Pipeline Perezoso de Transformación Funcional",
            "Pipelines",
            "Evaluación perezosa (.filter & .map)",
            "Los adaptadores de iteradores no ejecutan trabajo hasta que se llama a un método consumidor como .collect().",
            "Filtra los números pares de un vector, multiplícalos por 10 y recólectalos en un nuevo Vec.",
            "fn main() {\n    let datos = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];\n    let resultado: Vec<i32> = datos\n        .iter()\n        .filter(|&&x| x % 2 == 0)\n        .map(|&x| x * 10)\n        .collect();\n    println!(\"Resultado del pipeline: {:?}\", resultado);\n}",
        ),
        (
            "Uso de Turbofish (::<T>) con .collect()",
            "Colecciones",
            "Inferencia de tipos explícita",
            "La sintaxis Turbofish ::<HashSet<_>> resuelve de forma directa el tipo de estructura a construir con .collect().",
            "Recolecta elementos duplicados en un HashSet usando la sintaxis Turbofish.",
            "use std::collections::HashSet;\n\nfn main() {\n    let nombres = vec![\"Alice\", \"Bob\", \"Alice\", \"Charlie\"];\n    let unicos = nombres.into_iter().collect::<HashSet<_>>();\n    println!(\"Nombres únicos recopilados: {:?}\", unicos);\n}",
        ),
    ]
}

pub fn mostrar_tutorial_iteradores(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.iteradores_tab == 4 {
        let retos = retos_iteradores();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_iteradores_codelab",
                separator_id: "iteradores_editor_toolbar_sep_y",
                terminal_panel_id: "iteradores_terminal_panel",
                editor_scroll_id: "iteradores_codelab_editor",
                drawer_key: "iteradores",
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
                let orange = egui::Color32::from_rgb(255, 160, 50);
                let cyan = egui::Color32::from_rgb(100, 200, 255);
                let texto = egui::Color32::from_rgb(200, 210, 225);
                ui.add_space(10.0);
                match state.lessons.iteradores_tab {
                    0 => modos::mostrar_tab_modos(ui, state, orange, cyan, texto),
                    1 => adaptadores::mostrar_tab_adaptadores(ui, state, orange, cyan, texto),
                    2 => consumidores::mostrar_tab_consumidores(ui, state, orange, cyan, texto),
                    _ => info::mostrar_iteradores_info(ui, state, orange, cyan, texto),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Modos de Iteración", 0),
        ("Adaptadores", 1),
        ("Consumidores", 2),
        ("Info & Buenas Prácticas", 3),
    ];
    let active = state.lessons.iteradores_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Iteradores y Pipelines",
        &tabs,
        4,
        active,
        |st, idx| st.lessons.iteradores_tab = idx,
    );
}
