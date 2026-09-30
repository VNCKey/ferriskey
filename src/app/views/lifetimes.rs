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

pub fn retos_lifetimes() -> Vec<Reto> {
    vec![
        (
            "Dangling References",
            "Lifetimes",
            "Evitar referencias inválidas",
            "Una referencia solo puede utilizarse mientras el valor al que apunta sigue vivo. El compilador rechaza una referencia que intenta salir del Scope de su dato.",
            "Observa qué valor deja de estar disponible al terminar el bloque interno y explica por qué la referencia no puede utilizarse después.",
            "let referencia;
{
    let dato = 5;
    referencia = &dato;
}

// referencia ya no puede utilizarse aquí",
        ),
        (
            "Lifetime en Functions",
            "Lifetimes",
            "Relacionar entradas y retorno",
            "Cuando una función recibe varias referencias y devuelve una de ellas, la anotación `'a` expresa que el resultado depende de esas referencias. No alarga la vida de ningún dato.",
            "Completa la firma para indicar que el texto devuelto está relacionado con las dos referencias recibidas.",
            "fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}",
        ),
        (
            "Lifetime Elision",
            "Lifetimes",
            "Reconocer las reglas automáticas",
            "Rust puede omitir `'a` en casos comunes. Si existe una sola referencia de entrada, el compilador puede relacionarla automáticamente con la referencia de salida.",
            "Compara ambas firmas: expresan la misma relación, pero la segunda deja que Rust aplique la Elision Rule.",
            "fn primera_palabra<'a>(texto: &'a str) -> &'a str {
    texto.split(' ').next().unwrap_or(texto)
}

fn primera_palabra(texto: &str) -> &str {
    texto.split(' ').next().unwrap_or(texto)
}",
        ),
        (
            "Lifetimes en Structs",
            "Lifetimes",
            "Guardar una referencia en una Struct",
            "Una `Struct` que guarda una referencia necesita declarar su Lifetime. Así Rust puede comprobar que la Struct no sobreviva al texto que presta.",
            "Declara `Extracto<'a>` y conserva una vista prestada del texto original sin convertirla en una copia propia.",
            "struct Extracto<'a> {
    texto: &'a str,
}

impl<'a> Extracto<'a> {
    fn obtener(&self) -> &str {
        self.texto
    }
}",
        ),
        (
            "Lifetime 'static",
            "Lifetimes",
            "Datos válidos durante todo el programa",
            "`'static` describe una referencia válida durante toda la ejecución. Los literales de texto son el ejemplo más sencillo porque forman parte del binario.",
            "Identifica qué referencia puede vivir durante todo el programa y cuál depende de un valor local.",
            "let permanente: &'static str = \"Rust\";
let local = String::from(\"texto\");
let prestado: &str = &local;",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa referencias válidas, `'a`, Elision Rules, Structs con referencias y `'static`.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_lifetimes(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.lifetimes_tab == 5 {
        mostrar_code_lab_lifetimes(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.lifetimes_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_dangling_references(ui, state),
                2 => mostrar_funciones_lifetimes(ui, state),
                3 => mostrar_structs_lifetimes(ui, state),
                4 => mostrar_static(ui, state),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Lifetimes");
    session_intro(
        ui,
        "Esta sesión explica cómo Rust comprueba que una Reference siga siendo válida mientras se utiliza, sin cambiar la duración real del dato.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero veremos qué es una referencia inválida. Después relacionaremos referencias con `'a` en Functions, reconoceremos las reglas de Lifetime Elision y guardaremos referencias dentro de Structs. Finalmente conoceremos `'static`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Reference válida",
            "Una referencia solo puede utilizarse mientras el dato original sigue vivo.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "'a",
            "Una anotación que describe la relación entre el préstamo y los datos que referencia.",
        );

        card_overview(
            &mut columns[1],
            "Lifetime Elision",
            "Reglas automáticas que permiten omitir anotaciones en funciones sencillas.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "'static",
            "Una referencia válida durante toda la ejecución del programa.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Un Lifetime no hace que un valor viva más tiempo: le permite al compilador describir y comprobar cuánto tiempo puede utilizarse una Reference.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_dangling_references(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Dangling References");
    session_intro(
        ui,
        "Rust rechaza una referencia cuando el dato original termina antes que el préstamo.",
    );
    section_heading(ui, "El dato debe seguir vivo");
    texto_con_chips_inline(
        ui,
        "El `Scope` interno termina al cerrar el bloque. En ese momento `dato` deja de existir y una referencia externa ya no sería segura.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let referencia;
{
    let dato = 5;
    referencia = &dato;
}

// referencia ya no es válida aquí",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Regla práctica");
    texto_con_chips_inline(
        ui,
        "La referencia no puede sobrevivir al valor que presta. El Borrow Checker detecta el problema antes de ejecutar el programa.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_funciones_lifetimes(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Lifetimes en Functions");
    session_intro(
        ui,
        "Una anotación de Lifetime conecta las referencias de entrada con la referencia que retorna una función.",
    );
    section_heading(ui, "Relacionar entradas y salida");
    texto_con_chips_inline(
        ui,
        "En `mas_largo`, `'a` indica que el resultado será una vista de uno de los textos recibidos y no puede utilizarse después de que esos datos dejen de ser válidos.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Lifetime Elision");
    texto_con_chips_inline(
        ui,
        "Cuando solo existe una Reference de entrada, Rust puede escribir la relación automáticamente: `fn primera_palabra(texto: &str) -> &str`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_structs_lifetimes(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Lifetimes en Structs");
    session_intro(
        ui,
        "Una `Struct` con referencias debe declarar cómo se relaciona su contenido prestado con el dato original.",
    );
    section_heading(ui, "Guardar un préstamo");
    texto_con_chips_inline(
        ui,
        "`Extracto<'a>` no es dueño del texto: conserva una Reference. Por eso la instancia no puede vivir más que el texto original.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "struct Extracto<'a> {
    texto: &'a str,
}

impl<'a> Extracto<'a> {
    fn obtener(&self) -> &str {
        self.texto
    }
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn mostrar_static(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Lifetime 'static");
    session_intro(
        ui,
        "`'static` describe una referencia cuyos datos son válidos durante toda la ejecución del programa.",
    );
    section_heading(ui, "Literales y datos globales");
    texto_con_chips_inline(
        ui,
        "Un literal como `\"Rust\"` está disponible durante toda la ejecución porque forma parte del binario. `'static` no debe confundirse con una solución para cualquier referencia local.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let mensaje: &'static str = \"Rust\";
let otro: &str = mensaje;",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn preguntas_lifetimes() -> [SessionQuestion; 5] {
    [
        SessionQuestion {
            prompt: "1. ¿Qué evita un Lifetime?",
            options: [
                "Una referencia inválida",
                "Un número decimal",
                "Un archivo grande",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "2. ¿Qué describe `'a`?",
            options: [
                "Una relación entre referencias y datos",
                "Una variable mutable",
                "Un hilo",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "3. ¿Qué hace Lifetime Elision?",
            options: [
                "Permite omitir anotaciones en casos comunes",
                "Copia todos los datos",
                "Elimina el Borrow Checker",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "4. ¿Qué necesita una Struct que guarda una referencia?",
            options: [
                "Un parámetro de Lifetime",
                "Un Garbage Collector",
                "Una macro de impresión",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "5. ¿Qué significa `'static`?",
            options: [
                "Válido durante toda la ejecución",
                "Siempre mutable",
                "Guardado necesariamente en el Heap",
            ],
            correct: 0,
        },
    ]
}

fn mostrar_code_lab_lifetimes(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_lifetimes();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_lifetimes_codelab",
            separator_id: "lifetimes_editor_toolbar_sep_y",
            terminal_panel_id: "lifetimes_terminal_panel",
            editor_scroll_id: "lifetimes_codelab_editor",
            drawer_key: "lifetimes",
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
                    &mut state.lessons.lifetimes_preguntas_respuestas,
                    &preguntas_lifetimes(),
                    &["Lifetime", "'a", "Reference", "'static"],
                    "Comprueba si puedes reconocer la relación entre una Reference y el dato que presta.",
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
        ("Dangling Refs", 1),
        ("Functions 'a", 2),
        ("Structs & 'static", 3),
        ("Lifetimes", 4),
    ];
    let active = state.lessons.lifetimes_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Lifetimes",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.lifetimes_tab = idx,
    );
}
