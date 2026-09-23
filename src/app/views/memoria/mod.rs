pub mod borrowing;
pub mod heap_move;
pub mod ownership;
pub mod overview;
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
            "Fundamental",
            "Stack",
            "Es una zona de memoria rápida y organizada donde Rust guarda muchos valores locales de tamaño conocido.\n\nEs un comportamiento que permite copiar automáticamente un valor al asignarlo a otra variable, sin que la variable original deje de estar disponible.",
            "Añade este fragmento al archivo del proyecto y observa que las dos variables siguen disponibles después de copiarse.",
            "let x = 42;\nlet y = x; // Copy: x sigue siendo válido\n\nprintln!(\"x = {x}\");\nprintln!(\"y = {y}\");",
        ),
        (
            "String, Heap & Move Semantics",
            "Fundamental",
            "String",
            "String es un tipo de texto que puede cambiar de tamaño y conservar su contenido mientras el programa lo utiliza.\n\nEl Heap es una zona de memoria destinada a datos dinámicos. Puede reservar espacio adicional cuando un valor necesita crecer; por eso String utiliza el Heap para guardar sus caracteres y push_str puede añadir más texto.\n\nMove Semantics ocurre cuando un valor pasa a otra variable y la variable anterior deja de estar disponible.",
            "Añade el fragmento al archivo del proyecto. Observa cómo String puede crecer y cómo el valor pasa a 'movido' sin volver a utilizar 'texto'.",
            "let mut texto = String::from(\"Rust\");\ntexto.push_str(\" en el Heap\");\n\nlet movido = texto; // Move: el valor pasa a movido\nprintln!(\"{movido}\");",
        ),
        (
            "Ownership",
            "Fundamental",
            "Ownership",
            "Cada valor tiene una variable responsable: su propietario.\n\nUn valor solo puede tener un propietario activo a la vez; cuando pasa a otra variable, la responsabilidad también cambia.\n\nCuando el propietario sale de su Scope, Rust libera automáticamente el valor que ya no se necesita.",
            "Añade el fragmento y observa cómo una variable recibe la responsabilidad del valor. Después identifica qué ocurre cuando el propietario sale de su Scope.",
            "let original = String::from(\"Rust\");\nlet propietario = original;\n\nprintln!(\"{propietario}\");",
        ),
        (
            "Borrowing",
            "Fundamental",
            "Borrowing",
            "Borrowing permite utilizar un valor sin tomar su Ownership.\n\nUna Immutable Reference permite leer el valor y el propietario continúa siendo responsable.\n\nPuedes tener varias Immutable References al mismo tiempo.\n\nUna Mutable Reference permite modificar el valor, pero solo puede existir una durante ese acceso.\n\nLas reglas de Borrowing impiden mezclar una Reference de lectura con una Mutable Reference activa, y una Reference no puede superar la vida del valor que utiliza.",
            "Añade el fragmento y observa cómo se pueden realizar lecturas compartidas y después una modificación exclusiva, respetando el orden de los accesos.",
            "let mut texto = String::from(\"Rust\");\n\n{\n    let lectura_a = &texto;\n    let lectura_b = &texto;\n    println!(\"{lectura_a} y {lectura_b}\");\n}\n\nlet escritura = &mut texto;\nescritura.push_str(\" seguro\");\nprintln!(\"{escritura}\");",
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
    if state.lessons.strings_ownership_tab == 2 {
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
                    0 => overview::mostrar(ui),
                    1 => strings::mostrar_teoria_string_y_str(ui, state),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("String vs &str", 1),
    ];
    let active = state.lessons.strings_ownership_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Memory & Ownership",
        &tabs,
        2,
        active,
        |st, idx| st.lessons.strings_ownership_tab = idx,
    );
}
