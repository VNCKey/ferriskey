use crate::app::AppState;
use eframe::egui;

#[allow(dead_code)]
pub fn mostrar_pilares_conceptos(ui: &mut egui::Ui, state: &mut AppState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        mostrar_pilares_proyecto(ui, state);
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(20.0);
        mostrar_pilares_tiempo(ui, state);
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(20.0);
        mostrar_pilares_debug_vs_release(ui, state);
    });
}

#[allow(dead_code)]
pub fn mostrar_pilares_proyecto(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading(
        egui::RichText::new("Estructura de Proyectos en Rust")
            .size(20.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.label(
        "Al crear un proyecto con Cargo (ej: 'cargo new mi_proyecto'), Rust genera automáticamente la jerarquía de archivos y carpetas estándar.",
    );
    ui.add_space(12.0);

    // Tabla comparativa main.rs vs lib.rs
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(14);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_main_vs_lib_pilares")
            .striped(true)
            .spacing([25.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo de Proyecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Comando de Creación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Archivo Punto de Entrada")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Ejecutable (src/main.rs)
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Binario")
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(4.0);
                    let btn_color_main = if state.ui.show_railroad_modal == Some(2) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_main),
                            )
                            .frame(state.ui.show_railroad_modal == Some(2)),
                        )
                        .on_hover_text("Ver diagrama Railroad de sintaxis (fn main ejecutable)")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(2) {
                            None
                        } else {
                            Some(2)
                        };
                    }
                });
                ui.label(
                    egui::RichText::new("cargo new <nombre>")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("src/main.rs")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Programas independientes con función main() ejecutables por la CPU.");
                ui.end_row();

                // Fila 2: Librería (src/lib.rs)
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Library")
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(4.0);
                    let btn_color_lib = if state.ui.show_railroad_modal == Some(3) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_lib),
                            )
                            .frame(state.ui.show_railroad_modal == Some(3)),
                        )
                        .on_hover_text("Ver diagrama Railroad de sintaxis (librería lib.rs)")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(3) {
                            None
                        } else {
                            Some(3)
                        };
                    }
                });
                ui.label(
                    egui::RichText::new("cargo new <nombre> --lib")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("src/lib.rs")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Reutilización de código y módulos para ser consumidos por otros crates.");
                ui.end_row();
            });
    });
}

