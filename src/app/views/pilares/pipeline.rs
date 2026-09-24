use crate::app::AppState;
use eframe::egui;

#[derive(Clone, Copy)]
struct EtapaCompilacion {
    nombre: &'static str,
    subtitulo: &'static str,
    detalle: &'static str,
    salida: &'static str,
    color: egui::Color32,
}

fn color_con_alpha(color: egui::Color32, alpha: f32) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (255.0 * alpha.clamp(0.0, 1.0)) as u8,
    )
}

fn escalar_color(color: egui::Color32, factor: f32, alpha: f32) -> egui::Color32 {
    let canal = |valor: u8| (valor as f32 * factor).clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(
        canal(color.r()),
        canal(color.g()),
        canal(color.b()),
        (255.0 * alpha.clamp(0.0, 1.0)) as u8,
    )
}

#[derive(Clone, Copy)]
struct LosaIsometrica {
    ancho: f32,
    fondo: f32,
    grosor: f32,
    color: egui::Color32,
    alpha: f32,
    activa: bool,
}

fn dibujar_losa_isometrica(painter: &egui::Painter, centro: egui::Pos2, losa: LosaIsometrica) {
    let LosaIsometrica {
        ancho,
        fondo,
        grosor,
        color,
        alpha,
        activa,
    } = losa;
    let arriba = egui::pos2(centro.x, centro.y - fondo * 0.5);
    let derecha = egui::pos2(centro.x + ancho * 0.5, centro.y);
    let abajo = egui::pos2(centro.x, centro.y + fondo * 0.5);
    let izquierda = egui::pos2(centro.x - ancho * 0.5, centro.y);
    let abajo_derecha = derecha + egui::vec2(0.0, grosor);
    let abajo_centro = abajo + egui::vec2(0.0, grosor);
    let abajo_izquierda = izquierda + egui::vec2(0.0, grosor);

    if activa {
        for (expansion, opacidad) in [(10.0, 0.05), (6.0, 0.10), (3.0, 0.18)] {
            painter.add(egui::Shape::convex_polygon(
                vec![
                    arriba - egui::vec2(0.0, expansion * 0.5),
                    derecha + egui::vec2(expansion, 0.0),
                    abajo + egui::vec2(0.0, expansion * 0.5),
                    izquierda - egui::vec2(expansion, 0.0),
                ],
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(2.0, color_con_alpha(color, alpha * opacidad)),
            ));
        }
    }

    painter.add(egui::Shape::convex_polygon(
        vec![izquierda, abajo, abajo_centro, abajo_izquierda],
        escalar_color(color, 0.52, alpha),
        egui::Stroke::NONE,
    ));
    painter.add(egui::Shape::convex_polygon(
        vec![abajo, derecha, abajo_derecha, abajo_centro],
        escalar_color(color, 0.30, alpha),
        egui::Stroke::NONE,
    ));
    painter.add(egui::Shape::convex_polygon(
        vec![arriba, derecha, abajo, izquierda],
        escalar_color(color, if activa { 0.62 } else { 0.44 }, alpha),
        egui::Stroke::new(
            if activa { 2.2 } else { 1.2 },
            color_con_alpha(color, alpha * if activa { 0.95 } else { 0.50 }),
        ),
    ));

    let brillo = egui::Stroke::new(1.2, color_con_alpha(egui::Color32::WHITE, alpha * 0.30));
    painter.line_segment([arriba, derecha], brillo);
    painter.line_segment([arriba, izquierda], brillo);
    painter.line_segment(
        [abajo_izquierda, abajo_centro],
        egui::Stroke::new(1.0, color_con_alpha(color, alpha * 0.55)),
    );
    painter.line_segment(
        [abajo_centro, abajo_derecha],
        egui::Stroke::new(1.0, color_con_alpha(color, alpha * 0.35)),
    );
}

