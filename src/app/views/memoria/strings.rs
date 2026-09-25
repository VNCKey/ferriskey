use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui::{self, RichText};

/// Referencia visual y progresiva sobre `String` y `&str`.
pub fn mostrar_teoria_string_y_str(ui: &mut egui::Ui, state: &mut AppState) {
    let syntax_set = &state.editor.syntax_set;
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];

    session_title(ui, "String vs &str");
    session_intro(
        ui,
        "En Rust, el texto puede ser un valor que posee sus datos o una vista que solo los observa. Esta diferencia explica por qué algunos textos pueden crecer, cómo se prestan sin moverlos y qué ocurre con su memoria.",
    );

    section_heading(ui, "¿Qué es String?");
    ui.label(
        RichText::new(
            "String es un tipo de texto con Ownership. Posee los bytes que forman su contenido y administra un buffer en el Heap. Puede crecer durante la ejecución cuando la variable es mutable, por lo que es útil para texto que se construye o cambia.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);
    highlighted_code_block(
        ui,
        "let mut texto = String::from(\"Rust\");\ntexto.push_str(\" seguro\");",
        syntax_set,
        theme,
        "rs",
    );

    ui.add_space(16.0);
    section_heading(ui, "¿Qué es &str?");
    ui.label(
        RichText::new(
            "&str es una vista prestada de texto UTF-8. No posee los bytes ni decide cuándo se liberan: solo permite leer un texto que pertenece a otro valor o que ya forma parte del programa. Por eso suele utilizarse cuando una función necesita leer texto sin quedarse con su Ownership.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);
    highlighted_code_block(
        ui,
        "let texto = String::from(\"Rust\");\nlet vista: &str = &texto;\nprintln!(\"{vista}\");",
        syntax_set,
        theme,
        "rs",
    );

    ui.add_space(16.0);
    section_heading(ui, "Comparación rápida");
    ui.label(
        RichText::new(
            "Ambos representan texto, pero la diferencia está en quién posee los datos y qué puede hacer con ellos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_comparacion_string_str",
        &["Característica", "String", "&str"],
    )
    .min_col_width(115.0)
    .max_col_width(((ui.available_width() - 80.0) / 3.0).max(150.0))
    .spacing(16.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Ownership",
                "Posee su buffer de texto.",
                "No posee el texto; solo lo observa.",
            ),
            (
                "Memoria",
                "Administra un buffer dinámico en el Heap.",
                "Guarda una vista hacia texto existente.",
            ),
            (
                "Tamaño",
                "Puede crecer si la variable es mutable.",
                "La vista tiene una longitud fija.",
            ),
            (
                "Creación",
                "String::from(\"Rust\")",
                "\"Rust\" o una referencia a String.",
            ),
            (
                "Uso",
                "Se puede prestar con &texto.",
                "Se puede leer directamente mientras exista su texto original.",
            ),
        ];

        for (caracteristica, string, str_ref) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, caracteristica, Colors::ORANGE_RUST)
                });
                ui.label(
                    RichText::new(string)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                ui.label(
                    RichText::new(str_ref)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(16.0);
    section_heading(ui, "Un literal de texto");
    ui.label(
        RichText::new(
            "Los literales de texto suelen tener el tipo &str. El texto ya existe como parte del programa, por eso no necesitas crear una String si solo vas a leerlo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(8.0);
    highlighted_code_block(
        ui,
        "let saludo: &str = \"Hola, Rust!\";",
        syntax_set,
        theme,
        "rs",
    );

    ui.add_space(18.0);
    section_heading(ui, "String por dentro");
    ui.label(
        RichText::new(
            "Una String suele guardar en el Stack tres datos pequeños: un puntero al buffer, su longitud actual y su capacidad reservada. Los bytes del texto viven en el Heap. En una arquitectura de 64 bits, esos tres datos suelen ocupar 24 bytes en total.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    let button_label = if state.ui.show_railroad_modal == Some(7) {
        "Ocultar diagrama"
    } else {
        "Ver diagrama de memoria"
    };
    ui.horizontal(|ui| {
        let response = btn_action_small(ui, button_label)
            .on_hover_text("Abrir diagrama visual de la memoria de String");
        if response.clicked() {
            state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(7) {
                None
            } else {
                Some(7)
            };
        }
    });
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_anatomia_string",
        &["Campo", "Ubicación", "Qué representa"],
    )
    .min_col_width(110.0)
    .max_col_width(((ui.available_width() - 70.0) / 3.0).max(170.0))
    .spacing(18.0, 7.0)
    .show(ui, |body| {
        let campos = [
            (
                "ptr",
                "Stack",
                "Dirección del buffer de bytes que está en el Heap.",
            ),
            (
                "len",
                "Stack",
                "Cantidad de bytes que el texto utiliza actualmente.",
            ),
            (
                "capacity",
                "Stack",
                "Cantidad de bytes reservados antes de necesitar más espacio.",
            ),
            (
                "Buffer UTF-8",
                "Heap",
                "Bytes que representan el contenido del texto.",
            ),
        ];

        for (campo, ubicacion, descripcion) in campos {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, campo, Colors::CYAN_ACCENT)
                });
                ui.label(
                    RichText::new(ubicacion)
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
    section_heading(ui, "Texto UTF-8 y longitud");
    ui.label(
        RichText::new(
            "String y &str guardan texto UTF-8. Por eso .len() cuenta bytes y no necesariamente la cantidad de caracteres visibles: algunos caracteres necesitan más de un byte.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);
    highlighted_code_block(
        ui,
        "let texto = \"🦀\";\nprintln!(\"{} bytes\", texto.len()); // 4 bytes",
        syntax_set,
        theme,
        "rs",
    );

    ui.add_space(20.0);
    section_heading(ui, "Métodos de String y &str");
    ui.label(
        RichText::new(
            "Estas operaciones sirven para crear, convertir, consultar, modificar o producir texto. Las tablas muestran qué tipo de valor participa y qué resultado puedes esperar.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(12.0);

    let filas_creacion = [
        (
            "String::new()",
            "Crea una String vacía que puede recibir texto después.",
            "let texto = String::new()",
        ),
        (
            "String::from(&str)",
            "Crea una String con Ownership a partir de una vista de texto.",
            "String::from(\"Rust\")",
        ),
        (
            ".to_string()",
            "Crea una String a partir de un valor que puede representarse como texto.",
            "\"Rust\".to_string()",
        ),
        (
            ".as_str()",
            "Obtiene una vista &str de una String sin copiar sus bytes.",
            "texto.as_str() // &str",
        ),
    ];
    tabla_metodos(
        ui,
        "grid_creacion_string",
        "Crear y convertir",
        "Operaciones que ayudan a pasar de un texto prestado a una String con Ownership, o a obtener una vista sin copiar.",
        syntax_set,
        theme,
        &filas_creacion,
    );

    ui.add_space(14.0);

    let filas_consulta = [
        (".len()", "Cuenta los bytes utilizados por el texto.", "texto.len()"),
        (
            ".capacity()",
            "Consulta cuántos bytes están reservados para la String.",
            "texto.capacity()",
        ),
        (
            ".is_empty()",
            "Indica si el texto no contiene ningún byte.",
            "texto.is_empty()",
        ),
        (
            ".contains(str)",
            "Comprueba si el texto contiene una parte determinada.",
            "texto.contains(\"Rust\")",
        ),
        (
            ".starts_with(str)",
            "Comprueba si comienza con el texto indicado.",
            "texto.starts_with(\"Ru\")",
        ),
        (
            ".ends_with(str)",
            "Comprueba si termina con el texto indicado.",
            "texto.ends_with(\"!\")",
        ),
    ];
    tabla_metodos(
        ui,
        "grid_consulta_string",
        "Consulta",
        "Métodos que observan el texto sin modificarlo.",
        syntax_set,
        theme,
        &filas_consulta,
    );

    ui.add_space(14.0);
    let filas_modificacion = [
        (
            ".push(char)",
            "Añade un carácter al final de una String mutable.",
            "texto.push('!')",
        ),
        (
            ".push_str(&str)",
            "Añade una vista de texto al final de una String mutable.",
            "texto.push_str(\" Rust\")",
        ),
        (
            ".clear()",
            "Elimina todo el contenido y deja la String vacía.",
            "texto.clear()",
        ),
        (
            ".truncate(n)",
            "Conserva los primeros n bytes del texto.",
            "texto.truncate(4)",
        ),
    ];
    tabla_metodos(
        ui,
        "grid_modificacion_string",
        "Modificación",
        "Métodos que cambian el buffer; la String debe ser mutable.",
        syntax_set,
        theme,
        &filas_modificacion,
    );

    ui.add_space(14.0);
    let filas_transformacion = [
        (
            ".trim()",
            "Devuelve una vista sin espacios exteriores.",
            "texto.trim() // &str",
        ),
        (
            ".to_uppercase()",
            "Crea una nueva String en mayúsculas.",
            "texto.to_uppercase()",
        ),
        (
            ".to_lowercase()",
            "Crea una nueva String en minúsculas.",
            "texto.to_lowercase()",
        ),
        (
            ".replace(a, b)",
            "Crea una nueva String reemplazando coincidencias.",
            "texto.replace(\"Rust\", \"RUST\")",
        ),
    ];
    tabla_metodos(
        ui,
        "grid_transformacion_string",
        "Transformación",
        "Métodos que devuelven una vista o una nueva String sin cambiar el texto original.",
        syntax_set,
        theme,
        &filas_transformacion,
    );
}
