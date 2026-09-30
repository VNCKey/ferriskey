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

pub fn retos_concurrencia() -> Vec<Reto> {
    vec![
        (
            "Threads",
            "Concurrency",
            "Ejecutar trabajo en otro hilo",
            "`thread::spawn` crea un hilo del sistema operativo. Una `Closure` con `move` puede llevar al nuevo hilo el Ownership que necesita.",
            "Crea un hilo, devuelve un valor y espera su finalización mediante `join()`.",
            "use std::thread;

let handle = thread::spawn(|| {
    42
});

let resultado = handle.join().unwrap();",
        ),
        (
            "Message Passing",
            "Concurrency",
            "Comunicar hilos con canales",
            "Un canal separa al emisor del receptor. `mpsc` significa Multiple Producer, Single Consumer: varios hilos pueden enviar y uno puede recibir.",
            "Envía un mensaje desde un hilo y recíbelo en el hilo actual.",
            "use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(\"mensaje\").unwrap();
});

let recibido = rx.recv().unwrap();",
        ),
        (
            "Shared State",
            "Concurrency",
            "Compartir y proteger un dato",
            "`Arc<Mutex<T>>` permite que varios hilos sean propietarios de un dato y que solo uno lo modifique a la vez.",
            "Comparte un contador, adquiere el `Mutex` durante el cambio y deja terminar el préstamo antes de continuar.",
            "use std::sync::{Arc, Mutex};

let contador = Arc::new(Mutex::new(0));
let copia = Arc::clone(&contador);

{
    let mut valor = copia.lock().unwrap();
    *valor += 1;
}",
        ),
        (
            "Send y Sync",
            "Concurrency",
            "Reglas para cruzar hilos",
            "`Send` indica que un valor puede transferirse a otro hilo. `Sync` indica que sus referencias pueden compartirse de forma segura entre hilos.",
            "Relaciona `move`, `Send` y `Sync` con la regla que el compilador necesita comprobar antes de permitir el acceso concurrente.",
            "// La propiedad se transfiere al hilo
let mensaje = String::from(\"Rust\");
std::thread::spawn(move || {
    let _ = mensaje;
});",
        ),
        (
            "Data Race y bloqueo",
            "Concurrency",
            "Conservar una sincronización clara",
            "Rust evita Data Races en código seguro, pero un diseño concurrente todavía puede esperar demasiado o crear un Deadlock si mantiene bloqueos en un orden incorrecto.",
            "Mantén el Scope del `MutexGuard` lo más pequeño posible y evita adquirir dos bloqueos en órdenes diferentes.",
            "let primero = recurso_a.lock().unwrap();
// Usa el dato y libera el guard pronto.
drop(primero);

let segundo = recurso_b.lock().unwrap();",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa Threads, `move`, canales, `Arc<Mutex<T>>`, `Send` y `Sync`.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_concurrencia(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.concurrencia_tab == 5 {
        mostrar_code_lab_concurrencia(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.concurrencia_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_threads(ui, state),
                2 => mostrar_channels(ui, state),
                3 => mostrar_shared_state(ui, state),
                4 => mostrar_send_sync(ui, state),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Concurrency");
    session_intro(
        ui,
        "Esta sesión explica cómo ejecutar trabajo concurrente y cómo Rust protege los datos compartidos entre hilos.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero crearemos `Threads` y esperaremos su resultado. Después enviaremos mensajes mediante canales, compartiremos estado con `Arc<Mutex<T>>` y cerraremos con `Send`, `Sync` y las reglas para evitar Data Races.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Threads",
            "Un hilo permite ejecutar una parte del trabajo de forma independiente.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Message Passing",
            "Los canales comunican hilos sin exponer directamente la memoria compartida.",
        );

        card_overview(
            &mut columns[1],
            "Shared State",
            "`Arc<Mutex<T>>` combina propiedad compartida con acceso exclusivo temporal.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Send y Sync",
            "Traits que describen qué valores pueden moverse o compartirse entre hilos.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Rust no elimina la concurrencia: hace que las reglas de Ownership y Borrowing también protejan los datos cuando participan varios hilos.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_threads(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Threads");
    session_intro(
        ui,
        "`thread::spawn` inicia una tarea en un hilo del sistema operativo y devuelve un `JoinHandle`.",
    );
    section_heading(ui, "Crear y esperar");
    texto_con_chips_inline(
        ui,
        "`join()` espera al hilo y recupera su resultado. Si la Closure necesita Ownership de una variable externa, se utiliza `move`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "use std::thread;

let handle = thread::spawn(|| {
    42
});

let resultado = handle.join().unwrap();",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn mostrar_channels(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Message Passing");
    session_intro(
        ui,
        "Un canal ofrece un camino seguro para enviar valores desde un hilo hacia otro.",
    );
    section_heading(ui, "Emisor y receptor");
    texto_con_chips_inline(
        ui,
        "`tx.send()` envía el dato y `rx.recv()` espera hasta recibirlo. El valor enviado cambia de propietario al cruzar el canal.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(\"mensaje\").unwrap();
});

let recibido = rx.recv().unwrap();",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn mostrar_shared_state(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Shared State");
    session_intro(
        ui,
        "Cuando varios hilos necesitan modificar el mismo dato, hay que compartir el propietario y proteger el acceso.",
    );
    section_heading(ui, "Arc y Mutex");
    texto_con_chips_inline(
        ui,
        "`Arc<T>` comparte Ownership entre hilos. `Mutex<T>` garantiza que solo un hilo tenga el acceso mutable durante el préstamo obtenido con `lock()`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "use std::sync::{Arc, Mutex};

let contador = Arc::new(Mutex::new(0));
let copia = Arc::clone(&contador);

{
    let mut valor = copia.lock().unwrap();
    *valor += 1;
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Scope del bloqueo");
    texto_con_chips_inline(
        ui,
        "Mantén el `MutexGuard` durante el menor tiempo posible. Un Scope pequeño reduce la espera y hace más fácil razonar sobre el orden de los bloqueos.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_send_sync(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Send y Sync");
    session_intro(
        ui,
        "`Send` y `Sync` son Traits que el compilador utiliza para validar el paso de valores y referencias entre hilos.",
    );
    section_heading(ui, "Dos preguntas diferentes");
    texto_con_chips_inline(
        ui,
        "`Send` pregunta si el Ownership puede transferirse. `Sync` pregunta si varias referencias pueden compartirse de forma segura. Tipos como `Rc<T>` no están diseñados para cruzar hilos.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let mensaje = String::from(\"Rust\");
std::thread::spawn(move || {
    let _ = mensaje;
});",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Regla práctica");
    texto_con_chips_inline(
        ui,
        "La seguridad de tipos evita Data Races en código seguro, pero todavía debes diseñar con cuidado los bloqueos para evitar esperas innecesarias o Deadlocks.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn preguntas_concurrencia() -> [SessionQuestion; 5] {
    [
        SessionQuestion {
            prompt: "1. ¿Qué devuelve thread::spawn?",
            options: ["Un JoinHandle", "Un Cargo.toml", "Un String global"],
            correct: 0,
        },
        SessionQuestion {
            prompt: "2. ¿Qué hace join()?",
            options: [
                "Espera al hilo y obtiene su resultado",
                "Crea otro crate",
                "Libera todos los hilos",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "3. ¿Qué permite un canal mpsc?",
            options: [
                "Enviar mensajes entre hilos",
                "Modificar cualquier dato sin reglas",
                "Reemplazar un Mutex siempre",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "4. ¿Qué combina Arc<Mutex<T>>?",
            options: [
                "Ownership compartido y acceso protegido",
                "Dos funciones main",
                "Un Array y una Tuple",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "5. ¿Qué describe Send?",
            options: [
                "Que el Ownership puede transferirse entre hilos",
                "Que un valor es siempre mutable",
                "Que un hilo nunca espera",
            ],
            correct: 0,
        },
    ]
}

fn mostrar_code_lab_concurrencia(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_concurrencia();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_concurrencia_codelab",
            separator_id: "concurrencia_editor_toolbar_sep_y",
            terminal_panel_id: "concurrencia_terminal_panel",
            editor_scroll_id: "concurrencia_codelab_editor",
            drawer_key: "concurrencia",
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
                    &mut state.lessons.concurrencia_preguntas_respuestas,
                    &preguntas_concurrencia(),
                    &["Threads", "mpsc", "Arc", "Mutex", "Send", "Sync"],
                    "Comprueba si puedes distinguir ejecución paralela, comunicación y estado compartido.",
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
        ("Threads", 1),
        ("Message Passing", 2),
        ("Shared State", 3),
        ("Send & Sync", 4),
    ];
    let active = state.lessons.concurrencia_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Concurrency",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.concurrencia_tab = idx,
    );
}