fn dibujar_icono_etapa(
    painter: &egui::Painter,
    indice: usize,
    centro: egui::Pos2,
    escala: f32,
    color: egui::Color32,
    alpha: f32,
    tiempo: f32,
) {
    let trazo = egui::Stroke::new(1.8 * escala, color_con_alpha(color, alpha));
    let suave = egui::Stroke::new(1.0 * escala, color_con_alpha(color, alpha * 0.50));

    match indice {
        0 => {
            // Árbol de sintaxis abstracta.
            let nodos = [
                centro + egui::vec2(0.0, -15.0) * escala,
                centro + egui::vec2(-24.0, 4.0) * escala,
                centro + egui::vec2(24.0, 4.0) * escala,
                centro + egui::vec2(-34.0, 23.0) * escala,
                centro + egui::vec2(-13.0, 23.0) * escala,
            ];
            for (a, b) in [(0, 1), (0, 2), (1, 3), (1, 4)] {
                painter.line_segment([nodos[a], nodos[b]], trazo);
            }
            for (n, punto) in nodos.iter().enumerate() {
                painter.circle_filled(
                    *punto,
                    if n == 0 { 5.5 } else { 4.0 } * escala,
                    color_con_alpha(color, alpha),
                );
                painter.circle_stroke(*punto, 8.0 * escala, suave);
            }
        }
        1 => {
            // Escudo del sistema de tipos y borrow checker.
            let s = escala;
            let puntos = vec![
                centro + egui::vec2(-25.0, -18.0) * s,
                centro + egui::vec2(25.0, -18.0) * s,
                centro + egui::vec2(20.0, 12.0) * s,
                centro + egui::vec2(0.0, 28.0) * s,
                centro + egui::vec2(-20.0, 12.0) * s,
            ];
            painter.add(egui::Shape::convex_polygon(
                puntos,
                color_con_alpha(color, alpha * 0.16),
                trazo,
            ));
            painter.line_segment(
                [
                    centro + egui::vec2(-11.0, 2.0) * s,
                    centro + egui::vec2(-2.0, 11.0) * s,
                ],
                trazo,
            );
            painter.line_segment(
                [
                    centro + egui::vec2(-2.0, 11.0) * s,
                    centro + egui::vec2(15.0, -8.0) * s,
                ],
                trazo,
            );
        }
        2 => {
            // Grafo MIR con un pulso viajando entre bloques.
            let s = escala;
            let bloques = [
                egui::Rect::from_center_size(
                    centro + egui::vec2(-28.0, -10.0) * s,
                    egui::vec2(28.0, 15.0) * s,
                ),
                egui::Rect::from_center_size(
                    centro + egui::vec2(7.0, 10.0) * s,
                    egui::vec2(28.0, 15.0) * s,
                ),
                egui::Rect::from_center_size(
                    centro + egui::vec2(36.0, -13.0) * s,
                    egui::vec2(23.0, 15.0) * s,
                ),
            ];
            for bloque in bloques {
                painter.rect(
                    bloque,
                    3.0,
                    color_con_alpha(color, alpha * 0.14),
                    trazo,
                    egui::StrokeKind::Middle,
                );
            }
            painter.line_segment([bloques[0].right_center(), bloques[1].left_center()], suave);
            painter.line_segment([bloques[1].right_center(), bloques[2].left_center()], suave);
            let pulso = (tiempo * 0.8).fract();
            let inicio = bloques[0].center();
            let fin = bloques[2].center();
            painter.circle_filled(
                inicio.lerp(fin, pulso),
                3.5 * s,
                color_con_alpha(egui::Color32::WHITE, alpha),
            );
        }
        3 => {
            // Núcleo de codegen, con anillos en rotación.
            let radio = 17.0 * escala;
            painter.circle_filled(centro, radio * 0.48, color_con_alpha(color, alpha * 0.32));
            painter.circle_stroke(centro, radio, trazo);
            painter.circle_stroke(centro, radio * 1.45, suave);
            for n in 0..6 {
                let angulo = tiempo * 1.8 + n as f32 * std::f32::consts::TAU / 6.0;
                let punto = centro + egui::vec2(angulo.cos(), angulo.sin() * 0.55) * radio * 1.45;
                painter.circle_filled(punto, 2.8 * escala, color_con_alpha(color, alpha));
            }
        }
        _ => {
            // Binario final: bloque con indicador de ejecución.
            let cuerpo = egui::Rect::from_center_size(centro, egui::vec2(72.0, 39.0) * escala);
            painter.rect(
                cuerpo,
                5.0,
                color_con_alpha(color, alpha * 0.16),
                trazo,
                egui::StrokeKind::Middle,
            );
            painter.line_segment(
                [
                    centro + egui::vec2(-20.0, 0.0) * escala,
                    centro + egui::vec2(-7.0, 0.0) * escala,
                ],
                trazo,
            );
            let play = vec![
                centro + egui::vec2(4.0, -10.0) * escala,
                centro + egui::vec2(4.0, 10.0) * escala,
                centro + egui::vec2(20.0, 0.0) * escala,
            ];
            painter.add(egui::Shape::convex_polygon(
                play,
                color_con_alpha(color, alpha * 0.75),
                egui::Stroke::NONE,
            ));
        }
    }
}

