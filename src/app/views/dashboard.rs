use crate::app::AppState;
use eframe::egui;
use egui_plot::{Bar, BarChart, Corner, HLine, Legend, Line, Plot, PlotPoints, VLine};

pub fn mostrar_graficos(ui: &mut egui::Ui, state: &mut AppState) {
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Dashboard Analítico")
                .size(28.0)
                .strong()
                .color(egui::Color32::from_rgb(100, 200, 255)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("Visualización interactiva multimodelo con egui & egui_plot")
                .size(14.0)
                .italics(),
        );
    });

    ui.add_space(15.0);

    // Barra de Navegación de Pestañas (Tabs)
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.dashboard.dash_tab, 0, "Power BI Overview");
        ui.selectable_value(&mut state.dashboard.dash_tab, 1, "Bar Chart Race");
        ui.selectable_value(&mut state.dashboard.dash_tab, 2, "Pie & Donut Chart");
        ui.selectable_value(&mut state.dashboard.dash_tab, 3, "Index Chart");
        ui.selectable_value(&mut state.dashboard.dash_tab, 4, "Sankey Diagram");
        ui.selectable_value(&mut state.dashboard.dash_tab, 5, "Time Series Subplots");
    });

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(15.0);

    match state.dashboard.dash_tab {
        0 => mostrar_dashboard_power_bi(ui, state),
        1 => mostrar_bar_chart_race(ui, state),
        2 => mostrar_pie_donut_chart(ui, state),
        3 => mostrar_index_chart(ui, state),
        4 => mostrar_sankey_diagram(ui, state),
        _ => mostrar_time_series_subplots(ui, state),
    }
}

