pub mod closures;
pub mod overview;
pub mod parametros;
pub mod retorno;

use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

pub(crate) fn grupo_funciones(
    ui: &mut egui::Ui,
    izquierda: (&str, &str, &str),
    derecha: (&str, &str, &str),
    state: &AppState,
) {
    let syntax_set = &state.editor.syntax_set;
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];

    ui.columns(2, |columns| {
        columns[0].label(
            RichText::new(izquierda.0)
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        columns[0].add_space(6.0);
        texto_con_chips_inline(
            &mut columns[0],
            izquierda.1,
            Colors::TEXT_PRIMARY,
            Colors::CYAN_ACCENT,
        );

        columns[1].label(
            RichText::new(derecha.0)
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        columns[1].add_space(6.0);
        texto_con_chips_inline(
            &mut columns[1],
            derecha.1,
            Colors::TEXT_PRIMARY,
            Colors::CYAN_ACCENT,
        );
    });
    ui.add_space(10.0);

    ui.columns(2, |columns| {
        highlighted_code_block(&mut columns[0], izquierda.2, syntax_set, theme, "rs");
        highlighted_code_block(&mut columns[1], derecha.2, syntax_set, theme, "rs");
    });
}

pub fn retos_funciones() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Closure básico",
            "Closures",
            "|parámetros| cuerpo",
            "Un closure es una función anónima que se escribe entre barras verticales. Puede recibir parámetros, ejecutar una expresión y devolver su resultado.

A diferencia de una función con nombre, un closure puede guardarse en una variable y utilizarse como un valor.",
            "Crea un closure que reciba un número y devuelva su doble. Después ejecútalo con un valor distinto.",
            "let doble = |numero: i32| numero * 2;
let resultado = doble(5);",
        ),
        (
            "Captura del entorno",
            "Closures",
            "Lectura y modificación de variables externas",
            "Un closure puede utilizar variables declaradas fuera de su propio cuerpo. Rust determina cómo capturarlas según la forma en que las utiliza.

Cuando solo lee una variable, normalmente la toma mediante un préstamo inmutable. Con `FnMut`, también puede modificar una variable capturada de forma mutable.",
            "Declara un factor fuera del closure y úsalo para calcular un resultado. Después crea un contador mutable que el closure pueda incrementar.",
            "let factor = 10;
let multiplicar = |numero| numero * factor;
let resultado = multiplicar(5);",
        ),
        (
            "move y Ownership",
            "Closures",
            "Transferir valores al closure",
            "La palabra clave `move` obliga al closure a tomar la propiedad de las variables capturadas.

Esto resulta útil cuando el closure debe conservar los datos por más tiempo o ejecutarse en otro contexto. Después del movimiento, el código exterior ya no puede usar el valor que fue transferido.",
            "Mueve una `String` dentro de un closure y ejecútalo. Observa qué uso de la variable original dejaría de ser válido.",
            "let mensaje = String::from(\"Hola desde Rust\");
let imprimir = move || println!(\"{mensaje}\");

imprimir();",
        ),
        (
            "Closures como parámetros",
            "Higher-Order Functions",
            "impl Fn",
            "Una función puede recibir un closure como parámetro. `impl Fn(i32) -> i32` expresa que recibirá una operación que toma un `i32` y devuelve otro `i32`.

Así puedes separar la estructura de una función de la operación concreta que debe ejecutar.",
            "Crea una función que reciba un número y un closure, y usa ese closure para transformar el número.",
            "fn aplicar(valor: i32, operacion: impl Fn(i32) -> i32) -> i32 {
    operacion(valor)
}

let resultado = aplicar(7, |numero| numero * numero);",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa la sintaxis, la captura del entorno, `move`, `Fn` y el uso de closures como parámetros.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_funciones(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.funciones_tab == 4 {
        let retos = retos_funciones();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_funciones_codelab",
                separator_id: "funciones_editor_toolbar_sep_y",
                terminal_panel_id: "funciones_terminal_panel",
                editor_scroll_id: "funciones_codelab_editor",
                drawer_key: "funciones",
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
                    mostrar_preguntas_funciones(ui, state, orange);
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
    } else {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(10.0);
                match state.lessons.funciones_tab {
                    0 => overview::mostrar(ui),
                    1 => parametros::mostrar_tab_parametros(ui, state),
                    2 => retorno::mostrar_tab_retorno(ui, state),
                    3 => closures::mostrar_tab_closures(ui, state),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

fn mostrar_preguntas_funciones(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas: [(&str, [&str; 3], usize); 5] = [
        (
            "1. ¿Con qué símbolos se escriben los parámetros de un closure?",
            ["( )", "| |", "[ ]"],
            1,
        ),
        (
            "2. ¿Qué puede hacer un closure con una variable de su entorno?",
            ["Capturarla según cómo la utiliza", "Cambiar siempre su tipo", "Eliminarla automáticamente"],
            0,
        ),
        (
            "3. ¿Qué indica move en un closure?",
            ["Que no recibe parámetros", "Que toma Ownership de los valores capturados", "Que siempre devuelve bool"],
            1,
        ),
        (
            "4. ¿Qué expresa impl Fn(i32) -> i32?",
            ["Una operación que recibe y devuelve i32", "Una variable global", "Un módulo privado"],
            0,
        ),
        (
            "5. ¿Dónde puede guardarse un closure?",
            ["Solo dentro de un loop", "En una variable o como parámetro", "Únicamente en un struct"],
            1,
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
        for tag in ["Closure", "Capture", "move", "Fn"] {
            ui.add_space(4.0);
            inline_code_chip_color(ui, tag, egui::Color32::from_rgb(160, 185, 220));
        }
    });
    ui.add_space(12.0);
    ui.label(
        RichText::new("Comprueba si puedes reconocer cómo se escriben, capturan y utilizan los closures.")
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
                state.lessons.funciones_preguntas_respuestas[index] == Some(opcion_index);
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
                state.lessons.funciones_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.funciones_preguntas_respuestas[index] {
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
        .funciones_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.funciones_preguntas_respuestas[*index] == Some(*correcta)
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
        ("Parámetros & Ownership", 1),
        ("Retornos Múltiples", 2),
        ("Closures & Captura", 3),
    ];
    let active = state.lessons.funciones_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Functions & Closures",
        &tabs,
        4,
        active,
        |st, idx| st.lessons.funciones_tab = idx,
    );
}
