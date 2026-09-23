use crate::app::AppState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::conceptos::mostrar_selector_proyectos_estandar_con_archivos;
use crate::views::control_flujo::card_frame_tutorial;
use crate::views::pilares::anatomy::{codigo_inline_chip, codigo_resaltado_bloque, punto_lista};
use eframe::egui;
use std::sync::Arc;

fn mostrar_tabla_metodos(
    ui: &mut egui::Ui,
    id: &str,
    titulo: &str,
    introduccion: &str,
    filas: &[(&str, &str, &str, &str)],
) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new(titulo)
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(introduccion)
            .size(13.5)
            .color(text)
            .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new(id)
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Método / operación", "Qué hace", "Ejemplo", "Resultado"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                for (metodo, descripcion, ejemplo, resultado) in filas {
                    codigo_chip_color(ui, metodo, naranja);
                    texto_con_chips_inline(ui, descripcion, text, cyan);
                    codigo_chip_color(ui, ejemplo, cyan);
                    codigo_chip_color(ui, resultado, egui::Color32::from_rgb(140, 220, 180));
                    ui.end_row();
                }
            });
    });
}

fn mostrar_tabla_constantes(
    ui: &mut egui::Ui,
    id: &str,
    titulo: &str,
    introduccion: &str,
    filas: &[(&str, &str, &str, &str)],
) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new(titulo)
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(introduccion)
            .size(13.5)
            .color(text)
            .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new(id)
            .striped(true)
            .min_col_width(100.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Constante", "Qué representa", "Ejemplo", "Valor"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                for (constante, descripcion, ejemplo, valor) in filas {
                    codigo_chip_color(ui, constante, naranja);
                    texto_con_chips_inline(ui, descripcion, text, cyan);
                    codigo_chip_color(ui, ejemplo, cyan);
                    codigo_chip_color(ui, valor, egui::Color32::from_rgb(140, 220, 180));
                    ui.end_row();
                }
            });
    });
}

fn codigo_chip_color(ui: &mut egui::Ui, code: &str, color: egui::Color32) {
    ui.horizontal(|ui| {
        let mut chip = egui::Frame::new();
        chip.fill = egui::Color32::from_rgb(26, 32, 44);
        chip.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 90));
        chip.corner_radius = egui::CornerRadius::same(3);
        chip.inner_margin = egui::Margin::symmetric(6, 2);

        chip.show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(code)
                        .monospace()
                        .size(12.0)
                        .color(color),
                )
                .wrap_mode(egui::TextWrapMode::Extend),
            );
        });
    });
}

fn texto_con_chips_inline(
    ui: &mut egui::Ui,
    texto: &str,
    text_color: egui::Color32,
    chip_color: egui::Color32,
) {
    ui.horizontal(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.set_max_width(340.0);
            ui.spacing_mut().item_spacing.x = 4.0;
            ui.spacing_mut().item_spacing.y = 4.0;

            let mut in_code = false;
            for part in texto.split('`') {
                if part.is_empty() {
                    in_code = !in_code;
                    continue;
                }
                if in_code {
                    let mut chip = egui::Frame::new();
                    chip.fill = egui::Color32::from_rgb(26, 32, 44);
                    chip.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 90));
                    chip.corner_radius = egui::CornerRadius::same(3);
                    chip.inner_margin = egui::Margin::symmetric(6, 2);

                    chip.show(ui, |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(part)
                                    .monospace()
                                    .size(12.0)
                                    .color(chip_color),
                            )
                            .wrap_mode(egui::TextWrapMode::Extend),
                        );
                    });
                } else {
                    for word in part.split_whitespace() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(word)
                                    .size(12.0)
                                    .color(text_color),
                            )
                            .wrap_mode(egui::TextWrapMode::Extend),
                        );
                    }
                }
                in_code = !in_code;
            }
        });
    });
}

pub fn mostrar_categoria_enteros_interactiva(ui: &mut egui::Ui, state: &mut AppState) {
    mostrar_enteros_interactivo(ui, state, true);
}