pub fn mostrar_dashboard_power_bi(ui: &mut egui::Ui, state: &mut AppState) {
    let avail_w = ui.available_width();

    // 1. KPI Cards Row (Arriba)
    ui.horizontal(|ui| {
        let card_w = ((avail_w - 25.0) / 4.0).max(110.0);

        let kpis = [
            (
                "Ingresos Totales",
                "$1,248,500",
                "+18.4% YoY",
                egui::Color32::from_rgb(60, 200, 120),
            ),
            (
                "Gastos Operativos",
                "$684,200",
                "-4.2% Eficiencia",
                egui::Color32::from_rgb(240, 90, 90),
            ),
            (
                "Margen Neto",
                "45.2%",
                "+5.1% YoY",
                egui::Color32::from_rgb(160, 100, 250),
            ),
            (
                "Throughput Rust",
                "14,250 ops/s",
                "Nativo",
                egui::Color32::from_rgb(255, 180, 50),
            ),
        ];

        for (title, value, badge, color) in kpis.iter() {
            let mut frame = egui::Frame::new();
            frame.fill = egui::Color32::from_rgb(22, 24, 30);
            frame.inner_margin = egui::Margin::same(10);
            frame.corner_radius = egui::CornerRadius::same(8);
            frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 50, 65));
            frame.show(ui, |ui| {
                ui.set_width(card_w - 20.0);
                ui.label(
                    egui::RichText::new(*title)
                        .size(12.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(4.0);
                ui.heading(
                    egui::RichText::new(*value)
                        .size(17.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(*badge)
                        .size(11.0)
                        .strong()
                        .color(*color),
                );
            });
        }
    });

    ui.add_space(15.0);

    // 2. Barra Horizontal de Filtros Power BI (Slicers)
    let mut slicer_frame = egui::Frame::new();
    slicer_frame.fill = egui::Color32::from_rgb(25, 27, 35);
    slicer_frame.inner_margin = egui::Margin::same(12);
    slicer_frame.corner_radius = egui::CornerRadius::same(8);
    slicer_frame.show(ui, |ui| {
        ui.set_width(avail_w - 24.0);
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("Filtros Power BI:")
                    .size(15.0)
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.add_space(15.0);

            ui.checkbox(&mut state.dashboard.show_ingresos, "Ingresos");
            ui.add_space(10.0);
            ui.checkbox(&mut state.dashboard.show_gastos, "Gastos");
            ui.add_space(10.0);
            ui.checkbox(&mut state.dashboard.show_beneficios, "Beneficios");
            ui.add_space(25.0);

            ui.label(egui::RichText::new("Año Fiscal:").strong());
            ui.add(egui::Slider::new(&mut state.dashboard.year, 2020..=2026));
        });
    });

    ui.add_space(15.0);

    // 3. Gráfico Principal (Abajo a todo lo ancho)
    let mut plot_frame = egui::Frame::new();
    plot_frame.fill = egui::Color32::from_rgb(20, 20, 25);
    plot_frame.inner_margin = egui::Margin::same(12);
    plot_frame.corner_radius = egui::CornerRadius::same(8);
    plot_frame.show(ui, |ui| {
        ui.set_width(avail_w - 24.0);
        let base_multiplier = (state.dashboard.year - 2020) as f64 * 80.0;

        let mut ingresos_bars = vec![];
        let mut gastos_bars = vec![];
        let mut beneficios_points = vec![];

        for i in 1..=12 {
            let x = i as f64;
            let ingreso = base_multiplier + 200.0 + (i as f64 * 35.0) + (i % 3) as f64 * 40.0;
            let gasto = base_multiplier + 140.0 + (i as f64 * 20.0) + (i % 2) as f64 * 25.0;
            let beneficio = ingreso - gasto;

            ingresos_bars.push(
                Bar::new(x - 0.2, ingreso)
                    .width(0.35)
                    .fill(egui::Color32::from_rgb(60, 200, 120)),
            );
            gastos_bars.push(
                Bar::new(x + 0.2, gasto)
                    .width(0.35)
                    .fill(egui::Color32::from_rgb(230, 90, 90)),
            );
            beneficios_points.push([x, beneficio]);
        }

        Plot::new("power_bi_plot")
            .legend(Legend::default().position(Corner::RightBottom))
            .height(380.0)
            .show_grid([true, true])
            .show(ui, |plot_ui| {
                if state.dashboard.show_ingresos {
                    plot_ui.bar_chart(BarChart::new("Ingresos", ingresos_bars));
                }
                if state.dashboard.show_gastos {
                    plot_ui.bar_chart(BarChart::new("Gastos", gastos_bars));
                }
                if state.dashboard.show_beneficios {
                    plot_ui.line(
                        Line::new("Tendencia Beneficios", PlotPoints::new(beneficios_points))
                            .color(egui::Color32::from_rgb(160, 100, 250))
                            .width(3.0),
                    );
                }
            });
    });
}