pub fn mostrar_tutorial_compilacion(ui: &mut egui::Ui, state: &mut AppState) {
    // Esta vista contiene indicadores orbitales animados. Mantener el repintado
    // local evita obligar al resto de FerrisKey a renderizar continuamente.
    ui.ctx().request_repaint();

    const DURACION: f32 = 7.5;
    const ETAPAS: [EtapaCompilacion; 5] = [
        EtapaCompilacion {
            nombre: "01  PARSE / AST",
            subtitulo: "El código se convierte en estructura",
            detalle: "rustc tokeniza el archivo, valida su sintaxis y construye un árbol que representa expresiones, tipos y módulos.",
            salida: "Árbol de sintaxis (AST)",
            color: egui::Color32::from_rgb(67, 205, 255),
        },
        EtapaCompilacion {
            nombre: "02  TYPES + BORROW",
            subtitulo: "Seguridad antes de generar código",
            detalle: "Se resuelven los tipos y el borrow checker comprueba préstamos, ownership y tiempos de vida.",
            salida: "Programa validado",
            color: egui::Color32::from_rgb(255, 91, 125),
        },
        EtapaCompilacion {
            nombre: "03  MIR",
            subtitulo: "Una representación fácil de optimizar",
            detalle: "Rust baja el programa a MIR, simplifica el flujo de control y prepara monomorfización y optimizaciones.",
            salida: "MIR optimizado",
            color: egui::Color32::from_rgb(255, 190, 70),
        },
        EtapaCompilacion {
            nombre: "04  LLVM CODEGEN",
            subtitulo: "Del lenguaje a código de máquina",
            detalle: "LLVM aplica optimizaciones de bajo nivel y produce archivos objeto específicos para tu CPU y sistema.",
            salida: "Archivos objeto (.o)",
            color: egui::Color32::from_rgb(123, 103, 255),
        },
        EtapaCompilacion {
            nombre: "05  LINKER",
            subtitulo: "Todo se une en un ejecutable",
            detalle: "El enlazador combina tu código, dependencias y bibliotecas del sistema para crear el binario final.",
            salida: "Binario ejecutable",
            color: egui::Color32::from_rgb(64, 225, 157),
        },
    ];

    if state.lessons.anim_compilacion_activa {
        let dt = ui.ctx().input(|i| i.stable_dt).min(0.1);
        state.lessons.compilacion_progreso =
            (state.lessons.compilacion_progreso + dt / DURACION).min(1.0);
        if state.lessons.compilacion_progreso >= 1.0 {
            state.lessons.anim_compilacion_activa = false;
            state.lessons.compilacion_etapa_seleccionada = ETAPAS.len() - 1;
        } else {
            state.lessons.compilacion_etapa_seleccionada = (state.lessons.compilacion_progreso
                * ETAPAS.len() as f32)
                .floor()
                .min(4.0) as usize;
            ui.ctx().request_repaint();
        }
    }

    ui.add_space(18.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Del código al binario")
                .size(32.0)
                .strong()
                .color(egui::Color32::from_rgb(239, 244, 255)),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new("Explora el pipeline de compilación de Rust, etapa por etapa")
                .size(15.0)
                .color(egui::Color32::from_rgb(148, 160, 184)),
        );
    });
    ui.add_space(18.0);

    egui::Frame::new()
        .fill(egui::Color32::from_rgb(17, 21, 31))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let texto_boton = if state.lessons.anim_compilacion_activa {
                    "⏸  Pausar"
                } else if state.lessons.compilacion_progreso >= 1.0 {
                    "▶  Reproducir"
                } else {
                    "▶  Continuar"
                };
                if ui.button(texto_boton).clicked() {
                    if state.lessons.compilacion_progreso >= 1.0 {
                        state.lessons.compilacion_progreso = 0.0;
                        state.lessons.compilacion_etapa_seleccionada = 0;
                    }
                    state.lessons.anim_compilacion_activa = !state.lessons.anim_compilacion_activa;
                }
                if ui.button("↺  Reiniciar").clicked() {
                    state.lessons.compilacion_progreso = 0.0;
                    state.lessons.compilacion_etapa_seleccionada = 0;
                    state.lessons.anim_compilacion_activa = true;
                }

                ui.add_space(10.0);
                let porcentaje = (state.lessons.compilacion_progreso * 100.0).round() as u32;
                ui.add(
                    egui::ProgressBar::new(state.lessons.compilacion_progreso)
                        .desired_width((ui.available_width() - 70.0).max(100.0))
                        .text(format!("Compilación  {porcentaje}%")),
                );
            });
        });

    ui.add_space(12.0);
    let ancho = ui.available_width();
    let compacto = ancho < 690.0;
    let alto = if compacto { 650.0 } else { 510.0 };
    let (rect, respuesta) = ui.allocate_exact_size(egui::vec2(ancho, alto), egui::Sense::click());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 16.0, egui::Color32::from_rgb(10, 13, 21));
    painter.rect_stroke(
        rect.shrink(0.5),
        16.0,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(37, 45, 63)),
        egui::StrokeKind::Inside,
    );

    // Rejilla de fondo para reforzar la profundidad sin usar un motor 3D.
    let rejilla = egui::Color32::from_rgba_unmultiplied(91, 112, 150, 18);
    let paso = 32.0;
    let mut x = rect.left();
    while x <= rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(1.0, rejilla),
        );
        x += paso;
    }
    let mut y = rect.top();
    while y <= rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(1.0, rejilla),
        );
        y += paso;
    }

    let pila_ancho = if compacto {
        (rect.width() - 50.0).min(330.0)
    } else {
        (rect.width() * 0.42).clamp(260.0, 360.0)
    };
    let pila_fondo = pila_ancho * 0.36;
    let centro_x = if compacto {
        rect.center().x
    } else {
        rect.left() + rect.width() * 0.29
    };
    let inicio_y = rect.top() + 92.0;
    let separacion = 72.0;
    let puntero = respuesta.hover_pos();
    let tiempo = state.ui.tutorial_time as f32;
    let completadas = (state.lessons.compilacion_progreso * ETAPAS.len() as f32).clamp(0.0, 5.0);
    let mut etapa_hover = None;

    for indice in 0..ETAPAS.len() {
        let centro_y = inicio_y + indice as f32 * separacion;
        let zona = egui::Rect::from_center_size(
            egui::pos2(centro_x, centro_y),
            egui::vec2(pila_ancho + 30.0, separacion),
        );
        if puntero.is_some_and(|p| zona.contains(p)) {
            etapa_hover = Some(indice);
        }
    }
    if respuesta.clicked()
        && let Some(indice) = etapa_hover
    {
        state.lessons.compilacion_etapa_seleccionada = indice;
    }
    let etapa_visible = etapa_hover.unwrap_or(state.lessons.compilacion_etapa_seleccionada);

    // Sombra común de la pila.
    let sombra_centro = egui::pos2(centro_x + 12.0, inicio_y + 4.0 * separacion + 38.0);
    painter.add(egui::Shape::convex_polygon(
        vec![
            sombra_centro + egui::vec2(0.0, -pila_fondo * 0.35),
            sombra_centro + egui::vec2(pila_ancho * 0.57, 0.0),
            sombra_centro + egui::vec2(0.0, pila_fondo * 0.55),
            sombra_centro + egui::vec2(-pila_ancho * 0.57, 0.0),
        ],
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 90),
        egui::Stroke::NONE,
    ));

    // Se pinta de abajo hacia arriba para que el solapamiento sea natural.
    for indice in (0..ETAPAS.len()).rev() {
        let etapa = ETAPAS[indice];
        let fraccion = (completadas - indice as f32).clamp(0.0, 1.0);
        let disponible = fraccion > 0.0 || state.lessons.compilacion_progreso >= 1.0;
        let activa = indice == etapa_visible;
        let hover_offset = if activa { -7.0 } else { 0.0 };
        let entrada = if disponible {
            (1.0 - fraccion).powi(2) * -22.0
        } else {
            0.0
        };
        let centro = egui::pos2(
            centro_x,
            inicio_y + indice as f32 * separacion + hover_offset + entrada,
        );
        let alpha = if disponible {
            0.62 + fraccion * 0.38
        } else {
            0.20
        };

        dibujar_losa_isometrica(
            &painter,
            centro,
            LosaIsometrica {
                ancho: pila_ancho,
                fondo: pila_fondo,
                grosor: 18.0,
                color: etapa.color,
                alpha,
                activa,
            },
        );
        dibujar_icono_etapa(
            &painter,
            indice,
            centro + egui::vec2(0.0, -6.0),
            (pila_ancho / 330.0).clamp(0.75, 1.0),
            etapa.color,
            alpha,
            tiempo,
        );

        let badge = egui::pos2(centro.x - pila_ancho * 0.5 + 23.0, centro.y - 5.0);
        painter.circle_filled(badge, 11.0, escalar_color(etapa.color, 0.28, alpha));
        painter.circle_stroke(
            badge,
            11.0,
            egui::Stroke::new(1.2, color_con_alpha(etapa.color, alpha)),
        );
        painter.text(
            badge,
            egui::Align2::CENTER_CENTER,
            format!("{}", indice + 1),
            egui::FontId::monospace(10.0),
            color_con_alpha(egui::Color32::WHITE, alpha),
        );
    }

    // Pulso que recorre el pipeline durante la reproducción.
    if state.lessons.anim_compilacion_activa {
        let tramo = (state.lessons.compilacion_progreso * 5.0).min(4.999);
        let indice = tramo.floor() as usize;
        let local = tramo.fract();
        let y0 = inicio_y + indice as f32 * separacion;
        let y1 = inicio_y + (indice + 1).min(4) as f32 * separacion;
        let punto = egui::pos2(centro_x + pila_ancho * 0.54, egui::lerp(y0..=y1, local));
        painter.circle_filled(punto, 9.0, color_con_alpha(ETAPAS[indice].color, 0.16));
        painter.circle_filled(punto, 4.0, egui::Color32::WHITE);
    }

    let etapa = ETAPAS[etapa_visible];
    if compacto {
        let tarjeta = egui::Rect::from_min_max(
            egui::pos2(rect.left() + 18.0, rect.bottom() - 150.0),
            egui::pos2(rect.right() - 18.0, rect.bottom() - 18.0),
        );
        dibujar_panel_etapa(&painter, tarjeta, etapa, etapa_visible);
    } else {
        let tarjeta = egui::Rect::from_min_max(
            egui::pos2(rect.left() + rect.width() * 0.56, rect.top() + 68.0),
            egui::pos2(rect.right() - 24.0, rect.bottom() - 68.0),
        );
        dibujar_panel_etapa(&painter, tarjeta, etapa, etapa_visible);

        let origen = egui::pos2(
            centro_x + pila_ancho * 0.5 + 8.0,
            inicio_y + etapa_visible as f32 * separacion,
        );
        let codo = egui::pos2(tarjeta.left() - 18.0, origen.y);
        let destino = egui::pos2(tarjeta.left(), tarjeta.top() + 52.0);
        let conector = egui::Stroke::new(1.2, color_con_alpha(etapa.color, 0.65));
        painter.line_segment([origen, codo], conector);
        painter.line_segment([codo, destino], conector);
        painter.circle_filled(origen, 3.5, etapa.color);
    }

    if etapa_hover.is_some() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new("Pasa el cursor o haz clic en una capa para inspeccionarla.")
            .small()
            .color(egui::Color32::from_rgb(124, 137, 160)),
    );
}

