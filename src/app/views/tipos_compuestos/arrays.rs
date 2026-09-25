use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar_compuesto_array(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    _cyan: egui::Color32,
    texto: egui::Color32,
) {
    let syntax_set = &state.editor.syntax_set;
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];

    ui.label(
        RichText::new(
            "Un Array guarda elementos del mismo tipo y tiene una cantidad fija conocida durante la compilación. Sus valores se almacenan de forma contigua y pueden consultarse mediante índices.",
        )
        .font(Typography::body())
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    section_heading(ui, "Estructura de un Array");
    ui.label(
        RichText::new(
            "La forma [T; N] combina el tipo de cada elemento con la cantidad fija de posiciones. Las expresiones de la segunda columna muestran cómo se utiliza cada característica.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_array_comp",
        &["Pieza", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(100.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Tipo",
                "let arr: [i32; 3] = [10, 20, 30];",
                "[i32; 3]",
                "Tres valores i32; tanto T como N forman parte del tipo.",
            ),
            (
                "Acceso",
                "arr[0]",
                "i32",
                "Obtiene el elemento de la posición indicada; el primer índice es 0.",
            ),
            (
                "Longitud",
                "arr.len()",
                "usize",
                "Devuelve la cantidad fija de elementos del Array.",
            ),
            (
                "Repetición",
                "let zeros = [0u8; 4];",
                "[u8; 4]",
                "Crea cuatro elementos con el mismo valor inicial.",
            ),
            (
                "Slice",
                "let vista = &arr[..];",
                "&[i32]",
                "Crea una vista prestada del Array completo.",
            ),
        ];

        for (pieza, ejemplo, tipo, descripcion) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, pieza, naranja)
                });
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, tipo, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Destructuring Pattern");
    ui.label(
        RichText::new(
            "Un Destructuring Pattern permite separar los elementos de un Array en variables. El patrón debe respetar la cantidad fija del Array y puede usar .. para ignorar las posiciones que no necesitas.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(
                "Sin usar",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY),
        );
        inline_highlighted_code(ui, "..", syntax_set, theme, "rs");
        ui.label(
            RichText::new(
                ", la cantidad de nombres debe coincidir con la cantidad de posiciones. Cada nombre recibe el valor de su posición y el Array original conserva su tamaño fijo.",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY),
        );
    });
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_array_destructuring",
        &["Forma", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Extraer posiciones",
                "let [a, b, c] = arr;",
                "a, b, c",
                "Cada variable recibe el valor de su posición correspondiente.",
            ),
            (
                "Conservar el primero",
                "let [primero, ..] = arr;",
                "primero",
                "Obtiene el primer elemento e ignora el resto del Array.",
            ),
            (
                "Conservar el último",
                "let [.., ultimo] = arr;",
                "ultimo",
                "Ignora las posiciones anteriores y conserva el último elemento.",
            ),
        ];

        for (forma, ejemplo, resultado, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(forma)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, resultado, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Métodos principales");
    ui.label(
        RichText::new(
            "Aunque un Array tiene tamaño fijo, puede consultarse con métodos que también encontrarás al trabajar con Slices.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_array_metodos",
        &["Método", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Longitud",
                "arr.len()",
                "usize",
                "Devuelve la cantidad fija de elementos.",
            ),
            (
                "Primer elemento",
                "arr.first()",
                "&T",
                "Consulta el primer elemento si existe, sin usar un índice directamente.",
            ),
            (
                "Último elemento",
                "arr.last()",
                "&T",
                "Consulta el último elemento si existe.",
            ),
            (
                "Acceso seguro",
                "arr.get(0)",
                "&T",
                "Consulta una posición y evita acceder directamente a un índice inválido.",
            ),
            (
                "Comprobar valor",
                "arr.contains(&20)",
                "bool",
                "Indica si el Array contiene el valor buscado.",
            ),
        ];

        for (metodo, ejemplo, resultado, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(metodo)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, resultado, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Métodos adicionales");
    ui.label(
        RichText::new(
            "Estos métodos permiten comprobar el estado del Array, convertirlo en un Slice y modificar el orden de sus elementos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_array_metodos_adicionales",
        &["Método", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Comprobar si está vacío",
                "arr.is_empty()",
                "bool",
                "Indica si el Array tiene cero elementos.",
            ),
            (
                "Convertir en Slice",
                "arr.as_slice()",
                "&[T]",
                "Obtiene una vista prestada de todo el Array.",
            ),
            (
                "Convertir en Slice mutable",
                "arr.as_mut_slice()",
                "&mut [T]",
                "Obtiene una vista prestada que puede modificar los elementos.",
            ),
            (
                "Ordenar",
                "arr.sort()",
                "Array ordenado",
                "Ordena los elementos; el Array debe ser mutable.",
            ),
            (
                "Invertir",
                "arr.reverse()",
                "Array invertido",
                "Invierte el orden de los elementos; el Array debe ser mutable.",
            ),
            (
                "Intercambiar posiciones",
                "arr.swap(0, 1)",
                "Array modificado",
                "Intercambia dos posiciones; el Array debe ser mutable.",
            ),
        ];

        for (metodo, ejemplo, resultado, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(metodo)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, resultado, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Operaciones con Slices");
    ui.label(
        RichText::new(
            "Un Slice &[T] es una vista prestada. Puede representar todo el Array o solo un rango, sin copiar los elementos ni tomar su Ownership.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_slice_operaciones",
        &["Operación", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(100.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Slice parcial",
                "&arr[1..4]",
                "&[i32]",
                "Incluye los índices 1, 2 y 3; el límite final es exclusivo.",
            ),
            (
                "Slice completo",
                "&arr[..]",
                "&[i32]",
                "Crea una vista prestada de todos los elementos.",
            ),
            (
                "Slice mutable",
                "&mut arr[1..3]",
                "&mut [i32]",
                "Permite modificar ese rango mientras la referencia mutable esté activa.",
            ),
        ];

        for (operacion, ejemplo, tipo, descripcion) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, operacion, naranja)
                });
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, tipo, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Declaración e inicialización");
    ui.label(
        RichText::new(
            "Estas formas cubren los casos más comunes: escribir los valores directamente, repetir un valor, indicar el tipo o crear un Array que pueda modificarse.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_array_ejemplos",
        &["Caso de uso", "Ejemplo de Código", "Tipo / Valor", "Resultado"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Valores directos",
                "let nums = [10, 20, 30];",
                "[i32; 3]",
                "Array fijo con tres enteros.",
            ),
            (
                "Repetición",
                "let buffer = [0u8; 5];",
                "[u8; 5]",
                "Cinco bytes inicializados en cero.",
            ),
            (
                "Tipo explícito",
                "let coords: [f64; 2] = [1.5, 2.5];",
                "[f64; 2]",
                "Dos coordenadas de precisión decimal.",
            ),
            (
                "Array mutable",
                "let mut datos = [1, 2, 3];\ndatos[0] = 10;",
                "[i32; 3]",
                "El contenido cambia, pero la cantidad de posiciones permanece fija.",
            ),
        ];

        for (caso, ejemplo, tipo, resultado) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(caso)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, tipo, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(resultado)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    section_heading(ui, "Arrays multidimensionales");
    ui.label(
        RichText::new(
            "Un Array puede contener otros Arrays. La cantidad de filas y columnas también forma parte del tipo y cada posición se consulta con más de un índice.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_array_multidimensional",
        &["Caso", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Crear una matriz",
                "let matriz: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];",
                "[[i32; 3]; 2]",
                "Crea dos filas con tres enteros en cada una.",
            ),
            (
                "Acceder a un elemento",
                "matriz[0][1]",
                "i32",
                "El primer índice selecciona la fila y el segundo selecciona la columna.",
            ),
        ];

        for (caso, ejemplo, tipo, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(caso)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                table_code_snippet(ui, tipo, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(descripcion)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(18.0);
    card(ui, |ui| {
        ui.label(
            RichText::new("Nota: Tamaño del Array")
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);

        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(
                    "Un Array local suele vivir en",
                )
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
            );
            inline_highlighted_code(ui, "Stack", syntax_set, theme, "rs");
            ui.label(
                RichText::new(
                    ", pero su ubicación depende de dónde viva su propietario. Los Arrays muy grandes pueden agotar el",
                )
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
            );
            inline_highlighted_code(ui, "Stack", syntax_set, theme, "rs");
            ui.label(
                RichText::new(
                    "; cuando necesitas un tamaño dinámico o mucho espacio, normalmente conviene estudiar",
                )
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
            );
            inline_highlighted_code(ui, "Vec", syntax_set, theme, "rs");
            ui.label(
                RichText::new(".")
                    .font(Typography::body())
                    .color(Colors::TEXT_PRIMARY),
            );
        });
    });
}