pub fn mostrar_bar_chart_race(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        if ui
            .button(if state.dashboard.bcr_playing {
                "Pausar"
            } else {
                "Iniciar Carrera"
            })
            .clicked()
        {
            state.dashboard.bcr_playing = !state.dashboard.bcr_playing;
        }
        if ui.button("Reiniciar (2015)").clicked() {
            state.dashboard.bcr_year = 2015.0;
        }
        ui.add_space(10.0);
        ui.label("Velocidad:");
        ui.selectable_value(&mut state.dashboard.bcr_speed, 0.5, "0.5x");
        ui.selectable_value(&mut state.dashboard.bcr_speed, 1.0, "1x");
        ui.selectable_value(&mut state.dashboard.bcr_speed, 2.0, "2x");
        ui.selectable_value(&mut state.dashboard.bcr_speed, 4.0, "4x");

        ui.add_space(20.0);
        ui.add(egui::Slider::new(&mut state.dashboard.bcr_year, 2015.0..=2026.0).text("Año"));
    });

    if state.dashboard.bcr_playing {
        let dt = ui.input(|i| i.stable_dt);
        state.dashboard.bcr_year += dt * state.dashboard.bcr_speed * 1.5;
        if state.dashboard.bcr_year > 2026.0 {
            state.dashboard.bcr_year = 2015.0;
        }
        ui.ctx().request_repaint();
    }

    ui.add_space(15.0);

    let languages = [
        (
            "Rust",
            egui::Color32::from_rgb(240, 100, 40),
            [
                (2015.0, 8.0),
                (2018.0, 22.0),
                (2021.0, 52.0),
                (2024.0, 85.0),
                (2026.0, 99.0),
            ],
        ),
        (
            "Python",
            egui::Color32::from_rgb(60, 140, 230),
            [
                (2015.0, 65.0),
                (2018.0, 78.0),
                (2021.0, 91.0),
                (2024.0, 97.0),
                (2026.0, 100.0),
            ],
        ),
        (
            "JavaScript",
            egui::Color32::from_rgb(240, 210, 50),
            [
                (2015.0, 88.0),
                (2018.0, 92.0),
                (2021.0, 94.0),
                (2024.0, 95.0),
                (2026.0, 96.0),
            ],
        ),
        (
            "TypeScript",
            egui::Color32::from_rgb(40, 160, 240),
            [
                (2015.0, 12.0),
                (2018.0, 42.0),
                (2021.0, 72.0),
                (2024.0, 89.0),
                (2026.0, 94.0),
            ],
        ),
        (
            "Go",
            egui::Color32::from_rgb(50, 210, 210),
            [
                (2015.0, 20.0),
                (2018.0, 48.0),
                (2021.0, 68.0),
                (2024.0, 80.0),
                (2026.0, 87.0),
            ],
        ),
        (
            "C++",
            egui::Color32::from_rgb(100, 120, 200),
            [
                (2015.0, 75.0),
                (2018.0, 72.0),
                (2021.0, 70.0),
                (2024.0, 72.0),
                (2026.0, 74.0),
            ],
        ),
        (
            "Java",
            egui::Color32::from_rgb(220, 70, 70),
            [
                (2015.0, 82.0),
                (2018.0, 78.0),
                (2021.0, 72.0),
                (2024.0, 66.0),
                (2026.0, 62.0),
            ],
        ),
    ];

    let yr = state.dashboard.bcr_year;

    let mut current_data: Vec<(&str, egui::Color32, f32)> = languages
        .iter()
        .map(|(name, color, points)| {
            let val = if yr <= points[0].0 {
                points[0].1
            } else if yr >= points[points.len() - 1].0 {
                points[points.len() - 1].1
            } else {
                let mut v = points[0].1;
                for idx in 0..points.len() - 1 {
                    if yr >= points[idx].0 && yr <= points[idx + 1].0 {
                        let t = (yr - points[idx].0) / (points[idx + 1].0 - points[idx].0);
                        v = points[idx].1 + t * (points[idx + 1].1 - points[idx].1);
                        break;
                    }
                }
                v
            };
            (*name, *color, val)
        })
        .collect();

    current_data.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let available_w = ui.available_width();
    let (_, rect) = ui.allocate_space(egui::vec2(available_w, 400.0));

    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    ui.painter().text(
        rect.right_bottom() - egui::vec2(30.0, 30.0),
        egui::Align2::RIGHT_BOTTOM,
        format!("{:.0}", yr),
        egui::FontId::proportional(80.0),
        egui::Color32::from_black_alpha(80),
    );

    let max_val = 105.0;
    let bar_height = 36.0;
    let gap = 14.0;
    let start_y = rect.top() + 20.0;
    let start_x = rect.left() + 160.0;
    let max_bar_width = rect.width() - 250.0;

    for (rank, (name, color, val)) in current_data.iter().enumerate() {
        let bar_y = start_y + rank as f32 * (bar_height + gap);
        let bar_w = (val / max_val) * max_bar_width;

        ui.painter().text(
            egui::pos2(start_x - 15.0, bar_y + bar_height / 2.0),
            egui::Align2::RIGHT_CENTER,
            *name,
            egui::FontId::proportional(15.0).clone(),
            egui::Color32::WHITE,
        );

        let bar_rect = egui::Rect::from_min_size(
            egui::pos2(start_x, bar_y),
            egui::vec2(bar_w.max(10.0), bar_height),
        );
        ui.painter()
            .rect_filled(bar_rect, egui::CornerRadius::same(5), *color);

        ui.painter().text(
            egui::pos2(start_x + bar_w + 12.0, bar_y + bar_height / 2.0),
            egui::Align2::LEFT_CENTER,
            format!("{:.1}%", val),
            egui::FontId::proportional(14.0).clone(),
            egui::Color32::from_rgb(220, 220, 220),
        );
    }
}