pub(crate) fn mostrar_enteros_interactivo(
    ui: &mut egui::Ui,
    _state: &mut AppState,
    mostrar_encabezado: bool,
) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    if mostrar_encabezado {
        ui.heading(
            egui::RichText::new("Enteros")
                .size(22.0)
                .strong()
                .color(naranja),
        );
        ui.add_space(8.0);
    }
    ui.label(
        egui::RichText::new(
            "Un entero es un tipo de dato primitivo que representa números exactos sin fracciones ni parte decimal.",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    ui.horizontal_wrapped(|ui| {
        punto_lista(ui, naranja);
        codigo_chip_color(ui, "i", naranja);
        ui.label(
            egui::RichText::new(":")
                .strong()
                .size(13.5)
                .color(naranja),
        );
        ui.label(
            egui::RichText::new("Representa un entero con signo que admite valores negativos, cero y positivos.")
                .size(13.5)
                .color(text),
        );
    });
    ui.add_space(3.0);

    ui.horizontal_wrapped(|ui| {
        punto_lista(ui, cyan);
        codigo_chip_color(ui, "u", cyan);
        ui.label(
            egui::RichText::new(":")
                .strong()
                .size(13.5)
                .color(cyan),
        );
        ui.label(
            egui::RichText::new("Representa un entero sin signo que solo admite el cero y valores positivos.")
                .size(13.5)
                .color(text),
        );
    });
    ui.add_space(14.0);

    // 2 Tablas lado a lado: Enteros i a la izquierda, Enteros u a la derecha
    ui.columns(2, |columns| {
        // --- COLUMNA 1 (IZQUIERDA): ENTEROS i ---
        columns[0].vertical(|ui| {
            let mut frame_i = egui::Frame::new();
            frame_i.fill = egui::Color32::from_rgb(14, 18, 26);
            frame_i.inner_margin = egui::Margin::same(12);
            frame_i.corner_radius = egui::CornerRadius::same(8);
            frame_i.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            frame_i.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Enteros")
                            .size(15.0)
                            .strong()
                            .color(naranja),
                    );
                    codigo_chip_color(ui, "i", naranja);
                });
                ui.add_space(8.0);

                egui::Grid::new("grid_enteros_con_signo")
                    .striped(true)
                    .spacing([14.0, 9.0])
                    .show(ui, |ui| {
                        for encabezado in ["Tipo", "Bits", "Rango Mínimo .. Máximo"] {
                        ui.label(
                            egui::RichText::new(encabezado)
                                .strong()
                                .size(12.0)
                                .color(egui::Color32::WHITE),
                        );
                    }
                    ui.end_row();

                    let datos_con_signo = [
                        ("i8", "8", "-128 .. 127"),
                        ("i16", "16", "-32,768 .. 32,767"),
                        (
                            "i32",
                            "32",
                            "-2,147,483,648 .. 2,147,483,647 (default)",
                        ),
                        (
                            "i64",
                            "64",
                            "-9.22×10¹⁸ .. 9.22×10¹⁸",
                        ),
                        (
                            "i128",
                            "128",
                            "-1.70×10³⁸ .. 1.70×10³⁸",
                        ),
                        (
                            "isize",
                            "32/64",
                            "Depende de la CPU (punteros)",
                        ),
                    ];

                    for (tipo, bits, rango) in datos_con_signo {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(tipo)
                                    .monospace()
                                    .strong()
                                    .size(12.5)
                                    .color(naranja),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(bits)
                                    .monospace()
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(180, 190, 205)),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(rango)
                                    .monospace()
                                    .size(11.5)
                                    .color(egui::Color32::from_rgb(180, 190, 205)),
                            );
                        });
                        ui.end_row();
                    }
                });
            });
        });

        // --- COLUMNA 2 (DERECHA): ENTEROS u ---
        columns[1].vertical(|ui| {
            let mut frame_u = egui::Frame::new();
            frame_u.fill = egui::Color32::from_rgb(14, 18, 26);
            frame_u.inner_margin = egui::Margin::same(12);
            frame_u.corner_radius = egui::CornerRadius::same(8);
            frame_u.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            frame_u.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Enteros")
                            .size(15.0)
                            .strong()
                            .color(cyan),
                    );
                    codigo_chip_color(ui, "u", cyan);
                });
                ui.add_space(8.0);

                egui::Grid::new("grid_enteros_sin_signo")
                    .striped(true)
                    .spacing([14.0, 9.0])
                    .show(ui, |ui| {
                        for encabezado in ["Tipo", "Bits", "Rango Mínimo .. Máximo"] {
                            ui.label(
                                egui::RichText::new(encabezado)
                                    .strong()
                                    .size(12.0)
                                    .color(egui::Color32::WHITE),
                            );
                        }
                        ui.end_row();

                        let datos_sin_signo = [
                            ("u8", "8", "0 .. 255 (bytes, ASCII)"),
                            ("u16", "16", "0 .. 65,535"),
                            (
                                "u32",
                                "32",
                                "0 .. 4,294,967,295",
                            ),
                            ("u64", "64", "0 .. 1.84×10¹⁹"),
                            (
                                "u128",
                                "128",
                                "0 .. 3.40×10³⁸",
                            ),
                            (
                                "usize",
                                "32/64",
                                "0 .. Max memoria CPU (índices)",
                            ),
                        ];

                        for (tipo, bits, rango) in datos_sin_signo {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(tipo)
                                        .monospace()
                                        .strong()
                                        .size(12.5)
                                        .color(cyan),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(bits)
                                        .monospace()
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(180, 190, 205)),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(rango)
                                        .monospace()
                                        .size(11.5)
                                        .color(egui::Color32::from_rgb(180, 190, 205)),
                                );
                            });
                            ui.end_row();
                        }
                    });
            });
        });
    });

    ui.add_space(10.0);

    let mut note_frame = egui::Frame::new();
    note_frame.fill = egui::Color32::from_rgb(18, 24, 36);
    note_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95));
    note_frame.corner_radius = egui::CornerRadius::same(6);
    note_frame.inner_margin = egui::Margin::same(12);

    note_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new("Nota")
                .strong()
                .size(13.5)
                .color(naranja),
        );
        ui.add_space(5.0);

        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new("En Rust, cuando escribes un número entero literal sin especificar su tipo como en")
                    .size(13.0)
                    .color(text),
            );
            codigo_inline_chip(ui, "let x = 5;");
            ui.label(
                egui::RichText::new(", el compilador lo infiere automáticamente como")
                    .size(13.0)
                    .color(text),
            );
            codigo_inline_chip(ui, "i32");
            ui.label(
                egui::RichText::new("por defecto (equivalente a")
                    .size(13.0)
                    .color(text),
            );
            codigo_inline_chip(ui, "let x: i32 = 5;");
            ui.label(
                egui::RichText::new("), por ser el tipo más eficiente y equilibrado en la mayoría de procesadores modernos.")
                    .size(13.0)
                    .color(text),
            );
        });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Formas de Definir y Escribir Enteros")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Rust ofrece gran flexibilidad sintáctica para declarar enteros mediante anotaciones de tipo, sufijos literales, separadores visuales y diferentes bases numéricas:",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    let mut literal_frame = egui::Frame::new();
    literal_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    literal_frame.inner_margin = egui::Margin::same(12);
    literal_frame.corner_radius = egui::CornerRadius::same(8);
    literal_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    literal_frame.show(ui, |ui| {
        egui::Grid::new("grid_formas_definir_enteros")
            .striped(true)
            .min_col_width(110.0)
            .spacing([22.0, 11.0])
            .show(ui, |ui| {
                for encabezado in [
                    "Forma de Definición",
                    "Ejemplo de Código",
                    "Tipo / Valor",
                    "Descripción",
                ] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let formas_enteros: [(&str, Option<&str>, &str, &str, &str); 7] = [
                    (
                        "Tipo explícito",
                        None,
                        "let x: u32 = 1000;",
                        "u32 (1000)",
                        "Anotación de tipo tradicional después del nombre.",
                    ),
                    (
                        "Sufijo de tipo",
                        None,
                        "let x = 1000u32;",
                        "u32 (1000)",
                        "El tipo se añade directamente al final del número como `57u8` o `100i64`.",
                    ),
                    (
                        "Separador visual",
                        Some("_"),
                        "let x = 1_000_000;",
                        "i32 (1000000)",
                        "Los guiones bajos mejoran la legibilidad sin alterar el valor numérico.",
                    ),
                    (
                        "Hexadecimal",
                        Some("0x"),
                        "let x = 0xff;",
                        "i32 (255)",
                        "Base 16, ideal para representar direcciones, colores o máscaras de bytes.",
                    ),
                    (
                        "Octal",
                        Some("0o"),
                        "let x = 0o77;",
                        "i32 (63)",
                        "Base 8, común en configuración de permisos de archivos Unix.",
                    ),
                    (
                        "Binario",
                        Some("0b"),
                        "let x = 0b1111_0000;",
                        "i32 (240)",
                        "Base 2, esencial para operaciones a nivel de bits (bitwise flags).",
                    ),
                    (
                        "Literal de Byte",
                        Some("b'...'"),
                        "let x = b'A';",
                        "u8 (65)",
                        "Representa el valor numérico ASCII de un caracter como byte `u8`.",
                    ),
                ];

                for (forma, chip_tag, ej_codigo, valor, desc) in formas_enteros {
                    if let Some(tag) = chip_tag {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(forma)
                                    .strong()
                                    .size(12.5)
                                    .color(naranja),
                            );
                            codigo_chip_color(ui, tag, naranja);
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(forma)
                                    .strong()
                                    .size(12.5)
                                    .color(naranja),
                            );
                        });
                    }
                    codigo_chip_color(ui, ej_codigo, cyan);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(valor)
                                .monospace()
                                .size(12.0)
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                    });
                    texto_con_chips_inline(ui, desc, text, cyan);
                    ui.end_row();
                }
            });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Operaciones Aritméticas Básicas")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Rust proporciona los operadores aritméticos estándar para realizar cálculos entre números enteros:",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    let mut arith_frame = egui::Frame::new();
    arith_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    arith_frame.inner_margin = egui::Margin::same(12);
    arith_frame.corner_radius = egui::CornerRadius::same(8);
    arith_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    arith_frame.show(ui, |ui| {
        egui::Grid::new("grid_aritmetica_enteros")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Operación", "Operador", "Ejemplo de Código", "Resultado", "Descripción"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let operaciones = [
                    (
                        "Suma",
                        "+",
                        "let x = 10 + 5;",
                        "15",
                        "Suma aritmética de ambos operandos.",
                    ),
                    (
                        "Resta",
                        "-",
                        "let x = 20 - 7;",
                        "13",
                        "Resta o diferencia entre dos valores.",
                    ),
                    (
                        "Multiplicación",
                        "*",
                        "let x = 6 * 7;",
                        "42",
                        "Producto aritmético de los factores.",
                    ),
                    (
                        "División entera",
                        "/",
                        "let x = 14 / 4;",
                        "3",
                        "Trunca hacia cero; descarta cualquier parte decimal.",
                    ),
                    (
                        "Módulo / Resto",
                        "%",
                        "let x = 14 % 4;",
                        "2",
                        "Obtiene el resto o residuo de la división entera.",
                    ),
                    (
                        "Negación unaria",
                        "-",
                        "let x = -10i32;",
                        "-10",
                        "Invierte el signo (solo en tipos con signo `i8` a `i128`).",
                    ),
                ];

                for (op, simbolo, ej, res, desc) in operaciones {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(op)
                                .strong()
                                .size(12.5)
                                .color(naranja),
                        );
                    });
                    codigo_chip_color(ui, simbolo, naranja);
                    codigo_chip_color(ui, ej, cyan);
                    codigo_chip_color(ui, res, egui::Color32::from_rgb(140, 220, 180));
                    texto_con_chips_inline(ui, desc, text, cyan);
                    ui.end_row();
                }
            });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Métodos útiles de los enteros")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Además de almacenar números, los tipos enteros ofrecen métodos para realizar cálculos, comparaciones, operaciones a nivel de bits y conversiones de bytes:",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    let mut methods_frame = egui::Frame::new();
    methods_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    methods_frame.inner_margin = egui::Margin::same(12);
    methods_frame.corner_radius = egui::CornerRadius::same(8);
    methods_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    methods_frame.show(ui, |ui| {
        egui::Grid::new("grid_metodos_enteros")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Categoría", "Método", "Qué hace", "Ejemplo", "Resultado"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let metodos: [(&str, &str, &str, &str, &str); 15] = [
                    (
                        "Matemáticas",
                        "pow",
                        "Calcula una potencia con un exponente entero.",
                        "3u32.pow(4)",
                        "81",
                    ),
                    (
                        "Matemáticas",
                        "abs",
                        "Obtiene el valor absoluto de un entero con signo.",
                        "(-8i32).abs()",
                        "8",
                    ),
                    (
                        "Matemáticas",
                        "signum",
                        "Devuelve `-1`, `0` o `1` según el signo del entero.",
                        "(-8i32).signum()",
                        "-1",
                    ),
                    (
                        "Matemáticas",
                        "min / max",
                        "Devuelve el menor o el mayor entre dos valores.",
                        "10u8.max(20)",
                        "20",
                    ),
                    (
                        "Matemáticas",
                        "clamp",
                        "Limita un valor dentro de un mínimo y un máximo.",
                        "50u8.clamp(0, 10)",
                        "10",
                    ),
                    (
                        "Matemáticas",
                        "div_euclid",
                        "Realiza una división euclidiana, útil con negativos.",
                        "(-7i32).div_euclid(3)",
                        "-3",
                    ),
                    (
                        "Matemáticas",
                        "rem_euclid",
                        "Obtiene el resto euclidiano no negativo.",
                        "(-7i32).rem_euclid(3)",
                        "2",
                    ),
                    (
                        "Matemáticas",
                        "is_power_of_two",
                        "Indica si el entero es una potencia de dos.",
                        "16u32.is_power_of_two()",
                        "true",
                    ),
                    (
                        "Matemáticas",
                        "next_power_of_two",
                        "Obtiene la siguiente potencia de dos igual o mayor.",
                        "10u32.next_power_of_two()",
                        "16",
                    ),
                    (
                        "Matemáticas",
                        "ilog2 / ilog10",
                        "Calcula el logaritmo entero en base 2 o 10.",
                        "100u32.ilog10()",
                        "2",
                    ),
                    (
                        "Comparaciones",
                        "is_positive",
                        "Indica si el entero es estrictamente positivo.",
                        "8i32.is_positive()",
                        "true",
                    ),
                    (
                        "Comparaciones",
                        "is_negative",
                        "Indica si el entero es estrictamente negativo.",
                        "(-8i32).is_negative()",
                        "true",
                    ),
                    (
                        "Bits",
                        "count_ones",
                        "Cuenta cuántos bits están en 1.",
                        "0b1011u8.count_ones()",
                        "3",
                    ),
                    (
                        "Bits",
                        "leading_zeros",
                        "Cuenta los ceros al comienzo de la representación binaria.",
                        "8u8.leading_zeros()",
                        "5",
                    ),
                    (
                        "Bytes",
                        "to_le_bytes",
                        "Convierte el entero en bytes little-endian.",
                        "0x1234u16.to_le_bytes()",
                        "[0x34, 0x12]",
                    ),
                ];

                for (categoria, metodo, descripcion, ejemplo, resultado) in metodos {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(categoria)
                                .size(12.0)
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                    });
                    codigo_chip_color(ui, metodo, naranja);
                    texto_con_chips_inline(ui, descripcion, text, cyan);
                    codigo_chip_color(ui, ejemplo, cyan);
                    codigo_chip_color(ui, resultado, egui::Color32::from_rgb(140, 220, 180));
                    ui.end_row();
                }
            });
    });

    mostrar_tabla_constantes(
        ui,
        "grid_constantes_enteros",
        "Constantes asociadas a los enteros",
        "Cada tipo entero conoce sus propios límites y características:",
        &[
            (
                "MIN",
                "El valor mínimo que puede representar el tipo.",
                "i8::MIN",
                "-128",
            ),
            (
                "MAX",
                "El valor máximo que puede representar el tipo.",
                "u8::MAX",
                "255",
            ),
            (
                "BITS",
                "Cantidad de bits utilizados por el tipo.",
                "u32::BITS",
                "32",
            ),
        ],
    );
}

