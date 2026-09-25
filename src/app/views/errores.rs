use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

pub fn retos_errores() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "panic!",
            "Error Handling",
            "Fallo irrecuperable",
            "`panic!` detiene la ejecución cuando el programa llega a un estado que no puede continuar.

No es la respuesta habitual para un dato que el usuario puede corregir. Para esos casos normalmente conviene devolver un error recuperable.

Las macros `assert!` y `debug_assert!` también pueden detener el programa cuando una condición que debía cumplirse resulta falsa.",
            "Provoca un `panic!` de forma intencional y observa que la ejecución se detiene en ese punto.",
            "panic!(\"El estado no puede continuar\");",
        ),
        (
            "Result",
            "Error Handling",
            "Éxito o error recuperable",
            "`Result` representa una operación que puede terminar correctamente o fallar de una forma que el programa puede tratar.

Su camino de éxito se representa con `Ok` y su camino de error con `Err`.

Muchas operaciones de la biblioteca estándar devuelven un `Result`, como leer un archivo que quizá no exista.

El error no se oculta: queda en el valor y el código debe decidir qué hacer con él.",
            "Lee un archivo y conserva el resultado para decidir después si la operación terminó con `Ok` o con `Err`.",
            "let lectura = std::fs::read_to_string(\"config.txt\");

match lectura {
    Ok(contenido) => println!(\"{contenido}\"),
    Err(error) => println!(\"No se pudo leer: {error}\"),
}",
        ),
        (
            "Match de Result",
            "Error Handling",
            "Decidir según el resultado",
            "`match` permite separar claramente el camino de éxito y el camino de error.

En la rama `Ok`, el patrón recibe el valor que la operación produjo.

En la rama `Err`, el patrón recibe la información del fallo y puedes mostrarla, registrarla o elegir una alternativa.",
            "Escribe una rama para `Ok` y otra para `Err`. Comprueba que ambos caminos tengan una respuesta clara.",
            "let resultado = std::fs::read_to_string(\"config.txt\");

match resultado {
    Ok(texto) => println!(\"Archivo: {texto}\"),
    Err(error) => println!(\"Usaremos una configuración alternativa: {error}\"),
}",
        ),
        (
            "Operador ?",
            "Error Handling",
            "Propagación temprana",
            "El operador `?` comprueba un `Result` sin escribir un `match` completo.

Si encuentra `Ok`, extrae su valor y permite continuar con la siguiente línea.

Si encuentra `Err`, termina pronto la función actual y entrega ese error a quien la llamó.

La función que utiliza `?` debe estar preparada para devolver un resultado compatible. En este paso nos concentramos en el recorrido del error.",
            "Coloca esta línea dentro de una función que devuelva un resultado. Si el archivo falla, `?` propaga el error automáticamente.",
            "let contenido = std::fs::read_to_string(\"config.txt\")?;
println!(\"{contenido}\");",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa `panic!`, `Result`, `Ok`, `Err`, `match` y el operador `?`.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_errores(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.errores_tab == 5 {
        mostrar_code_lab_errores(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.errores_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_panic(ui, state),
                2 => mostrar_result(ui, state),
                3 => mostrar_operador_pregunta(ui, state),
                4 => mostrar_diseno_errores(ui),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Error Handling");
    session_intro(
        ui,
        "Esta sesión explica cómo Rust hace visibles los fallos y obliga al programa a decidir qué hacer con ellos, sin ocultar errores detrás de excepciones inesperadas.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero diferenciaremos un fallo irrecuperable con `panic!` de un error que el programa puede tratar. Después conoceremos `Result`, sus caminos `Ok` y `Err`, y la forma de elegir una respuesta con `match`. Finalmente usaremos el operador `?` para propagar un error sin repetir el mismo patrón en cada línea.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "panic!",
            "Detiene la ejecución cuando continuar sería inseguro o el estado del programa es imposible.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Result",
            "Representa una operación con dos caminos: éxito mediante Ok o fallo recuperable mediante Err.",
        );

        card_overview(
            &mut columns[1],
            "match y ?",
            "match permite decidir explícitamente; ? propaga un error a la función que llamó sin escribir un match repetido.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Decisión del programa",
            "Un error no es solo un mensaje: es información que puedes mostrar, registrar, corregir o propagar.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Rust te obliga a hacer visible el camino del error. Así puedes distinguir entre detener el programa porque el estado es imposible y continuar con una respuesta controlada cuando una operación puede fallar.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_panic(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "panic!");
    session_intro(
        ui,
        "`panic!` representa un fallo irrecuperable: el programa no puede continuar de forma segura desde ese estado.",
    );

    section_heading(ui, "Cuándo aparece");
    texto_con_chips_inline(
        ui,
        "Puedes provocar un `panic!` de forma explícita o llegar a él mediante una operación que no puede cumplir sus reglas. Úsalo para estados imposibles, no como respuesta normal a datos que pueden estar mal escritos.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(14.0);

    ui.columns(2, |columns| {
        columns[0].label(
            RichText::new("Fallo explícito")
                .font(Typography::card_title())
                .color(Colors::ORANGE_RUST),
        );
        columns[0].add_space(6.0);
        texto_con_chips_inline(
            &mut columns[0],
            "La macro `panic!` detiene el flujo y muestra el mensaje indicado.",
            Colors::TEXT_PRIMARY,
            Colors::CYAN_ACCENT,
        );
        columns[0].add_space(8.0);
        highlighted_code_block(
            &mut columns[0],
            "panic!(\"El estado no puede continuar\");",
            &state.editor.syntax_set,
            &state.editor.theme_set.themes["base16-ocean.dark"],
            "rs",
        );

        columns[1].label(
            RichText::new("Condición que debía cumplirse")
                .font(Typography::card_title())
                .color(Colors::ORANGE_RUST),
        );
        columns[1].add_space(6.0);
        texto_con_chips_inline(
            &mut columns[1],
            "`assert!` detiene el programa cuando una condición importante resulta falsa.",
            Colors::TEXT_PRIMARY,
            Colors::CYAN_ACCENT,
        );
        columns[1].add_space(8.0);
        highlighted_code_block(
            &mut columns[1],
            "let edad = 20;
assert!(edad >= 18, \"La edad no permite continuar\");",
            &state.editor.syntax_set,
            &state.editor.theme_set.themes["base16-ocean.dark"],
            "rs",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Regla práctica");
    texto_con_chips_inline(
        ui,
        "Si el usuario, un archivo o una red pueden producir el fallo, normalmente no conviene usar `panic!`: es mejor devolver un `Result` y decidir cómo recuperarse.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_result(ui: &mut egui::Ui, state: &mut AppState) {
    session_title(ui, "Result");
    session_intro(
        ui,
        "`Result` permite representar una operación que puede terminar bien o fallar de forma controlada.",
    );

    section_heading(ui, "Dos caminos explícitos");
    texto_con_chips_inline(
        ui,
        "El camino `Ok` contiene el resultado correcto. El camino `Err` contiene información sobre el fallo. Rust no te permite olvidar que la operación puede haber fallado.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let lectura = std::fs::read_to_string(\"config.txt\");

match lectura {
    Ok(contenido) => println!(\"{contenido}\"),
    Err(error) => println!(\"No se pudo leer: {error}\"),
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );

    ui.add_space(16.0);
    section_heading(ui, "Simula el resultado");
    ui.checkbox(
        &mut state.lessons.err_pipeline_fail,
        "Simular que la operación termina con Err",
    );
    ui.add_space(8.0);
    let (estado, descripcion) = if state.lessons.err_pipeline_fail {
        ("Err", "La operación falló y el programa puede decidir qué respuesta ofrecer.")
    } else {
        ("Ok", "La operación terminó correctamente y el programa puede utilizar su valor.")
    };
    ui.horizontal_wrapped(|ui| {
        inline_code_chip_color(ui, estado, Colors::ORANGE_RUST);
        ui.label(
            RichText::new(descripcion)
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
    });

    ui.add_space(16.0);
    section_heading(ui, "No ignores el error");
    texto_con_chips_inline(
        ui,
        "Cuando recibas un `Result`, decide si mostrarás un mensaje, usarás un valor alternativo, volverás a intentar la operación o propagarás el error.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_operador_pregunta(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Operator ?");
    session_intro(
        ui,
        "El operador `?` reduce el código repetido cuando una función solo debe continuar si cada operación termina correctamente.",
    );

    section_heading(ui, "El recorrido corto");
    texto_con_chips_inline(
        ui,
        "Cuando la expresión produce `Ok`, `?` extrae el valor. Cuando produce `Err`, `?` termina pronto la función actual y entrega ese error a quien la llamó.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "let contenido = std::fs::read_to_string(\"config.txt\")?;
println!(\"{contenido}\");",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );

    ui.add_space(14.0);
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Si llega Ok",
            "La variable recibe el contenido y la siguiente línea puede ejecutarse.",
        );
        card_overview(
            &mut columns[1],
            "Si llega Err",
            "La función termina pronto y el error continúa hacia su llamador.",
        );
    });

    ui.add_space(16.0);
    section_heading(ui, "Requisito importante");
    texto_con_chips_inline(
        ui,
        "La función que utiliza `?` debe poder devolver un resultado compatible. Aquí nos concentramos en entender el flujo; más adelante veremos cómo escribir firmas más completas.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_diseno_errores(ui: &mut egui::Ui) {
    session_title(ui, "Error Design");
    session_intro(
        ui,
        "Manejar errores no consiste solo en hacer que el programa compile: también implica elegir una respuesta útil para cada situación.",
    );

    section_heading(ui, "Cómo decidir");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "¿Puede recuperarse?",
            "Si un archivo falta o una entrada no es válida, devuelve un error controlado para que el llamador decida.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "¿Es un estado imposible?",
            "Si una condición interna demuestra que el programa está en un estado inválido, `panic!` puede hacer visible el problema.",
        );

        card_overview(
            &mut columns[1],
            "Mensaje útil",
            "Explica qué operación falló y qué dato puede ayudar a corregirla. Un mensaje claro reduce el tiempo de diagnóstico.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Propagación",
            "El código que conoce el contexto debe decidir la respuesta; las funciones internas pueden entregar el error sin ocultarlo.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Un buen diseño de errores conserva la información, separa la operación de la decisión y evita que un fallo esperado cierre el programa innecesariamente.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_code_lab_errores(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_errores();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_errores_codelab",
            separator_id: "errores_editor_toolbar_sep_y",
            terminal_panel_id: "errores_terminal_panel",
            editor_scroll_id: "errores_codelab_editor",
            drawer_key: "errores",
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
                mostrar_preguntas_errores(ui, state, orange);
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

fn mostrar_preguntas_errores(ui: &mut egui::Ui, state: &mut AppState, orange: egui::Color32) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas: [(&str, [&str; 3], usize); 5] = [
        (
            "1. ¿Cuándo representa panic! una buena decisión?",
            [
                "Cuando un dato del usuario puede corregirse",
                "Cuando el programa llega a un estado imposible",
                "Para evitar revisar un Result",
            ],
            1,
        ),
        (
            "2. ¿Qué caminos representa Result?",
            ["Ok y Err", "Start y Stop", "True y False"],
            0,
        ),
        (
            "3. ¿Qué contiene normalmente Ok?",
            ["El valor correcto de la operación", "Siempre un mensaje de error", "Una macro de compilación"],
            0,
        ),
        (
            "4. ¿Qué hace ? cuando recibe Err?",
            ["Ignora el error", "Termina pronto y lo propaga", "Convierte todo en String"],
            1,
        ),
        (
            "5. ¿Qué permite match con un Result?",
            ["Separar el camino Ok del camino Err", "Eliminar la necesidad de tipos", "Crear una variable global"],
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
        for tag in ["panic!", "Result", "Ok", "Err", "?"] {
            ui.add_space(4.0);
            inline_code_chip(ui, tag);
        }
    });
    ui.add_space(12.0);
    ui.label(
        RichText::new("Comprueba si puedes distinguir un fallo irrecuperable de un error que el programa puede tratar.")
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
                state.lessons.errores_preguntas_respuestas[index] == Some(opcion_index);
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
                state.lessons.errores_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.errores_preguntas_respuestas[index] {
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
        .errores_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.errores_preguntas_respuestas[*index] == Some(*correcta)
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
        ("panic!", 1),
        ("Result", 2),
        ("Operator ?", 3),
        ("Error Design", 4),
    ];
    let active = state.lessons.errores_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Error Handling",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.errores_tab = idx,
    );
}
