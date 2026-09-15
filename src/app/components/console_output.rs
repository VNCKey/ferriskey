use eframe::egui;

pub fn formatear_salida_consola(text: &str, _es_error: bool) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font_id = egui::FontId::monospace(14.0);

    let color_error = egui::Color32::from_rgb(255, 90, 90);
    let color_warning = egui::Color32::from_rgb(255, 200, 50);
    let color_note = egui::Color32::from_rgb(100, 200, 255);
    let color_help = egui::Color32::from_rgb(100, 255, 150);
    let color_normal = egui::Color32::from_rgb(220, 225, 235);

    let mut current_context_warning = false;

    for line in text.split('\n') {
        let trimmed = line.trim_start();

        if trimmed.starts_with("warning:") || trimmed.starts_with("warning") {
            current_context_warning = true;
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_warning),
            );
        } else if trimmed.starts_with("error:") || trimmed.starts_with("error") {
            current_context_warning = false;
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_error),
            );
        } else if trimmed.starts_with("note:") || trimmed.starts_with("= note:") {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_note),
            );
        } else if trimmed.starts_with("help:") || trimmed.starts_with("= help:") {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_help),
            );
        } else if trimmed.starts_with("-->") {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_note),
            );
        } else if line.contains('|') {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            if parts.len() == 2 {
                job.append(
                    parts[0],
                    0.0,
                    egui::TextFormat::simple(font_id.clone(), color_note),
                );
                job.append(
                    "|",
                    0.0,
                    egui::TextFormat::simple(font_id.clone(), color_note),
                );

                let code_part = parts[1];
                if code_part.contains("help:") {
                    if let Some(idx) = code_part.find("help:") {
                        let (prefix, help_str) = code_part.split_at(idx);
                        let prefix_color = if current_context_warning {
                            color_warning
                        } else {
                            color_error
                        };
                        job.append(
                            prefix,
                            0.0,
                            egui::TextFormat::simple(font_id.clone(), prefix_color),
                        );
                        job.append(
                            help_str,
                            0.0,
                            egui::TextFormat::simple(font_id.clone(), color_help),
                        );
                    } else {
                        job.append(
                            code_part,
                            0.0,
                            egui::TextFormat::simple(font_id.clone(), color_help),
                        );
                    }
                } else if code_part.contains('^') || code_part.contains('~') {
                    let indicator_color = if current_context_warning {
                        color_warning
                    } else {
                        color_error
                    };
                    job.append(
                        code_part,
                        0.0,
                        egui::TextFormat::simple(font_id.clone(), indicator_color),
                    );
                } else {
                    job.append(
                        code_part,
                        0.0,
                        egui::TextFormat::simple(font_id.clone(), color_normal),
                    );
                }
            } else {
                job.append(
                    line,
                    0.0,
                    egui::TextFormat::simple(font_id.clone(), color_normal),
                );
            }
        } else {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_normal),
            );
        }
        job.append(
            "\n",
            0.0,
            egui::TextFormat::simple(font_id.clone(), color_normal),
        );
    }

    job
}
