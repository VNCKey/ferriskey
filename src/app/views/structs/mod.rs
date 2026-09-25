pub mod definicion;
pub mod enums;
pub mod info;
pub mod pattern_matching;
pub mod overview;
pub mod traits;

use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui::{self, RichText};

pub(crate) fn grupo_custom_types(
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

pub fn retos_structs() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "Structs",
            "Custom Types",
            "Campos nombrados y construcción",
            "Un `Struct` agrupa datos relacionados bajo un tipo propio. Sus campos tienen nombre y cada instancia puede conservar un estado diferente.

Para crear una instancia escribes el nombre del tipo y asignas un valor a cada campo. Todos los campos deben inicializarse, aunque el orden en que los escribas no importa.

Después puedes acceder a los campos con el operador `.` y leer sus valores desde la instancia.

Si la instancia se declara con `let mut`, puedes modificar sus campos sin cambiar la forma del tipo.

La `Struct Update Syntax` usa `..` para reutilizar los campos restantes de otra instancia. Si esos campos contienen valores que se mueven, la instancia original puede dejar de ser válida para ellos.

Una `Tuple Struct` usa posiciones en lugar de nombres para sus campos. Es útil cuando el significado del tipo es importante y la estructura es pequeña.

Una `Unit-like Struct` no tiene campos. Puede servir como una marca o como un tipo que representa una idea sin datos internos.

También puedes extraer los campos mediante un `Destructuring Pattern` y convertirlos en variables independientes.

Con `#[derive(...)]` puedes pedirle a Rust que genere comportamientos comunes para una Struct, como imprimirla con `Debug` o clonarla con `Clone`.

El bloque `impl` reúne el comportamiento relacionado con el tipo: funciones asociadas y métodos.

Una función asociada como `nuevo` no recibe `self`; se llama con `Usuario::nuevo(...)` y suele construir una instancia.

Dentro de `impl`, `Self` representa el tipo actual. `Self { nombre, edad }` equivale a `Usuario { nombre: nombre, edad: edad }` cuando los nombres coinciden.

Un método con `&self` presta la instancia para leer sus campos sin consumirla.

Un método con `&mut self` presta la instancia con permiso para modificarla. La variable original debe ser mutable.

Un método con `self` toma el `Ownership` de la instancia. Después de llamarlo, el valor original ya no puede utilizarse.

Un método puede devolver `Self` para permitir `Method Chaining`: llamar varias operaciones una después de otra sobre la misma instancia.",
            "Define un `Struct` Usuario con nombre y edad. Después crea una instancia y consulta sus campos.",
            "struct Usuario {
    nombre: String,
    edad: u32,
}

let usuario = Usuario {
    nombre: String::from(\"Ferris\"),
    edad: 5,
};",
        ),
        (
            "Enums",
            "Custom Types",
            "Variantes y datos asociados",
            "Un `Enum` representa un valor que puede adoptar una variante entre varias posibilidades.

Un `Enum` solo tiene una `Variant` activa a la vez. Cada variante representa una posibilidad distinta del mismo tipo.

Una variante simple no guarda datos. Es útil para estados como activo, apagado o finalizado.

Para crear un valor utilizas el nombre del `Enum`, seguido de `::` y el nombre de la variante.

Una variante también puede guardar un dato, como un texto o un número. El dato forma parte del valor creado.

Una variante con campos nombrados permite describir mejor datos relacionados, como la posición de un evento.

Con `match` puedes revisar qué variante contiene el valor. Rust exige cubrir todas las posibilidades del `Enum`.

El patrón de cada rama puede extraer los datos guardados por una variante y convertirlos en variables locales.

Un `match` también puede producir un valor. Todas sus ramas deben devolver el mismo tipo.

Cuando solo te interesa una variante, `if let` permite comprobarla y extraer sus datos sin escribir todas las demás ramas.

Un `Enum` también puede tener comportamiento propio dentro de un bloque `impl`, igual que una `Struct`.",
            "Crea un `Enum` Estado con variantes simples y utiliza una variante que transporte un dato.",
            "enum Estado {
    Activo,
    Apagado,
}

enum Mensaje {
    Texto(String),
    Fin,
}

