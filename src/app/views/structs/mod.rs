pub mod definicion;
pub mod enums;
pub mod info;
pub mod traits;

use crate::app::AppState;
use eframe::egui;

pub fn retos_structs() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Definición de Structs y Métodos impl",
            "Structs",
            "Estructuras nombradas y constructores",
            "Las estructuras agrupan campos nombrados. El bloque 'impl' asocia funciones constructoras y métodos.",
            "Define el struct Usuario y crea una función asociada 'nuevo'.",
            "struct Usuario {\n    nombre: String,\n    edad: u32,\n}\n\nimpl Usuario {\n    fn nuevo(nombre: &str, edad: u32) -> Self {\n        Self {\n            nombre: nombre.to_string(),\n            edad,\n        }\n    }\n}\n\nfn main() {\n    let u = Usuario::nuevo(\"Ferris\", 5);\n    println!(\"Usuario: {} ({})\", u.nombre, u.edad);\n}",
        ),
        (
            "Tuple Structs y Unit Structs",
            "Structs",
            "Estructuras de tupla y marca",
            "Rust soporta Tuple Structs (campos indexados por posición) y Unit Structs (sin campos).",
            "Crea un Tuple Struct 'Color' y un Unit Struct 'Marcador'.",
            "struct Color(u8, u8, u8);\nstruct Marcador;\n\nfn main() {\n    let rojo = Color(255, 0, 0);\n    let _m = Marcador;\n    println!(\"Color R: {}\", rojo.0);\n}",
        ),
        (
            "Métodos Mutables (&mut self)",
            "Métodos",
            "Mutabilidad de estado",
            "Los métodos que modifican los campos de un struct reciben '&mut self' como primer parámetro.",
            "Crea un struct Contador con un método 'incrementar(&mut self)'.",
            "struct Contador {\n    valor: u32,\n}\n\nimpl Contador {\n    fn incrementar(&mut self) {\n        self.valor += 1;\n    }\n}\n\nfn main() {\n    let mut c = Contador { valor: 0 };\n    c.incrementar();\n    println!(\"Valor actual: {}\", c.valor);\n}",
        ),
    ]
}

pub fn mostrar_tutorial_structs(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.structs_tab == 4 {
        let retos = retos_structs();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_structs_codelab",
                separator_id: "structs_editor_toolbar_sep_y",
                terminal_panel_id: "structs_terminal_panel",
                editor_scroll_id: "structs_codelab_editor",
                drawer_key: "structs",
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
                match state.lessons.structs_tab {
                    0 => definicion::mostrar_tab_structs(ui, state, orange, cyan, texto),
                    1 => enums::mostrar_tab_enums(ui, state, orange, cyan, texto),
                    2 => traits::mostrar_tab_traits_custom(ui, state, orange, cyan, texto),
                    _ => info::mostrar_structs_info(ui, state, orange, cyan, texto),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Structs & impl", 0),
        ("Enums & impl", 1),
        ("Traits & impl", 2),
        ("Info & Métodos", 3),
    ];
    let active = state.lessons.structs_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Custom Types",
        &tabs,
        4,
        active,
        |st, idx| st.lessons.structs_tab = idx,
    );
}
