pub mod arrays;
pub mod overview;
pub mod slices;
pub mod tuplas;

use crate::app::AppState;
use crate::views::colecciones::mostrar_contenido_colecciones;
use eframe::egui;

pub fn retos_tipos_compuestos() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Array",
            "Tipos Compuestos",
            "Tamaño fijo",
            "Un Array contiene elementos del mismo tipo y tiene una cantidad fija conocida en compilación. Sus elementos se almacenan en posiciones consecutivas.\n\nLa forma `[T; N]` expresa el tipo de elemento y la cantidad de posiciones. En esta etapa aprenderás a crearlo, inicializarlo, consultarlo, modificarlo y separarlo con un Destructuring Pattern.\n\nPuedes escribir valores individuales o repetir un valor con `[valor; cantidad]`.\n\nLos índices empiezan en cero. Para modificar una posición, el Array debe ser mutable con `let mut`.\n\nUn Destructuring Pattern conserva las posiciones y puede ignorar datos con `_` o `..`.",
            "Recorre el Array completo: crea cuatro valores, consulta un índice, modifica una posición y extrae el primero y el último elemento con un Destructuring Pattern.",
            "let mut notas: [u8; 4] = [10, 12, 15, 18];\nlet ceros = [0; 4];\nlet segunda = notas[1];\nnotas[0] = 20;\nlet [primera, _, .., ultima] = notas;",
        ),
        (
            "Tuplas",
            "Tipos Compuestos",
            "Valores heterogéneos",
            "Una Tuple agrupa varios valores en una sola estructura. Sus elementos pueden tener tipos diferentes, pero la cantidad y el tipo de cada posición quedan definidos desde el principio.\n\nPuedes consultar cada posición con `.0`, `.1` y `.2`. El índice empieza en cero y cada posición conserva su propio tipo.\n\nUna Tuple puede ser mutable con `let mut`, lo que permite cambiar un elemento sin cambiar la cantidad de posiciones.\n\nUn Destructuring Pattern separa la Tuple en variables nuevas. `_` ignora una posición y `..` permite omitir varias posiciones.",
            "Crea una Tuple, accede a una posición, modifica una Tuple mutable y termina separando sus valores con un Destructuring Pattern.",
            "let datos = (\"Alicia\", 26, true);\nlet nombre = datos.0;\n\nlet mut punto = (10, 20);\npunto.0 = 15;\n\nlet (nombre, edad, activo) = datos;\nlet (primero, .., ultimo) = (10, 20, 30);",
        ),
        (
            "Collections",
            "Tipos Compuestos",
            "Vec y HashMap",
            "Las Collections reúnen varios valores para trabajar con ellos como un conjunto. En esta etapa conocerás `Vec`, una secuencia que puede crecer, y `HashMap`, una colección que relaciona una clave con un valor.\n\nUn `Vec<T>` almacena elementos del mismo tipo y permite agregar o quitar posiciones durante la ejecución. `push`, `pop`, `insert` y `remove` cambian su contenido.\n\n`HashMap` organiza datos mediante pares clave-valor. Puedes insertar una relación, comprobar si existe una clave y eliminarla.\n\nLa cantidad de una Collection se consulta con `len`; su tamaño dinámico no funciona igual que el tamaño fijo de un Array.",
            "Crea un `Vec`, agrega y quita elementos, y después crea un `HashMap` sencillo para relacionar una palabra con un número.",
            "let mut niveles = vec![1, 2, 4];\nniveles.push(8);\nniveles.insert(1, 3);\nniveles.remove(0);\n\nuse std::collections::HashMap;\nlet mut puntos = HashMap::new();\npuntos.insert(\"Rust\", 100);\nlet existe = puntos.contains_key(\"Rust\");",
        ),
        (
            "Slices",
            "Tipos Compuestos",
            "Vistas prestadas",
            "Un Slice es una vista sobre una parte de un Array, un `Vec` o un texto. No crea una colección nueva ni copia los elementos; solo indica qué rango puede leerse.\n\nLa expresión `&datos[inicio..fin]` incluye el inicio y deja fuera el final. También puedes tomar todo el contenido con `&datos[..]`.\n\nUn Slice puede ser inmutable (`&[T]`) o mutable (`&mut [T]`). El mutable permite cambiar una parte del propietario original.\n\nEn texto, `&str` es un Slice prestado de una `String`. Los rangos deben respetar los límites de los caracteres Unicode.",
            "Crea una vista de un Array, modifica otra vista mutable y termina tomando un `&str` desde una `String`.",
            "let datos = [10, 20, 30, 40, 50];\nlet vista = &datos[1..4];\n\nlet mut valores = [10, 20, 30, 40];\nlet parte = &mut valores[1..3];\nparte[0] = 99;\n\nlet texto = String::from(\"Rust\");\nlet palabra: &str = &texto[..];",
        ),
        (
            "Questions",
            "Evaluation",
            "10 Questions",
            "Esta evaluación repasa los conceptos principales de Array, Tuples, Collections y Slices. Lee cada pregunta y elige una respuesta.",
            "Responde correctamente las diez preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_tipos_compuestos(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.compuestos_tab == 5 {
        let retos = retos_tipos_compuestos();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_tipos_compuestos_codelab",
                separator_id: "tipos_compuestos_editor_toolbar_sep_y",
                terminal_panel_id: "tipos_compuestos_terminal_panel",
                editor_scroll_id: "tipos_compuestos_codelab_editor",
                drawer_key: "tipos_compuestos",
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
                    mostrar_preguntas_tipos_compuestos(ui, state, orange);
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
                let orange = egui::Color32::from_rgb(255, 160, 50);
                let cyan = egui::Color32::from_rgb(100, 200, 255);
                let texto = egui::Color32::from_rgb(200, 210, 225);
                ui.add_space(10.0);
                match state.lessons.compuestos_tab {
                    0 => overview::mostrar(ui),
                    1 => arrays::mostrar_compuesto_array(ui, state, orange, cyan, texto),
                    2 => tuplas::mostrar_compuesto_tupla(ui, state, orange, cyan, texto),
                    3 => mostrar_contenido_colecciones(ui, state),
                    4 => slices::mostrar_compuesto_slice(ui, state, orange, cyan, texto),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

fn mostrar_preguntas_tipos_compuestos(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas: [(&str, [&str; 3], usize); 10] = [
        (
            "1. ¿Qué característica define a un Array?",
            [
                "Puede mezclar cualquier tipo sin límite",
                "Tiene elementos del mismo tipo y tamaño fijo",
                "Siempre relaciona una clave con un valor",
            ],
            1,
        ),
        (
            "2. ¿Cuál es el primer índice de un Array?",
            ["0", "1", "Depende del tipo"],
            0,
        ),
        (
            "3. ¿Qué significa [0; 4]?",
            [
                "Un rango de cero a cuatro",
                "Cuatro posiciones inicializadas con cero",
                "Una Tuple con cuatro ceros de tipos distintos",
            ],
            1,
        ),
        (
            "4. ¿Qué permite un Destructuring Pattern en un Array?",
            [
                "Separar posiciones en variables",
                "Cambiar el tamaño del Array",
                "Convertirlo automáticamente en un Vec",
            ],
            0,
        ),
        (
            "5. ¿Qué diferencia principal tiene una Tuple?",
            [
                "Todos sus elementos deben ser del mismo tipo",
                "Puede agrupar valores de tipos diferentes",
                "Siempre crece con push",
            ],
            1,
        ),
        (
            "6. ¿Cómo accedes al segundo elemento de una Tuple?",
            ["tuple.0", "tuple.1", "tuple[1]"],
            1,
        ),
        (
            "7. ¿Qué colección puede crecer durante la ejecución?",
            ["Array", "Vec", "Tuple"],
            1,
        ),
        (
            "8. ¿Qué organiza un HashMap?",
            [
                "Posiciones fijas de un mismo tipo",
                "Rangos de memoria sin copiar",
                "Relaciones entre claves y valores",
            ],
            2,
        ),
        (
            "9. En &datos[1..4], ¿qué posiciones incluye el Slice?",
            ["1, 2 y 3", "1, 2, 3 y 4", "Solo la posición 4"],
            0,
        ),
        (
            "10. ¿Qué permite un &mut [T]?",
            [
                "Modificar una parte del propietario original",
                "Cambiar el tipo de todos los elementos",
                "Crear una copia independiente automáticamente",
            ],
            0,
        ),
    ];

    ui.add_space(4.0);
    ui.heading(
        egui::RichText::new("5. Questions")
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

        for tag in ["Array", "Tuples", "Vec", "HashMap", "Slices"] {
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
            "Comprueba si puedes reconocer cuándo utilizar un Array, una Tuple, una Collection o un Slice.",
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
                state.lessons.compuestos_preguntas_respuestas[index] == Some(opcion_index);
            let (opcion_rect, respuesta) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), 25.0),
                egui::Sense::click(),
            );
            let hovered = respuesta.hovered();
            let opcion_fill = if seleccionada {
                egui::Color32::from_rgba_unmultiplied(255, 160, 50, 32)
            } else if hovered {
                egui::Color32::from_rgba_unmultiplied(255, 160, 50, 18)
            } else {
                egui::Color32::from_rgb(15, 20, 30)
            };
            let opcion_stroke = if seleccionada {
                orange
            } else if hovered {
                egui::Color32::from_rgba_unmultiplied(255, 180, 80, 150)
            } else {
                egui::Color32::from_rgb(35, 48, 70)
            };
            ui.painter().rect(
                opcion_rect,
                egui::CornerRadius::same(4),
                opcion_fill,
                egui::Stroke::new(if seleccionada { 1.2 } else { 1.0 }, opcion_stroke),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                egui::pos2(opcion_rect.left() + 10.0, opcion_rect.center().y),
                egui::Align2::LEFT_CENTER,
                *opcion,
                egui::FontId::proportional(12.5),
                if seleccionada { orange } else { text_col },
            );
            if respuesta.clicked() {
                state.lessons.compuestos_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.compuestos_preguntas_respuestas[index] {
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
        .compuestos_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.compuestos_preguntas_respuestas[*index] == Some(*correcta)
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

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("Arrays", 1),
        ("Tuplas", 2),
        ("Colecciones", 3),
        ("Slices", 4),
    ];
    let active = state.lessons.compuestos_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Tipos Compuestos",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.compuestos_tab = idx,
    );
}