pub fn mostrar_categoria_enteros(ui: &mut egui::Ui) {
    let mut state = AppState::default();
    mostrar_enteros_interactivo(ui, &mut state, true);
}

pub fn mostrar_categoria_flotantes(ui: &mut egui::Ui) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    ui.heading(
        egui::RichText::new("Decimales")
            .size(22.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(
            "Los tipos de coma flotante representan números reales con parte decimal bajo el estándar IEEE-754.",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    ui.horizontal_wrapped(|ui| {
        punto_lista(ui, naranja);
        codigo_chip_color(ui, "f32", naranja);
        ui.label(
            egui::RichText::new(":")
                .strong()
                .size(13.5)
                .color(naranja),
        );
        ui.label(
            egui::RichText::new("Ideal para gráficos 3D, simulaciones físicas y ahorro de memoria.")
                .size(13.5)
                .color(text),
        );
    });
    ui.add_space(3.0);

    ui.horizontal_wrapped(|ui| {
        punto_lista(ui, cyan);
        codigo_chip_color(ui, "f64", cyan);
        ui.label(
            egui::RichText::new(":")
                .strong()
                .size(13.5)
                .color(cyan),
        );
        ui.label(
            egui::RichText::new("Tipo por defecto en Rust para decimales. Alta precisión matemática.")
                .size(13.5)
                .color(text),
        );
    });
    ui.add_space(14.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_flotantes")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Tipo", "Precisión", "Tamaño", "Ejemplo de Código"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let datos_flotantes = [
                    (
                        "f32",
                        "Simple (~6-9 dígitos)",
                        "32 bits (4 bytes)",
                        "let pi: f32 = 3.14159;",
                    ),
                    (
                        "f64",
                        "Doble (~15-17 dígitos)",
                        "64 bits (8 bytes)",
                        "let pi: f64 = 3.141592653589793;",
                    ),
                ];

                for (tipo, precision, tamano, ej) in datos_flotantes {
                    let col = if tipo == "f32" { naranja } else { cyan };
                    codigo_chip_color(ui, tipo, col);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(precision).size(12.0).color(egui::Color32::from_rgb(180, 190, 205)));
                    });
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(tamano).monospace().size(12.0).color(egui::Color32::from_rgb(180, 190, 205)));
                    });
                    codigo_chip_color(ui, ej, cyan);
                    ui.end_row();
                }
            });
    });

    ui.add_space(10.0);

    let mut note_frame = egui::Frame::new();
    note_frame.fill = egui::Color32::from_rgb(18, 24, 36);
    note_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95));
    note_frame.corner_radius = egui::CornerRadius::same(6);
    note_frame.inner_margin = egui::Margin::same(12);

    note_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new("Nota")
                .strong()
                .size(13.5)
                .color(naranja),
        );
        ui.add_space(5.0);

        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new("En Rust, cuando escribes un número decimal literal sin especificar su tipo como en")
                    .size(13.0)
                    .color(text),
            );
            codigo_inline_chip(ui, "let x = 3.14;");
            ui.label(
                egui::RichText::new(", el compilador lo infiere automáticamente como")
                    .size(13.0)
                    .color(text),
            );
            codigo_inline_chip(ui, "f64");
            ui.label(
                egui::RichText::new("por defecto (equivalente a")
                    .size(13.0)
                    .color(text),
            );
            codigo_inline_chip(ui, "let x: f64 = 3.14;");
            ui.label(
                egui::RichText::new("), ya que en CPUs modernas ofrece mayor precisión con un rendimiento prácticamente idéntico a f32.")
                    .size(13.0)
                    .color(text),
            );
        });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Formas de Definir y Escribir Decimales")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Rust permite definir flotantes mediante anotaciones de tipo, sufijos literales, separadores visuales y notación científica:",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(8.0);

    let mut literal_frame = egui::Frame::new();
    literal_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    literal_frame.inner_margin = egui::Margin::same(12);
    literal_frame.corner_radius = egui::CornerRadius::same(8);
    literal_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    literal_frame.show(ui, |ui| {
        egui::Grid::new("grid_formas_definir_flotantes")
            .striped(true)
            .min_col_width(110.0)
            .spacing([22.0, 11.0])
            .show(ui, |ui| {
                for encabezado in [
                    "Forma de Definición",
                    "Ejemplo de Código",
                    "Tipo / Valor",
                    "Descripción",
                ] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let formas_flotantes: [(&str, Option<&str>, &str, &str, &str); 5] = [
                    (
                        "Tipo explícito",
                        None,
                        "let x: f64 = 3.1415;",
                        "f64 (3.1415)",
                        "Anotación de tipo tradicional después del nombre.",
                    ),
                    (
                        "Sufijo de tipo",
                        None,
                        "let x = 3.14f32;",
                        "f32 (3.14)",
                        "El tipo se añade directamente al final del número como `f32` o `f64`.",
                    ),
                    (
                        "Separador visual",
                        Some("_"),
                        "let x = 1_000.500_25;",
                        "f64 (1000.50025)",
                        "Los guiones bajos mejoran la legibilidad sin alterar el valor numérico.",
                    ),
                    (
                        "Notación científica",
                        Some("e / E"),
                        "let x = 2.5e3;",
                        "f64 (2500.0)",
                        "Exponente en base 10 (`2.5 * 10³`). También admite exponentes negativos como `1e-4`.",
                    ),
                    (
                        "Punto flotante implícito",
                        Some("."),
                        "let x = 5.0;",
                        "f64 (5.0)",
                        "El punto decimal indica al compilador que se trata de un número flotante.",
                    ),
                ];

                for (forma, chip_tag, ej_codigo, valor, desc) in formas_flotantes {
                    if let Some(tag) = chip_tag {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(forma)
                                    .strong()
                                    .size(12.5)
                                    .color(naranja),
                            );
                            codigo_chip_color(ui, tag, naranja);
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(forma)
                                    .strong()
                                    .size(12.5)
                                    .color(naranja),
                            );
                        });
                    }
                    codigo_chip_color(ui, ej_codigo, cyan);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(valor)
                                .monospace()
                                .size(12.0)
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                    });
                    texto_con_chips_inline(ui, desc, text, cyan);
                    ui.end_row();
                }
            });
    });

    ui.add_space(14.0);

    let mut note_frame = egui::Frame::new();
    note_frame.fill = egui::Color32::from_rgb(18, 24, 36);
    note_frame.inner_margin = egui::Margin::same(12);
    note_frame.corner_radius = egui::CornerRadius::same(8);
    note_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 100));

    note_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new("Operaciones y División Entera vs Flotante")
                .strong()
                .size(14.0)
                .color(naranja),
        );
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, naranja);
            ui.label(egui::RichText::new("División entera (truncada):").size(13.0).color(text));
            codigo_chip_color(ui, "5 / 2", cyan);
            ui.label(egui::RichText::new("evalúa a").size(13.0).color(text));
            codigo_chip_color(ui, "2", egui::Color32::from_rgb(140, 220, 180));
        });
        ui.add_space(3.0);
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, cyan);
            ui.label(egui::RichText::new("División flotante exacta:").size(13.0).color(text));
            codigo_chip_color(ui, "5.0 / 2.0", cyan);
            ui.label(egui::RichText::new("evalúa a").size(13.0).color(text));
            codigo_chip_color(ui, "2.5", egui::Color32::from_rgb(140, 220, 180));
        });
        ui.add_space(3.0);
        ui.horizontal_wrapped(|ui| {
            punto_lista(ui, egui::Color32::from_rgb(180, 190, 205));
            ui.label(egui::RichText::new("No se pueden mezclar tipos directamente; requiere casting:").size(13.0).color(text));
            codigo_chip_color(ui, "(5 as f64) / 2.0", cyan);
        });
    });

    mostrar_tabla_metodos(
        ui,
        "grid_metodos_flotantes",
        "Métodos útiles de los flotantes",
        "Estos métodos ayudan a redondear, calcular y comprobar propiedades de f32 y f64:",
        &[
            (
                "abs",
                "Obtiene el valor absoluto.",
                "(-3.5f64).abs()",
                "3.5",
            ),
            ("floor", "Redondea hacia abajo al entero inferior.", "3.8f64.floor()", "3.0"),
            ("ceil", "Redondea hacia arriba al entero superior.", "3.2f64.ceil()", "4.0"),
            (
                "round",
                "Redondea al entero más cercano.",
                "3.5f64.round()",
                "4.0",
            ),
            (
                "trunc",
                "Elimina la parte fraccionaria.",
                "3.8f64.trunc()",
                "3.0",
            ),
            (
                "fract",
                "Obtiene la parte fraccionaria.",
                "3.8f64.fract()",
                "0.8",
            ),
            ("sqrt", "Calcula la raíz cuadrada.", "16.0f64.sqrt()", "4.0"),
            (
                "powi",
                "Calcula potencia con exponente entero.",
                "2.0f64.powi(3)",
                "8.0",
            ),
            (
                "powf",
                "Calcula potencia con exponente flotante.",
                "2.0f64.powf(0.5)",
                "1.4142",
            ),
            (
                "min / max",
                "Devuelve el menor o el mayor valor.",
                "2.0f64.max(5.0)",
                "5.0",
            ),
            (
                "clamp",
                "Limita el valor entre mínimo y máximo.",
                "10.0f64.clamp(0.0, 5.0)",
                "5.0",
            ),
            (
                "is_nan",
                "Indica si el valor es NaN (no es un número).",
                "(0.0f64 / 0.0).is_nan()",
                "true",
            ),
            (
                "is_finite",
                "Indica si no es infinito ni NaN.",
                "3.0f64.is_finite()",
                "true",
            ),
            (
                "to_radians",
                "Convierte grados a radianes.",
                "180.0f64.to_radians()",
                "3.14159",
            ),
        ],
    );

    mostrar_tabla_constantes(
        ui,
        "grid_constantes_flotantes",
        "Constantes asociadas a los flotantes",
        "Los tipos f32 y f64 definen valores especiales para límites numéricos, precisión e infinitos:",
        &[
            (
                "MIN / MAX",
                "Límites de valores finitos representables.",
                "f64::MAX",
                "1.797e+308",
            ),
            (
                "MIN_POSITIVE",
                "Menor valor positivo normalizado.",
                "f32::MIN_POSITIVE",
                "1.175e-38",
            ),
            (
                "EPSILON",
                "Diferencia entre 1.0 y el siguiente representable.",
                "f64::EPSILON",
                "2.220e-16",
            ),
            (
                "INFINITY",
                "Valor de infinito positivo.",
                "f32::INFINITY",
                "inf",
            ),
            (
                "NAN",
                "Valor que representa un resultado no numérico.",
                "f64::NAN",
                "NaN",
            ),
        ],
    );
}

