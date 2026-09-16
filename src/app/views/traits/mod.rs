pub mod estandar;
pub mod info;
pub mod polimorfismo;

use crate::app::AppState;
use eframe::egui;

pub fn retos_traits() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "Definición e Implementación de Traits",
            "Traits",
            "Interfaces de comportamiento",
            "Un Trait define un contrato de métodos que los tipos pueden implementar para compartir comportamiento.",
            "Define el trait 'Imprimible' con 'imprimir(&self)' e impleméntalo para Mensaje.",
            "trait Imprimible {\n    fn imprimir(&self);\n}\n\nstruct Mensaje(String);\n\nimpl Imprimible for Mensaje {\n    fn imprimir(&self) {\n        println!(\"{}\", self.0);\n    }\n}\n\nfn main() {\n    let m = Mensaje(String::from(\"Hola desde Trait\"));\n    m.imprimir();\n}",
        ),
        (
            "Implementación de Traits Estándar (Display)",
            "Traits Estándar",
            "Formateo personalizado con std::fmt",
            "Implementar std::fmt::Display permite formatear una estructura en macros como println!(\"{}\", obj).",
            "Implementa Display para el struct Punto { x: i32, y: i32 }.",
            "use std::fmt;\n\nstruct Punto { x: i32, y: i32 }\n\nimpl fmt::Display for Punto {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n        write!(f, \"({}, {})\", self.x, self.y)\n    }\n}\n\nfn main() {\n    let p = Punto { x: 5, y: 10 };\n    println!(\"Punto: {}\", p);\n}",
        ),
        (
            "Polimorfismo Dinámico (&dyn Trait)",
            "Polimorfismo",
            "Dynamic Dispatch con Trait Objects",
            "Los Trait Objects (&dyn Trait o Box<dyn Trait>) permiten almacenar diferentes tipos que implementan el mismo trait en una colección.",
            "Crea una referencia trait object '&dyn Imprimible' e invoca su método resumen.",
            "trait Imprimible {\n    fn resumen(&self) -> String;\n}\n\nstruct Noticia(String);\nimpl Imprimible for Noticia {\n    fn resumen(&self) -> String { format!(\"Noticia: {}\", self.0) }\n}\n\nfn main() {\n    let n = Noticia(String::from(\"Rust 2026 liberado!\"));\n    let item: &dyn Imprimible = &n;\n    println!(\"{}\", item.resumen());\n}",
        ),
    ]
}

pub fn mostrar_tutorial_traits(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.traits_tab == 3 {
        let retos = retos_traits();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_traits_codelab",
                separator_id: "traits_editor_toolbar_sep_y",
                terminal_panel_id: "traits_terminal_panel",
                editor_scroll_id: "traits_codelab_editor",
                drawer_key: "traits",
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
                match state.lessons.traits_tab {
                    0 => estandar::mostrar_tab_estandar(ui, state, orange, cyan, texto),
                    1 => polimorfismo::mostrar_tab_polimorfismo(ui, state, orange, cyan, texto),
                    _ => info::mostrar_traits_info(ui, state, orange, cyan, texto),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Traits Estándar & derive", 0),
        ("Polimorfismo (dyn Trait)", 1),
        ("Info & Guía de Traits", 2),
    ];
    let active = state.lessons.traits_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Traits y Polimorfismo",
        &tabs,
        3,
        active,
        |st, idx| st.lessons.traits_tab = idx,
    );
}