pub fn mostrar_pie_donut_chart(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Radio Rosquilla (Donut):").strong());
        ui.add(egui::Slider::new(&mut state.dashboard.pie_donut_hole, 0.0..=0.7).text("Hole"));
        ui.add_space(20.0);
        ui.checkbox(
            &mut state.dashboard.pie_exploded,
            "Explotar Rebanadas al Pasar el Mouse",
        );
    });

    ui.add_space(15.0);

    let slices = [
        (
            "Infraestructura Cloud",
            1897000.0,
            egui::Color32::from_rgb(60, 140, 240),
        ),
        (
            "I+D & Rust Core",
            1355000.0,
            egui::Color32::from_rgb(240, 100, 40),
        ),
        (
            "Ciberseguridad",
            975000.0,
            egui::Color32::from_rgb(160, 80, 220),
        ),
        (
            "Licencias & SaaS",
            650000.0,
            egui::Color32::from_rgb(40, 190, 110),
        ),
        (
            "Capacitación & Equipo",
            542000.0,
            egui::Color32::from_rgb(240, 190, 50),
        ),
    ];

    let total: f64 = slices.iter().map(|s| s.1).sum();

    ui.horizontal(|ui| {
        let (_, rect) = ui.allocate_space(egui::vec2(400.0, 380.0));

        ui.painter()
            .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

        let center = rect.center();
        let r_out = 140.0;
        let r_in = r_out * state.dashboard.pie_donut_hole;

        let pointer_pos = ui.input(|i| i.pointer.hover_pos());
        let mut current_angle: f32 = -std::f32::consts::FRAC_PI_2;
        let mut hovered_slice: Option<usize> = None;

        for (idx, (name, val, color)) in slices.iter().enumerate() {
            let slice_angle = ((*val / total) as f32) * std::f32::consts::TAU;
            let a1 = current_angle;
            let a2 = current_angle + slice_angle;
            let mid_angle = (a1 + a2) / 2.0;

            let is_hovered = if let Some(p) = pointer_pos {
                let dist = p.distance(center);
                if dist >= r_in && dist <= r_out + 25.0 {
                    let mut click_angle = (p.y - center.y).atan2(p.x - center.x);
                    if click_angle < a1 {
                        click_angle += std::f32::consts::TAU;
                    }
                    click_angle >= a1 && click_angle <= a2
                } else {
                    false
                }
            } else {
                false
            };

            if is_hovered {
                hovered_slice = Some(idx);
            }

            let explode_offset = if is_hovered && state.dashboard.pie_exploded {
                18.0
            } else {
                0.0
            };
            let slice_center = center
                + egui::vec2(
                    mid_angle.cos() * explode_offset,
                    mid_angle.sin() * explode_offset,
                );

            let steps = (((a2 - a1).abs() / 0.08).ceil() as usize).max(8);

            let mut outer_pts = vec![];
            let mut inner_pts = vec![];

            for step in 0..=steps {
                let t = step as f32 / steps as f32;
                let sub_a = a1 + (a2 - a1) * t;
                outer_pts.push(slice_center + egui::vec2(sub_a.cos() * r_out, sub_a.sin() * r_out));
                inner_pts.push(slice_center + egui::vec2(sub_a.cos() * r_in, sub_a.sin() * r_in));
            }

            for step in 0..steps {
                let v1 = inner_pts[step];
                let v2 = outer_pts[step];
                let v3 = outer_pts[step + 1];
                let v4 = inner_pts[step + 1];

                ui.painter().add(egui::Shape::convex_polygon(
                    vec![v1, v2, v3, v4],
                    *color,
                    egui::Stroke::NONE,
                ));
            }

            let border_stroke = if is_hovered {
                egui::Stroke::new(2.5, egui::Color32::WHITE)
            } else {
                egui::Stroke::new(1.2, egui::Color32::from_rgb(20, 22, 28))
            };

            let mut boundary = vec![];
            boundary.extend_from_slice(&outer_pts);
            for p in inner_pts.iter().rev() {
                boundary.push(*p);
            }
            ui.painter()
                .add(egui::Shape::closed_line(boundary, border_stroke));

            let pct = (*val / total) * 100.0;
            let p_edge =
                slice_center + egui::vec2(mid_angle.cos() * r_out, mid_angle.sin() * r_out);
            let p_elbow = slice_center
                + egui::vec2(
                    mid_angle.cos() * (r_out + 20.0),
                    mid_angle.sin() * (r_out + 20.0),
                );
            let is_right = mid_angle.cos() >= 0.0;
            let p_text = p_elbow + egui::vec2(if is_right { 18.0 } else { -18.0 }, 0.0);

            let leader_color = if is_hovered {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 180)
            };

            ui.painter()
                .line_segment([p_edge, p_elbow], egui::Stroke::new(1.2, leader_color));
            ui.painter()
                .line_segment([p_elbow, p_text], egui::Stroke::new(1.2, leader_color));
            ui.painter().circle_filled(p_edge, 2.5, leader_color);

            ui.painter().text(
                p_text + egui::vec2(if is_right { 4.0 } else { -4.0 }, 0.0),
                if is_right {
                    egui::Align2::LEFT_CENTER
                } else {
                    egui::Align2::RIGHT_CENTER
                },
                format!("{}: {:.1}%", name.split(' ').next().unwrap_or(name), pct),
                egui::FontId::proportional(11.0),
                if is_hovered {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::LIGHT_GRAY
                },
            );

            current_angle = a2;
        }

        if r_in > 30.0 {
            ui.painter()
                .circle_filled(center, r_in - 2.0, egui::Color32::from_rgb(20, 22, 28));
            ui.painter().text(
                center - egui::vec2(0.0, 10.0),
                egui::Align2::CENTER_CENTER,
                "Total Presupuesto",
                egui::FontId::proportional(12.0),
                egui::Color32::GRAY,
            );
            ui.painter().text(
                center + egui::vec2(0.0, 10.0),
                egui::Align2::CENTER_CENTER,
                format!("${:.2}M", total / 1_000_000.0),
                egui::FontId::proportional(18.0),
                egui::Color32::WHITE,
            );
        }

        ui.add_space(20.0);

        ui.vertical(|ui| {
            ui.heading("Desglose del Presupuesto");
            ui.add_space(10.0);
            for (idx, (name, val, color)) in slices.iter().enumerate() {
                let pct = (*val / total) * 100.0;
                let is_sel = hovered_slice == Some(idx);
                let mut text =
                    egui::RichText::new(format!("{}: {:.1}% (${:.0}k)", name, pct, val / 1000.0))
                        .size(14.0)
                        .color(if is_sel {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::LIGHT_GRAY
                        });
                if is_sel {
                    text = text.strong();
                }
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 3.0, *color);
                    ui.label(text);
                });
                ui.add_space(8.0);
            }
        });
    });
}