pub fn mostrar_categoria_booleanos(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    ui.heading(
        egui::RichText::new("Booleanos")
            .size(22.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "El tipo booleano representa una verdad lógica simple. En Rust solo existen dos valores posibles: true y false, ocupando 1 byte en memoria.",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_booleanos")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Tipo", "Valores Posibles", "Tamaño", "Ejemplo de Código", "Descripción"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                codigo_chip_color(ui, "bool", naranja);
                ui.horizontal(|ui| {
                    codigo_chip_color(ui, "true", egui::Color32::from_rgb(140, 220, 180));
                    ui.label(egui::RichText::new("|").color(egui::Color32::from_rgb(100, 120, 150)));
                    codigo_chip_color(ui, "false", egui::Color32::from_rgb(255, 120, 120));
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("1 byte (8 bits)").monospace().size(12.0).color(egui::Color32::from_rgb(180, 190, 205)));
                });
                codigo_chip_color(ui, "let es_activo: bool = true;", cyan);
                texto_con_chips_inline(ui, "Base del control de flujo en condiciones `if` y bucles `while`.", text, cyan);
                ui.end_row();
            });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Operadores de Comparación")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Casi todos los valores booleanos en tus programas nacerán al evaluar relaciones entre datos:")
            .size(13.5)
            .color(text),
    );
    ui.add_space(8.0);

    let mut frame_comp = egui::Frame::new();
    frame_comp.fill = egui::Color32::from_rgb(14, 18, 26);
    frame_comp.inner_margin = egui::Margin::same(12);
    frame_comp.corner_radius = egui::CornerRadius::same(8);
    frame_comp.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame_comp.show(ui, |ui| {
        egui::Grid::new("grid_operadores_comparacion")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Operador", "Nombre", "Ejemplo", "Resultado"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let comps = [
                    ("==", "Igual a", "1 == 2", "false"),
                    ("!=", "Distinto de", "1 != 2", "true"),
                    ("<", "Menor que", "1 < 2", "true"),
                    (">", "Mayor que", "1 > 2", "false"),
                    ("<=", "Menor o igual", "1 <= 2", "true"),
                    (">=", "Mayor o igual", "1 >= 2", "false"),
                ];

                for (simbolo, nombre, ej, res) in comps {
                    codigo_chip_color(ui, simbolo, naranja);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(nombre).size(12.0).color(egui::Color32::from_rgb(180, 190, 205)));
                    });
                    codigo_chip_color(ui, ej, cyan);
                    let res_color = if res == "true" {
                        egui::Color32::from_rgb(140, 220, 180)
                    } else {
                        egui::Color32::from_rgb(255, 120, 120)
                    };
                    codigo_chip_color(ui, res, res_color);
                    ui.end_row();
                }
            });
    });

    mostrar_tabla_metodos(
        ui,
        "grid_metodos_booleanos",
        "Operaciones lógicas de bool",
        "Operadores booleanos fundamentales para combinar expresiones lógicas:",
        &[
            (
                "!valor",
                "Invierte true a false y false a true (NOT).",
                "!true",
                "false",
            ),
            (
                "a && b",
                "Verdadero solo si ambos son true con cortocircuito (AND).",
                "true && false",
                "false",
            ),
            (
                "a || b",
                "Verdadero si al menos uno es true con cortocircuito (OR).",
                "true || false",
                "true",
            ),
            (
                "a ^ b",
                "Verdadero si los valores son diferentes (XOR).",
                "true ^ false",
                "true",
            ),
        ],
    );

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Simulador Lógico Interactivo")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Interactúa con las compuertas lógicas alternando las entradas A y B:")
            .size(13.5)
            .color(text),
    );
    ui.add_space(8.0);

    let mut frame_sim = egui::Frame::new();
    frame_sim.fill = egui::Color32::from_rgb(20, 25, 35);
    frame_sim.inner_margin = egui::Margin::same(12);
    frame_sim.corner_radius = egui::CornerRadius::same(8);
    frame_sim.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 80, 110));

    frame_sim.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Entrada A:")
                    .strong()
                    .color(egui::Color32::WHITE),
            );
            if ui
                .selectable_label(
                    state.dashboard.bool_sim_a,
                    egui::RichText::new(if state.dashboard.bool_sim_a {
                        "TRUE"
                    } else {
                        "FALSE"
                    })
                    .strong(),
                )
                .clicked()
            {
                state.dashboard.bool_sim_a = !state.dashboard.bool_sim_a;
            }

            ui.add_space(20.0);

            ui.label(
                egui::RichText::new("Entrada B:")
                    .strong()
                    .color(egui::Color32::WHITE),
            );
            if ui
                .selectable_label(
                    state.dashboard.bool_sim_b,
                    egui::RichText::new(if state.dashboard.bool_sim_b {
                        "TRUE"
                    } else {
                        "FALSE"
                    })
                    .strong(),
                )
                .clicked()
            {
                state.dashboard.bool_sim_b = !state.dashboard.bool_sim_b;
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        let a = state.dashboard.bool_sim_a;
        let b = state.dashboard.bool_sim_b;

        let resultados = [
            (
                "A && B",
                "AND",
                "Verdadero solo si AMBAS son verdaderas.",
                a && b,
            ),
            (
                "A || B",
                "OR",
                "Verdadero si AL MENOS UNA es verdadera.",
                a || b,
            ),
            (
                "A ^ B",
                "XOR",
                "Verdadero si son DIFERENTES entre sí.",
                a ^ b,
            ),
            ("!A", "NOT", "Invierte el valor de A.", !a),
        ];

        egui::Grid::new("grid_simulador_logico")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Operación", "Compuerta", "Descripción", "Resultado"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                for (op, compuerta, desc, res) in resultados {
                    codigo_chip_color(ui, op, cyan);
                    codigo_chip_color(ui, compuerta, naranja);
                    texto_con_chips_inline(ui, desc, text, cyan);

                    let (res_text, res_color) = if res {
                        ("TRUE", egui::Color32::from_rgb(140, 220, 180))
                    } else {
                        ("FALSE", egui::Color32::from_rgb(255, 120, 120))
                    };

                    codigo_chip_color(ui, res_text, res_color);
                    ui.end_row();
                }
            });
    });
}

