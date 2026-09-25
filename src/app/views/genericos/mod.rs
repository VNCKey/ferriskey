use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

pub fn retos_genericos() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Generic Functions",
            "Generics",
            "Una función, varios tipos",
            "Una función genérica puede trabajar con distintos tipos sin escribir una función separada para cada uno.

El parámetro de tipo se escribe entre `<` y `>` y funciona como un nombre temporal para el tipo que llegará después.

La función conserva la seguridad de tipos: cada llamada concreta utiliza un tipo definido por el valor que recibe.

En esta primera etapa usaremos funciones que reciben un valor y devuelven ese mismo valor, sin agregar restricciones avanzadas.",
            "Crea una función genérica que reciba un valor y lo devuelva. Llámala con un entero y con un texto.",
            "fn identidad<T>(valor: T) -> T {
    valor
}

let numero = identidad(5);
let texto = identidad(\"Rust\");",
        ),
        (
            "Generic Structs",
            "Generics",
            "Un tipo para distintos valores",
            "Una `Struct` genérica puede guardar distintos tipos sin duplicar la definición de sus campos.

El parámetro de tipo se declara junto al nombre de la `Struct` y se utiliza en el campo que debe adaptarse.

Cada instancia concreta conoce el tipo que contiene. Una `Caja` puede guardar un entero en una instancia y un texto en otra.

Primero aprenderemos a declarar e instanciar el tipo. Los métodos genéricos y las restricciones se verán después, cuando sean necesarios.",
            "Define una `Struct` Caja<T> y crea una instancia para un entero y otra para un texto.",
            "struct Caja<T> {
    valor: T,
}

let numero = Caja { valor: 10 };
let texto = Caja { valor: \"Rust\" };",
        ),
        (
            "impl Trait",
            "Generics",
            "Una forma sencilla de aceptar comportamiento",
            "`impl Trait` permite indicar que una función acepta un valor que cumple un comportamiento, sin escribir el nombre del tipo concreto.

En esta introducción lo utilizaremos como parámetro de una función. El lector se concentra en el comportamiento que necesita la función.

El `Trait` sigue definiendo el contrato y `impl Trait` solo ofrece una forma más breve de expresar el parámetro.

No veremos todavía `Trait Bounds` combinados, cláusulas `where` ni múltiples restricciones. Esas formas se estudiarán más adelante.",
            "Crea un `Trait` Describible, implementa su comportamiento para una `Struct` y recibe esa `Struct` mediante `impl Trait`.",
            "trait Describible {
    fn describir(&self);
}

struct Mensaje;

impl Describible for Mensaje {
    fn describir(&self) {
        println!(\"Mensaje de Rust\");
    }
}

fn mostrar(item: impl Describible) {
    item.describir();
}",
        ),
        (
            "Type Inference",
            "Generics",
            "El compilador completa el tipo",
            "Rust suele inferir el tipo genérico a partir del valor que entregas a la función o a la `Struct`.

No necesitas escribir `<i32>` en cada llamada cuando el compilador puede deducirlo.

La inferencia no elimina la comprobación de tipos: solo evita repetir información que ya está clara.",
            "Llama a la misma función genérica con valores de tipos distintos y observa que Rust infiere cada caso.",
            "fn identidad<T>(valor: T) -> T {
    valor
}

