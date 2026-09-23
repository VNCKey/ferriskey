use crate::app::AppState;
use crate::views::pilares::anatomy::codigo_inline_chip;
use eframe::egui;

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

    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let green = egui::Color32::from_rgb(100, 220, 150);
    let text_color = egui::Color32::from_rgb(200, 210, 225);

    let mut card_frame = egui::Frame::new();
    card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    card_frame.inner_margin = egui::Margin::same(12);
    card_frame.corner_radius = egui::CornerRadius::same(8);
    card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    let mut mini_card = egui::Frame::new();
    mini_card.fill = egui::Color32::from_rgb(14, 18, 26);
    mini_card.inner_margin = egui::Margin::same(8);
    mini_card.corner_radius = egui::CornerRadius::same(6);
    mini_card.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    // --- SECCIÓN 1: INTRODUCCIÓN Y FILOSOFÍA DE LAS VARIABLES ---
    ui.heading(
        egui::RichText::new("Mecánicas Centrales de Rust")
            .size(24.0)
            .strong()
            .color(title_color),
    );
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new(
            "En Rust, las variables no son simples contenedores de memoria pasivos. El lenguaje estructura el manejo de datos bajo principios estrictos de inmutabilidad por defecto, control explícito de mutabilidad y ciclo de vida determinista. Esto ayuda a mantener el estado del programa predecible y libre de carreras de datos.",
        )
        .size(15.0)
        .color(text_color)
        .line_height(Some(22.0)),
    );
    ui.horizontal_wrapped(|ui| {
        codigo_inline_chip(ui, "RAII");
        codigo_inline_chip(ui, "Data Races");
    });
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new(
            "Estos conceptos sientan las bases antes de interactuar con el sistema que verifica los préstamos y con las reglas de propiedad. En lugar de lidiar con efectos secundarios imprevistos, el compilador te ayuda a modelar flujos de datos limpios y seguros.",
        )
        .size(15.0)
        .color(text_color)
        .line_height(Some(22.0)),
    );
    ui.horizontal_wrapped(|ui| {
        codigo_inline_chip(ui, "Borrow Checker");
        codigo_inline_chip(ui, "Ownership");
    });
    ui.add_space(12.0);

    ui.label(
        egui::RichText::new(
            "Core resume el recorrido completo de esta sesión. El Code Lab desarrolla cada tema paso a paso con ejemplos pequeños y práctica.",
        )
        .size(14.0)
        .color(text_color)
        .line_height(Some(21.0)),
    );
    ui.horizontal_wrapped(|ui| {
        codigo_inline_chip(ui, "Blocks & Scope");
        codigo_inline_chip(ui, "Statements & Expressions");
        codigo_inline_chip(ui, "Data Types");
        codigo_inline_chip(ui, "Comments");
        codigo_inline_chip(ui, "Doc Comments");
        codigo_inline_chip(ui, "Functions");
        codigo_inline_chip(ui, "Code Lab");
    });
    ui.add_space(12.0);

    // Mini-tarjetas 2x2 conceptuales (estilo Foundations)
    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Inmutabilidad por Defecto").strong().color(title_color));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("Previene que múltiples partes del código alteren datos de forma no coordinada.").color(text_color));
            });
            ui.add_space(8.0);

            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Mutabilidad Explícita (mut)").strong().color(title_color));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("La intención de modificar un valor debe ser consciente y visible en la declaración.").color(text_color));
            });
        });

        cols[1].vertical(|ui| {
            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Gestión Determinista (RAII)").strong().color(title_color));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("Las variables liberan sus recursos automáticamente al salir del bloque donde fueron creadas.").color(text_color));
            });
            ui.add_space(8.0);

            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Orientado a Expresiones").strong().color(title_color));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("Casi todas las construcciones sintácticas producen un valor evaluado directamente.").color(text_color));
            });
        });
    });

    ui.add_space(20.0);

    // --- SECCIÓN 2: DECLARACIONES Y ENLACES (BINDINGS) ---
    ui.heading(
        egui::RichText::new("Declaraciones y Tipos de Enlace")
            .size(18.0)
            .strong()
            .color(title_color),
    );
    ui.add_space(8.0);

    ui.columns(3, |cols| {
        // Card 1: let
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("let").size(17.0).monospace().strong().color(title_color));
                ui.add_space(6.0);
                let mut badge = egui::Frame::new();
                badge.fill = egui::Color32::from_rgb(18, 44, 30);
                badge.corner_radius = egui::CornerRadius::same(4);
                badge.inner_margin = egui::Margin::symmetric(6, 2);
                badge.show(ui, |ui| {
                    ui.label(egui::RichText::new("Inmutable").size(11.0).strong().color(green));
                });
            });
            ui.label(egui::RichText::new("Enlace Inmutable por Defecto").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Asigna un nombre a un valor en el stack local.").color(text_color));
            ui.label(egui::RichText::new("• Una vez inicializado, el compilador prohíbe reasignaciones accidentales.").color(text_color));
        });

        // Card 2: let mut
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("let mut").size(17.0).monospace().strong().color(title_color));
                ui.add_space(6.0);
                let mut badge = egui::Frame::new();
                badge.fill = egui::Color32::from_rgb(44, 30, 18);
                badge.corner_radius = egui::CornerRadius::same(4);
                badge.inner_margin = egui::Margin::symmetric(6, 2);
                badge.show(ui, |ui| {
                    ui.label(egui::RichText::new("Mutable").size(11.0).strong().color(title_color));
                });
            });
            ui.label(egui::RichText::new("Mutabilidad Controlada").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Habilita la modificación y reasignación de datos.").color(text_color));
            ui.label(egui::RichText::new("• Exige conservar el mismo tipo de dato asignado originalmente.").color(text_color));
        });

        // Card 3: const
        anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("const").size(17.0).monospace().strong().color(title_color));
                ui.add_space(6.0);
                let mut badge = egui::Frame::new();
                badge.fill = egui::Color32::from_rgb(18, 30, 44);
                badge.corner_radius = egui::CornerRadius::same(4);
                badge.inner_margin = egui::Margin::symmetric(6, 2);
                badge.show(ui, |ui| {
                    ui.label(egui::RichText::new("Compile-time").size(11.0).strong().color(cyan));
                });
            });
            ui.label(egui::RichText::new("Constante en Compilación").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Evaluada antes de ejecutar; requiere tipo de dato explícito obligatorio.").color(text_color));
            ui.label(egui::RichText::new("• Puede declararse en cualquier ámbito, incluido el global del módulo.").color(text_color));
        });
    });

    ui.add_space(10.0);

    ui.columns(2, |cols| {
        // Card 4: static
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(120.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("static").size(17.0).monospace().strong().color(title_color));
                ui.add_space(6.0);
                let mut badge = egui::Frame::new();
                badge.fill = egui::Color32::from_rgb(25, 20, 35);
                badge.corner_radius = egui::CornerRadius::same(4);
                badge.inner_margin = egui::Margin::symmetric(6, 2);
                badge.show(ui, |ui| {
                    ui.label(egui::RichText::new("Global / 'static").size(11.0).strong().color(egui::Color32::from_rgb(200, 160, 255)));
                });
            });
            ui.label(egui::RichText::new("Ubicación de Memoria Global Fija").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Posee una dirección de memoria fija y vive durante toda la ejecución del programa.").color(text_color));
            ui.label(egui::RichText::new("• Útil para buffers globales, banderas del sistema y cadenas fijas.").color(text_color));
        });

        // Card 5: type
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(120.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("type").size(17.0).monospace().strong().color(title_color));
                ui.add_space(6.0);
                let mut badge = egui::Frame::new();
                badge.fill = egui::Color32::from_rgb(18, 30, 44);
                badge.corner_radius = egui::CornerRadius::same(4);
                badge.inner_margin = egui::Margin::symmetric(6, 2);
                badge.show(ui, |ui| {
                    ui.label(egui::RichText::new("Type Alias").size(11.0).strong().color(cyan));
                });
            });
            ui.label(egui::RichText::new("Alias de Tipo Descriptivo").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Crea un sinónimo legible para simplificar tipos de datos complejos.").color(text_color));
            ui.label(egui::RichText::new("• No crea un tipo nuevo, sino un alias para mejorar la claridad del código.").color(text_color));
        });
    });

    ui.add_space(20.0);

    // --- SECCIÓN 3: TRANSFORMACIÓN, ÁMBITOS Y EVALUACIÓN ---
    ui.heading(
        egui::RichText::new("Shadowing, Scopes y Expresiones")
            .size(18.0)
            .strong()
            .color(title_color),
    );
    ui.add_space(8.0);

    ui.columns(3, |cols| {
        // Shadowing
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(150.0);
            ui.heading(egui::RichText::new("Shadowing").size(17.0).strong().color(title_color));
            ui.label(egui::RichText::new("Ensombrecimiento de Variables").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Permite re-declarar una variable con 'let' usando el mismo nombre.").color(text_color));
            ui.label(egui::RichText::new("• Hace posible transformar un valor o cambiar su tipo sin perder la inmutabilidad.").color(text_color));
            ui.label(egui::RichText::new("• La variable anterior queda oculta y protegida en su ámbito.").color(text_color));
        });

        // Scopes & Blocks
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(150.0);
            ui.heading(egui::RichText::new("Scopes y Bloques").size(17.0).strong().color(title_color));
            ui.label(egui::RichText::new("Ciclo de Vida Acotado ({})").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Las llaves delimitan el alcance donde una variable es válida.").color(text_color));
            ui.label(egui::RichText::new("• Al cerrarse el bloque, el compilador llama al destructor y libera la memoria.").color(text_color));
            ui.label(egui::RichText::new("• Garantiza ausencia de fugas de memoria sin recolector de basura.").color(text_color));
        });

        // Expressions vs Statements
        anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(150.0);
            ui.heading(egui::RichText::new("Expresiones vs Sentencias").size(17.0).strong().color(title_color));
            ui.label(egui::RichText::new("Evaluación de Valores").strong().color(egui::Color32::WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Las sentencias (con ';') ejecutan acciones sin producir un valor.").color(text_color));
            ui.label(egui::RichText::new("• Las expresiones (sin ';') devuelven un valor evaluado inmediatamente.").color(text_color));
            ui.label(egui::RichText::new("• Los bloques de código y las ramas de control son expresiones completas.").color(text_color));
        });
    });

    ui.add_space(20.0);
}