pub fn mostrar_categoria_caracteres(ui: &mut egui::Ui, state: &AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    ui.heading(
        egui::RichText::new("Caracteres (char)")
            .size(22.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "En Rust, un char es un valor escalar Unicode de 4 bytes (32 bits), lo que significa que soporta ASCII, tildes, alfabetos globales y emojis nativamente.",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_caracteres")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Tipo", "Sintaxis", "Tamaño", "Ejemplo de Código", "Características"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                codigo_chip_color(ui, "char", naranja);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Comillas simples ''").size(12.0).color(egui::Color32::from_rgb(180, 190, 205)));
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("4 bytes (32 bits)").monospace().size(12.0).color(egui::Color32::from_rgb(180, 190, 205)));
                });
                codigo_chip_color(ui, "let letra: char = '🦀';", cyan);
                texto_con_chips_inline(ui, "Soporta Unicode completo `U+0000` a `U+D7FF` y `U+E000` a `U+10FFFF`.", text, cyan);
                ui.end_row();
            });
    });

    mostrar_tabla_metodos(
        ui,
        "grid_metodos_char",
        "Métodos útiles de char",
        "Los métodos de char ayudan a clasificar caracteres Unicode y transformar texto:",
        &[
            (
                "is_alphabetic",
                "Indica si el carácter es una letra.",
                "'ñ'.is_alphabetic()",
                "true",
            ),
            (
                "is_numeric",
                "Indica si representa un dígito numérico.",
                "'7'.is_numeric()",
                "true",
            ),
            (
                "is_alphanumeric",
                "Indica si es una letra o un número.",
                "'A'.is_alphanumeric()",
                "true",
            ),
            (
                "is_whitespace",
                "Indica si es un espacio, salto o tabulación.",
                "' '.is_whitespace()",
                "true",
            ),
            (
                "is_uppercase",
                "Comprueba si está en mayúscula.",
                "'A'.is_uppercase()",
                "true",
            ),
            (
                "is_lowercase",
                "Comprueba si está en minúscula.",
                "'a'.is_lowercase()",
                "true",
            ),
            (
                "is_ascii",
                "Indica si pertenece al conjunto ASCII (0..=127).",
                "'A'.is_ascii()",
                "true",
            ),
            (
                "to_ascii_uppercase",
                "Convierte un carácter ASCII a mayúscula.",
                "'a'.to_ascii_uppercase()",
                "'A'",
            ),
            (
                "to_ascii_lowercase",
                "Convierte un carácter ASCII a minúscula.",
                "'A'.to_ascii_lowercase()",
                "'a'",
            ),
            (
                "len_utf8",
                "Indica cuántos bytes UTF-8 ocupa en memoria.",
                "'🦀'.len_utf8()",
                "4",
            ),
        ],
    );

    mostrar_tabla_constantes(
        ui,
        "grid_constantes_char",
        "Constantes asociadas a char",
        "El tipo char incluye constantes para los límites de rangos Unicode y caracteres de sustitución:",
        &[
            ("MIN", "Primer valor escalar Unicode válido.", "char::MIN", "'\\0'"),
            ("MAX", "Último valor escalar Unicode válido.", "char::MAX", "'\\u{10FFFF}'"),
            (
                "REPLACEMENT_CHARACTER",
                "Carácter usado para sustituir secuencias inválidas.",
                "char::REPLACEMENT_CHARACTER",
                "'\\u{FFFD}'",
            ),
        ],
    );

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Representación Unicode en Memoria (Bajo el Capó)")
            .size(18.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "Como char es un valor escalar Unicode de 32 bits, podemos convertirlo explícitamente a un u32 o formato Hexadecimal para inspeccionar su valor real:",
        )
        .size(13.5)
        .color(text),
    );
    ui.add_space(8.0);

    let codigo_unicode = "fn main() {
    let letra: char = '🦀';
    let numero_crudo = letra as u32; // Casting a entero de 32 bits: 129408 (0x1F980)

    // Convertimos de u32 a char de nuevo (devuelve Option porque podría no ser un Unicode válido)
    let volver = char::from_u32(numero_crudo).unwrap();

    println!(\"Unicode de '🦀': {} [U+{:X}]\", numero_crudo, numero_crudo);
    println!(\"Recuperado: {}\", volver);
}";
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
    codigo_resaltado_bloque(ui, codigo_unicode, &state.editor.syntax_set, theme, "rs");
}