let numero = identidad(5);
let texto = identidad(\"Rust\");",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa funciones genéricas, `Structs` genéricas, `impl Trait` e inferencia de tipos.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_genericos(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.genericos_tab == 5 {
        mostrar_code_lab_genericos(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.genericos_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_funciones_genericas(ui, state),
                2 => mostrar_structs_genericas(ui, state),
                3 => mostrar_impl_trait(ui, state),
                4 => mostrar_info_genericos(ui),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Generics");
    session_intro(
        ui,
        "Esta sesión explica cómo escribir código reutilizable para distintos tipos sin perder la comprobación de tipos de Rust.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero crearemos `Generic Functions` que reciben valores de distintos tipos. Después construiremos `Generic Structs` para guardar datos adaptables. Finalmente conoceremos `impl Trait` como una forma breve de pedir un comportamiento y revisaremos cómo funciona la `Type Inference`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Generic Functions",
            "Una sola función puede recibir distintos tipos mediante un parámetro como `T`.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Generic Structs",
            "Una definición de `Struct` puede reutilizarse con diferentes tipos de datos.",
        );

        card_overview(
            &mut columns[1],
            "impl Trait",
            "Permite pedir un comportamiento sin escribir el tipo concreto en el parámetro de una función.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Type Inference",
            "El compilador completa el tipo cuando la información de la llamada ya es suficiente.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Los Generics permiten reutilizar una forma de código. El tipo concreto cambia en cada uso, pero las reglas de Rust siguen comprobando que todo sea correcto.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_funciones_genericas(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Generic Functions");
    session_intro(
        ui,
        "Una función genérica describe una operación que puede utilizarse con más de un tipo.",
    );

    section_heading(ui, "Parámetro de tipo");
    texto_con_chips_inline(
        ui,
        "La letra `T` es un nombre temporal para el tipo que recibirá la función. No es un tipo concreto: en cada llamada Rust sustituye `T` por el tipo real del valor.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "fn identidad<T>(valor: T) -> T {
    valor
}

let numero = identidad(5);
let texto = identidad(\"Rust\");",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );

    ui.add_space(16.0);
    section_heading(ui, "Una función, varios usos");
    texto_con_chips_inline(
        ui,
        "La misma definición trabaja con el entero y con el texto porque solo devuelve el valor recibido. Rust conserva un tipo concreto diferente en cada llamada.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_structs_genericas(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Generic Structs");
    session_intro(
        ui,
        "Una `Struct` genérica puede guardar distintos tipos de datos sin repetir toda su definición.",
    );

    section_heading(ui, "Declarar e instanciar");
    texto_con_chips_inline(
        ui,
        "El parámetro `T` se declara junto al nombre de la `Struct` y se utiliza en el campo adaptable. Cada instancia concreta indica el tipo mediante el valor que guarda.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "struct Caja<T> {
    valor: T,
}

let numero = Caja { valor: 10 };
let texto = Caja { valor: \"Rust\" };",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );

    ui.add_space(16.0);
    section_heading(ui, "La forma se reutiliza");
    texto_con_chips_inline(
        ui,
        "`Caja<i32>` y `Caja<&str>` representan instancias distintas de la misma idea. En esta etapa nos enfocamos en la declaración y la creación; todavía no necesitamos agregar métodos genéricos.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_impl_trait(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "impl Trait");
    session_intro(
        ui,
        "`impl Trait` permite recibir un valor por el comportamiento que ofrece, sin escribir el tipo concreto en la firma corta de una función.",
    );

    section_heading(ui, "Pedir comportamiento");
    texto_con_chips_inline(
        ui,
        "El `Trait` define el contrato. La expresión `impl Trait` indica que el parámetro puede ser cualquier tipo que cumpla ese contrato.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "trait Describible {
    fn describir(&self);
}

struct Mensaje;

impl Describible for Mensaje {
    fn describir(&self) {
        println!(\"Mensaje de Rust\");
    }
}

fn mostrar(item: impl Describible) {
    item.describir();
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );

    ui.add_space(16.0);
    section_heading(ui, "Alcance de esta introducción");
    texto_con_chips_inline(
        ui,
        "Aquí aprenderás la forma sencilla como parámetro. Los `Trait Bounds` combinados, `where` y los casos avanzados de `impl Trait` se dejarán para una etapa posterior.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_info_genericos(ui: &mut egui::Ui) {
    session_title(ui, "Info & Limits");
    session_intro(
        ui,
        "Los Generics son una herramienta amplia. Esta pestaña marca qué conceptos ya puedes usar y cuáles conviene reservar para después.",
    );

    section_heading(ui, "Lo que ya puedes reconocer");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "<T>",
            "Un parámetro de tipo que representa un tipo concreto en cada uso.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Type Inference",
            "La capacidad del compilador para deducir el tipo a partir de los valores usados.",
        );

        card_overview(
            &mut columns[1],
            "impl Trait",
            "Una forma breve de pedir un comportamiento en un parámetro.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Siguiente etapa",
            "Trait Bounds, múltiples restricciones, where y métodos sobre tipos genéricos.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea importante");
    texto_con_chips_inline(
        ui,
        "No necesitas aprender toda la sintaxis genérica en una sola sesión. Primero entiende cómo se reemplaza `T`; después podrás agregar reglas cuando una operación necesite capacidades específicas.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_code_lab_genericos(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_genericos();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_genericos_codelab",
            separator_id: "genericos_editor_toolbar_sep_y",
            terminal_panel_id: "genericos_terminal_panel",
            editor_scroll_id: "genericos_codelab_editor",
            drawer_key: "genericos",
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
                mostrar_preguntas_genericos(ui, state, orange);
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

fn mostrar_preguntas_genericos(ui: &mut egui::Ui, state: &mut AppState, orange: egui::Color32) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas: [(&str, [&str; 3], usize); 5] = [
        (
            "1. ¿Qué representa T en una función genérica?",
            ["Un tipo temporal que se concreta en cada uso", "Una variable global", "Un valor fijo"],
            0,
        ),
        (
            "2. ¿Qué permite una Generic Struct?",
            ["Guardar distintos tipos usando una misma definición", "Eliminar todos los campos", "Evitar la compilación"],
            0,
        ),
        (
            "3. ¿Qué puede inferir Rust?",
            ["El tipo concreto de una llamada cuando hay información suficiente", "El nombre de cualquier archivo", "La intención del usuario"],
            0,
        ),
        (
            "4. ¿Qué expresa impl Trait en esta sesión?",
            ["Un parámetro que cumple un comportamiento", "Una variable mutable", "Un lifetime"],
            0,
        ),
        (
            "5. ¿Qué estudiaremos después?",
            ["Trait Bounds y restricciones combinadas", "Cómo eliminar los tipos", "Cómo evitar toda función"],
            0,
        ),
    ];

    ui.add_space(4.0);
    ui.heading(
        RichText::new("5. Questions")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        inline_code_chip_color(ui, "Evaluation", egui::Color32::from_rgb(0, 200, 120));
        for tag in ["<T>", "Struct<T>", "impl Trait", "Inference"] {
            ui.add_space(4.0);
            inline_code_chip(ui, tag);
        }
    });
    ui.add_space(12.0);
    ui.label(
        RichText::new("Comprueba si puedes reconocer la función de cada forma genérica sin mezclar todavía restricciones avanzadas.")
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
    );
    ui.add_space(14.0);
    crate::views::pilares::anatomy::titulo_seccion(ui, "Comprueba lo aprendido", orange);
    ui.add_space(8.0);

    for (index, (pregunta, opciones, correcta)) in preguntas.iter().enumerate() {
        ui.label(
            RichText::new(*pregunta)
                .strong()
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(4.0);

        for (opcion_index, opcion) in opciones.iter().enumerate() {
            let seleccionada =
                state.lessons.genericos_preguntas_respuestas[index] == Some(opcion_index);
            let (rect, respuesta) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), 25.0),
                egui::Sense::click(),
            );
            let hovered = respuesta.hovered();
            let fill = if seleccionada {
                egui::Color32::from_rgba_unmultiplied(255, 160, 50, 32)
            } else if hovered {
                egui::Color32::from_rgba_unmultiplied(255, 160, 50, 18)
            } else {
                egui::Color32::from_rgb(15, 20, 30)
            };
            let stroke = if seleccionada {
                orange
            } else if hovered {
                egui::Color32::from_rgba_unmultiplied(255, 180, 80, 150)
            } else {
                egui::Color32::from_rgb(35, 48, 70)
            };
            ui.painter().rect(
                rect,
                egui::CornerRadius::same(4),
                fill,
                egui::Stroke::new(if seleccionada { 1.2 } else { 1.0 }, stroke),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                egui::pos2(rect.left() + 10.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                *opcion,
                egui::FontId::proportional(12.5),
                if seleccionada { orange } else { text_col },
            );
            if respuesta.clicked() {
                state.lessons.genericos_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.genericos_preguntas_respuestas[index] {
            let es_correcta = respuesta == *correcta;
            ui.label(
                RichText::new(if es_correcta {
                    "Correcto"
                } else {
                    "Revisa este concepto y vuelve a intentarlo"
                })
                .size(11.5)
                .color(if es_correcta { orange } else { text_col }),
            );
        }
        ui.add_space(10.0);
    }

    let respondidas = state
        .lessons
        .genericos_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.genericos_preguntas_respuestas[*index] == Some(*correcta)
        })
        .count();
    let completada = respondidas == preguntas.len() && correctas == preguntas.len();
    crate::views::pilares::anatomy::titulo_seccion(
        ui,
        if completada {
            "Sesión completada"
        } else {
            "Progreso de la evaluación"
        },
        orange,
    );
    ui.label(
        RichText::new(if completada {
            "¡Excelente! Has respondido correctamente las cinco preguntas.".to_owned()
        } else {
            format!("Respuestas correctas: {correctas}/5 · Respondidas: {respondidas}/5")
        })
        .size(13.0)
        .color(if completada { orange } else { text_col }),
    );
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("Generic Functions", 1),
        ("Generic Structs", 2),
        ("impl Trait", 3),
        ("Info & Limits", 4),
    ];
    let active = state.lessons.genericos_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Generics",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.genericos_tab = idx,
    );
}