pub fn mostrar_index_chart(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Año Base (Punto 100%):").strong());
        ui.add(
            egui::Slider::new(&mut state.dashboard.index_baseline_year, 2015.0..=2025.0)
                .text("Año Base"),
        );
        ui.label(
            egui::RichText::new("*(Todos los gráficos se re-escalan a 100% en esta fecha)*")
                .italics()
                .color(egui::Color32::GRAY),
        );
    });

    ui.add_space(15.0);

    let baseline = state.dashboard.index_baseline_year;

    let series_raw = [
        (
            "Rust Repositories",
            egui::Color32::from_rgb(240, 100, 40),
            vec![
                (2015.0, 50.0),
                (2017.0, 180.0),
                (2019.0, 450.0),
                (2021.0, 1200.0),
                (2023.0, 3100.0),
                (2026.0, 7800.0),
            ],
        ),
        (
            "Python AI Packages",
            egui::Color32::from_rgb(60, 140, 230),
            vec![
                (2015.0, 2000.0),
                (2017.0, 4500.0),
                (2019.0, 9800.0),
                (2021.0, 18000.0),
                (2023.0, 32000.0),
                (2026.0, 65000.0),
            ],
        ),
        (
            "JS/TS Web Frameworks",
            egui::Color32::from_rgb(240, 210, 50),
            vec![
                (2015.0, 5000.0),
                (2017.0, 8500.0),
                (2019.0, 14000.0),
                (2021.0, 21000.0),
                (2023.0, 29000.0),
                (2026.0, 38000.0),
            ],
        ),
        (
            "Go Microservices",
            egui::Color32::from_rgb(50, 210, 210),
            vec![
                (2015.0, 300.0),
                (2017.0, 900.0),
                (2019.0, 2200.0),
                (2021.0, 5100.0),
                (2023.0, 11000.0),
                (2026.0, 22000.0),
            ],
        ),
    ];

    let mut indexed_lines = vec![];

    for (name, color, points) in series_raw.iter() {
        let base_val = if baseline <= points[0].0 {
            points[0].1
        } else if baseline >= points[points.len() - 1].0 {
            points[points.len() - 1].1
        } else {
            let mut v = points[0].1;
            for i in 0..points.len() - 1 {
                if baseline >= points[i].0 && baseline <= points[i + 1].0 {
                    let t = (baseline - points[i].0) / (points[i + 1].0 - points[i].0);
                    v = points[i].1 + t * (points[i + 1].1 - points[i].1);
                    break;
                }
            }
            v
        };

        let norm_pts: Vec<[f64; 2]> = points
            .iter()
            .map(|(x, y)| [*x as f64, (100.0 * (y / base_val)) as f64])
            .collect();

        indexed_lines.push(
            Line::new(*name, PlotPoints::new(norm_pts))
                .color(*color)
                .width(2.5),
        );
    }

    Plot::new("index_chart_plot")
        .legend(Legend::default().position(Corner::LeftTop))
        .height(380.0)
        .show_grid([true, true])
        .show(ui, |plot_ui| {
            plot_ui.hline(
                HLine::new("Base 100%", 100.0)
                    .color(egui::Color32::WHITE)
                    .width(1.5),
            );
            plot_ui.vline(
                VLine::new("Año Base", baseline as f64)
                    .color(egui::Color32::LIGHT_YELLOW)
                    .width(1.5),
            );

            for line in indexed_lines {
                plot_ui.line(line);
            }
        });
}

