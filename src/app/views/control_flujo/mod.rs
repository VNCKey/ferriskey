pub mod bucles;
pub mod condicionales;
pub mod info;
pub mod match_expr;

use crate::app::AppState;
use eframe::egui;

pub fn retos_control_flujo() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Devolución de Valores con loop y break",
            "Bucles",
            "Retorno de valores en bucles infinitos",
            "En Rust, el bucle 'loop' puede devolver una expresión al terminarse mediante 'break expresión;'.",
            "Incrementa un contador dentro de loop y retorna el resultado multiplicado por 2.",
            "fn main() {\n    let mut contador = 0;\n    let resultado = loop {\n        contador += 1;\n        if contador == 10 {\n            break contador * 2;\n        }\n    };\n    println!(\"Resultado devuelto por el bucle loop: {}\", resultado);\n}",
        ),
        (
            "Pattern Matching con Rangos y Match Guards",
            "Pattern Matching",
            "Match destructurativo y condicional",
            "La expresión 'match' permite asociar patrones con rangos inclusive (0..=12) y guards de tipo 'if' para filtrar condiciones complejas.",
            "Clasifica distintas edades usando rangos y un match guard 'n if n >= 18 && n < 65'.",
            "fn clasificar_edad(edad: u32) {\n    match edad {\n        0..=12 => println!(\"Niño ({})\", edad),\n        13..=17 => println!(\"Adolescente ({})\", edad),\n        n if n >= 18 && n < 65 => println!(\"Adulto ({})\", n),\n        _ => println!(\"Adulto mayor\"),\n    }\n}\n\nfn main() {\n    clasificar_edad(10);\n    clasificar_edad(15);\n    clasificar_edad(30);\n}",
        ),
        (
            "Extracción concisa con if let",
            "Control de Flujo",
            "Coincidencia de un único patrón",
            "'if let' simplifica el tratamiento de un único patrón como Option::Some o Result::Ok ignorando los demás.",
            "Desempaqueta un Option<&str> usando 'if let Some(config)'.",
            "fn main() {\n    let configuracion_opcional: Option<&str> = Some(\"Modo Oscuro\");\n    if let Some(config) = configuracion_opcional {\n        println!(\"Configuración activa: {}\", config);\n    } else {\n        println!(\"Sin configuración predeterminada\");\n    }\n}",
        ),
    ]
}

pub fn mostrar_tutorial_control_flujo(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.controlflujo_tab == 4 {
        let retos = retos_control_flujo();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_control_flujo_codelab",
                separator_id: "control_flujo_editor_toolbar_sep_y",
                terminal_panel_id: "control_flujo_terminal_panel",
                editor_scroll_id: "control_flujo_codelab_editor",
                drawer_key: "control_flujo",
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
                match state.lessons.controlflujo_tab {
                    0 => condicionales::mostrar_tab_condicionales(ui, state),
                    1 => bucles::mostrar_tab_bucles(ui, state),
                    2 => match_expr::mostrar_tab_match(ui, state),
                    _ => info::mostrar_control_flujo_info(ui, state, orange, cyan, texto),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn card_frame_tutorial() -> egui::Frame {
    let mut f = egui::Frame::new();
    f.fill = egui::Color32::from_rgb(14, 18, 26);
    f.inner_margin = egui::Margin::same(12);
    f.corner_radius = egui::CornerRadius::same(8);
    f.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));
    f
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Condicionales", 0),
        ("Bucles", 1),
        ("Match", 2),
        ("Info de Control", 3),
    ];
    let active = state.lessons.controlflujo_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Control de Flujo",
        &tabs,
        4,
        active,
        |st, idx| st.lessons.controlflujo_tab = idx,
    );
}
