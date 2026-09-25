use crate::app::ui::*;
use eframe::egui::RichText;

pub fn mostrar(ui: &mut egui::Ui) {
    session_title(ui, "Control de Flujo");
    session_intro(
        ui,
        "Esta sesión explica cómo un programa decide, repite acciones y selecciona comportamientos según los datos que recibe.",
    );

    ui.label(
        RichText::new(
            "En Rust, las construcciones de Control de Flujo también pueden ser expresiones: algunas ejecutan un bloque y otras producen un valor que puedes guardar en una variable.",
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
            "Condicionales",
            "Aprenderás a tomar decisiones con if, else if y else, y a obtener valores directamente desde sus ramas.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Bucles",
            "Conocerás loop, while y for, además de los rangos, break y continue para controlar las repeticiones.",
        );

        card_overview(
            &mut columns[1],
            "Match",
            "Usarás Pattern Matching para comparar valores, cubrir todos los casos y devolver resultados de forma segura.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Code Lab",
            "La barra lateral reúne una explicación progresiva y el editor central permite practicar cada construcción en un proyecto Cargo.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Cómo elegir una construcción");
    ui.label(
        RichText::new(
            "Usa una condicional cuando debas elegir entre caminos. Usa un bucle cuando necesites repetir una acción. Usa match cuando tengas varios patrones que comparar y quieras que Rust te ayude a cubrir todos los casos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    EducationalTable::new(
        "tabla_mapa_control_flujo",
        &["Construcción", "Pregunta que responde", "Resultado habitual", "Cuándo utilizarla"],
    )
    .min_col_width(100.0)
    .max_col_width(((ui.available_width() - 100.0) / 4.0).max(130.0))
    .spacing(16.0, 7.0)
    .show(ui, |body| {
        let filas = [
            (
                "if",
                "¿Se cumple esta condición?",
                "Ejecuta una rama",
                "Decisiones booleanas simples o encadenadas.",
            ),
            (
                "loop / while / for",
                "¿Debo repetir esta acción?",
                "Repite un bloque",
                "Procesos repetitivos, rangos y colecciones.",
            ),
            (
                "match",
                "¿Con qué patrón coincide?",
                "Ejecuta un brazo o devuelve un valor",
                "Varios casos posibles y Pattern Matching.",
            ),
        ];

        for (construccion, pregunta, resultado, uso) in filas {
            body.row(|ui| {
                cell_centered(ui, |ui| {
                    inline_code_chip_color(ui, construccion, Colors::ORANGE_RUST)
                });
                ui.label(
                    RichText::new(pregunta)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                ui.label(
                    RichText::new(resultado)
                        .font(Typography::body_small())
                        .color(Colors::TEXT_PRIMARY),
                );
                texto_con_chips_inline(ui, uso, Colors::TEXT_PRIMARY, Colors::CYAN_ACCENT);
            });
        }
    });

    ui.add_space(16.0);
    ui.label(
        RichText::new(
            "Las pestañas siguientes explican cada construcción con tablas, ejemplos y reglas. Después puedes abrir Code Lab para practicar y finalizar con Questions.",
        )
        .font(Typography::body_small())
        .color(Colors::TEXT_MUTED)
        .line_height(Some(19.0)),
    );
}