pub fn mostrar_categoria_casting(ui: &mut egui::Ui, state: &AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];

    ui.heading(
        egui::RichText::new("Conversiones de Tipo (Casting con as)")
            .size(22.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(
            "En Rust no existe la coerción implícita de tipos. Para transformar y operar entre diferentes tipos primitivos se requiere una conversión explícita usando la palabra clave as.",
        )
        .size(13.5)
        .color(text)
        .line_height(Some(20.0)),
    );
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_casting")
            .striped(true)
            .min_col_width(90.0)
            .spacing([20.0, 11.0])
            .show(ui, |ui| {
                for encabezado in ["Conversión", "Sintaxis con 'as'", "Comportamiento", "Ejemplo", "Resultado"] {
                    ui.label(
                        egui::RichText::new(encabezado)
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.end_row();

                let castings = [
                    (
                        "Entero a Decimal",
                        "i32 as f64",
                        "Conversión exacta sin pérdida de datos.",
                        "(10i32 as f64) + 0.5",
                        "10.5",
                    ),
                    (
                        "Decimal a Entero",
                        "f64 as i32",
                        "Trunca la parte decimal (redondeo hacia cero).",
                        "3.99f64 as i32",
                        "3",
                    ),
                    (
                        "Entero a Índice",
                        "u32 as usize",
                        "Permite indexar arrays y slices con seguridad.",
                        "array[pos as usize]",
                        "array[2]",
                    ),
                    (
                        "Caracter a Entero",
                        "char as u32",
                        "Obtiene el código de punto Unicode escalar.",
                        "'🦀' as u32",
                        "129408",
                    ),
                    (
                        "Byte a Caracter",
                        "u8 as char",
                        "Convierte un byte ASCII a su carácter correspondiente.",
                        "65u8 as char",
                        "'A'",
                    ),
                ];

                for (conversion, sintaxis, comp, ej, res) in castings {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(conversion).strong().size(12.5).color(naranja));
                    });
                    codigo_chip_color(ui, sintaxis, naranja);
                    texto_con_chips_inline(ui, comp, text, cyan);
                    codigo_chip_color(ui, ej, cyan);
                    codigo_chip_color(ui, res, egui::Color32::from_rgb(140, 220, 180));
                    ui.end_row();
                }
            });
    });

    ui.add_space(18.0);

    // Tarjetas comparativas de Coerción vs Casting
    ui.columns(2, |cols| {
        let mut card_err = egui::Frame::new();
        card_err.fill = egui::Color32::from_rgb(14, 18, 26);
        card_err.inner_margin = egui::Margin::same(12);
        card_err.corner_radius = egui::CornerRadius::same(8);
        card_err.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        card_err.show(&mut cols[0], |ui| {
            ui.label(
                egui::RichText::new("Coerción Implícita (Prohibida)")
                    .strong()
                    .size(15.0)
                    .color(naranja),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Rust previene bugs sutiles exigiendo que ambos operandos compartan exactamente el mismo tipo:")
                    .size(13.0)
                    .color(text),
            );
            ui.add_space(8.0);

            codigo_resaltado_bloque(
                ui,
                "let x: i32 = 10;\nlet y: f64 = 2.5;\n// let z = x + y; // ERROR: mismatched types",
                &state.editor.syntax_set,
                theme,
                "rs",
            );
        });

        let mut card_ok = egui::Frame::new();
        card_ok.fill = egui::Color32::from_rgb(14, 18, 26);
        card_ok.inner_margin = egui::Margin::same(12);
        card_ok.corner_radius = egui::CornerRadius::same(8);
        card_ok.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        card_ok.show(&mut cols[1], |ui| {
            ui.label(
                egui::RichText::new("Casting Explícito con 'as' (Válido)")
                    .strong()
                    .size(15.0)
                    .color(naranja),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Al indicar explícitamente la conversión, el desarrollador asume el control del tipo resultante:")
                    .size(13.0)
                    .color(text),
            );
            ui.add_space(8.0);

            codigo_resaltado_bloque(
                ui,
                "let x: i32 = 10;\nlet y: f64 = 2.5;\nlet z = (x as f64) + y; // Válido: 12.5",
                &state.editor.syntax_set,
                theme,
                "rs",
            );
        });
    });
}

#[allow(dead_code)]
pub fn mostrar_macro_println(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("🧩 ¿Por qué `println!` termina con `!`?");
    ui.label("El signo `!` indica que estás invocando una macro. Una macro recibe tokens y genera código durante la compilación.");
    ui.add_space(8.0);
    ui.columns(2, |columns| {
        columns[0].group(|ui| {
            ui.label(egui::RichText::new("Lo que escribes").strong());
            ui.code("println!(\"Hola, Ferris!\");");
            ui.label("La macro valida el formato y construye los argumentos de impresión.");
        });
        columns[1].group(|ui| {
            ui.label(egui::RichText::new("Modelo mental").strong());
            ui.code("tokens → expansión → código compilable");
            ui.label("Las macros son más potentes que una simple sustitución de texto.");
        });
    });
    ui.add_space(10.0);
    if ui.button("🔬 Expandir println! con cargo expand").clicked() {
        state.ui.show_macro_expansion = true;
    }

    let mut abierto = state.ui.show_macro_expansion;
    egui::Window::new("Expansión didáctica de println!")
        .open(&mut abierto)
        .collapsible(false)
        .default_width(620.0)
        .show(ui.ctx(), |ui| {
            ui.label("Representación simplificada para entender la idea:");
            ui.code("std::io::_print(format_args!(\"Hola, Ferris!\\n\"));");
            ui.add_space(8.0);
            ui.label("La expansión exacta depende de la versión del compilador y puede usar detalles internos.");
            ui.separator();
            ui.code("cargo install cargo-expand");
            ui.code("cargo expand");
        });
    state.ui.show_macro_expansion = abierto;
}

#[allow(dead_code)]
pub fn mostrar_tutorial_tipos_datos(ui: &mut egui::Ui, state: &mut AppState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Tipos compuestos")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        for (i, label) in [(0, "Array [T; N]"), (1, "Slice &[T]"), (2, "Tupla")] {
            let activo = state.lessons.compuestos_tab == i;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.compuestos_tab = i;
            }
            ui.add_space(4.0);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.lessons.compuestos_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Comparar").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.lessons.compuestos_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Teórico:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(10.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state.lessons.compuestos_tab < 3 {
                mostrar_selector_proyectos_estandar_con_archivos(
                    ui,
                    &mut state.project.selected_project,
                    &mut state.project.selected_file,
                    &mut state.terminal.term_cwd,
                    "combo_proyectos_tipos_compuestos",
                    &mut state.lessons.datatypes_code,
                );

                ui.add_space(10.0);

                let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
                mostrar_editor_interactivo(
                    ui,
                    &mut state.lessons.datatypes_code,
                    Arc::clone(&state.lessons.datatypes_output),
                    "",
                    ejecutar_codigo_rust,
                    &state.editor.syntax_set,
                    theme,
                );

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(12.0);
            }

            match state.lessons.compuestos_tab {
                0 => mostrar_compuesto_array(ui, state, naranja, cyan, texto),
                1 => mostrar_compuesto_slice(ui, state, naranja, cyan, texto),
                2 => mostrar_compuesto_tupla(ui, state, naranja, cyan, texto),
                _ => mostrar_compuesto_comparar(ui, naranja, cyan, texto),
            }
        });
}

