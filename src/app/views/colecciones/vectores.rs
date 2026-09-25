use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar_coleccion_vectores(
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
            "Un Vector es una Collection ordenada de elementos del mismo tipo. Posee un buffer en el Heap y puede crecer o reducirse durante la ejecución.",
        )
        .font(Typography::body())
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    section_heading(ui, "Operaciones principales de Vec<T>");
    ui.label(
        RichText::new(
            "Estas operaciones cubren la creación, modificación, consulta y acceso seguro a un Vector.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_vec_operaciones",
        &["Operación", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Crear con macro",
                "let mut v = vec![1, 2, 3];",
                "Vec<i32>",
                "Crea un Vector con valores iniciales en el Heap.",
            ),
            (
                "Crear vacío",
                "let mut v: Vec<i32> = Vec::new();",
                "Vec<i32>",
                "Crea un Vector vacío listo para recibir elementos.",
            ),
            (
                "Reservar capacidad",
                "let mut v = Vec::with_capacity(4);",
                "capacidad = 4",
                "Reserva espacio inicial para reducir reasignaciones posteriores.",
            ),
            (
                "Añadir al final",
                "v.push(4);",
                "len + 1",
                "Inserta un elemento al final y puede aumentar la capacidad.",
            ),
            (
                "Extraer el último",
                "v.pop();",
                "Valor",
                "Elimina y devuelve el último elemento si existe.",
            ),
            (
                "Acceso por índice",
                "v[0]",
                "i32",
                "Accede directamente; un índice inválido produce panic.",
            ),
            (
                "Acceso seguro",
                "v.get(0)",
                "Referencia",
                "Consulta una posición sin producir panic por rango inválido.",
            ),
            (
                "Consultar longitud",
                "v.len()",
                "usize",
                "Devuelve la cantidad de elementos almacenados.",
            ),
            (
                "¿Está vacío?",
                "v.is_empty()",
                "bool",
                "Indica si el Vector no contiene elementos.",
            ),
            (
                "Primer elemento",
                "v.first()",
                "Referencia",
                "Consulta el primer elemento si existe.",
            ),
            (
                "Último elemento",
                "v.last()",
                "Referencia",
                "Consulta el último elemento si existe.",
            ),
            (
                "Comprobar valor",
                "v.contains(&2)",
                "bool",
                "Indica si el Vector contiene el valor buscado.",
            ),
        ];

        for (operacion, ejemplo, resultado, descripcion) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, operacion, naranja)
                });
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
    section_heading(ui, "Modificar cantidad y orden");
    ui.label(
        RichText::new(
            "Estos métodos cambian el contenido o la forma en que están ordenados los elementos del Vector.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_vec_modificacion",
        &["Método", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Insertar por índice",
                "v.insert(1, 20);",
                "len + 1",
                "Inserta un valor y desplaza los elementos siguientes.",
            ),
            (
                "Eliminar por índice",
                "v.remove(1);",
                "len - 1",
                "Elimina una posición y conserva el orden de los demás elementos.",
            ),
            (
                "Acortar",
                "v.truncate(2);",
                "len = 2",
                "Conserva los primeros elementos y elimina el resto.",
            ),
            (
                "Eliminar rápidamente",
                "v.swap_remove(1);",
                "len - 1",
                "Elimina una posición reemplazándola por el último elemento; puede cambiar el orden.",
            ),
            (
                "Añadir otro Vector",
                "v.append(&mut otro);",
                "otro vacío",
                "Mueve todos los elementos de otro Vector al final de v.",
            ),
            (
                "Cambiar tamaño",
                "v.resize(3, 0);",
                "len = 3",
                "Aumenta o reduce la cantidad de elementos usando un valor de relleno.",
            ),
            (
                "Invertir",
                "v.reverse();",
                "orden invertido",
                "Invierte el orden de los elementos.",
            ),
            (
                "Ordenar",
                "v.sort();",
                "ordenado",
                "Ordena los elementos del Vector.",
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
    section_heading(ui, "Convertir un Array en Vec<T>");
    ui.label(
        RichText::new(
            "Cuando una secuencia fija necesita crecer o reducirse, puedes convertir el Array en un Vector dinámico.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_vec_conversion",
        &["Operación", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        body.row(|ui| {
            ui.label(
                RichText::new("Array a Vector")
                    .font(Typography::body_small())
                    .color(Colors::TEXT_PRIMARY),
            );
            table_code_snippet(
                ui,
                "let arr = [1, 2, 3];\nlet v = arr.to_vec();",
                syntax_set,
                theme,
                "rs",
            );
            table_code_snippet(ui, "Vec<i32>", syntax_set, theme, "rs");
            ui.label(
                RichText::new("Crea un Vector con los valores del Array.")
                    .font(Typography::body_small())
                    .color(Colors::TEXT_PRIMARY),
            );
        });
    });

    ui.add_space(18.0);
    card(ui, |ui| {
        ui.label(
            RichText::new("Nota: Destructuring Pattern en Vec<T>")
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(
                    "Un Vec<T> tiene tamaño dinámico, por lo que normalmente no se desestructura como un Array o una Tupla. Para obtener sus valores se utilizan métodos de consulta como",
                )
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
            );
            inline_highlighted_code(ui, "first", syntax_set, theme, "rs");
            ui.label(
                RichText::new(",")
                    .font(Typography::body())
                    .color(Colors::TEXT_PRIMARY),
            );
            inline_highlighted_code(ui, "get", syntax_set, theme, "rs");
            ui.label(
                RichText::new(" y ")
                    .font(Typography::body())
                    .color(Colors::TEXT_PRIMARY),
            );
            ui.label(
                RichText::new(".")
                    .font(Typography::body())
                    .color(Colors::TEXT_PRIMARY),
            );
        });
    });

    ui.add_space(18.0);
    section_heading(ui, "Capacidad y memoria");
    ui.label(
        RichText::new(
            "Un Vector mantiene una longitud y una capacidad. La longitud indica cuántos elementos existen; la capacidad indica cuánto espacio está reservado antes de necesitar otra reserva.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_vec_memoria",
        &["Concepto", "Ejemplo de Código", "Valor", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Longitud",
                "v.len()",
                "Cantidad actual",
                "Elementos que el Vector contiene ahora.",
            ),
            (
                "Capacidad",
                "v.capacity()",
                "Espacio reservado",
                "Elementos que puede almacenar antes de reservar más memoria.",
            ),
            (
                "Vaciar",
                "v.clear();",
                "len = 0",
                "Elimina los elementos, pero puede conservar la capacidad reservada.",
            ),
        ];

        for (concepto, ejemplo, valor, descripcion) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, concepto, naranja)
                });
                table_code_snippet(ui, ejemplo, syntax_set, theme, "rs");
                ui.label(
                    RichText::new(valor)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
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
            RichText::new("Nota: Vec<T> y tamaño dinámico")
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(
                    "Vec<T> posee sus elementos y administra un buffer dinámico en el Heap. Cuando el Vector necesita más espacio, puede reservar una región nueva y mover sus elementos.",
                )
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
            );
        });
    });
}
