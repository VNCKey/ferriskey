pub mod estructura;
pub mod overview;

use crate::app::AppState;
use eframe::egui;

pub fn retos_modulos() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Declaration & Module Tree",
            "Fundamental",
            "mod",
            "Un módulo agrupa código relacionado dentro de un espacio de nombres propio.\n\nLa palabra clave mod declara un módulo y permite organizar funciones, tipos y otros módulos bajo un nombre.\n\nLa raíz del crate puede declarar módulos inline o conectarlos con archivos separados.",
            "Añade el módulo al archivo del proyecto y utiliza su función mediante la ruta correspondiente.",
            "mod utilidades {\n    pub fn saludar() {\n        println!(\"Hola desde utilidades\");\n    }\n}\n\nutilidades::saludar();",
        ),
        (
            "Visibility",
            "Fundamental",
            "pub · pub(crate)",
            "Los ítems son privados por defecto dentro de su módulo. Esta regla protege la implementación y evita exponer detalles por accidente.\n\nLa palabra clave pub hace visible una función, tipo o módulo desde las rutas permitidas.\n\npub(crate) amplía la visibilidad a todo el crate, pero mantiene el ítem oculto para usuarios externos de una librería.",
            "Compara qué función puede llamarse desde fuera del módulo y qué función permanece privada.",
            "mod red {\n    pub fn conectar() {}\n    pub(crate) fn diagnostico() {}\n    fn secreto() {}\n}",
        ),
        (
            "Paths & Imports",
            "Navigation",
            "crate · self · super · use",
            "Una ruta indica el camino para llegar a un ítem dentro del árbol de módulos.\n\ncrate:: comienza en la raíz del crate y permite escribir una ruta absoluta dentro del proyecto.\n\nself:: representa el módulo actual y super:: sube al módulo padre.\n\nuse crea un acceso directo para no repetir una ruta larga cada vez que utilizas un ítem.",
            "Escribe una ruta completa y después crea un import con use para utilizar el mismo ítem de forma más clara.",
            "mod red {\n    pub fn conectar() {}\n}\n\nuse crate::red::conectar;\nconectar();",
        ),
        (
            "Re-export & Public API",
            "API Design",
            "pub use",
            "Una implementación puede vivir en módulos internos sin convertirse automáticamente en parte de la API pública.\n\nLa expresión pub use re-exporta un ítem desde otra ruta y crea una entrada pública más cómoda.\n\nEste patrón permite ofrecer una API limpia: el usuario importa lo importante sin conocer toda la estructura interna.",
            "Crea una entrada pública en la raíz y deja que la implementación permanezca dentro de un módulo interno.",
            "mod interno {\n    pub fn procesar() {}\n}\n\npub use interno::procesar;\n\nprocesar();",
        ),
        (
            "Module File Structure",
            "Architecture",
            "src/",
            "La declaración mod nombre; le indica a Rust qué módulo debe buscar y conecta el código con un archivo o una carpeta dentro de src/.\n\nUn módulo puede vivir en un archivo como red.rs o tener submódulos dentro de una carpeta como red/.\n\nEn Rust moderno, colocar red.rs junto a la carpeta red/ suele ser más claro que utilizar mod.rs como archivo principal.",
            "Abre el explorador del proyecto y relaciona cada declaración mod nombre; con el archivo que Rust busca dentro de src/.",
            "src/\n├── main.rs\n├── red.rs\n└── red/\n    └── http.rs",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "La evaluación repasa los conceptos principales de Modules & Visibility.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_modulos(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.modulos_tab == 2 {
        let retos = retos_modulos();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_modulos_codelab",
                separator_id: "modulos_editor_toolbar_sep_y",
                terminal_panel_id: "modulos_terminal_panel",
                editor_scroll_id: "modulos_codelab_editor",
                drawer_key: "modulos",
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
                    mostrar_preguntas_modulos(ui, state, orange);
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
                match state.lessons.modulos_tab {
                    0 => overview::mostrar(ui),
                    1 => estructura::mostrar_teoria_modulos(ui, state),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("Structure & Best Practices", 1),
    ];
    let active = state.lessons.modulos_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Modules & Visibility",
        &tabs,
        2,
        active,
        |st, idx| st.lessons.modulos_tab = idx,
    );
}

fn mostrar_preguntas_modulos(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas = [
        (
            "1. ¿Qué palabra declara un módulo en Rust?",
            ["mod", "module", "namespace"],
            0,
        ),
        (
            "2. ¿Cómo son los ítems dentro de un módulo por defecto?",
            ["Públicos", "Privados", "Globales"],
            1,
        ),
        (
            "3. ¿Qué ruta comienza en la raíz del crate?",
            ["self::", "super::", "crate::"],
            2,
        ),
        (
            "4. ¿Para qué sirve use?",
            [
                "Para crear un acceso directo a una ruta",
                "Para convertir un módulo en público",
                "Para compilar en modo release",
            ],
            0,
        ),
        (
            "5. ¿Qué permite pub(crate)?",
            [
                "Exponer un ítem a todo el crate, pero no a usuarios externos",
                "Ocultar un ítem incluso dentro de su módulo",
                "Crear automáticamente un nuevo archivo",
            ],
            0,
        ),
    ];

    ui.add_space(4.0);
    ui.heading(
        egui::RichText::new("6. Questions")
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

        for tag in ["mod", "Visibility", "Paths", "pub use"] {
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
            "Comprueba si puedes reconocer las reglas básicas de organización, visibilidad y navegación entre módulos.",
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
                state.lessons.modulos_preguntas_respuestas[index] == Some(opcion_index);
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
                state.lessons.modulos_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.modulos_preguntas_respuestas[index] {
            let es_correcta = respuesta == *correcta;
            ui.label(
                egui::RichText::new(if es_correcta {
                    "Correcto"
                } else {
                    "Revisa este concepto y vuelve a intentarlo"
                })
                .size(11.5)
                .color(if es_correcta {
                    egui::Color32::from_rgb(255, 180, 80)
                } else {
                    egui::Color32::from_rgb(255, 150, 100)
                }),
            );
        }
        ui.add_space(10.0);
    }

    let respondidas = state
        .lessons
        .modulos_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.modulos_preguntas_respuestas[*index] == Some(*correcta)
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
            "¡Excelente! Has respondido correctamente las cinco preguntas.".to_owned()
        } else {
            format!("Respuestas correctas: {correctas}/5 · Respondidas: {respondidas}/5")
        })
        .size(13.0)
        .color(if completada { orange } else { text_col }),
    );
}