fn mostrar_compuesto_array(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un array `[T; N]` guarda N valores del mismo tipo, contiguos, con tamaño fijo \
             conocido en compilación. Suele vivir en el stack.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_array_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Pieza")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Nota")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();
                ui.label(
                    egui::RichText::new("Tipo")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("[i32; 5]").monospace().color(cyan));
                ui.label("T y N fijos; N es parte del tipo.");
                ui.end_row();
                ui.label(
                    egui::RichText::new("Acceso")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr[i]").monospace().color(cyan));
                ui.label("Fuera de rango → panic en runtime.");
                ui.end_row();
                ui.label(
                    egui::RichText::new("len")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("arr.len()").monospace().color(cyan));
                ui.label("Siempre N; no crece como un Vec.");
                ui.end_row();
            });
    });

    ui.add_space(12.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Tipo T:").strong().color(texto));
        ui.selectable_value(&mut state.lessons.arr_elem_type, 0, "i8");
        ui.selectable_value(&mut state.lessons.arr_elem_type, 1, "i32");
        ui.selectable_value(&mut state.lessons.arr_elem_type, 2, "f64");
        ui.selectable_value(&mut state.lessons.arr_elem_type, 3, "bool");
        ui.selectable_value(&mut state.lessons.arr_elem_type, 4, "char");
        ui.add_space(12.0);
        ui.label(egui::RichText::new("N:").strong().color(texto));
        ui.add(egui::Slider::new(&mut state.lessons.arr_len, 1..=8).text("elems"));
    });

    let mut custom_items: Vec<String> = Vec::new();
    if let Some(pos_eq) = state.lessons.arr_code.find("= [") {
        let rest = &state.lessons.arr_code[pos_eq + 3..];
        if let Some(pos_end) = rest.find(']') {
            custom_items = rest[..pos_end]
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !custom_items.is_empty() {
                state.lessons.arr_len = custom_items.len().clamp(1, 8);
            }
        }
    }
    if state.lessons.arr_code.contains("i8") {
        state.lessons.arr_elem_type = 0;
    } else if state.lessons.arr_code.contains("f64") {
        state.lessons.arr_elem_type = 2;
    } else if state.lessons.arr_code.contains("bool") {
        state.lessons.arr_elem_type = 3;
    } else if state.lessons.arr_code.contains("char") {
        state.lessons.arr_elem_type = 4;
    } else if state.lessons.arr_code.contains("i32") || state.lessons.arr_code.contains("u32") {
        state.lessons.arr_elem_type = 1;
    }

    let (type_str, elem_size, default_samples) = match state.lessons.arr_elem_type {
        0 => (
            "i8",
            1,
            vec!["-12", "45", "127", "-8", "0", "99", "-50", "12"],
        ),
        1 => (
            "i32",
            4,
            vec!["100", "-500", "2048", "42", "0", "999", "-123", "8888"],
        ),
        2 => (
            "f64",
            8,
            vec![
                "3.14", "9.81", "-0.5", "2.71", "100.0", "0.001", "-45.2", "1.61",
            ],
        ),
        3 => (
            "bool",
            1,
            vec![
                "true", "false", "true", "true", "false", "false", "true", "false",
            ],
        ),
        _ => (
            "char",
            4,
            vec!["'R'", "'u'", "'s'", "'t'", "'🦀'", "'⚡'", "'🔥'", "'A'"],
        ),
    };
    let samples: Vec<&str> = if !custom_items.is_empty() {
        custom_items.iter().map(|s| s.as_str()).collect()
    } else {
        default_samples
    };
    let total_stack_bytes = state.lessons.arr_len * elem_size;

    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(format!(
            "Firma: [{type_str}; {}]  ·  Stack ≈ {total_stack_bytes} bytes",
            state.lessons.arr_len
        ))
        .monospace()
        .color(cyan),
    );

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 120.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let cell_w = (rect.width() - 40.0) / state.lessons.arr_len as f32;
    let start_x = rect.left() + 20.0;
    let y = rect.center().y;
    for i in 0..state.lessons.arr_len {
        let box_x = start_x + (i as f32 * cell_w) + cell_w / 2.0;
        let box_rect =
            egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(cell_w - 6.0, 48.0));
        let is_active = i == state.lessons.arr_active_idx;
        let fill = if is_active {
            egui::Color32::from_rgb(48, 36, 22)
        } else {
            egui::Color32::from_rgb(28, 36, 52)
        };
        let stroke_c = if is_active { naranja } else { cyan };
        painter.rect(
            box_rect,
            5.0,
            fill,
            egui::Stroke::new(1.5, stroke_c),
            egui::StrokeKind::Middle,
        );
        painter.text(
            egui::pos2(box_rect.center().x, box_rect.top() + 6.0),
            egui::Align2::CENTER_TOP,
            format!("[{i}]"),
            egui::FontId::proportional(11.0),
            egui::Color32::LIGHT_GRAY,
        );
        painter.text(
            box_rect.center() + egui::vec2(0.0, 4.0),
            egui::Align2::CENTER_CENTER,
            samples[i % samples.len()],
            egui::FontId::monospace(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("Índice arr[i]:");
        ui.add(
            egui::Slider::new(
                &mut state.lessons.arr_active_idx,
                0..=state.lessons.arr_len.max(1),
            )
            .text("i"),
        );
        if ui.button("arr.len()").clicked() {
            state.lessons.arr_action_msg = format!("arr.len() = {}", state.lessons.arr_len);
        }
        if ui.button("size_of").clicked() {
            state.lessons.arr_action_msg = format!("≈ {total_stack_bytes} bytes en stack");
        }
    });
    if state.lessons.arr_active_idx >= state.lessons.arr_len {
        ui.label(
            egui::RichText::new(format!(
                "PANIC: índice {} fuera de rango (len {})",
                state.lessons.arr_active_idx, state.lessons.arr_len
            ))
            .color(egui::Color32::from_rgb(255, 120, 120)),
        );
    } else {
        ui.label(
            egui::RichText::new(format!(
                "arr[{}] = {}",
                state.lessons.arr_active_idx,
                samples[state.lessons.arr_active_idx % samples.len()]
            ))
            .color(egui::Color32::from_rgb(120, 220, 140)),
        );
    }
    if !state.lessons.arr_action_msg.is_empty() {
        ui.label(
            egui::RichText::new(&state.lessons.arr_action_msg)
                .italics()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
    }

    ui.add_space(12.0);
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.lessons.arr_code,
        Arc::clone(&state.lessons.arr_output),
        "",
        ejecutar_codigo_rust,
        &state.editor.syntax_set,
        theme,
    );
}

fn mostrar_compuesto_slice(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un slice `&[T]` es una vista (préstamo) sobre una secuencia contigua: \
             fat pointer = puntero + longitud. No es dueño de los datos (Ownership).",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_slice_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Pieza")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Nota")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();
                ui.label(
                    egui::RichText::new("Tipo")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("&[i32]").monospace().color(cyan));
                ui.label("Referencia; el array/String sigue siendo dueño.");
                ui.end_row();
                ui.label(
                    egui::RichText::new("Rango")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("&arr[1..4]").monospace().color(cyan));
                ui.label("Inicio inclusivo, fin exclusivo.");
                ui.end_row();
                ui.label(
                    egui::RichText::new("&str")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("&str ≈ &[u8] UTF-8")
                        .monospace()
                        .color(cyan),
                );
                ui.label("El slice de texto que ya viste en Strings.");
                ui.end_row();
            });
    });

    let slice_max = 6;
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Rango:");
        ui.add(egui::Slider::new(&mut state.lessons.slice_start, 0..=slice_max - 1).text("start"));
        ui.add(egui::Slider::new(&mut state.lessons.slice_end, 1..=slice_max).text("end"));
    });
    if state.lessons.slice_start >= state.lessons.slice_end {
        state.lessons.slice_end = state.lessons.slice_start + 1;
    }
    let slice_len = state.lessons.slice_end - state.lessons.slice_start;

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 130.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let cell_w = 72.0;
    let start_x = rect.left() + 36.0;
    let y = rect.center().y + 4.0;
    let vals = ["10", "20", "30", "40", "50", "60"];
    for (i, value) in vals.iter().enumerate().take(slice_max) {
        let box_x = start_x + (i as f32 * cell_w) + cell_w / 2.0;
        let box_rect =
            egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(cell_w - 8.0, 44.0));
        let in_slice = i >= state.lessons.slice_start && i < state.lessons.slice_end;
        painter.rect(
            box_rect,
            4.0,
            if in_slice {
                egui::Color32::from_rgb(28, 48, 72)
            } else {
                egui::Color32::from_rgb(24, 28, 36)
            },
            egui::Stroke::new(
                1.2,
                if in_slice {
                    cyan
                } else {
                    egui::Color32::from_rgb(60, 70, 85)
                },
            ),
            egui::StrokeKind::Middle,
        );
        painter.text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            value,
            egui::FontId::monospace(13.0),
            egui::Color32::WHITE,
        );
    }
    let slice_min_x = start_x + (state.lessons.slice_start as f32 * cell_w);
    let slice_max_x = start_x + (state.lessons.slice_end as f32 * cell_w);
    let slice_rect = egui::Rect::from_min_max(
        egui::pos2(slice_min_x + 2.0, y - 30.0),
        egui::pos2(slice_max_x - 2.0, y + 30.0),
    );
    painter.rect_stroke(
        slice_rect,
        6.0,
        egui::Stroke::new(2.0, naranja),
        egui::StrokeKind::Middle,
    );
    painter.text(
        egui::pos2(slice_rect.center().x, slice_rect.top() - 4.0),
        egui::Align2::CENTER_BOTTOM,
        format!(
            "&arr[{}..{}]  len={slice_len}",
            state.lessons.slice_start, state.lessons.slice_end
        ),
        egui::FontId::proportional(12.0),
        naranja,
    );

    ui.add_space(12.0);
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.lessons.slice_code,
        Arc::clone(&state.lessons.slice_output),
        "",
        ejecutar_codigo_rust,
        &state.editor.syntax_set,
        theme,
    );
}

