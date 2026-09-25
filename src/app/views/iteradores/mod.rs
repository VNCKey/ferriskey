pub mod adaptadores;
pub mod consumidores;
pub mod info;
pub mod modos;
pub mod overview;
pub mod pipeline;

use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

pub(crate) fn grupo_iteradores(
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

pub fn retos_iteradores() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Modos de iteración",
            "Iteradores",
            ".iter · .iter_mut · .into_iter",
            "Una colección puede recorrerse prestando referencias, permitiendo modificar sus elementos o transfiriendo su propiedad.

`iter` conserva la colección, `iter_mut` permite modificarla y `into_iter` consume la colección para entregar sus valores.",
            "Recorre una lista con `iter`, cambia sus valores con `iter_mut` y observa cómo `into_iter` consume la colección.",
            "let mut numeros = vec![1, 2, 3];

for numero in numeros.iter() {
    let _ = numero;
}

for numero in numeros.iter_mut() {
    *numero *= 2;
}",
        ),
        (
            "Adaptadores perezosos",
            "Pipelines",
            ".filter · .map",
            "Los adaptadores transforman un iterador y construyen un pipeline. Son perezosos: describen el trabajo, pero no lo ejecutan hasta que aparece un consumidor.",
            "Filtra los números pares, multiplícalos por diez y reúne el resultado en un nuevo `Vec`.",
            "let datos = vec![1, 2, 3, 4, 5, 6];
let resultado: Vec<i32> = datos
    .iter()
    .filter(|&&numero| numero % 2 == 0)
    .map(|&numero| numero * 10)
    .collect();",
        ),
        (
            "Consumidores",
            "Iteradores",
            ".collect · .sum · .find",
            "Un consumidor avanza el iterador y produce un resultado final. Puede crear una colección, calcular un valor o buscar un elemento.",
            "Calcula una suma y después utiliza `collect` para materializar un nuevo `Vec` a partir de un pipeline.",
            "let numeros = vec![1, 2, 3, 4];
let suma: i32 = numeros.iter().sum();

let dobles: Vec<i32> = numeros
    .iter()
    .map(|numero| numero * 2)
    .collect();",
        ),
        (
            "Pipeline y tipos de salida",
            "Colecciones",
            "collect::<T>()",
            "`collect` necesita saber qué colección debe construir. Puedes dejar que el tipo de la variable lo indique o usar Turbofish para escribirlo de forma explícita.",
            "Elimina duplicados y recolecta los valores en un `HashSet` usando `collect::<HashSet<_>>()`.",
            "use std::collections::HashSet;

let nombres = vec![\"Alice\", \"Bob\", \"Alice\"];
let unicos = nombres
    .into_iter()
    .collect::<HashSet<_>>();",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa los modos de iteración, los adaptadores perezosos, los consumidores y los pipelines.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_iteradores(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.iteradores_tab == 5 {
        let retos = retos_iteradores();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_iteradores_codelab",
                separator_id: "iteradores_editor_toolbar_sep_y",
                terminal_panel_id: "iteradores_terminal_panel",
                editor_scroll_id: "iteradores_codelab_editor",
                drawer_key: "iteradores",
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
                    mostrar_preguntas_iteradores(ui, state, orange);
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
                match state.lessons.iteradores_tab {
                    0 => overview::mostrar(ui),
                    1 => modos::mostrar_tab_modos(ui, state),
                    2 => adaptadores::mostrar_tab_adaptadores(ui, state),
                    3 => consumidores::mostrar_tab_consumidores(ui, state),
                    4 => info::mostrar_iteradores_info(ui, state),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

fn mostrar_preguntas_iteradores(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas: [(&str, [&str; 3], usize); 5] = [
        (
            "1. ¿Qué hace `.iter()`?",
            ["Presta referencias sin consumir la colección", "Modifica siempre los elementos", "Elimina la colección"],
            0,
        ),
        (
            "2. ¿Qué permite `.iter_mut()`?",
            ["Transferir Ownership", "Modificar elementos mediante referencias mutables", "Crear un HashSet"],
            1,
        ),
        (
            "3. ¿Qué ocurre normalmente con `.into_iter()`?",
            ["Consume la colección y entrega sus valores", "Solo lee los índices", "Ordena automáticamente los datos"],
            0,
        ),
        (
            "4. ¿Cuándo se ejecuta un adaptador como `.map()`?",
            ["Al escribir el método", "Cuando un consumidor avanza el iterador", "Solo al compilar"],
            1,
        ),
        (
            "5. ¿Para qué sirve `.collect()`?",
            ["Materializa el resultado en una colección u otro tipo compatible", "Convierte todo en String", "Detiene siempre el programa"],
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
        for tag in ["iter", "iter_mut", "into_iter", "map", "collect"] {
            ui.add_space(4.0);
            inline_code_chip_color(ui, tag, egui::Color32::from_rgb(160, 185, 220));
        }
    });
    ui.add_space(12.0);
    ui.label(
        RichText::new("Comprueba si puedes distinguir entre prestar, modificar, consumir y materializar un pipeline.")
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
                state.lessons.iteradores_preguntas_respuestas[index] == Some(opcion_index);
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
                state.lessons.iteradores_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.iteradores_preguntas_respuestas[index] {
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
        .iteradores_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.iteradores_preguntas_respuestas[*index] == Some(*correcta)
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
        ("Modos de Iteración", 1),
        ("Adaptadores", 2),
        ("Consumidores", 3),
        ("Info & Buenas Prácticas", 4),
    ];
    let active = state.lessons.iteradores_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Iteradores y Pipelines",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.iteradores_tab = idx,
    );
}
