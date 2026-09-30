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

pub fn retos_smart_pointers() -> Vec<Reto> {
    vec![
        (
            "Box<T>",
            "Smart Pointers",
            "Guardar datos en el Heap",
            "`Box<T>` guarda un puntero de tamaño conocido en el Stack y coloca el valor real en el Heap. Es útil cuando el dato es grande o recursivo.",
            "Crea un `Box` y accede al valor que contiene. Después observa cómo permite declarar una lista recursiva.",
            "let numero = Box::new(42);
let valor = *numero;

enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}",
        ),
        (
            "Rc<T> y Arc<T>",
            "Smart Pointers",
            "Compartir Ownership",
            "`Rc<T>` permite varios propietarios en un solo hilo. `Arc<T>` utiliza un contador atómico cuando el valor se comparte entre hilos.",
            "Crea dos propietarios de lectura y observa el contador con `strong_count`.",
            "use std::rc::Rc;

let dato = Rc::new(String::from(\"Rust\"));
let copia_a = Rc::clone(&dato);
let copia_b = Rc::clone(&dato);
let total = Rc::strong_count(&dato);",
        ),
        (
            "RefCell<T>",
            "Smart Pointers",
            "Mutabilidad interior",
            "`RefCell<T>` permite modificar el valor interior mediante reglas comprobadas en Runtime. Si se rompe una regla de Borrowing, el programa entra en `panic!`.",
            "Obtén un préstamo mutable, modifica el valor y deja terminar su Scope antes de pedir otro préstamo.",
            "use std::cell::RefCell;

let contador = RefCell::new(10);
{
    let mut valor = contador.borrow_mut();
    *valor += 5;
}
let resultado = *contador.borrow();",
        ),
        (
            "Rc<RefCell<T>>",
            "Smart Pointers",
            "Propiedad compartida y cambio",
            "La combinación `Rc<RefCell<T>>` permite varios propietarios en un solo hilo y mutabilidad interior. Es útil en estructuras compartidas, pero debe usarse con una intención clara.",
            "Comparte una celda entre dos propietarios y modifica el mismo valor desde uno de ellos.",
            "use std::cell::RefCell;
use std::rc::Rc;

let estado = Rc::new(RefCell::new(0));
let referencia = Rc::clone(&estado);
*referencia.borrow_mut() += 1;",
        ),
        (
            "Deref y Drop",
            "Smart Pointers",
            "Acceso y limpieza de recursos",
            "`Deref` permite que un tipo se comporte como una referencia al valor interior. `Drop` describe la limpieza automática cuando el propietario sale de su Scope.",
            "Relaciona el acceso mediante `*` con el valor interior y recuerda que Rust libera el recurso cuando termina su propietario.",
            "let dato = Box::new(String::from(\"Rust\"));
let referencia: &String = &dato;
let texto: &str = referencia;

// dato se libera al terminar su Scope",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>` y la combinación de propiedad compartida con mutabilidad interior.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_smart_pointers(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.smart_pointers_tab == 5 {
        mostrar_code_lab_smart_pointers(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.smart_pointers_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_box(ui, state),
                2 => mostrar_rc_arc(ui, state),
                3 => mostrar_refcell(ui, state),
                4 => mostrar_deref_drop(ui, state),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Smart Pointers");
    session_intro(
        ui,
        "Esta sesión presenta tipos que envuelven un valor y añaden una forma concreta de almacenarlo, compartirlo o modificarlo.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero ubicaremos datos en el Heap con `Box<T>`. Después veremos Ownership compartido con `Rc<T>` y `Arc<T>`, mutabilidad interior con `RefCell<T>` y el comportamiento de `Deref` y `Drop`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Box<T>",
            "Un propietario que guarda el valor en el Heap y conserva un puntero de tamaño conocido.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Rc<T> y Arc<T>",
            "Conteo de propietarios para compartir lectura en uno o varios hilos.",
        );

        card_overview(
            &mut columns[1],
            "RefCell<T>",
            "Traslada la comprobación del Borrowing al Runtime y permite Interior Mutability.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Deref y Drop",
            "Acceso transparente al valor interior y limpieza automática al terminar el Scope.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Un Smart Pointer no elimina Ownership: lo organiza de otra forma. Cada tipo expresa si existe un propietario, varios propietarios o un permiso especial de modificación.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_box(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Box<T>");
    session_intro(
        ui,
        "`Box<T>` coloca el valor en el Heap y conserva un puntero propio en el Stack.",
    );
    section_heading(ui, "Crear y acceder");
    texto_con_chips_inline(
        ui,
        "El `Box` sigue teniendo un único propietario. El operador `*` permite acceder al valor que guarda la caja.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let numero = Box::new(42);
let valor = *numero;",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Tipos recursivos");
    texto_con_chips_inline(
        ui,
        "Un tipo recursivo necesita una indirección para tener un tamaño conocido. `Box<Lista>` crea ese límite entre un nodo y el siguiente.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_rc_arc(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Rc<T> y Arc<T>");
    session_intro(
        ui,
        "Estos tipos permiten compartir Ownership mediante un contador de referencias.",
    );
    section_heading(ui, "Elegir el contador");
    texto_con_chips_inline(
        ui,
        "Usa `Rc<T>` cuando todo ocurre en un solo hilo. Usa `Arc<T>` cuando el valor debe cruzar límites de hilos de forma segura.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "use std::rc::Rc;

let dato = Rc::new(String::from(\"Rust\"));
let copia_a = Rc::clone(&dato);
let copia_b = Rc::clone(&dato);
let total = Rc::strong_count(&dato);",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn mostrar_refcell(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "RefCell<T>");
    session_intro(
        ui,
        "`RefCell<T>` permite modificar el valor interior cuando el diseño necesita Interior Mutability.",
    );
    section_heading(ui, "Préstamos en Runtime");
    texto_con_chips_inline(
        ui,
        "`borrow_mut()` obtiene un préstamo mutable y `borrow()` obtiene uno inmutable. El préstamo debe terminar antes de solicitar otro incompatible.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "use std::cell::RefCell;

let contador = RefCell::new(10);
{
    let mut valor = contador.borrow_mut();
    *valor += 5;
}
let resultado = *contador.borrow();",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);
    section_heading(ui, "Advertencia");
    texto_con_chips_inline(
        ui,
        "Si dos préstamos incompatibles están activos al mismo tiempo, `RefCell<T>` provoca un `panic!` en Runtime. La flexibilidad tiene un coste de comprobación.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_deref_drop(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Deref y Drop");
    session_intro(
        ui,
        "Dos Traits de la biblioteca estándar explican cómo un Smart Pointer se comporta y cómo libera sus recursos.",
    );
    section_heading(ui, "Acceso y limpieza");
    texto_con_chips_inline(
        ui,
        "`Deref` permite acceder al contenido como una referencia. `Drop` se ejecuta cuando el propietario sale de su `Scope`; en la mayoría de los casos Rust realiza esa limpieza automáticamente.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let dato = Box::new(String::from(\"Rust\"));
let referencia: &String = &dato;
let texto: &str = referencia;

// dato se libera al terminar su Scope",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
}

fn preguntas_smart_pointers() -> [SessionQuestion; 5] {
    [
        SessionQuestion {
            prompt: "1. ¿Dónde guarda Box<T> el valor real?",
            options: ["En el Heap", "Siempre en un registro", "En Cargo.lock"],
            correct: 0,
        },
        SessionQuestion {
            prompt: "2. ¿Cuándo conviene Rc<T>?",
            options: [
                "Para compartir en un solo hilo",
                "Para acceder desde cualquier hilo",
                "Para reemplazar todos los Strings",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "3. ¿Qué añade Arc<T>?",
            options: [
                "Conteo atómico entre hilos",
                "Mutabilidad sin reglas",
                "Un compilador nuevo",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "4. ¿Cuándo comprueba RefCell<T> sus préstamos?",
            options: ["En Runtime", "Solo al escribir Cargo.toml", "Nunca"],
            correct: 0,
        },
        SessionQuestion {
            prompt: "5. ¿Qué ocurre al terminar el Scope del propietario?",
            options: [
                "El recurso puede liberarse automáticamente",
                "El valor se vuelve global",
                "Se desactiva el Ownership",
            ],
            correct: 0,
        },
    ]
}

fn mostrar_code_lab_smart_pointers(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_smart_pointers();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_smart_pointers_codelab",
            separator_id: "smart_pointers_editor_toolbar_sep_y",
            terminal_panel_id: "smart_pointers_terminal_panel",
            editor_scroll_id: "smart_pointers_codelab_editor",
            drawer_key: "smart_pointers",
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
                    &mut state.lessons.smart_pointers_preguntas_respuestas,
                    &preguntas_smart_pointers(),
                    &["Box<T>", "Rc<T>", "Arc<T>", "RefCell<T>"],
                    "Comprueba si puedes distinguir almacenamiento, propiedad compartida y mutabilidad interior.",
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
        ("Box<T>", 1),
        ("Rc / Arc", 2),
        ("RefCell<T>", 3),
        ("Deref & Drop", 4),
    ];
    let active = state.lessons.smart_pointers_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Smart Pointers",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.smart_pointers_tab = idx,
    );
}