fn mostrar_compuesto_tupla(
    ui: &mut egui::Ui,
    state: &mut AppState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Una tupla agrupa valores de tipos distintos, sin nombres de campo. \
             Acceso por `.0`, `.1`… o desestructuración. Puente natural hacia `struct`.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_tupla_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Pieza")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Nota")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();
                ui.label(
                    egui::RichText::new("Tipo")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("(i32, bool, f64)")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Heterogénea; el orden define el tipo.");
                ui.end_row();
                ui.label(
                    egui::RichText::new("Campo")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(egui::RichText::new("t.0  t.1").monospace().color(cyan));
                ui.label("Índices fijos desde cero.");
                ui.end_row();
                ui.label(
                    egui::RichText::new("Destruct")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label(
                    egui::RichText::new("let (a, b, c) = t;")
                        .monospace()
                        .color(cyan),
                );
                ui.label("Muy usado al devolver varios valores desde fn.");
                ui.end_row();
            });
    });

    ui.add_space(10.0);
    let row = |ui: &mut egui::Ui, label: &str, slot: &mut usize| {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.selectable_value(slot, 0, "i32");
            ui.selectable_value(slot, 1, "bool");
            ui.selectable_value(slot, 2, "f64");
            ui.selectable_value(slot, 3, "char");
        });
    };
    row(ui, "Campo .0:", &mut state.lessons.tup_t0);
    row(ui, "Campo .1:", &mut state.lessons.tup_t1);
    row(ui, "Campo .2:", &mut state.lessons.tup_t2);

    let info = |id: usize| match id {
        0 => ("i32", "100", egui::Color32::from_rgb(60, 140, 240)),
        1 => ("bool", "true", egui::Color32::from_rgb(40, 180, 100)),
        2 => ("f64", "3.14", egui::Color32::from_rgb(240, 140, 40)),
        _ => ("char", "'R'", egui::Color32::from_rgb(180, 120, 240)),
    };
    let (n0, v0, c0) = info(state.lessons.tup_t0);
    let (n1, v1, c1) = info(state.lessons.tup_t1);
    let (n2, v2, c2) = info(state.lessons.tup_t2);

    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(format!("Firma: ({n0}, {n1}, {n2})"))
            .monospace()
            .color(cyan),
    );

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 110.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let y = rect.center().y;
    for (i, (name, val, col)) in [(n0, v0, c0), (n1, v1, c1), (n2, v2, c2)]
        .into_iter()
        .enumerate()
    {
        let x = rect.left() + 90.0 + i as f32 * 180.0;
        let r = egui::Rect::from_center_size(egui::pos2(x, y), egui::vec2(150.0, 52.0));
        painter.rect(
            r,
            6.0,
            egui::Color32::from_rgb(22, 28, 40),
            egui::Stroke::new(2.0, col),
            egui::StrokeKind::Middle,
        );
        painter.text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            format!(".{i}: {val} ({name})"),
            egui::FontId::monospace(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(12.0);
    let theme = &state.editor.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.lessons.tup_code,
        Arc::clone(&state.lessons.tup_output),
        "",
        ejecutar_codigo_rust,
        &state.editor.syntax_set,
        theme,
    );
}

fn mostrar_compuesto_comparar(
    ui: &mut egui::Ui,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Elige la forma según homogeneidad, tamaño fijo y si necesitas nombres de campo. \
             Después, `struct` pondrá nombres; `Vec` hará crecer lo homogéneo.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_compuestos_vs")
            .striped(true)
            .spacing([14.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Homogéneo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Dueño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Siguiente paso")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("[T; N]")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label("Sí");
                ui.label("Fijo N");
                ui.label("El array");
                ui.label(egui::RichText::new("Vec<T>").monospace().color(cyan));
                ui.end_row();

                ui.label(
                    egui::RichText::new("&[T]")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label("Sí");
                ui.label("Dinámico (vista)");
                ui.label("No (préstamo)");
                ui.label(egui::RichText::new("&str / APIs").monospace().color(cyan));
                ui.end_row();

                ui.label(
                    egui::RichText::new("(A,B,…)")
                        .monospace()
                        .strong()
                        .color(naranja),
                );
                ui.label("No");
                ui.label("Fijo #campos");
                ui.label("La tupla");
                ui.label(egui::RichText::new("struct").monospace().color(cyan));
                ui.end_row();
            });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(
            "Cuando la tupla se vuelve confusa (¿qué era .2?) → sesión Structs & impl.",
        )
        .italics()
        .color(egui::Color32::from_rgb(140, 150, 165)),
    );
}

#[allow(dead_code)]
pub fn mostrar_categoria_numeros(ui: &mut egui::Ui) {
    mostrar_categoria_enteros(ui);
    ui.add_space(25.0);
    mostrar_categoria_flotantes(ui);

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);

    ui.heading(
        egui::RichText::new("Operadores Numéricos")
            .strong()
            .color(naranja)
            .size(20.0),
    );
    ui.add_space(10.0);
    ui.label("Rust incluye los operadores matemáticos y de comparación estándar. Es importante recordar que en Rust no puedes operar entre diferentes tipos numéricos sin hacer un casting explícito primero.");
    ui.add_space(15.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    ui.label(
        egui::RichText::new("Aritméticos")
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    frame.show(ui, |ui| {
        egui::Grid::new("grid_operadores_aritmeticos")
            .striped(true)
            .spacing([30.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Operador")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Nombre")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                let ops = [
                    ("+", "Suma", "let suma = 5 + 10;"),
                    ("-", "Resta", "let resta = 95 - 4;"),
                    ("*", "Multiplicación", "let mult = 4 * 30;"),
                    ("/", "División", "let div = 56.0 / 32.2;"),
                    ("%", "Módulo (Resto)", "let resto = 43 % 5;"),
                ];

                for (simbolo, nombre, ej) in ops {
                    codigo_chip_color(ui, simbolo, naranja);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(nombre)
                                .size(12.0)
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                    });
                    codigo_chip_color(ui, ej, cyan);
                    ui.end_row();
                }
            });
    });

    ui.add_space(15.0);

    ui.label(
        egui::RichText::new("Asignación Compuesta")
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    frame.show(ui, |ui| {
        egui::Grid::new("grid_operadores_asignacion")
            .striped(true)
            .spacing([30.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Operador")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Equivalente")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                let ops = [
                    ("+=", "x = x + y", "let mut x = 5; x += 2;"),
                    ("-=", "x = x - y", "let mut x = 5; x -= 2;"),
                    ("*=", "x = x * y", "let mut x = 5; x *= 2;"),
                    ("/=", "x = x / y", "let mut x = 5; x /= 2;"),
                    ("%=", "x = x % y", "let mut x = 5; x %= 2;"),
                ];

                for (simbolo, equiv, ej) in ops {
                    codigo_chip_color(ui, simbolo, naranja);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(equiv)
                                .size(12.0)
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                    });
                    codigo_chip_color(ui, ej, cyan);
                    ui.end_row();
                }
            });
    });
}