let estado = Estado::Activo;
let mensaje = Mensaje::Texto(String::from(\"Hola\"));",
        ),
        (
            "Traits",
            "Custom Types",
            "Comportamiento compartido",
            "Un `Trait` define un comportamiento que distintos tipos pueden implementar.

La declaración del `Trait` indica qué comportamiento debe existir, pero no decide cómo lo realiza cada tipo.

Un método requerido dentro del `Trait` debe implementarse en cada tipo que adopte ese contrato.

Una `Struct` puede implementar un `Trait` con `impl Trait for Tipo` y escribir allí el comportamiento concreto.

Después de implementarlo, puedes llamar al método desde una instancia del tipo.

Un método con cuerpo dentro del `Trait` es un método por defecto. La implementación puede utilizarlo directamente.

Si un tipo necesita un comportamiento diferente, puede sobrescribir el método por defecto dentro de su propio `impl`.

El mismo `Trait` puede implementarse para un `Enum`; dentro de la implementación puedes usar `match` para responder según su variante.",
            "Declara un `Trait` Describible e impleméntalo para una `Struct` Usuario.",
            "trait Describible {
    fn describir(&self);
}

struct Usuario {
    nombre: String,
}

impl Describible for Usuario {
    fn describir(&self) {
        println!(\"Usuario: {}\", self.nombre);
    }
}

let usuario = Usuario {
    nombre: String::from(\"Ana\"),
};
usuario.describir();",
        ),
        (
            "Pattern Matching",
            "Custom Types",
            "if let y while let",
            "Los patrones permiten comprobar la variante de un `Enum` y extraer sus datos.

Un `Pattern` describe la forma que debe tener un valor para coincidir.

`match` compara el valor con varias ramas y cada rama contiene el código que corresponde a una posibilidad.

Un `match` debe ser exhaustivo: tiene que cubrir todas las variantes o utilizar un patrón general como `_`.

Los patrones pueden entrar en una variante con datos y extraer sus valores en nuevas variables.

También pueden revisar variantes con campos nombrados y comprobar solo los campos que interesan.

El operador `|` permite atender varias posibilidades con la misma rama.

Un `match guard` añade una condición después del patrón para hacer la comprobación más precisa.

Como `match` es una expresión, su resultado puede guardarse en una variable.

`if let` sirve cuando solo necesitas una coincidencia. `while let` repite una acción mientras el patrón siga coincidiendo.",
            "Comprueba una variante con `if let` y procesa valores sucesivos con `while let`.",
            "enum Mensaje {
    Texto(String),
    Fin,
}

let mensaje = Mensaje::Texto(String::from(\"Hola\"));
if let Mensaje::Texto(texto) = mensaje {
    let _ = texto;
}",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa Structs, Enums, Traits y Pattern Matching.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_structs(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.structs_tab == 5 {
        let retos = retos_structs();
        let total_retos = retos.len();
        let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
            ui,
            state,
            crate::views::pilares::anatomy::CodeLabConfig {
                project_selector_id: "combo_proyectos_structs_codelab",
                separator_id: "structs_editor_toolbar_sep_y",
                terminal_panel_id: "structs_terminal_panel",
                editor_scroll_id: "structs_codelab_editor",
                drawer_key: "structs",
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
                    mostrar_preguntas_custom_types(ui, state, orange);
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
                match state.lessons.structs_tab {
                    0 => overview::mostrar(ui),
                    1 => definicion::mostrar_tab_structs(ui, state),
                    2 => enums::mostrar_tab_enums(ui, state),
                    3 => traits::mostrar_tab_traits_custom(ui, state),
                    4 => info::mostrar_structs_info(ui, state),
                    _ => overview::mostrar(ui),
                }
                ui.add_space(20.0);
            });
    }
}

fn mostrar_preguntas_custom_types(
    ui: &mut egui::Ui,
    state: &mut AppState,
    orange: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);
    let preguntas: [(&str, [&str; 3], usize); 5] = [
        (
            "1. ¿Qué agrupa una Struct?",
            ["Campos relacionados bajo un tipo propio", "Solo variantes exclusivas", "Únicamente métodos"],
            0,
        ),
        (
            "2. ¿Qué representa un Enum?",
            ["Una colección que siempre crece", "Una variante entre varias posibilidades", "Una referencia mutable"],
            1,
        ),
        (
            "3. ¿Qué define un Trait?",
            ["Un contrato de comportamiento compartido", "Un archivo obligatorio", "Una variable global"],
            0,
        ),
        (
            "4. ¿Para qué sirve impl?",
            ["Para conectar comportamiento con un tipo", "Para reservar memoria manualmente", "Para ocultar todos los campos"],
            0,
        ),
        (
            "5. ¿Cuándo es útil if let?",
            ["Cuando solo interesa una coincidencia de patrón", "Para crear un nuevo Enum", "Para repetir siempre un bloque"],
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
        for tag in ["Struct", "Enum", "Trait", "impl", "if let"] {
            ui.add_space(4.0);
            inline_code_chip_color(ui, tag, egui::Color32::from_rgb(160, 185, 220));
        }
    });
    ui.add_space(12.0);
    ui.label(
        RichText::new("Comprueba si puedes elegir el tipo personalizado adecuado y reconocer su comportamiento.")
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
                state.lessons.structs_preguntas_respuestas[index] == Some(opcion_index);
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
                state.lessons.structs_preguntas_respuestas[index] = Some(opcion_index);
            }
            ui.add_space(3.0);
        }

        if let Some(respuesta) = state.lessons.structs_preguntas_respuestas[index] {
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
        .structs_preguntas_respuestas
        .iter()
        .filter(|respuesta| respuesta.is_some())
        .count();
    let correctas = preguntas
        .iter()
        .enumerate()
        .filter(|(index, (_, _, correcta))| {
            state.lessons.structs_preguntas_respuestas[*index] == Some(*correcta)
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
        ("Structs", 1),
        ("Enums", 2),
        ("Traits", 3),
        ("Info & Memory", 4),
    ];
    let active = state.lessons.structs_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Custom Types",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.structs_tab = idx,
    );
}
