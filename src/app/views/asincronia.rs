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

pub fn retos_asincronia() -> Vec<Reto> {
    vec![
        (
            "async fn y .await",
            "Async / Await",
            "Describir una operación asíncrona",
            "Una función `async` devuelve un `Future`. El cuerpo no comienza a ejecutarse hasta que un Runtime lo espera mediante `.await`.",
            "Declara una función `async` y señala el punto donde el programa espera su resultado.",
            "async fn obtener_saludo() -> String {
    String::from(\"Hola, Rust\")
}

let saludo = obtener_saludo().await;",
        ),
        (
            "Future perezoso",
            "Async / Await",
            "Esperar no es bloquear",
            "Un `Future` representa trabajo pendiente. Mientras espera I/O, el Runtime puede ejecutar otra tarea en el mismo hilo.",
            "Distingue la creación del Future del momento en que `.await` permite obtener su resultado.",
            "let futuro = obtener_datos();
// Todavía no se ha obtenido el resultado.
let datos = futuro.await;",
        ),
        (
            "Tokio Runtime",
            "Async / Await",
            "Ejecutar tareas asíncronas",
            "Rust aporta la sintaxis `async` y `await`, pero necesita un Runtime como Tokio para conducir los Futures y reaccionar cuando el I/O está listo.",
            "Configura el Runtime con `#[tokio::main]` y lanza una tarea con `tokio::spawn`.",
            "#[tokio::main]
async fn main() {
    let tarea = tokio::spawn(async {
        42
    });

    let resultado = tarea.await.unwrap();
}",
        ),
        (
            "join! y select!",
            "Async / Await",
            "Coordinar varios eventos",
            "`join!` espera varios Futures. `select!` reacciona al primero que termina, una herramienta útil para timeouts, cancelación y eventos alternativos.",
            "Elige `join!` cuando necesitas todos los resultados y `select!` cuando solo necesitas responder al primer evento disponible.",
            "let (uno, dos) = tokio::join!(tarea_a(), tarea_b());

tokio::select! {
    resultado = tarea_a() => { let _ = resultado; }
    _ = tokio::time::sleep(duracion) => { }
}",
        ),
        (
            "Async y Blocking",
            "Async / Await",
            "No bloquear el Runtime",
            "Una operación bloqueante detiene el hilo mientras espera. En código asíncrono conviene utilizar APIs compatibles con el Runtime o mover trabajo pesado a una tarea apropiada.",
            "Clasifica una operación como I/O asíncrono, cálculo intensivo o llamada bloqueante antes de elegir cómo ejecutarla.",
            "// I/O asíncrono: cede el control al Runtime
let respuesta = cliente.get(url).send().await?;

// Trabajo bloqueante: no lo mezcles sin planificarlo
let resultado = tokio::task::spawn_blocking(|| calcular());",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa `async fn`, `Future`, `.await`, Tokio, `join!`, `select!` y operaciones bloqueantes.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_asincronia(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.asincronia_tab == 5 {
        mostrar_code_lab_asincronia(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.asincronia_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_futures_await(ui, state),
                2 => mostrar_tokio_runtime(ui, state),
                3 => mostrar_join_select(ui, state),
                4 => mostrar_blocking(ui, state),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Async / Await");
    session_intro(
        ui,
        "Esta sesión explica cómo Rust representa trabajo pendiente y cómo un Runtime coordina operaciones de I/O sin bloquear un hilo por cada espera.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero conoceremos `async fn`, `Future` y `.await`. Después conectaremos esos conceptos con el Runtime de Tokio, coordinaremos tareas con `join!` y `select!` y distinguiremos I/O asíncrono de trabajo bloqueante.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Future",
            "Un valor que representa una operación pendiente y puede avanzar por etapas.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            ".await",
            "El punto en que una tarea espera su resultado y cede el control al Runtime.",
        );

        card_overview(
            &mut columns[1],
            "Tokio Runtime",
            "El ejecutor que conduce Futures y despierta tareas cuando el I/O está listo.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "join! y select!",
            "Formas de esperar todos los eventos o responder al primero que termine.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Async no significa crear un hilo por cada operación. Significa representar una espera para que el Runtime pueda aprovechar el hilo mientras el recurso externo responde.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_futures_await(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "async fn y .await");
    session_intro(
        ui,
        "Una función `async` describe una operación que puede detenerse y continuar después.",
    );
    section_heading(ui, "Crear y esperar");
    texto_con_chips_inline(
        ui,
        "La llamada crea un `Future`; `.await` espera su resultado dentro de otra función asíncrona. Sin un Runtime, el Future no tiene quién lo conduzca.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "async fn obtener_saludo() -> String {
    String::from(\"Hola, Rust\")
}

let saludo = obtener_saludo().await;",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "El punto de espera");
    texto_con_chips_inline(
        ui,
        "`.await` no es una llamada normal que bloquee todo el programa: permite que el Runtime avance otra tarea mientras esta operación espera I/O.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_tokio_runtime(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Tokio Runtime");
    session_intro(
        ui,
        "Tokio conduce los Futures y proporciona tareas, temporizadores, canales y herramientas de I/O asíncrono.",
    );
    section_heading(ui, "Iniciar y lanzar");
    texto_con_chips_inline(
        ui,
        "`#[tokio::main]` prepara el Runtime para `main`. `tokio::spawn` registra una tarea que puede avanzar junto con otras tareas asíncronas.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "#[tokio::main]
async fn main() {
    let tarea = tokio::spawn(async {
        42
    });

    let resultado = tarea.await.unwrap();
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn mostrar_join_select(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "join! y select!");
    session_intro(
        ui,
        "Estas macros coordinan varias operaciones asíncronas sin convertir cada operación en un hilo independiente.",
    );
    section_heading(ui, "Todos o el primero");
    texto_con_chips_inline(
        ui,
        "Usa `join!` cuando necesitas todos los resultados. Usa `select!` cuando necesitas reaccionar al primer evento, como un mensaje o un timeout.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let (uno, dos) = tokio::join!(tarea_a(), tarea_b());

tokio::select! {
    resultado = tarea_a() => { let _ = resultado; }
    _ = tokio::time::sleep(duracion) => { }
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn mostrar_blocking(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Async y Blocking");
    session_intro(
        ui,
        "El modelo asíncrono funciona mejor cuando las esperas ceden el control en lugar de bloquear el hilo del Runtime.",
    );
    section_heading(ui, "Elegir la herramienta");
    texto_con_chips_inline(
        ui,
        "Una petición de red asíncrona puede usar `.await`. Un cálculo pesado o una API bloqueante necesita otra estrategia para no detener las demás tareas.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let respuesta = cliente.get(url).send().await?;

let resultado = tokio::task::spawn_blocking(|| calcular());",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Regla práctica");
    texto_con_chips_inline(
        ui,
        "No conviertas todo en `async` por costumbre: identifica primero si esperas I/O, si haces cálculo o si llamas a una API bloqueante.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn preguntas_asincronia() -> [SessionQuestion; 5] {
    [
        SessionQuestion {
            prompt: "1. ¿Qué devuelve una async fn?",
            options: ["Un Future", "Un hilo obligatorio", "Un Cargo.lock"],
            correct: 0,
        },
        SessionQuestion {
            prompt: "2. ¿Cuándo se obtiene normalmente el resultado?",
            options: [
                "Al usar .await",
                "Al escribir el nombre de la función",
                "Al cerrar Cargo",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "3. ¿Qué aporta Tokio?",
            options: [
                "Un Runtime que conduce Futures",
                "Un nuevo lenguaje",
                "Un reemplazo de Ownership",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "4. ¿Cuándo usarías select!?",
            options: [
                "Para responder al primer evento disponible",
                "Para esperar siempre todos los resultados",
                "Para declarar una Struct",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "5. ¿Qué riesgo tiene una operación bloqueante dentro del Runtime?",
            options: [
                "Puede detener otras tareas",
                "Convierte todo en paralelo",
                "Elimina los Futures",
            ],
            correct: 0,
        },
    ]
}

fn mostrar_code_lab_asincronia(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_asincronia();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_asincronia_codelab",
            separator_id: "asincronia_editor_toolbar_sep_y",
            terminal_panel_id: "asincronia_terminal_panel",
            editor_scroll_id: "asincronia_codelab_editor",
            drawer_key: "asincronia",
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
                    &mut state.lessons.asincronia_preguntas_respuestas,
                    &preguntas_asincronia(),
                    &["Future", ".await", "Tokio", "join!", "select!"],
                    "Comprueba si puedes distinguir un Future, un Runtime y la forma adecuada de coordinar tareas.",
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
        ("Futures & .await", 1),
        ("Tokio Runtime", 2),
        ("join! & select!", 3),
        ("Blocking", 4),
    ];
    let active = state.lessons.asincronia_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Async / Await",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.asincronia_tab = idx,
    );
}
