use crate::app::AppState;
use crate::app::ui::{inline_code_chip, section_heading, session_title, tag_chip, Colors, Spacing, Typography};
use eframe::egui::{self, RichText};

pub fn mostrar(ui: &mut egui::Ui, state: &mut AppState) {
    ui.set_max_width(ui.available_width());
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);

    let elapsed = ui.input(|i| i.time) - state.ui.anim_trigger;
    if elapsed < 2.0 {
        ui.ctx().request_repaint();
    }

    let mut anim_delay = 0.0f64;

    let anim_card = |ui: &mut egui::Ui,
                     frame: &egui::Frame,
                     delay: &mut f64,
                     add_contents: &mut dyn FnMut(&mut egui::Ui)| {
        let local = (elapsed - *delay).max(0.0);
        *delay += 0.08;
        let raw_t = (local / 0.5).clamp(0.0, 1.0) as f32;
        let t = 1.0 - (1.0 - raw_t).powi(4);
        ui.scope(|ui| {
            ui.set_width(ui.available_width());
            ui.multiply_opacity(t);
            ui.add_space((1.0 - t) * 25.0);
            frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                add_contents(ui);
            });
        });
    };

    let mut card_frame = egui::Frame::new();
    card_frame.fill = Colors::BG_CARD;
    card_frame.inner_margin = Spacing::card_margin();
    card_frame.corner_radius = Spacing::card_rounding();
    card_frame.stroke = Spacing::card_stroke();

    let mut mini_card = egui::Frame::new();
    mini_card.fill = Colors::BG_CARD;
    mini_card.inner_margin = egui::Margin::same(10);
    mini_card.corner_radius = egui::CornerRadius::same(Spacing::ROUND_SM);
    mini_card.stroke = Spacing::card_stroke();

    // --- SECCIÓN 1: INTRODUCCIÓN Y FILOSOFÍA DE LAS VARIABLES ---
    session_title(ui, "Mecánicas Centrales de Rust");
    ui.add_space(8.0);

    ui.label(
        RichText::new(
            "En Rust, las variables no son simples contenedores de memoria pasivos. El lenguaje estructura el manejo de datos bajo principios estrictos de inmutabilidad por defecto, control explícito de mutabilidad y ciclo de vida determinista. Esto ayuda a mantener el estado del programa predecible y libre de carreras de datos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(22.0)),
    );
    ui.horizontal_wrapped(|ui| {
        inline_code_chip(ui, "RAII");
        inline_code_chip(ui, "Data Races");
    });
    ui.add_space(8.0);

    ui.label(
        RichText::new(
            "Estos conceptos sientan las bases antes de interactuar con el sistema que verifica los préstamos y con las reglas de propiedad. En lugar de lidiar con efectos secundarios imprevistos, el compilador te ayuda a modelar flujos de datos limpios y seguros.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(22.0)),
    );
    ui.horizontal_wrapped(|ui| {
        inline_code_chip(ui, "Borrow Checker");
        inline_code_chip(ui, "Ownership");
    });
    ui.add_space(12.0);

    ui.label(
        RichText::new(
            "Core resume el recorrido completo de esta sesión. El Code Lab desarrolla cada tema paso a paso con ejemplos pequeños y práctica.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_MUTED)
        .line_height(Some(21.0)),
    );
    ui.horizontal_wrapped(|ui| {
        inline_code_chip(ui, "Blocks & Scope");
        inline_code_chip(ui, "Statements & Expressions");
        inline_code_chip(ui, "Data Types");
        inline_code_chip(ui, "Comments");
        inline_code_chip(ui, "Doc Comments");
        inline_code_chip(ui, "Functions");
        inline_code_chip(ui, "Code Lab");
    });
    ui.add_space(14.0);

    // Mini-tarjetas 2x2 conceptuales
    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(RichText::new("Inmutabilidad por Defecto").strong().font(Typography::card_title()).color(Colors::ORANGE_RUST));
                ui.add_space(3.0);
                ui.label(RichText::new("Previene que múltiples partes del código alteren datos de forma no coordinada.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
            ui.add_space(8.0);

            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(RichText::new("Mutabilidad Explícita (mut)").strong().font(Typography::card_title()).color(Colors::ORANGE_RUST));
                ui.add_space(3.0);
                ui.label(RichText::new("La intención de modificar un valor debe ser consciente y visible en la declaración.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
        });

        cols[1].vertical(|ui| {
            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(RichText::new("Gestión Determinista (RAII)").strong().font(Typography::card_title()).color(Colors::ORANGE_RUST));
                ui.add_space(3.0);
                ui.label(RichText::new("Las variables liberan sus recursos automáticamente al salir del bloque donde fueron creadas.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
            ui.add_space(8.0);

            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(RichText::new("Orientado a Expresiones").strong().font(Typography::card_title()).color(Colors::ORANGE_RUST));
                ui.add_space(3.0);
                ui.label(RichText::new("Casi todas las construcciones sintácticas producen un valor evaluado directamente.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
        });
    });

    ui.add_space(20.0);

    // --- SECCIÓN 2: DECLARACIONES Y ENLACES (BINDINGS) ---
    section_heading(ui, "Declaraciones y Tipos de Enlace");
    ui.add_space(8.0);

    ui.columns(3, |cols| {
        // Card 1: let
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("let").font(Typography::card_title()).monospace().strong().color(Colors::ORANGE_RUST));
                ui.add_space(6.0);
                tag_chip(ui, "Inmutable");
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Enlace Inmutable por Defecto").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Asigna un nombre a un valor en el stack local.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Una vez inicializado, el compilador prohíbe reasignaciones accidentales.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // Card 2: let mut
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("let mut").font(Typography::card_title()).monospace().strong().color(Colors::ORANGE_RUST));
                ui.add_space(6.0);
                tag_chip(ui, "Mutable");
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Mutabilidad Controlada").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Habilita la modificación y reasignación de datos.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Exige conservar el mismo tipo de dato asignado originalmente.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // Card 3: const
        anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("const").font(Typography::card_title()).monospace().strong().color(Colors::ORANGE_RUST));
                ui.add_space(6.0);
                tag_chip(ui, "Compile-time");
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Constante en Compilación").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Evaluada antes de ejecutar; requiere tipo de dato explícito obligatorio.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Puede declararse en cualquier ámbito, incluido el global del módulo.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });
    });

    ui.add_space(10.0);

    ui.columns(2, |cols| {
        // Card 4: static
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(120.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("static").font(Typography::card_title()).monospace().strong().color(Colors::ORANGE_RUST));
                ui.add_space(6.0);
                tag_chip(ui, "Global / 'static");
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Ubicación de Memoria Global Fija").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Posee una dirección de memoria fija y vive durante toda la ejecución del programa.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Útil para buffers globales, banderas del sistema y cadenas fijas.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // Card 5: type
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(120.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("type").font(Typography::card_title()).monospace().strong().color(Colors::ORANGE_RUST));
                ui.add_space(6.0);
                tag_chip(ui, "Type Alias");
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Alias de Tipo Descriptivo").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Crea un sinónimo legible para simplificar tipos de datos complejos.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• No crea un tipo nuevo, sino un alias para mejorar la claridad del código.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });
    });

    ui.add_space(20.0);

    // --- SECCIÓN 3: TRANSFORMACIÓN, ÁMBITOS Y EVALUACIÓN ---
    section_heading(ui, "Shadowing, Scopes y Expresiones");
    ui.add_space(8.0);

    ui.columns(3, |cols| {
        // Shadowing
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(150.0);
            ui.heading(RichText::new("Shadowing").font(Typography::card_title()).strong().color(Colors::ORANGE_RUST));
            ui.add_space(4.0);
            ui.label(RichText::new("Ensombrecimiento de Variables").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Permite re-declarar una variable con 'let' usando el mismo nombre.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Hace posible transformar un valor o cambiar su tipo sin perder la inmutabilidad.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• La variable anterior queda oculta y protegida en su ámbito.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // Scopes & Blocks
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(150.0);
            ui.heading(RichText::new("Scopes y Bloques").font(Typography::card_title()).strong().color(Colors::ORANGE_RUST));
            ui.add_space(4.0);
            ui.label(RichText::new("Ciclo de Vida Acotado ({})").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Las llaves delimitan el alcance donde una variable es válida.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Al cerrarse el bloque, el compilador llama al destructor y libera la memoria.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Garantiza ausencia de fugas de memoria sin recolector de basura.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // Expressions vs Statements
        anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(150.0);
            ui.heading(RichText::new("Expresiones vs Sentencias").font(Typography::card_title()).strong().color(Colors::ORANGE_RUST));
            ui.add_space(4.0);
            ui.label(RichText::new("Evaluación de Valores").strong().font(Typography::body()).color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(RichText::new("• Las sentencias (con ';') ejecutan acciones sin producir un valor.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Las expresiones (sin ';') devuelven un valor evaluado inmediatamente.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(RichText::new("• Los bloques de código y las ramas de control son expresiones completas.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });
    });

    ui.add_space(20.0);
}
