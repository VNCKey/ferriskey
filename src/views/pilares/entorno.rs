use crate::app::AppState;
use eframe::egui;

#[allow(dead_code)]
pub fn mostrar_pilares_entorno_trabajo(ui: &mut egui::Ui, state: &mut AppState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        mostrar_pilares_entorno_contenido(ui, state);
    });
}

pub fn mostrar_pilares_entorno_contenido(ui: &mut egui::Ui, state: &mut AppState) {
    // --- SCROLL REVEAL ENGINE ---
    let elapsed = ui.input(|i| i.time) - state.ui.anim_trigger;
    if elapsed < 2.0 {
        ui.ctx().request_repaint(); // Forzar a redibujar hasta que terminen todas
    }

    let mut anim_delay = 0.0f64;

    let anim_card = |ui: &mut egui::Ui,
                     frame: &egui::Frame,
                     delay: &mut f64,
                     add_contents: &mut dyn FnMut(&mut egui::Ui)| {
        let local = (elapsed - *delay).max(0.0);
        *delay += 0.1; // 100ms delay for the next card
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

    // Estilo unificado de tarjetas
    let mut card_frame = egui::Frame::new();
    card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    card_frame.inner_margin = egui::Margin::same(12);
    card_frame.corner_radius = egui::CornerRadius::same(8);
    card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    let title_color = egui::Color32::from_rgb(255, 160, 50);
    let text_color = egui::Color32::from_rgb(200, 210, 225);

    // --- SECCIÓN: ¿QUÉ ES RUST? (INTRODUCCIÓN, HISTORIA Y ORIGEN) ---
    ui.heading(
        egui::RichText::new("¿Qué es Rust?")
            .size(26.0)
            .strong()
            .color(title_color),
    );
    ui.add_space(8.0);

    ui.label(
            egui::RichText::new("Rust es un lenguaje de programación de sistemas moderno creado en 2006 por Graydon Hoare con un objetivo claro: eliminar los frecuentes fallos de seguridad y gestión manual de memoria diseñando un compilador estricto. Su diseño permite construir software con rendimiento extremo, seguridad absoluta de memoria y concurrencia segura sin Data Races.")
                .size(15.0)
                .color(text_color)
                .line_height(Some(22.0)),
        );
    ui.add_space(8.0);

    ui.label(
            egui::RichText::new("El nombre 'Rust' proviene de los hongos de la roya (Rust Fungi), organismos biológicos altamente resistentes capaces de sobrevivir durante años en condiciones extremas. Graydon se inspiró en esa resistencia natural para bautizar un lenguaje enfocado en la robustez y supervivencia del software.")
                .size(15.0)
                .color(text_color)
                .line_height(Some(22.0)),
        );
    ui.add_space(8.0);

    // --- SUBSECCIÓN: COMPILACIÓN NATIVA ---
    ui.heading(
        egui::RichText::new("Compilación Nativa")
            .size(20.0)
            .strong()
            .color(title_color),
    );
    ui.add_space(6.0);

    ui.label(
            egui::RichText::new("Rust transforma el código fuente en un binario nativo que el sistema operativo y la CPU pueden ejecutar. En el proceso participan Cargo, rustc, LLVM y el enlazador; el resultado final puede ser un ejecutable ELF en Linux, EXE en Windows o Mach-O en macOS.")
                .size(15.0)
                .color(text_color)
                .line_height(Some(22.0)),
        );
    ui.add_space(12.0);

    // Estilo de tarjeta súper compacta (sin inflar espacio)
    let mut mini_card = egui::Frame::new();
    mini_card.fill = egui::Color32::from_rgb(14, 18, 26);
    mini_card.inner_margin = egui::Margin::same(8);
    mini_card.corner_radius = egui::CornerRadius::same(6);
    mini_card.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    // Grid simétrico de 2 columnas con 4 mini-tarjetas 100% proporcionales en ancho y alto
    ui.columns(2, |cols| {
            // Columna Izquierda: Speed & Concurrency
            cols[0].vertical(|ui| {
                mini_card.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(65.0);
                    ui.label(egui::RichText::new("Speed").strong().color(title_color));
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new("Ejecución a velocidad nativa sin pausas por Garbage Collector.").color(text_color));
                });
                ui.add_space(8.0);

                mini_card.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(65.0);
                    ui.label(egui::RichText::new("Concurrency").strong().color(title_color));
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new("Ejecución de hilos simultáneos y seguros sin Data Races.").color(text_color));
                });
            });

            // Columna Derecha: Safety & Portability
            cols[1].vertical(|ui| {
                mini_card.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(65.0);
                    ui.label(egui::RichText::new("Safety").strong().color(title_color));
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new("Garantía estática contra punteros nulos y fallos de memoria en compilación.").color(text_color));
                });
                ui.add_space(8.0);

                mini_card.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(65.0);
                    ui.label(egui::RichText::new("Portability").strong().color(title_color));
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new("Soporte para arquitecturas de Servidor, Escritorio, Móvil, Microcontroladores y WebAssembly.").color(text_color));
                });
            });
        });
    ui.add_space(20.0);

    // --- FILA 1: NÚCLEO DE CONSTRUCCIÓN Y ECOSISTEMA ---
    ui.heading(
        egui::RichText::new("Núcleo de Construcción y Ecosistema")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    ui.columns(3, |columns| {
            // Pilar 1: rustc
            anim_card(&mut columns[0], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://rustc.svg",
                            include_bytes!("../../../diagramas/rustc.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustc").size(17.0).strong().color(title_color));
                    ui.add_space(4.0);
                    if ui.button(egui::RichText::new("🔍 Ver").small().color(egui::Color32::from_rgb(100, 200, 255)))
                        .on_hover_text("Abrir diagrama del pipeline de compilación de rustc")
                        .clicked()
                    {
                        state.ui.show_rustc_compilador_modal = true;
                    }
                });
                ui.label(egui::RichText::new("El Compilador Real").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Traduce tu código Rust (.rs) a código máquina optimizado (ELF/EXE/WASM) usando LLVM.").color(text_color));
                ui.label(egui::RichText::new("• Realiza las verificaciones de seguridad de memoria y el Borrow Checker.").color(text_color));
            });

            // Pilar 2: Cargo
            anim_card(&mut columns[1], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://cargo2.svg",
                            include_bytes!("../../../diagramas/cargo2.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Cargo").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Orquestador / Manager").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Gestor de proyectos y administrador de paquetes oficial de Rust.").color(text_color));
                ui.label(egui::RichText::new("• Automatiza la descarga de dependencias, compilación y pruebas.").color(text_color));
            });

            // Pilar 3: Crates / crates.io
            anim_card(&mut columns[2], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://crates.svg",
                            include_bytes!("../../../diagramas/crates.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Crates").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Las Librerías y crates.io").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Una 'Crate' es la unidad de código ejecutable o biblioteca en Rust.").color(text_color));
                ui.label(egui::RichText::new("• crates.io es el registro público mundial donde la comunidad comparte paquetes.").color(text_color));
            });
        });

    ui.add_space(18.0);

    // --- FILA 2: CALIDAD, ESTILO Y DIAGNÓSTICO ---
    ui.heading(
        egui::RichText::new("Calidad, Estilo y Diagnósticos")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    ui.columns(3, |cols| {
            // 1. Clippy
            anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://clippy.svg",
                            include_bytes!("../../../diagramas/clippy.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Clippy").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Linter Oficial").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Analiza tu código con más de 650 reglas avanzadas para detectar anti-patrones.").color(text_color));
                ui.label(egui::RichText::new("• Enseña las mejores prácticas del código idiomático en Rust.").color(text_color));
            });

            // 2. Rustfmt
            anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://format2.svg",
                            include_bytes!("../../../diagramas/format2.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Rustfmt").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Formateador Estándar").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Aplica automáticamente el libro de estilo unificado a todo el proyecto.").color(text_color));
                ui.label(egui::RichText::new("• Elimina discusiones de sangría y espacios en equipos de trabajo.").color(text_color));
            });

            // 3. Error Index
            anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://error.svg",
                            include_bytes!("../../../diagramas/error.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Error Index").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Enciclopedia de Errores").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Enciclopedia explicativa completa para cada código de error del compilador.").color(text_color));
                ui.label(egui::RichText::new("• Muestra ejemplos de código correcto e incorrecto para aprender del error.").color(text_color));
            });
        });

    ui.add_space(18.0);

    // --- FILA 3: PRODUCTIVIDAD, IDE Y DOCUMENTACIÓN ---
    ui.heading(
        egui::RichText::new("Productividad, IDE y Documentación")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);

    ui.columns(3, |cols| {
            // 1. rustup
            anim_card(&mut cols[0], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://rustup.svg",
                            include_bytes!("../../../diagramas/rustup.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustup").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Administrador de Toolchains").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Administra las versiones de Rust (Stable, Nightly) y la compilación cruzada.").color(text_color));
                ui.label(egui::RichText::new("• Permite añadir objetivos como WebAssembly (wasm32) fácilmente.").color(text_color));
            });

            // 2. rust-analyzer
            anim_card(&mut cols[1], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://analyzer.svg",
                            include_bytes!("../../../diagramas/analyzer.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rust-analyzer").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Servidor de Lenguaje (LSP)").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Proporciona autocompletado en vivo e inlays de tipos inferidos en tu IDE.").color(text_color));
                ui.label(egui::RichText::new("• Soporta VS Code, Antigravity y Neovim.").color(text_color));
            });

            // 3. rustdoc
            anim_card(&mut cols[2], &card_frame, &mut anim_delay, &mut |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://doc.svg",
                            include_bytes!("../../../diagramas/doc.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustdoc").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Generador de Documentación").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Lee los comentarios de documentación (///) y genera una web HTML completa.").color(text_color));
                ui.label(egui::RichText::new("• Ejecuta doctests automáticamente para garantizar que la documentación funcione.").color(text_color));
            });
        });
}