pub fn mostrar_sankey_diagram(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.label(
        egui::RichText::new("Diagrama de Flujo de Recursos y Asignación Financiera")
            .size(15.0)
            .italics(),
    );
    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 420.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let sources = [
        (
            "Ventas SaaS Directas",
            1200.0,
            egui::Color32::from_rgb(60, 200, 120),
        ),
        (
            "Licencias Enterprise",
            600.0,
            egui::Color32::from_rgb(80, 150, 240),
        ),
        (
            "Servicios Consultoría",
            200.0,
            egui::Color32::from_rgb(180, 100, 240),
        ),
    ];

    let hub = (
        "Ingresos Totales Brutos",
        2000.0,
        egui::Color32::from_rgb(240, 180, 50),
    );

    let targets = [
        (
            "Desarrollo Core Rust",
            800.0,
            egui::Color32::from_rgb(240, 100, 40),
        ),
        (
            "Infraestructura Cloud",
            500.0,
            egui::Color32::from_rgb(60, 160, 240),
        ),
        (
            "Marketing & Ventas",
            400.0,
            egui::Color32::from_rgb(240, 140, 180),
        ),
        (
            "Ganancia Neta Límpida",
            300.0,
            egui::Color32::from_rgb(40, 200, 100),
        ),
    ];

    let col1_x = rect.left() + 40.0;
    let col2_x = rect.center().x - 60.0;
    let col3_x = rect.right() - 200.0;
    let node_w = 160.0;

    let mut src_ports = vec![];
    let mut curr_y = rect.top() + 40.0;
    for (name, val, color) in sources.iter() {
        let h = (val / 2000.0) * 320.0;
        let n_rect = egui::Rect::from_min_size(egui::pos2(col1_x, curr_y), egui::vec2(node_w, h));
        ui.painter().rect_filled(n_rect, 5.0, *color);
        ui.painter().text(
            n_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{}\n${:.0}k", name, val),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        src_ports.push((egui::pos2(n_rect.right(), n_rect.center().y), h, *color));
        curr_y += h + 20.0;
    }

    let hub_h = 320.0;
    let hub_y = rect.top() + 40.0;
    let hub_rect = egui::Rect::from_min_size(egui::pos2(col2_x, hub_y), egui::vec2(node_w, hub_h));
    ui.painter().rect_filled(hub_rect, 5.0, hub.2);
    ui.painter().text(
        hub_rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{}\n${:.0}k", hub.0, hub.1),
        egui::FontId::proportional(13.0),
        egui::Color32::WHITE,
    );

    let mut tgt_ports = vec![];
    curr_y = rect.top() + 40.0;
    for (name, val, color) in targets.iter() {
        let h = (val / 2000.0) * 320.0;
        let n_rect = egui::Rect::from_min_size(egui::pos2(col3_x, curr_y), egui::vec2(node_w, h));
        ui.painter().rect_filled(n_rect, 5.0, *color);
        ui.painter().text(
            n_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{}\n${:.0}k", name, val),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        tgt_ports.push((egui::pos2(n_rect.left(), n_rect.center().y), h, *color));
        curr_y += h + 15.0;
    }

    for (port, h, color) in src_ports.iter() {
        let p1 = *port;
        let p2 = egui::pos2(hub_rect.left(), p1.y);

        let steps = 20;
        let mut top_pts = vec![];
        let mut bot_pts = vec![];

        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let x = p1.x + t * (p2.x - p1.x);
            let s_t = 3.0 * t * t - 2.0 * t * t * t;
            let y = p1.y + s_t * (p2.y - p1.y);
            top_pts.push(egui::pos2(x, y - h / 2.0));
            bot_pts.push(egui::pos2(x, y + h / 2.0));
        }

        let ribbon_color = egui::Color32::from_rgba_premultiplied(
            color.r() / 2,
            color.g() / 2,
            color.b() / 2,
            100,
        );

        for step in 0..steps {
            let quad = vec![
                top_pts[step],
                top_pts[step + 1],
                bot_pts[step + 1],
                bot_pts[step],
            ];
            ui.painter().add(egui::Shape::convex_polygon(
                quad,
                ribbon_color,
                egui::Stroke::NONE,
            ));
        }
    }

    for (port, h, color) in tgt_ports.iter() {
        let p1 = egui::pos2(hub_rect.right(), port.y);
        let p2 = *port;

        let steps = 20;
        let mut top_pts = vec![];
        let mut bot_pts = vec![];

        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let x = p1.x + t * (p2.x - p1.x);
            let s_t = 3.0 * t * t - 2.0 * t * t * t;
            let y = p1.y + s_t * (p2.y - p1.y);
            top_pts.push(egui::pos2(x, y - h / 2.0));
            bot_pts.push(egui::pos2(x, y + h / 2.0));
        }

        let ribbon_color = egui::Color32::from_rgba_premultiplied(
            color.r() / 2,
            color.g() / 2,
            color.b() / 2,
            100,
        );

        for step in 0..steps {
            let quad = vec![
                top_pts[step],
                top_pts[step + 1],
                bot_pts[step + 1],
                bot_pts[step],
            ];
            ui.painter().add(egui::Shape::convex_polygon(
                quad,
                ribbon_color,
                egui::Stroke::NONE,
            ));
        }
    }
}

