use super::comandos::COMANDOS_TALLER;
use crate::app::AppState;
use eframe::egui;

pub fn mostrar_modal_settings(ctx: &egui::Context, state: &mut AppState) {
    let mut abierto = state.ui.show_settings_modal;
    if !abierto {
        return;
    }

    egui::Window::new("⚙️ Centro de Control y Referencia")
        .open(&mut abierto)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .default_size([680.0, 420.0])
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            // Header estilo VS Code / Zed Settings Hub
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut state.ui.settings_tab,
                    0,
                    egui::RichText::new("⌨️ Atajos de Teclado").strong().size(15.0),
                );
                ui.add_space(12.0);
                ui.selectable_value(
                    &mut state.ui.settings_tab,
                    1,
                    egui::RichText::new("📦 Comandos de Cargo").strong().size(15.0),
                );
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(10.0);

            match state.ui.settings_tab {
                0 => {
                    ui.label("Combinaciones de teclas globales habilitadas en toda la aplicación:");
                    ui.add_space(10.0);

                    let mut table_frame = egui::Frame::new();
                    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                    table_frame.inner_margin = egui::Margin::same(12);
                    table_frame.corner_radius = egui::CornerRadius::same(8);
                    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                    table_frame.show(ui, |ui| {
                        egui::Grid::new("tabla_atajos_teclado_grid")
                            .striped(true)
                            .spacing([25.0, 10.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Atajo").strong().color(egui::Color32::WHITE));
                                ui.label(egui::RichText::new("Acción Principal").strong().color(egui::Color32::WHITE));
                                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                                ui.end_row();

                                // Ctrl + T
                                ui.label(egui::RichText::new("Ctrl + T").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("💻 Terminal").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                                ui.label("Abrir / Cerrar la terminal flotante del sistema.");
                                ui.end_row();

                                // Ctrl + I
                                ui.label(egui::RichText::new("Ctrl + I").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("ℹ️ Info (Salida)").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                                ui.label("Abrir / Cerrar la ventana de salida de Cargo.");
                                ui.end_row();

                                // Ctrl + S
                                ui.label(egui::RichText::new("Ctrl + S").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("💾 Guardar Proyecto").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                                ui.label("Guardar cambios del archivo de proyecto activo.");
                                ui.end_row();

                                // Esc / Ctrl + W
                                ui.label(egui::RichText::new("Esc / Ctrl + W").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("❌ Cerrar Modales").strong().color(egui::Color32::from_rgb(180, 190, 205)));
                                ui.label("Cerrar todas las ventanas flotantes activas.");
                                ui.end_row();
                            });
                    });
                }
                _ => {
                    ui.label("Guía de referencia rápida de comandos de compilación y herramientas de Cargo:");
                    ui.add_space(10.0);

                    let mut table_frame = egui::Frame::new();
                    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                    table_frame.inner_margin = egui::Margin::same(12);
                    table_frame.corner_radius = egui::CornerRadius::same(8);
                    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                    let mut comando_elegido = None;

                    table_frame.show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(260.0)
                            .show(ui, |ui| {
                                egui::Grid::new("tabla_comandos_cargo_settings")
                                    .striped(true)
                                    .spacing([20.0, 10.0])
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("Comando").strong().color(egui::Color32::WHITE));
                                        ui.label(egui::RichText::new("Propósito / Descripción").strong().color(egui::Color32::WHITE));
                                        ui.label(egui::RichText::new("Acción").strong().color(egui::Color32::WHITE));
                                        ui.end_row();

                                        for (comando, descripcion) in COMANDOS_TALLER {
                                            ui.label(
                                                egui::RichText::new(*comando)
                                                    .monospace()
                                                    .strong()
                                                    .color(egui::Color32::from_rgb(255, 160, 50)),
                                            );
                                            ui.label(
                                                egui::RichText::new(*descripcion)
                                                    .color(egui::Color32::from_rgb(180, 190, 205)),
                                            );
                                            if ui.button(egui::RichText::new("▶ Usar").small().color(egui::Color32::from_rgb(100, 200, 255))).clicked() {
                                                comando_elegido = Some(*comando);
                                            }
                                            ui.end_row();
                                        }
                                    });
                            });
                    });

                    if let Some(cmd) = comando_elegido {
                        state.terminal.term_input = cmd.to_owned();
                        state.ui.show_terminal_modal = true;
                    }
                }
            }
        });

    state.ui.show_settings_modal = abierto;
}
