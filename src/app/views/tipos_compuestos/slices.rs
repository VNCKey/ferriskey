use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui::{self, RichText};

#[allow(dead_code)]
pub fn mostrar_compuesto_slice(
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
            "Un Slice es una vista prestada sobre una secuencia contigua. Puede observar un Array o un Vector completo, o solamente una parte, sin copiar los elementos ni tomar su Ownership.",
        )
        .font(Typography::body())
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    section_heading(ui, "Crear un Slice");
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(
                "El rango usa un límite inicial incluido y un límite final exclusivo. La expresión devuelve una referencia como",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY),
        );
        inline_highlighted_code(ui, "&[T]", syntax_set, theme, "rs");
        ui.label(
            RichText::new("o")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_highlighted_code(ui, "&mut [T]", syntax_set, theme, "rs");
        ui.label(
            RichText::new(", según el permiso de acceso.")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
    });
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_slice_comp",
        &["Operación", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Slice parcial",
                "&arr[1..4]",
                "&[i32]",
                "Incluye los índices 1, 2 y 3; el límite final 4 no se incluye.",
            ),
            (
                "Slice completo",
                "&arr[..]",
                "&[i32]",
                "Crea una vista prestada de todos los elementos del Array.",
            ),
            (
                "Desde el inicio",
                "&arr[..3]",
                "&[i32]",
                "Incluye los elementos desde el inicio hasta el índice 2.",
            ),
            (
                "Hasta el final",
                "&arr[2..]",
                "&[i32]",
                "Incluye los elementos desde el índice 2 hasta el final.",
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
    section_heading(ui, "Usar un Slice en una función");
    ui.label(
        RichText::new(
            "Una función puede recibir &[T] para leer tanto un Array como una parte de un Vector. El mismo parámetro acepta diferentes tamaños sin copiar la secuencia.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_slice_funciones",
        &["Uso", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Recibir una vista",
                "fn mostrar(valores: &[i32]) {}",
                "&[i32]",
                "La función puede leer cualquier secuencia de i32 prestada.",
            ),
            (
                "Pasar un Array",
                "mostrar(&arr);",
                "Vista completa",
                "Rust convierte el Array en una vista prestada compatible.",
            ),
            (
                "Pasar un rango",
                "mostrar(&arr[1..4]);",
                "Vista parcial",
                "La función recibe solo los elementos seleccionados.",
            ),
        ];

        for (uso, ejemplo, resultado, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(uso)
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
    section_heading(ui, "Métodos principales de un Slice");
    ui.label(
        RichText::new(
            "Un Slice no posee los elementos, pero sí permite consultar su longitud, buscar valores y obtener referencias a sus posiciones.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_slice_metodos",
        &["Método", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Longitud",
                "vista.len()",
                "usize",
                "Devuelve cuántos elementos contiene la vista.",
            ),
            (
                "¿Está vacío?",
                "vista.is_empty()",
                "bool",
                "Indica si el Slice no contiene elementos.",
            ),
            (
                "Primer elemento",
                "vista.first()",
                "&T",
                "Obtiene una referencia al primer elemento si existe.",
            ),
            (
                "Último elemento",
                "vista.last()",
                "&T",
                "Obtiene una referencia al último elemento si existe.",
            ),
            (
                "Acceso seguro",
                "vista.get(0)",
                "&T",
                "Consulta una posición sin provocar un error por índice inválido.",
            ),
            (
                "Comprobar valor",
                "vista.contains(&20)",
                "bool",
                "Indica si el Slice contiene el valor buscado.",
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
    section_heading(ui, "Slices de texto");
    ui.label(
        RichText::new(
            "Un String puede prestar una vista de texto con el tipo &str. El rango debe respetar los límites de los caracteres UTF-8.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_slice_texto",
        &["Operación", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Crear desde String",
                "let texto = String::from(\"Hola Rust\");\nlet vista = &texto[..];",
                "&str",
                "Presta una vista del texto completo sin copiarlo.",
            ),
            (
                "Obtener una parte",
                "let vista = &texto[0..4];",
                "&str",
                "Presta los bytes correspondientes a \"Hola\"; el rango debe estar en un límite válido.",
            ),
        ];

        for (operacion, ejemplo, tipo, descripcion) in filas {
            body.row(|ui| {
                ui.label(
                    RichText::new(operacion)
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
            RichText::new("Nota: un Slice no posee los elementos")
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(
                    "Un Slice guarda una vista hacia datos existentes. Por eso depende de que el Array o el Vector original siga disponible y no puede crecer por sí mismo como",
                )
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
            );
            inline_highlighted_code(ui, "Vec<T>", syntax_set, theme, "rs");
            ui.label(
                RichText::new(".")
                    .font(Typography::body())
                    .color(Colors::TEXT_PRIMARY),
            );
        });
    });
}