pub fn mostrar_time_series_subplots(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.checkbox(
            &mut state.dashboard.ts_show_ma,
            "Medias Móviles (SMA 20/50)",
        );
        ui.checkbox(&mut state.dashboard.ts_show_volume, "Volumen");
        ui.checkbox(&mut state.dashboard.ts_show_rsi, "RSI Indicator");
    });

    ui.add_space(10.0);

    let days = 100;
    let mut price_pts = vec![];
    let mut sma20_pts = vec![];
    let mut sma50_pts = vec![];
    let mut bollinger_upper = vec![];
    let mut bollinger_lower = vec![];
    let mut volume_bars = vec![];
    let mut rsi_pts = vec![];

    let mut curr_price = 150.0;

    for d in 1..=days {
        let x = d as f64;
        let change = ((d % 7) as f64 - 3.2) * 2.5 + ((d % 3) as f64 - 1.0) * 1.5;
        curr_price = (curr_price + change).max(50.0);

        price_pts.push([x, curr_price]);

        let sma20 = curr_price * 0.95 + (d as f64 * 0.1);
        let sma50 = curr_price * 0.90 + (d as f64 * 0.15);
        let b_upper = sma20 + 15.0;
        let b_lower = sma20 - 15.0;

        sma20_pts.push([x, sma20]);
        sma50_pts.push([x, sma50]);
        bollinger_upper.push([x, b_upper]);
        bollinger_lower.push([x, b_lower]);

        let vol = 1000.0 + ((d % 5) as f64 * 400.0) + (change.abs() * 200.0);
        let vol_color = if change >= 0.0 {
            egui::Color32::from_rgb(60, 200, 120)
        } else {
            egui::Color32::from_rgb(230, 80, 80)
        };
        volume_bars.push(Bar::new(x, vol).width(0.7).fill(vol_color));

        let rsi = 50.0 + (change * 5.0) + ((d % 4) as f64 * 4.0);
        rsi_pts.push([x, rsi.clamp(10.0, 90.0)]);
    }

    ui.label(egui::RichText::new("Panel 1: Precio de Cotización & Indicadores").strong());
    Plot::new("subplot_price")
        .height(200.0)
        .legend(Legend::default().position(Corner::LeftTop))
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Precio", PlotPoints::new(price_pts))
                    .color(egui::Color32::WHITE)
                    .width(2.0),
            );
            if state.dashboard.ts_show_ma {
                plot_ui.line(
                    Line::new("SMA 20", PlotPoints::new(sma20_pts))
                        .color(egui::Color32::from_rgb(240, 180, 50))
                        .width(1.5),
                );
                plot_ui.line(
                    Line::new("SMA 50", PlotPoints::new(sma50_pts))
                        .color(egui::Color32::from_rgb(60, 160, 240))
                        .width(1.5),
                );
                plot_ui.line(
                    Line::new("Bollinger Sup", PlotPoints::new(bollinger_upper))
                        .color(egui::Color32::GRAY)
                        .width(1.0),
                );
                plot_ui.line(
                    Line::new("Bollinger Inf", PlotPoints::new(bollinger_lower))
                        .color(egui::Color32::GRAY)
                        .width(1.0),
                );
            }
        });

    ui.add_space(10.0);

    if state.dashboard.ts_show_volume {
        ui.label(egui::RichText::new("Panel 2: Volumen Operado Diario").strong());
        Plot::new("subplot_volume")
            .height(100.0)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(BarChart::new("Volumen", volume_bars));
            });
        ui.add_space(10.0);
    }

    if state.dashboard.ts_show_rsi {
        ui.label(egui::RichText::new("Panel 3: Índice de Fuerza Relativa (RSI)").strong());
        Plot::new("subplot_rsi").height(100.0).show(ui, |plot_ui| {
            plot_ui.hline(
                HLine::new("Límite Sobrecompra (70)", 70.0)
                    .color(egui::Color32::RED)
                    .width(1.0),
            );
            plot_ui.hline(
                HLine::new("Límite Sobrevendido (30)", 30.0)
                    .color(egui::Color32::GREEN)
                    .width(1.0),
            );
            plot_ui.line(
                Line::new("RSI (14)", PlotPoints::new(rsi_pts))
                    .color(egui::Color32::from_rgb(180, 100, 240))
                    .width(1.5),
            );
        });
    }
}
