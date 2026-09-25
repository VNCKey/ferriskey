use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar_coleccion_hashmap(
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
            "Un HashMap<K, V> almacena pares clave-valor. La clave permite encontrar su valor asociado y el mapa puede crecer dinámicamente en el Heap.",
        )
        .font(Typography::body())
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    section_heading(ui, "Operaciones principales de HashMap<K, V>");
    ui.label(
        RichText::new(
            "HashMap requiere importar su tipo desde la biblioteca estándar. Sus operaciones principales crean, insertan, consultan y eliminan pares clave-valor.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_hashmap_operaciones",
        &["Operación", "Ejemplo de Código", "Resultado", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Importar tipo",
                "use std::collections::HashMap;",
                "HashMap<K, V>",
                "Hace disponible el tipo de mapa de la biblioteca estándar.",
            ),
            (
                "Crear nuevo",
                "let mut map = HashMap::new();",
                "HashMap<K, V>",
                "Crea un mapa vacío que puede recibir pares clave-valor.",
            ),
            (
                "Insertar par",
                "map.insert(\"clave\", 10);",
                "Valor anterior",
                "Inserta un valor o reemplaza el valor anterior de la clave.",
            ),
            (
                "Buscar por clave",
                "map.get(\"clave\");",
                "Referencia",
                "Devuelve una referencia al valor si la clave existe.",
            ),
            (
                "¿Contiene una clave?",
                "map.contains_key(\"clave\")",
                "bool",
                "Indica si el mapa contiene la clave buscada.",
            ),
            (
                "Eliminar por clave",
                "map.remove(\"clave\");",
                "Valor eliminado",
                "Elimina el par y devuelve el valor que estaba asociado.",
            ),
            (
                "¿Está vacío?",
                "map.is_empty()",
                "bool",
                "Indica si el mapa no contiene pares clave-valor.",
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
    card(ui, |ui| {
        ui.label(
            RichText::new("Nota: Destructuring Pattern en HashMap")
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(
                    "Un HashMap no se desarma como una Tupla porque sus entradas se organizan como pares clave-valor y su tamaño puede cambiar. En esta etapa se consulta mediante sus métodos; el recorrido de sus pares se verá más adelante.",
                )
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
            );
        });
    });

    ui.add_space(18.0);
    section_heading(ui, "Claves y valores");
    ui.label(
        RichText::new(
            "Las claves identifican los valores. Una misma clave no puede representar dos valores al mismo tiempo: al insertar una clave existente, el mapa actualiza su valor.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_hashmap_datos",
        &["Concepto", "Ejemplo de Código", "Tipo / Valor", "Descripción"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(14.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "Clave",
                "\"usuario_id\"",
                "K",
                "Identifica de forma única el valor almacenado.",
            ),
            (
                "Valor",
                "42",
                "V",
                "Dato asociado a una clave.",
            ),
            (
                "Longitud",
                "map.len()",
                "usize",
                "Cuenta cuántos pares clave-valor contiene el mapa.",
            ),
            (
                "Vaciar",
                "map.clear();",
                "len = 0",
                "Elimina todos los pares almacenados.",
            ),
        ];

        for (concepto, ejemplo, tipo, descripcion) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, concepto, naranja)
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
    card(ui, |ui| {
        ui.label(
            RichText::new("Nota: el orden de un HashMap")
                .font(Typography::card_title())
                .strong()
                .color(Colors::ORANGE_RUST),
        );
        ui.add_space(5.0);
        ui.label(
            RichText::new(
                "HashMap está diseñado para buscar por clave, no para conservar el orden de inserción. Si necesitas recorrer elementos en un orden concreto, debes elegir otra estructura o mantener el orden por separado.",
            )
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY)
            .line_height(Some(19.0)),
        );
    });
}
