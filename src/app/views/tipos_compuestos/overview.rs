use crate::app::ui::*;
use eframe::egui::{self, RichText};

pub fn mostrar(ui: &mut egui::Ui) {
    session_title(ui, "Tipos Compuestos");
    session_intro(
        ui,
        "Esta sesión explica cómo Rust combina varios valores dentro de una sola estructura. Verás cuándo utilizar Arrays, Tuplas, Collections y Slices, y cómo elegir la opción adecuada según el tipo de datos y la forma en que necesitas acceder a ellos.",
    );

    ui.label(
        RichText::new(
            "La idea central es comparar tres características: si los elementos tienen el mismo tipo, si el tamaño es fijo o dinámico y si la estructura posee los datos o solo ofrece una vista prestada.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    section_heading(ui, "Recorrido de la sesión");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Arrays",
            "Agrupan valores del mismo tipo con una cantidad fija de elementos. Su tamaño forma parte del tipo y resulta útil cuando conoces la longitud desde el principio.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Collections",
            "Permiten almacenar y gestionar cantidades variables de datos. Verás estructuras como Vec y otras Collections habituales del ecosistema de Rust.",
        );

        card_overview(
            &mut columns[1],
            "Tuplas",
            "Agrupan valores relacionados aunque tengan tipos diferentes. Su posición identifica cada elemento y su estructura tiene un tamaño fijo.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Slices",
            "Son vistas prestadas sobre una secuencia contigua. Permiten trabajar con una parte de un Array o una Collection sin copiar sus elementos ni tomar su Ownership.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Cómo elegir");
    ui.label(
        RichText::new(
            "Usa un Array cuando el tamaño sea fijo y todos los elementos compartan tipo. Usa una Tupla para agrupar pocos valores relacionados de tipos distintos. Usa una Collection cuando la cantidad pueda cambiar y un Slice cuando solo necesites prestar una secuencia a otra parte del programa.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_mapa_tipos_compuestos",
        &["Estructura", "Elementos", "Tamaño", "Ownership"],
    )
    .min_col_width(105.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(16.0, 7.0)
    .show(ui, |body| {
        let filas = [
            ("Array", "Mismo tipo", "Fijo", "Posee los valores"),
            ("Tupla", "Puede variar", "Fijo", "Posee los valores"),
            ("Collection", "Depende de la colección", "Dinámico", "Posee los valores"),
            ("Slice", "Mismo tipo", "Vista variable", "No posee los valores"),
        ];

        for (estructura, elementos, tamano, ownership) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, estructura, Colors::ORANGE_RUST)
                });
                ui.label(
                    RichText::new(elementos)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                ui.label(
                    RichText::new(tamano)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                ui.label(
                    RichText::new(ownership)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
            });
        }
    });

    ui.add_space(16.0);
    ui.label(
        RichText::new(
            "Las pestañas siguientes desarrollan cada estructura con tablas, ejemplos y métodos. El Code Lab reúne ejercicios para practicar su declaración, acceso y uso en un proyecto Cargo.",
        )
        .font(Typography::body_small())
        .color(Colors::TEXT_MUTED)
        .line_height(Some(19.0)),
    );
}
