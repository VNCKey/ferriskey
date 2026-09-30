use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui;

type Reto = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

pub fn retos_testing() -> Vec<Reto> {
    vec![
        (
            "Unit Tests",
            "Testing",
            "Probar una unidad pequeña",
            "Los Unit Tests viven junto al código y pueden revisar funciones públicas o privadas dentro de un módulo marcado con `#[cfg(test)]`.",
            "Escribe una función pequeña y agrega un módulo de pruebas con `#[test]`.",
            "pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_dos_valores() {
        assert_eq!(sumar(2, 3), 5);
    }
}",
        ),
        (
            "Assertions",
            "Testing",
            "Describir lo que debe cumplirse",
            "Las macros `assert!`, `assert_eq!` y `assert_ne!` convierten una expectativa en una comprobación automática. Si falla, el test informa qué ocurrió.",
            "Elige la aserción adecuada para comprobar una condición, una igualdad o una diferencia.",
            "#[test]
fn comprobar_resultados() {
    assert!(10 > 2);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
}",
        ),
        (
            "Integration Tests",
            "Testing",
            "Probar la API pública",
            "Los Integration Tests viven en la carpeta `tests/` y utilizan la crate como lo haría un consumidor externo. Por eso solo pueden acceder a elementos públicos.",
            "Crea un archivo dentro de `tests/` y prueba una función expuesta por `src/lib.rs`.",
            "// tests/suma.rs
use mi_crate::sumar;

#[test]
fn prueba_la_api_publica() {
    assert_eq!(sumar(10, 20), 30);
}",
        ),
        (
            "Doc-Tests",
            "Testing",
            "Ejemplos que también se prueban",
            "Un ejemplo dentro de un comentario `///` puede convertirse en una prueba. `cargo test` lo compila y comprueba junto con el resto de tests.",
            "Documenta una función con un ejemplo ejecutable y mantenlo sincronizado con el comportamiento real.",
            "/// Suma dos valores.
///
/// ```
/// assert_eq!(mi_crate::sumar(2, 2), 4);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}",
        ),
        (
            "Test Commands",
            "Testing",
            "Ejecutar la comprobación adecuada",
            "Cargo permite ejecutar todos los tests, buscar un test concreto, mostrar la salida de `println!` o incluir los tests marcados con `#[ignore]`.",
            "Prueba cada comando y observa qué parte de la suite se ejecuta en cada caso.",
            "cargo test
cargo test nombre_del_test
cargo test -- --nocapture
cargo test -- --ignored",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa Unit Tests, Assertions, Integration Tests, Doc-Tests y los comandos principales de Cargo.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_testing(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.testing_tab == 5 {
        mostrar_code_lab_testing(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.testing_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_unit_tests(ui, state),
                2 => mostrar_integration_tests(ui, state),
                3 => mostrar_doc_tests(ui, state),
                4 => mostrar_test_commands(ui, state),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Testing");
    session_intro(
        ui,
        "Esta sesión enseña a comprobar el comportamiento de tu código con las herramientas de pruebas que Cargo integra en Rust.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero escribiremos `Unit Tests` para una función pequeña. Después conoceremos las `Assertions`, separaremos los `Integration Tests` y convertiremos ejemplos de documentación en `Doc-Tests`. Al final practicaremos los comandos de Cargo.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Unit Tests",
            "Comprueban una unidad pequeña junto al código que la implementa.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Assertions",
            "Expresan el resultado que el test espera encontrar.",
        );

        card_overview(
            &mut columns[1],
            "Integration Tests",
            "Prueban la API pública desde la perspectiva de un usuario externo.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Doc-Tests",
            "Mantienen los ejemplos de documentación compilables y verificables.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Un test no reemplaza el diseño: hace visible el comportamiento esperado y avisa cuando una modificación rompe ese contrato.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_unit_tests(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Unit Tests");
    session_intro(
        ui,
        "Un Unit Test comprueba una parte pequeña del código en el mismo módulo donde vive.",
    );
    section_heading(ui, "Estructura mínima");
    texto_con_chips_inline(
        ui,
        "`#[cfg(test)]` incluye el módulo solo durante las pruebas y `#[test]` marca la función que Cargo debe ejecutar.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_dos_valores() {
        assert_eq!(sumar(2, 3), 5);
    }
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Regla práctica");
    texto_con_chips_inline(
        ui,
        "Un test debe tener un propósito claro: preparar datos, ejecutar una operación y comprobar un resultado.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_integration_tests(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Integration Tests");
    session_intro(
        ui,
        "Los Integration Tests prueban cómo se utiliza tu crate desde fuera de su implementación interna.",
    );
    section_heading(ui, "La carpeta `tests/`");
    texto_con_chips_inline(
        ui,
        "Cada archivo `.rs` dentro de `tests/` funciona como un consumidor separado. Esa separación ayuda a comprobar que tu API pública sea realmente utilizable.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "// tests/suma.rs
use mi_crate::sumar;

#[test]
fn prueba_la_api_publica() {
    assert_eq!(sumar(10, 20), 30);
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Qué no prueba");
    texto_con_chips_inline(
        ui,
        "El test no debe depender de campos o funciones privadas: desde `tests/` solo existe el contrato que tu crate expone con `pub`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_doc_tests(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Doc-Tests");
    session_intro(
        ui,
        "Un ejemplo de documentación puede ser también una prueba automática.",
    );
    section_heading(ui, "Documentación ejecutable");
    texto_con_chips_inline(
        ui,
        "El bloque de código dentro de `///` se compila durante `cargo test`. Así un ejemplo que deja de funcionar no permanece oculto en la documentación.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "/// Suma dos valores.
///
/// ```
/// assert_eq!(mi_crate::sumar(2, 2), 4);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn mostrar_test_commands(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Test Commands");
    session_intro(
        ui,
        "Cargo permite elegir qué pruebas ejecutar y qué información mostrar durante el proceso.",
    );
    section_heading(ui, "Comandos frecuentes");
    texto_con_chips_inline(
        ui,
        "Empieza con `cargo test`. Después añade un filtro o pasa argumentos al arnés con `--` cuando necesites inspeccionar un caso concreto.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "cargo test
cargo test nombre_del_test
cargo test -- --nocapture
cargo test -- --ignored",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "sh",
    );
}

fn preguntas_testing() -> [SessionQuestion; 5] {
    [
        SessionQuestion {
            prompt: "1. ¿Qué atributo marca una función de prueba?",
            options: ["#[test]", "#[run]", "#[check]"],
            correct: 0,
        },
        SessionQuestion {
            prompt: "2. ¿Qué comprueba assert_eq!?",
            options: [
                "Que dos valores sean iguales",
                "Que un hilo termine",
                "Que un archivo exista siempre",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "3. ¿Dónde viven los Integration Tests?",
            options: ["En la carpeta tests/", "En target/", "En Cargo.lock"],
            correct: 0,
        },
        SessionQuestion {
            prompt: "4. ¿Qué hace cargo test con un Doc-Test?",
            options: [
                "Compila y comprueba el ejemplo",
                "Lo convierte en una dependencia",
                "Lo elimina del comentario",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "5. ¿Qué permite --nocapture?",
            options: [
                "Ver la salida de println! del test",
                "Desactivar todas las pruebas",
                "Crear una crate nueva",
            ],
            correct: 0,
        },
    ]
}

fn mostrar_code_lab_testing(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_testing();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_testing_codelab",
            separator_id: "testing_editor_toolbar_sep_y",
            terminal_panel_id: "testing_terminal_panel",
            editor_scroll_id: "testing_codelab_editor",
            drawer_key: "testing",
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
            if titulo == "Questions" {
                mostrar_evaluacion(
                    ui,
                    &mut state.lessons.testing_preguntas_respuestas,
                    &preguntas_testing(),
                    &["Unit Tests", "Assertions", "Integration", "Doc-Tests"],
                    "Comprueba si puedes elegir el tipo de test y el comando adecuado para cada situación.",
                    orange,
                );
            } else {
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
            }
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
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("Unit Tests", 1),
        ("Integration", 2),
        ("Doc-Tests", 3),
        ("Commands", 4),
    ];
    let active = state.lessons.testing_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Testing",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.testing_tab = idx,
    );
}
