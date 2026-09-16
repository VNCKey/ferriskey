pub mod borrowing;
pub mod heap_move;
pub mod ownership;
pub mod stack_copy;
pub mod strings;

use crate::app::AppState;
use eframe::egui;

pub fn retos_memoria() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Stack & Copy Semantics",
            "Fundamentos",
            "Copias implícitas bit a bit",
            "Los tipos primitivos enteros implementan el Trait Copy. Al asignarse a una nueva variable, el valor se duplica en el stack sin perder el dueño original.",
            "Ejecuta el código en la terminal y observa cómo ambas variables 'x' e 'y' permanecen válidas simultáneamente.",
            "fn main() {\n    let x = 42;\n    let y = x; // Copia implícita en Stack\n\n    println!(\"x sigue siendo válido: {}\", x);\n    println!(\"y contiene una copia: {}\", y);\n}",
        ),
        (
            "Heap & Ownership Move",
            "Ownership",
            "Transferencia de propiedad",
            "Tipos dinámicos como String almacenan su contenido en el Heap. Al asignar 's1' a 's2', Rust desplaza (Move) la propiedad para evitar doble liberación de memoria.",
            "Descomenta la línea del println!(s1) y compila con Ctrl+S para observar el error del Borrow Checker.",
            "fn main() {\n    let s1 = String::from(\"FerrisKey\");\n    let s2 = s1; // Move semantics: Ownership transferida\n\n    // println!(\"{}\", s1); // ❌ Error: use of moved value `s1`\n    println!(\"s2 es el único dueño activo: {}\", s2);\n}",
        ),
        (
            "Préstamos Mutables (&mut T)",
            "Borrowing",
            "Préstamo mutable exclusivo",
            "Para modificar un valor sin transferir ownership, pasamos una referencia mutable (&mut T). Solo puede existir un préstamo mutable a la vez.",
            "Observa cómo la función 'agregar_sufijo' modifica la variable 'mensaje' in-place.",
            "fn agregar_sufijo(texto: &mut String) {\n    texto.push_str(\" - Aprende Rust sin errores\");\n}\n\nfn main() {\n    let mut mensaje = String::from(\"FerrisKey\");\n    agregar_sufijo(&mut mensaje);\n    println!(\"Resultado: {}\", mensaje);\n}",
        ),
        (
            "Préstamo Inmutable vs Mutable",
            "Borrow Checker",
            "Aliasing & Mutabilidad",
            "Pueden existir múltiples referencias inmutables (&T), pero no se puede tener un préstamo mutable (&mut T) mientras existan lectores activos.",
            "Analiza las Non-Lexical Lifetimes (NLL) que permiten a 'r3' coexistir tras el último uso de 'r1' y 'r2'.",
            "fn main() {\n    let mut datos = String::from(\"Rust\");\n    let r1 = &datos;\n    let r2 = &datos;\n    println!(\"Lectores: {}, {}\", r1, r2);\n\n    let r3 = &mut datos; // Válido tras NLL\n    r3.push_str(\" 2026\");\n    println!(\"Modificado: {}\", r3);\n}",
        ),
        (
            "String vs &str",
            "Slices",
            "Propiedad vs Referencia prestada",
            "String es un buffer dinámico poseído en el Heap, mientras que &str es un slice (vista prestada de caracteres UTF-8).",
            "Llama a la función 'procesar_slice' pasando tanto un String como un literal &str.",
            "fn procesar_slice(slice: &str) {\n    println!(\"Contenido: '{}' | Longitud: {} bytes\", slice, slice.len());\n}\n\nfn main() {\n    let texto_heap: String = String::from(\"FerrisKey Desktop\");\n    let texto_literal: &str = \"Hola Rust\";\n    procesar_slice(&texto_heap);\n    procesar_slice(texto_literal);\n}",
        ),
    ]
}

pub fn mostrar_tutorial_strings_ownership(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.strings_ownership_tab == 5 {
        let retos = retos_memoria();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_memoria_codelab",
                separator_id: "memoria_editor_toolbar_sep_y",
                terminal_panel_id: "memoria_terminal_panel",
                editor_scroll_id: "memoria_codelab_editor",
                drawer_key: "memoria",
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
                match state.lessons.strings_ownership_tab {
                    0 => stack_copy::mostrar_tab_stack_copy(ui, state),
                    1 => heap_move::mostrar_tab_heap_move(ui, state),
                    2 => ownership::mostrar_tab_ownership(ui),
                    3 => borrowing::mostrar_tab_borrowing(ui),
                    _ => strings::mostrar_teoria_string_y_str(ui, state),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Stack Copy", 0),
        ("Heap Move", 1),
        ("Ownership", 2),
        ("Borrowing", 3),
        ("String vs &str", 4),
    ];
    let active = state.lessons.strings_ownership_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Memoria y Ownership",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.strings_ownership_tab = idx,
    );
}
