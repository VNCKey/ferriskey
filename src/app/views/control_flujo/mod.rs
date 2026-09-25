pub mod bucles;
pub mod condicionales;
pub mod match_expr;
pub mod overview;

use crate::app::AppState;
use eframe::egui;

pub fn retos_control_flujo() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Condicionales",
            "Control de Flujo",
            "if · else · else if",
            "Las condicionales permiten elegir qué bloque se ejecuta según una condición booleana. En Rust no necesitas paréntesis alrededor de la condición.\n\n`else if` permite revisar varias alternativas de forma ordenada.\n\n`if` también es una expresión: puede producir un valor cuando todas sus ramas devuelven el mismo tipo.\n\nLos operadores `&&`, `||` y `!` ayudan a combinar o invertir condiciones.",
            "Construye una decisión que indique si una persona puede acceder. Después cambia la condición y observa cómo cambia el valor obtenido.",
            "let edad = 20;\nlet tiene_permiso = true;\nlet acceso = if edad >= 18 && tiene_permiso {\n    \"permitido\"\n} else {\n    \"denegado\"\n};",
        ),
        (
            "Bucles",
            "Control de Flujo",
            "loop · while · for",
            "Los bucles repiten un bloque de código. `loop` repite indefinidamente hasta encontrar un `break`.\n\n`while` repite mientras una condición sea verdadera y vuelve a comprobarla antes de cada vuelta.\n\n`for` recorre un rango o una colección. Los rangos pueden ser exclusivos (`1..5`) o inclusivos (`1..=5`).\n\n`break` termina el bucle y `continue` salta a la siguiente vuelta. Una etiqueta permite controlar un bucle externo cuando existen bucles anidados.",
            "Practica las tres formas de repetición: cuenta con `for`, usa `while` con una condición y termina un `loop` mediante `break`.",
            "let mut contador = 0;\nwhile contador < 3 {\n    contador += 1;\n}\n\nfor numero in 1..=3 {\n    let _ = numero;\n}\n\nlet resultado = loop {\n    break 42;\n};",
        ),
        (
            "Match",
            "Control de Flujo",
            "Pattern Matching",
            "`match` compara un valor con varios patrones y ejecuta el primer patrón que coincide. Cada brazo usa la forma patrón => expresión.\n\nRust exige que el conjunto de brazos sea exhaustivo; debes cubrir todos los casos posibles.\n\nPuedes usar literales y rangos para describir coincidencias concretas o grupos de valores.\n\nLas alternativas con `|`, el comodín `_` y un `match guard` con `if` permiten expresar casos más flexibles.\n\nComo `match` es una expresión, también puede devolver un valor para asignarlo a una variable.",
            "Clasifica una puntuación con rangos y devuelve una letra. Añade un patrón `_` para cubrir todos los valores restantes.",
            "let nota = 85;\nlet resultado = match nota {\n    90..=100 => 'A',\n    70..=89 => 'B',\n    _ => 'F',\n};",
        ),
        (
            "Questions",
            "Evaluation",
            "10 Questions",
            "Esta evaluación repasa decisiones, bucles, rangos, `break`, `continue` y `Pattern Matching`.",
            "Responde correctamente las diez preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_control_flujo(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.controlflujo_tab == 4 {
        let retos = retos_control_flujo();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_control_flujo_codelab",
                separator_id: "control_flujo_editor_toolbar_sep_y",
                terminal_panel_id: "control_flujo_terminal_panel",
                editor_scroll_id: "control_flujo_codelab_editor",
                drawer_key: "control_flujo",
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
                    mostrar_preguntas_control_flujo(ui, state, orange);
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
                match state.lessons.controlflujo_tab {
                    0 => overview::mostrar(ui),
                    1 => condicionales::mostrar_tab_condicionales(ui, state),
                    2 => bucles::mostrar_tab_bucles(ui, state),
                    3 => match_expr::mostrar_tab_match(ui, state),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

fn mostrar_preguntas_control_flujo(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas: [(&str, [&str; 3], usize); 10] = [
        (
            "1. ¿Qué tipo debe tener una condición de if?",
            ["bool", "String", "usize"],
            0,
        ),
        (
            "2. ¿Qué permite if como expresión?",
            [
                "Obtener un valor de sus ramas",
                "Cambiar el tipo de una variable automáticamente",
                "Crear un bucle infinito",
            ],
            0,
        ),
        (
            "3. ¿Qué operador exige que ambas condiciones sean verdaderas?",
            ["||", "&&", "!"],
            1,
        ),
        (
            "4. ¿Qué bucle continúa hasta encontrar break?",
            ["loop", "while", "for"],
            0,
        ),
        (
            "5. ¿Qué rango incluye el número final?",
            ["1..5", "1..=5", "1...5"],
            1,
        ),
        (
            "6. ¿Qué hace continue?",
            [
                "Termina todos los bucles",
                "Devuelve un valor",
                "Salta a la siguiente vuelta",
            ],
            2,
        ),
        (
            "7. ¿Qué exige Rust en un match?",
            [
                "Que tenga siempre un solo brazo",
                "Que cubra todos los casos posibles",
                "Que use únicamente números",
            ],
            1,
        ),
        (
            "8. ¿Qué patrón captura cualquier caso restante?",
            ["_", "..", "?"],
            0,
        ),
        (
            "9. ¿Qué añade un match guard?",
            [
                "Una condición if adicional",
                "Un nuevo módulo",
                "Una repetición automática",
            ],
            0,
        ),
        (
            "10. ¿Qué puede devolver un match?",
            ["Un valor mediante sus brazos", "Solo bool", "Nada nunca"],
            0,
        ),
    ];

    ui.add_space(4.0);
    ui.heading(
        egui::RichText::new("4. Questions")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        let mut category = egui::Frame::new();
        category.fill = egui::Color32::from_rgb(20, 38, 28);
        category.inner_margin = egui::Margin::symmetric(8, 2);
        category.corner_radius = egui::CornerRadius::same(10);
        category.show(ui, |ui| {
            ui.label(
                egui::RichText::new("Evaluation")
                    .size(11.0)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 200, 120)),
            );
        });
        for tag in ["if", "loop", "for", "match"] {
            ui.add_space(4.0);
            let mut topic = egui::Frame::new();
            topic.fill = egui::Color32::from_rgb(22, 28, 38);
            topic.inner_margin = egui::Margin::symmetric(8, 2);
            topic.corner_radius = egui::CornerRadius::same(10);
            topic.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(tag)
                        .size(11.0)
                        .color(egui::Color32::from_rgb(160, 185, 220)),
                );
            });
        }
    });
    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(
            "Comprueba si puedes elegir la construcción correcta para cada decisión, repetición o coincidencia de patrones.",
        )
        .size(13.5)
        .color(text_col)
        .line_height(Some(19.0)),
    );
    ui.add_space(14.0);
    crate::views::pilares::anatomy::titulo_seccion(ui, "Comprueba lo aprendido", orange);
    ui.add_space(8.0);

    for (index, (pregunta, opciones, correcta)) in preguntas.iter().enumerate() {
        ui.label(
            egui::RichText::new(*pregunta)
                .strong()
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(4.0);

        for (opcion_index, opcion) in opciones.iter().enumerate() {
            let seleccionada =
                state.lessons.controlflujo_preguntas_respuestas[index] == Some(opcion_index);
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
                state.lessons.controlflujo_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.controlflujo_preguntas_respuestas[index] {
            let es_correcta = respuesta == *correcta;
            ui.label(
                egui::RichText::new(if es_correcta {
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
        .controlflujo_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.controlflujo_preguntas_respuestas[*index] == Some(*correcta)
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
        egui::RichText::new(if completada {
            "¡Excelente! Has respondido correctamente las diez preguntas.".to_owned()
        } else {
            format!("Respuestas correctas: {correctas}/10 · Respondidas: {respondidas}/10")
        })
        .size(13.0)
        .color(if completada { orange } else { text_col }),
    );
}

pub fn card_frame_tutorial() -> egui::Frame {
    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));
    frame
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("Condicionales", 1),
        ("Bucles", 2),
        ("Match", 3),
    ];
    let active = state.lessons.controlflujo_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Control de Flujo",
        &tabs,
        4,
        active,
        |st, idx| st.lessons.controlflujo_tab = idx,
    );
}
