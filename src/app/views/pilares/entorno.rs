use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui;

#[allow(dead_code)]
pub fn mostrar_pilares_entorno_trabajo(ui: &mut egui::Ui, state: &mut AppState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        mostrar_pilares_entorno_contenido(ui, state);
    });
}

pub fn mostrar_pilares_entorno_contenido(ui: &mut egui::Ui, state: &mut AppState) {
    ui.set_max_width(ui.available_width());
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);

    // --- SCROLL REVEAL ENGINE ---
    let elapsed = ui.input(|i| i.time) - state.ui.anim_trigger;
    if elapsed < 2.0 {
        ui.ctx().request_repaint(); // Forzar a redibujar hasta que terminen las transiciones
    }

    let mut anim_delay = 0.0f64;

    let anim_card = |ui: &mut egui::Ui,
                     frame: &egui::Frame,
                     delay: &mut f64,
                     add_contents: &mut dyn FnMut(&mut egui::Ui)| {
        let local = (elapsed - *delay).max(0.0);
        *delay += 0.1; // 100ms de retraso entre tarjetas consecutivas
        let raw_t = (local / 0.6).clamp(0.0, 1.0) as f32;
        let t = 1.0 - (1.0 - raw_t) * (1.0 - raw_t) * (1.0 - raw_t) * (1.0 - raw_t);
        ui.scope(|ui| {
            ui.set_width(ui.available_width());
            ui.multiply_opacity(t);
            ui.add_space((1.0 - t) * 40.0);
            frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                add_contents(ui);
            });
        });
    };

    // Estilo unificado de tarjetas usando el Theme del UI module
    let mut card_frame = egui::Frame::new();
    card_frame.fill = Colors::BG_CARD;
    card_frame.inner_margin = Spacing::card_margin();
    card_frame.corner_radius = Spacing::card_rounding();
    card_frame.stroke = Spacing::card_stroke();

    // --- SECCIÓN: ¿QUÉ ES RUST? (INTRODUCCIÓN, HISTORIA Y ORIGEN) ---
    session_title(ui, "¿Qué es Rust?");

    ui.label(
        egui::RichText::new(
            "Rust es un lenguaje de programación de sistemas moderno creado en 2006 por Graydon Hoare con un objetivo claro: eliminar los frecuentes fallos de seguridad y gestión manual de memoria mediante un compilador estricto. Su diseño permite construir software con rendimiento extremo, seguridad de memoria y concurrencia segura sin carreras de datos.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(22.0)),
    );
    ui.horizontal_wrapped(|ui| {
        inline_code_chip(ui, "Data Races");
    });
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new("El nombre 'Rust' proviene de los hongos de la roya (Rust Fungi), organismos biológicos altamente resistentes capaces de sobrevivir durante años en condiciones extremas. Graydon se inspiró en esa resistencia natural para bautizar un lenguaje enfocado en la robustez y supervivencia del software.")
            .font(Typography::body())
            .color(Colors::TEXT_PRIMARY)
            .line_height(Some(22.0)),
    );
    ui.add_space(12.0);

    // --- SUBSECCIÓN: COMPILACIÓN NATIVA ---
    section_heading(ui, "Compilación Nativa");

    ui.label(
        egui::RichText::new(
            "Rust transforma el código fuente en un binario nativo que el sistema operativo y la CPU pueden ejecutar. En el proceso participan varias herramientas del ecosistema y el enlazador; el resultado final puede ser un ejecutable adaptado al sistema operativo.",
        )
        .font(Typography::body())
        .color(Colors::TEXT_PRIMARY)
        .line_height(Some(22.0)),
    );
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        inline_code_chip(ui, "Cargo");
        inline_code_chip(ui, "rustc");
        inline_code_chip(ui, "LLVM");
        inline_code_chip(ui, "ELF");
        inline_code_chip(ui, "EXE");
        inline_code_chip(ui, "Mach-O");
    });
    ui.add_space(14.0);

    // Mini-tarjetas compactas para las 4 ventajas clave
    let mut mini_card = egui::Frame::new();
    mini_card.fill = Colors::BG_CARD;
    mini_card.inner_margin = egui::Margin::same(8);
    mini_card.corner_radius = egui::CornerRadius::same(Spacing::ROUND_SM);
    mini_card.stroke = Spacing::card_stroke();

    // Grid simétrico de 2 columnas con 4 mini-tarjetas proporcionales
    ui.columns(2, |cols| {
        // Columna Izquierda: Speed & Concurrency
        cols[0].vertical(|ui| {
            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Speed").strong().color(Colors::ORANGE_RUST));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("Ejecución a velocidad nativa sin pausas por Garbage Collector.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
            ui.add_space(8.0);

            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Concurrency").strong().color(Colors::ORANGE_RUST));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("Ejecución de hilos simultáneos y seguros sin Data Races.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
        });

        // Columna Derecha: Safety & Portability
        cols[1].vertical(|ui| {
            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Safety").strong().color(Colors::ORANGE_RUST));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("Garantía estática contra punteros nulos y fallos de memoria en compilación.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
            ui.add_space(8.0);

            mini_card.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(65.0);
                ui.label(egui::RichText::new("Portability").strong().color(Colors::ORANGE_RUST));
                ui.add_space(2.0);
                ui.label(egui::RichText::new("Soporte para arquitecturas de Servidor, Escritorio, Móvil, Microcontroladores y WebAssembly.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            });
        });
    });
    ui.add_space(20.0);

    // --- FILA 1: NÚCLEO DE CONSTRUCCIÓN Y ECOSISTEMA ---
    section_heading(ui, "Núcleo de Construcción y Ecosistema");

    ui.columns(3, |columns| {
        // Pilar 1: rustc
        anim_card(&mut columns[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://rustc.svg",
                        include_bytes!("../../../../assets/diagramas/rustc.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("rustc").size(17.0).strong().color(Colors::ORANGE_RUST));
                ui.add_space(4.0);
                if btn_action_small(ui, "Ver")
                    .on_hover_text("Abrir diagrama del pipeline de compilación de rustc")
                    .clicked()
                {
                    state.ui.show_rustc_compilador_modal = true;
                }
            });
            ui.label(egui::RichText::new("El Compilador Real").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Traduce tu código Rust (.rs) a código máquina optimizado (ELF/EXE/WASM) usando LLVM.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Realiza las verificaciones de seguridad de memoria y el Borrow Checker.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // Pilar 2: Cargo
        anim_card(&mut columns[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://cargo2.svg",
                        include_bytes!("../../../../assets/diagramas/cargo2.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("Cargo").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("El Orquestador / Manager").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Gestor de proyectos y administrador de paquetes oficial de Rust.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Automatiza la descarga de dependencias, compilación y pruebas.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // Pilar 3: Crates / crates.io
        anim_card(&mut columns[2], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://crates.svg",
                        include_bytes!("../../../../assets/diagramas/crates.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("Crates").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("Las Librerías y crates.io").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Una 'Crate' es la unidad de código ejecutable o biblioteca en Rust.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• crates.io es el registro público mundial donde la comunidad comparte paquetes.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });
    });

    ui.add_space(18.0);

    // --- FILA 2: CALIDAD, ESTILO Y DIAGNÓSTICO ---
    section_heading(ui, "Calidad, Estilo y Diagnósticos");

    ui.columns(3, |cols| {
        // 1. Clippy
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://clippy.svg",
                        include_bytes!("../../../../assets/diagramas/clippy.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("Clippy").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("El Linter Oficial").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Analiza tu código con más de 650 reglas avanzadas para detectar anti-patrones.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Enseña las mejores prácticas del código idiomático en Rust.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // 2. Rustfmt
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://format2.svg",
                        include_bytes!("../../../../assets/diagramas/format2.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("Rustfmt").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("El Formateador Estándar").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Aplica automáticamente el libro de estilo unificado a todo el proyecto.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Elimina discusiones de sangría y espacios en equipos de trabajo.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // 3. Error Index
        anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://error.svg",
                        include_bytes!("../../../../assets/diagramas/error.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("Error Index").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("Enciclopedia de Errores").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Enciclopedia explicativa completa para cada código de error del compilador.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Muestra ejemplos de código correcto e incorrecto para aprender del error.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });
    });

    ui.add_space(18.0);

    // --- FILA 3: PRODUCTIVIDAD, IDE Y DOCUMENTACIÓN ---
    section_heading(ui, "Productividad, IDE y Documentación");

    ui.columns(3, |cols| {
        // 1. rustup
        anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://rustup.svg",
                        include_bytes!("../../../../assets/diagramas/rustup.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("rustup").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("Administrador de Toolchains").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Administra las versiones de Rust (Stable, Nightly) y la compilación cruzada.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Permite añadir objetivos como WebAssembly (wasm32) fácilmente.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // 2. rust-analyzer
        anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://analyzer.svg",
                        include_bytes!("../../../../assets/diagramas/analyzer.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("rust-analyzer").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("Servidor de Lenguaje (LSP)").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Proporciona autocompletado en vivo e inlays de tipos inferidos en tu IDE.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Soporta VS Code, Antigravity y Neovim.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });

        // 3. rustdoc
        anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
            ui.set_min_height(140.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_bytes(
                        "bytes://doc.svg",
                        include_bytes!("../../../../assets/diagramas/doc.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(22.0, 22.0))
                    .tint(Colors::ORANGE_RUST),
                );
                ui.add_space(4.0);
                ui.heading(egui::RichText::new("rustdoc").size(17.0).strong().color(Colors::ORANGE_RUST));
            });
            ui.label(egui::RichText::new("Generador de Documentación").strong().color(Colors::TEXT_WHITE));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("• Lee los comentarios de documentación (///) y genera una web HTML completa.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
            ui.label(egui::RichText::new("• Ejecuta doctests automáticamente para garantizar que la documentación funcione.").font(Typography::body_small()).color(Colors::TEXT_PRIMARY));
        });
    });
}
