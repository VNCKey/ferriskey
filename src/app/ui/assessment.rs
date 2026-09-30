use eframe::egui::{self, RichText};

use super::{inline_code_chip, inline_code_chip_color};

/// Pregunta breve para las evaluaciones de las sesiones.
#[derive(Clone, Copy)]
pub struct SessionQuestion {
    pub prompt: &'static str,
    pub options: [&'static str; 3],
    pub correct: usize,
}

/// Renderiza una evaluación con el mismo lenguaje visual en todas las sesiones.
pub fn mostrar_evaluacion(
    ui: &mut egui::Ui,
    answers: &mut [Option<usize>; 5],
    questions: &[SessionQuestion; 5],
    tags: &[&str],
    introduction: &str,
    orange: egui::Color32,
) {
    let text_col = egui::Color32::from_rgb(205, 215, 230);

    ui.add_space(4.0);
    ui.heading(
        RichText::new("5. Questions")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        inline_code_chip_color(ui, "Evaluation", egui::Color32::from_rgb(0, 200, 120));
        for tag in tags {
            ui.add_space(4.0);
            inline_code_chip(ui, tag);
        }
    });
    ui.add_space(12.0);
    ui.label(
        RichText::new(introduction)
            .size(13.5)
            .color(text_col)
            .line_height(Some(19.0)),
    );
    ui.add_space(14.0);
    crate::views::pilares::anatomy::titulo_seccion(ui, "Comprueba lo aprendido", orange);
    ui.add_space(8.0);

    for (index, question) in questions.iter().enumerate() {
        ui.label(
            RichText::new(question.prompt)
                .strong()
                .size(13.0)
                .color(text_col),
        );
        ui.add_space(4.0);

        for (option_index, option) in question.options.iter().enumerate() {
            let selected = answers[index] == Some(option_index);
            let (rect, response) = ui
                .allocate_exact_size(egui::vec2(ui.available_width(), 25.0), egui::Sense::click());
            let fill = if selected {
                egui::Color32::from_rgba_unmultiplied(255, 160, 50, 32)
            } else if response.hovered() {
                egui::Color32::from_rgba_unmultiplied(255, 160, 50, 18)
            } else {
                egui::Color32::from_rgb(15, 20, 30)
            };
            let stroke = if selected {
                orange
            } else if response.hovered() {
                egui::Color32::from_rgba_unmultiplied(255, 180, 80, 150)
            } else {
                egui::Color32::from_rgb(35, 48, 70)
            };
            ui.painter().rect(
                rect,
                egui::CornerRadius::same(4),
                fill,
                egui::Stroke::new(if selected { 1.2 } else { 1.0 }, stroke),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                egui::pos2(rect.left() + 10.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                *option,
                egui::FontId::proportional(12.5),
                if selected { orange } else { text_col },
            );
            if response.clicked() {
                answers[index] = Some(option_index);
            }
            ui.add_space(3.0);
        }

        if let Some(answer) = answers[index] {
            let correct = answer == question.correct;
            ui.label(
                RichText::new(if correct {
                    "Correcto"
                } else {
                    "Revisa este concepto y vuelve a intentarlo"
                })
                .size(11.5)
                .color(if correct { orange } else { text_col }),
            );
        }
        ui.add_space(10.0);
    }

    let answered = answers.iter().filter(|answer| answer.is_some()).count();
    let correct = questions
        .iter()
        .enumerate()
        .filter(|(index, question)| answers[*index] == Some(question.correct))
        .count();
    let completed = answered == questions.len() && correct == questions.len();

    crate::views::pilares::anatomy::titulo_seccion(
        ui,
        if completed {
            "Sesión completada"
        } else {
            "Progreso de la evaluación"
        },
        orange,
    );
    ui.label(
        RichText::new(if completed {
            "¡Excelente! Has respondido correctamente las cinco preguntas.".to_owned()
        } else {
            format!("Respuestas correctas: {correct}/5 · Respondidas: {answered}/5")
        })
        .size(13.0)
        .color(if completed { orange } else { text_col }),
    );
}