#[allow(dead_code)]
pub fn mostrar_desglose_template_con_imagen(ui: &mut egui::Ui, state: &mut AppState) {
    ui.add_space(15.0);
    let proj_name = state
        .project
        .selected_project
        .as_deref()
        .unwrap_or("mi_proyecto");
    let es_lib = proj_name.contains("lib") || proj_name.contains("libreria");
    let src_file = if es_lib { "src/lib.rs" } else { "src/main.rs" };
    let src_desc = if es_lib {
        "Punto de entrada de la librería conteniendo funciones exportables."
    } else {
        "Punto de entrada ejecutable conteniendo la función fn main()."
    };

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        ui.heading(
            egui::RichText::new(format!("Desglose de Proyecto ({})", proj_name))
                .size(16.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
        ui.add_space(8.0);

        ui.columns(2, |cols| {
            cols[0].label("Estructura de archivos generada en disco:");
            cols[0].add_space(6.0);

            egui::Grid::new("grid_desglose_archivos")
                .striped(true)
                .spacing([15.0, 6.0])
                .show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("Archivo / Carpeta")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.label(
                        egui::RichText::new("Propósito en Cargo")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.end_row();

                    ui.label(
                        egui::RichText::new("Cargo.toml")
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(
                        "Manifiesto con metadatos de tu proyecto (nombre, versión, dependencias).",
                    );
                    ui.end_row();

                    ui.label(
                        egui::RichText::new("Cargo.lock")
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label("Registro de versiones fijadas de dependencias.");
                    ui.end_row();

                    ui.label(
                        egui::RichText::new(src_file)
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(src_desc);
                    ui.end_row();

                    ui.label(
                        egui::RichText::new("target/")
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label("Carpeta binaria donde rustc compila los ejecutables.");
                    ui.end_row();
                });

            cols[1].add(
                egui::Image::new(egui::include_image!("../../../assets/taller/7.png"))
                    .fit_to_exact_size(egui::vec2(340.0, 220.0))
                    .corner_radius(8),
            );
        });
    });
}

#[allow(dead_code)]
pub fn mostrar_pilares_tiempo(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading(
        egui::RichText::new("Fases de Vida del Código: Compile Time vs Run Time")
            .size(20.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.label("En Rust existe una clara división entre la fase previa de análisis en desarrollo y la ejecución final por la CPU:");
    ui.add_space(12.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(14);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_compilacion_vs_ejecucion_pilares")
            .striped(true)
            .spacing([25.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Aspecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Tiempo de Compilación")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    let btn_color_compile = if state.ui.show_railroad_modal == Some(4) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_compile),
                            )
                            .frame(state.ui.show_railroad_modal == Some(4)),
                        )
                        .on_hover_text("Ver diagrama de flujo (Tiempo de Compilación)")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(4) {
                            None
                        } else {
                            Some(4)
                        };
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Tiempo de Ejecución")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    let btn_color_run = if state.ui.show_railroad_modal == Some(5) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_run),
                            )
                            .frame(state.ui.show_railroad_modal == Some(5)),
                        )
                        .on_hover_text("Ver diagrama de flujo (Tiempo de Ejecución)")
                        .clicked()
                    {
                        state.ui.show_railroad_modal = if state.ui.show_railroad_modal == Some(5) {
                            None
                        } else {
                            Some(5)
                        };
                    }
                });
                ui.end_row();

                // Fila 1: Quién lo ejecuta
                ui.label(
                    egui::RichText::new("Ejecutado Por")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("rustc / LLVM / Cargo");
                ui.label("Procesador / CPU del sistema");
                ui.end_row();

                // Fila 2: Tareas Principales
                ui.label(
                    egui::RichText::new("Operaciones")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Verificación de tipos, Borrow Checker, inferencia y optimización.");
                ui.label("Instrucciones máquina en CPU, asignación en Stack/Heap, E/S de red.");
                ui.end_row();

                // Fila 3: Costo de Errores
                ui.label(
                    egui::RichText::new("Costo de Errores")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Cero impacto en producción (Error de Compilación).");
                ui.label("Posible caida o Panics si no se maneja Result/Option.");
                ui.end_row();
            });
    });
}

pub fn mostrar_pilares_debug_vs_release(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.heading(
        egui::RichText::new("Perfiles de Compilación: Dev vs Release")
            .size(20.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.label(
        "Cargo ofrece dos modos de compilación principales optimizados para distintas etapas del ciclo de desarrollo:",
    );
    ui.add_space(12.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(14);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_debug_vs_release_pilares")
            .striped(true)
            .spacing([25.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Característica")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Perfil Dev (Desarrollo)")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Perfil Release (Producción)")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("Comando Cargo")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("cargo build / run")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("cargo build --release")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("Directorio Salida")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(egui::RichText::new("target/debug/").monospace());
                ui.label(egui::RichText::new("target/release/").monospace());
                ui.end_row();

                ui.label(
                    egui::RichText::new("Velocidad Compilación")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Rápida (iteración rápida en desarrollo)");
                ui.label("Más lenta (análisis y optimización profunda)");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Nivel Optimización LLVM")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("opt-level = 0 (sin optimizaciones)");
                ui.label("opt-level = 3 (máxima velocidad de ejecución)");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Símbolos Depuración")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Incluidos (gdb / lldb / stack tracebacks)");
                ui.label("Omitidos por defecto (binario compacto)");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Chequeo Overflow Enteros")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Activo (panics en desbordamientos)");
                ui.label("Normalmente inactivo (la operación puede envolver el valor)");
                ui.end_row();
            });
    });
}