fn dibujar_panel_etapa(
    painter: &egui::Painter,
    rect: egui::Rect,
    etapa: EtapaCompilacion,
    indice: usize,
) {
    painter.rect(
        rect,
        12.0,
        egui::Color32::from_rgba_unmultiplied(18, 23, 35, 245),
        egui::Stroke::new(1.0, color_con_alpha(etapa.color, 0.45)),
        egui::StrokeKind::Inside,
    );
    painter.rect_filled(
        egui::Rect::from_min_size(rect.min, egui::vec2(4.0, rect.height())),
        2.0,
        etapa.color,
    );
    painter.text(
        rect.min + egui::vec2(24.0, 22.0),
        egui::Align2::LEFT_TOP,
        etapa.nombre,
        egui::FontId::monospace(15.0),
        etapa.color,
    );
    painter.text(
        rect.min + egui::vec2(24.0, 50.0),
        egui::Align2::LEFT_TOP,
        etapa.subtitulo,
        egui::FontId::proportional(17.0),
        egui::Color32::from_rgb(234, 239, 249),
    );

    let ancho_texto = (rect.width() - 48.0).max(120.0);
    let detalle = painter.layout(
        etapa.detalle.to_owned(),
        egui::FontId::proportional(14.0),
        egui::Color32::from_rgb(158, 170, 192),
        ancho_texto,
    );
    painter.galley(
        rect.min + egui::vec2(24.0, 82.0),
        detalle,
        egui::Color32::WHITE,
    );

    let salida_y = rect.bottom() - 50.0;
    painter.line_segment(
        [
            egui::pos2(rect.left() + 24.0, salida_y - 12.0),
            egui::pos2(rect.right() - 24.0, salida_y - 12.0),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(42, 50, 67)),
    );
    painter.text(
        egui::pos2(rect.left() + 24.0, salida_y),
        egui::Align2::LEFT_TOP,
        format!("SALIDA  →  {}", etapa.salida),
        egui::FontId::monospace(12.0),
        egui::Color32::from_rgb(196, 205, 220),
    );
    painter.text(
        rect.right_top() + egui::vec2(-18.0, 18.0),
        egui::Align2::RIGHT_TOP,
        format!("{}/5", indice + 1),
        egui::FontId::monospace(11.0),
        egui::Color32::from_rgb(112, 124, 146),
    );
}
