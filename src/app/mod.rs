pub mod components;
pub mod content;
pub mod routes;
pub mod state;
pub mod views;

use eframe::egui;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use crate::application::project_service::ProjectService;
use crate::components::modals::{
    mostrar_modal_comparacion_compiladores, mostrar_modal_railroad_let, mostrar_modal_salida_cargo,
    mostrar_modal_settings, mostrar_modal_terminal,
};
use crate::infrastructure::persistence::{SessionSnapshot, load_session, save_session};
use crate::routes::AppRoute;
use crate::state::{
    DashboardState, EditorState, LessonsState, ProjectState, TerminalState, UiState,
};
use crate::views::conceptos::{ejecutar_cargo_run_proyecto, mostrar_comenzando};
use crate::views::control_flujo::mostrar_tutorial_control_flujo;
use crate::views::dashboard::mostrar_graficos;
use crate::views::enums::mostrar_tutorial_enums;
use crate::views::errores::mostrar_tutorial_errores;
use crate::views::funciones::mostrar_tutorial_funciones;
use crate::views::genericos::mostrar_tutorial_genericos;
use crate::views::iteradores::mostrar_tutorial_iteradores;
use crate::views::landing::mostrar_landing_page;
use crate::views::lib::mostrar_referencia_tipos;
use crate::views::memoria::mostrar_tutorial_strings_ownership;
use crate::views::pilares::mostrar_tutorial_cargo;
use crate::views::playground::{mostrar_editor, mostrar_editor_nube};
use crate::views::structs::mostrar_tutorial_structs;
use crate::views::traits::mostrar_tutorial_traits;

#[derive(Default)]
pub struct AppState {
    pub ui: UiState,
    pub dashboard: DashboardState,
    pub project: ProjectState,
    pub editor: EditorState,
    pub terminal: TerminalState,
    pub lessons: LessonsState,
}

impl AppState {
    pub fn restaurar_sesion(&mut self) {
        let Ok(snapshot) = load_session() else {
            return;
        };
        let Some(project) = snapshot.selected_project else {
            return;
        };

        self.project.selected_project = Some(project);
        self.project.selected_file = snapshot.selected_file;
        self.cargar_archivo_proyecto_activo();
    }

    pub fn obtener_codigo_activo(&self) -> &str {
        if self.project.selected_project.is_some() {
            &self.project.shared_project_code
        } else {
            match self.ui.ruta_actual {
                AppRoute::TutorialControlFlujo => &self.lessons.controlflujo_code,
                AppRoute::TutorialTiposDatos => &self.lessons.datatypes_code,
                AppRoute::TutorialStrings
                | AppRoute::TutorialOwnership
                | AppRoute::TutorialMemoria => &self.lessons.ownership_code,
                AppRoute::TutorialFunciones => &self.lessons.funciones_code,
                AppRoute::TutorialIteradores => &self.lessons.iteradores_code,
                AppRoute::TutorialStructs => &self.lessons.structs_code,
                AppRoute::TutorialEnums => &self.lessons.enums_code,
                AppRoute::TutorialColecciones => &self.lessons.colecciones_code,
                AppRoute::TutorialErrores => &self.lessons.errores_code,
                AppRoute::TutorialTraits => &self.lessons.traits_code,
                AppRoute::TutorialGenericos => &self.lessons.genericos_code,
                AppRoute::TutorialModulos => &self.lessons.modulos_code,
                AppRoute::Playground => &self.lessons.playground_code,
                AppRoute::PlaygroundNube => &self.lessons.playground_nube_code,
                AppRoute::Comenzando => &self.lessons.conceptos_code,
                AppRoute::TutorialCargo | AppRoute::TutorialCompilacion => {
                    &self.lessons.estructura_code
                }
                _ => &self.lessons.conceptos_code,
            }
        }
    }

    pub fn obtener_editor_activo_mut(&mut self) -> (&mut String, Arc<Mutex<String>>) {
        if self.project.selected_project.is_some() {
            (
                &mut self.project.shared_project_code,
                Arc::clone(&self.project.shared_project_output),
            )
        } else {
            match self.ui.ruta_actual {
                AppRoute::TutorialControlFlujo => (
                    &mut self.lessons.controlflujo_code,
                    Arc::clone(&self.lessons.controlflujo_output),
                ),
                AppRoute::TutorialTiposDatos => (
                    &mut self.lessons.datatypes_code,
                    Arc::clone(&self.lessons.datatypes_output),
                ),
                AppRoute::TutorialStrings
                | AppRoute::TutorialOwnership
                | AppRoute::TutorialMemoria => (
                    &mut self.lessons.ownership_code,
                    Arc::clone(&self.lessons.ownership_output),
                ),
                AppRoute::TutorialFunciones => (
                    &mut self.lessons.funciones_code,
                    Arc::clone(&self.lessons.funciones_output),
                ),
                AppRoute::TutorialIteradores => (
                    &mut self.lessons.iteradores_code,
                    Arc::clone(&self.lessons.iteradores_output),
                ),
                AppRoute::TutorialStructs => (
                    &mut self.lessons.structs_code,
                    Arc::clone(&self.lessons.structs_output),
                ),
                AppRoute::TutorialEnums => (
                    &mut self.lessons.enums_code,
                    Arc::clone(&self.lessons.enums_output),
                ),
                AppRoute::TutorialColecciones => (
                    &mut self.lessons.colecciones_code,
                    Arc::clone(&self.lessons.colecciones_output),
                ),
                AppRoute::TutorialErrores => (
                    &mut self.lessons.errores_code,
                    Arc::clone(&self.lessons.errores_output),
                ),
                AppRoute::TutorialTraits => (
                    &mut self.lessons.traits_code,
                    Arc::clone(&self.lessons.traits_output),
                ),
                AppRoute::TutorialGenericos => (
                    &mut self.lessons.genericos_code,
                    Arc::clone(&self.lessons.genericos_output),
                ),
                AppRoute::TutorialModulos => (
                    &mut self.lessons.modulos_code,
                    Arc::clone(&self.lessons.modulos_output),
                ),
                AppRoute::Playground => (
                    &mut self.lessons.playground_code,
                    Arc::clone(&self.lessons.playground_output),
                ),
                AppRoute::PlaygroundNube => (
                    &mut self.lessons.playground_nube_code,
                    Arc::clone(&self.lessons.playground_nube_output),
                ),
                AppRoute::Comenzando => (
                    &mut self.lessons.conceptos_code,
                    Arc::clone(&self.lessons.conceptos_output),
                ),
                AppRoute::TutorialCargo | AppRoute::TutorialCompilacion => (
                    &mut self.lessons.estructura_code,
                    Arc::clone(&self.lessons.estructura_output),
                ),
                _ => (
                    &mut self.lessons.conceptos_code,
                    Arc::clone(&self.lessons.conceptos_output),
                ),
            }
        }
    }

    pub fn cargar_archivo_proyecto_activo(&mut self) {
        if let Some(ref proj) = self.project.selected_project.clone() {
            match ProjectService::load_selected_file(
                &self.terminal.term_cwd,
                proj,
                self.project.selected_file.as_deref(),
            ) {
                Ok((target_file, content)) => {
                    let relative = target_file
                        .strip_prefix(ProjectService::resolve_project_dir(
                            &self.terminal.term_cwd,
                            proj,
                        ))
                        .unwrap_or(&target_file)
                        .to_string_lossy()
                        .replace('\\', "/");
                    let (code_ref, _) = self.obtener_editor_activo_mut();
                    *code_ref = content;
                    self.editor.project_editor_path = Some(relative.clone());
                    self.editor.project_editor_status = format!("Cargado · {relative}");
                }
                Err(error) => {
                    self.editor.project_editor_status = error.to_string();
                }
            }
        }
    }

    pub fn sincronizar_proyecto_terminal(&mut self) {
        let pending = self
            .terminal
            .term_pending_project
            .lock()
            .ok()
            .and_then(|mut event| event.take());

        if let Some((project_dir, project_name, is_lib)) = pending {
            // La ruta entregada por el evento es la que Cargo acaba de crear.
            // Mantenerla directamente evita perder proyectos anidados cuando
            // la terminal ya estaba dentro de otro proyecto.
            self.terminal.term_cwd = project_dir.clone();
            self.project.selected_project = Some(project_name);
            self.project.selected_file = Some(if is_lib {
                "src/lib.rs".to_string()
            } else {
                "src/main.rs".to_string()
            });
            self.lessons.estructura_tab = if is_lib { 2 } else { 1 };

            let target_file = project_dir.join(if is_lib { "src/lib.rs" } else { "src/main.rs" });
            match ProjectService::read_file(&target_file) {
                Ok(content) => self.project.shared_project_code = content,
                Err(error) => {
                    self.project.shared_project_code.clear();
                    self.editor.project_editor_status = error.to_string();
                }
            }
        }

        let pending_manifest = self
            .terminal
            .term_pending_manifest
            .lock()
            .ok()
            .and_then(|mut manifest| manifest.take());

        if self.project.selected_file.as_deref() == Some("Cargo.toml")
            && let Some(manifest_path) = pending_manifest
        {
            match ProjectService::read_file(&manifest_path) {
                Ok(content) => self.project.shared_project_code = content,
                Err(error) => self.editor.project_editor_status = error.to_string(),
            }
        }
    }

    pub fn obtener_output_activo(&self) -> Arc<Mutex<String>> {
        if self.project.selected_project.is_some() {
            Arc::clone(&self.project.shared_project_output)
        } else {
            match self.ui.ruta_actual {
                AppRoute::TutorialControlFlujo => Arc::clone(&self.lessons.controlflujo_output),
                AppRoute::TutorialTiposDatos => Arc::clone(&self.lessons.datatypes_output),
                AppRoute::TutorialStrings
                | AppRoute::TutorialOwnership
                | AppRoute::TutorialMemoria => Arc::clone(&self.lessons.ownership_output),
                AppRoute::TutorialFunciones => Arc::clone(&self.lessons.funciones_output),
                AppRoute::TutorialIteradores => Arc::clone(&self.lessons.iteradores_output),
                AppRoute::TutorialStructs => Arc::clone(&self.lessons.structs_output),
                AppRoute::TutorialEnums => Arc::clone(&self.lessons.enums_output),
                AppRoute::TutorialColecciones => Arc::clone(&self.lessons.colecciones_output),
                AppRoute::TutorialErrores => Arc::clone(&self.lessons.errores_output),
                AppRoute::TutorialTraits => Arc::clone(&self.lessons.traits_output),
                AppRoute::TutorialGenericos => Arc::clone(&self.lessons.genericos_output),
                AppRoute::TutorialModulos => Arc::clone(&self.lessons.modulos_output),
                AppRoute::Playground => Arc::clone(&self.lessons.playground_output),
                AppRoute::PlaygroundNube => Arc::clone(&self.lessons.playground_nube_output),
                AppRoute::Comenzando => Arc::clone(&self.lessons.conceptos_output),
                AppRoute::TutorialCargo | AppRoute::TutorialCompilacion => {
                    Arc::clone(&self.lessons.estructura_output)
                }
                _ => Arc::clone(&self.lessons.conceptos_output),
            }
        }
    }

    pub fn guardar_proyecto_activo(&mut self) {
        let Some(proj) = self.project.selected_project.clone() else {
            return;
        };

        let content = self.project.shared_project_code.clone();
        match ProjectService::save_selected_file(
            &self.terminal.term_cwd,
            &proj,
            self.project.selected_file.as_deref(),
            &content,
        ) {
            Ok(target_file) => {
                let relative_file = target_file
                    .strip_prefix(ProjectService::resolve_project_dir(
                        &self.terminal.term_cwd,
                        &proj,
                    ))
                    .unwrap_or(&target_file)
                    .to_string_lossy()
                    .replace('\\', "/");
                self.editor.project_editor_path = Some(relative_file.clone());
                self.editor.project_editor_code = content;
                self.editor.project_editor_status = format!("Guardado · {relative_file}");
                let _ = save_session(&SessionSnapshot {
                    selected_project: self.project.selected_project.clone(),
                    selected_file: self.project.selected_file.clone(),
                });
            }
            Err(error) => {
                self.editor.project_editor_status = format!("No se pudo guardar: {error}");
            }
        }
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sincronizar_proyecto_terminal();

        // FORZAR desactivación GLOBAL de CUALQUIER modo debug (bordes rojos, etc.).
        // egui solo expone Style::debug en compilaciones Debug; mantener este
        // bloque condicionado permite generar el ejecutable Release.
        #[cfg(debug_assertions)]
        {
            ui.style_mut().debug.debug_on_hover = false;
            ui.style_mut().debug.show_expand_width = false;
            ui.style_mut().debug.show_expand_height = false;
            ui.style_mut().debug.show_resize = false;
            ui.style_mut().debug.show_interactive_widgets = false;
            ui.style_mut().debug.warn_if_rect_changes_id = false;
            for theme in [egui::Theme::Dark, egui::Theme::Light] {
                ui.ctx().style_mut_of(theme, |style| {
                    style.debug.debug_on_hover = false;
                    style.debug.show_expand_width = false;
                    style.debug.show_expand_height = false;
                    style.debug.show_resize = false;
                    style.debug.show_interactive_widgets = false;
                    style.debug.warn_if_rect_changes_id = false;
                });
            }
        }

        // Alternar pantalla completa con F11 o salir con la tecla Escape
        let is_fullscreen = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
        if ui.ctx().input(|i| i.key_pressed(egui::Key::F11)) {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Fullscreen(!is_fullscreen));
        } else if is_fullscreen && ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        }

        // Actualizar tiempo de animación (dt)
        self.ui.tutorial_time += ui.ctx().input(|i| i.stable_dt) as f64;

        // --- COLORES PRINCIPALES DEL ENTORNO ---
        // Aquí puedes "jugar" con los colores (R, G, B)
        let color_fondo_principal = egui::Color32::from_rgb(20, 22, 27); // Gris azulado oscuro (Main)
        let color_footer = egui::Color32::from_rgb(13, 15, 19); // Mismo color oscuro que la Barra Lateral

        // --- 0. STATUS BAR GLOBAL (FOOTER) ---
        if self.ui.ruta_actual != AppRoute::LandingPage {
            let mut is_expanded = self.ui.mostrar_status_bar;
            let status_bar_response = egui::Panel::bottom("status_bar")
                .frame(egui::Frame::default().fill(color_footer).inner_margin(4.0))
                .show_collapsible(ui, &mut is_expanded, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(5.0);

                        // Toggle Sidebar Izquierda
                        let img_sidebar = egui::Image::new(egui::include_image!(
                            "../../assets/icons/sidebar-left.svg"
                        ))
                        .fit_to_exact_size(egui::Vec2::new(16.0, 16.0));
                        let mut btn_sidebar = egui::Button::image(img_sidebar);
                        if self.ui.mostrar_sidebar {
                            btn_sidebar = btn_sidebar.fill(egui::Color32::from_rgb(50, 60, 80));
                        }
                        if ui
                            .add(btn_sidebar)
                            .on_hover_text("Mostrar/Ocultar Menú Lateral (Ctrl+B)")
                            .clicked()
                        {
                            self.ui.mostrar_sidebar = !self.ui.mostrar_sidebar;
                        }

                        // Toggle Header
                        let has_top_nav = self.ui.ruta_actual == AppRoute::Comenzando
                            || self.ui.ruta_actual == AppRoute::TutorialCargo;
                        if has_top_nav {
                            let img_top = egui::Image::new(egui::include_image!(
                                "../../assets/icons/layout-top.svg"
                            ))
                            .fit_to_exact_size(egui::Vec2::new(16.0, 16.0));
                            let mut btn_top = egui::Button::image(img_top);
                            if self.ui.mostrar_nav_superior {
                                btn_top = btn_top.fill(egui::Color32::from_rgb(50, 60, 80));
                            }
                            if ui
                                .add(btn_top)
                                .on_hover_text("Mostrar/Ocultar Header (Ctrl+H)")
                                .clicked()
                            {
                                self.ui.mostrar_nav_superior = !self.ui.mostrar_nav_superior;
                            }
                        }

                        // Elementos a la derecha (estilo VS Code / Zed)
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("FerrisKey v0.1")
                                    .small()
                                    .color(egui::Color32::from_rgb(130, 140, 155)),
                            );
                            ui.add_space(12.0);

                            if let Some(ref proj) = self.project.selected_project {
                                if let Some(ref file) = self.project.selected_file {
                                    ui.label(
                                        egui::RichText::new(file)
                                            .size(11.5)
                                            .color(egui::Color32::from_rgb(255, 180, 50)),
                                    );
                                    ui.label(
                                        egui::RichText::new("/")
                                            .size(11.5)
                                            .color(egui::Color32::from_rgb(70, 90, 120)),
                                    );
                                }
                                ui.label(
                                    egui::RichText::new(proj)
                                        .size(11.5)
                                        .color(egui::Color32::from_rgb(100, 200, 255)),
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new("Libre")
                                        .size(11.5)
                                        .color(egui::Color32::from_rgb(120, 145, 175)),
                                );
                            }
                        });
                    });
                });
            let status_bar_top = status_bar_response.map(|response| response.response.rect.min.y);
            ui.ctx().data_mut(|data| {
                data.insert_temp(egui::Id::new("ferriskey_status_bar_top"), status_bar_top);
            });
            self.ui.mostrar_status_bar = is_expanded;
        }

        // --- 1. PANEL DE NAVEGACIÓN (SIDEBAR - Solo si no está en LandingPage) ---
        if self.ui.ruta_actual != AppRoute::LandingPage {
            crate::components::sidebar::mostrar_sidebar(ui, self);
        }

        // --- 1.5 HEADER DE NAVEGACIÓN SUPERIOR ---
        match self.ui.ruta_actual {
            AppRoute::Comenzando => crate::views::conceptos::mostrar_nav_superior(ui, self),
            AppRoute::TutorialCargo => crate::views::pilares::mostrar_nav_superior(ui, self),
            AppRoute::TutorialOwnership | AppRoute::TutorialStrings | AppRoute::TutorialMemoria => {
                crate::views::memoria::mostrar_nav_superior(ui, self);
            }
            AppRoute::TutorialModulos => crate::views::modulos::mostrar_nav_superior(ui, self),
            AppRoute::TutorialTiposDatos | AppRoute::TutorialColecciones => {
                crate::views::tipos_compuestos::mostrar_nav_superior(ui, self);
            }
            AppRoute::TutorialControlFlujo => crate::views::control_flujo::mostrar_nav_superior(ui, self),
            AppRoute::TutorialFunciones => crate::views::funciones::mostrar_nav_superior(ui, self),
            AppRoute::TutorialIteradores => crate::views::iteradores::mostrar_nav_superior(ui, self),
            AppRoute::TutorialStructs => crate::views::structs::mostrar_nav_superior(ui, self),
            AppRoute::TutorialEnums => crate::views::enums::mostrar_nav_superior(ui, self),
            AppRoute::TutorialGenericos => crate::views::genericos::mostrar_nav_superior(ui, self),
            AppRoute::TutorialTraits => crate::views::traits::mostrar_nav_superior(ui, self),
            _ => {}
        }

        // --- 2. PANEL CENTRAL ---
        let is_code_lab = match self.ui.ruta_actual {
            AppRoute::TutorialCargo => self.lessons.pilares_step == 1,
            AppRoute::Comenzando => self.lessons.conceptos_tab == 0,
            AppRoute::TutorialOwnership | AppRoute::TutorialStrings | AppRoute::TutorialMemoria => {
                self.lessons.strings_ownership_tab == 5
            }
            AppRoute::TutorialModulos => self.lessons.modulos_tab == 5,
            AppRoute::TutorialTiposDatos | AppRoute::TutorialColecciones => {
                self.lessons.compuestos_tab == 5
            }
            AppRoute::TutorialControlFlujo => self.lessons.controlflujo_tab == 4,
            AppRoute::TutorialFunciones => self.lessons.funciones_tab == 3,
            AppRoute::TutorialIteradores => self.lessons.iteradores_tab == 4,
            AppRoute::TutorialStructs => self.lessons.structs_tab == 4,
            AppRoute::TutorialEnums => self.lessons.enums_tab == 4,
            AppRoute::TutorialGenericos => self.lessons.genericos_tab == 4,
            AppRoute::TutorialTraits => self.lessons.traits_tab == 3,
            _ => false,
        };

        let central_panel = if self.ui.ruta_actual == AppRoute::LandingPage {
            egui::CentralPanel::default().frame(egui::Frame::new().fill(egui::Color32::BLACK))
        } else if is_code_lab {
            egui::CentralPanel::default().frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(10, 14, 22))
                    .inner_margin(0.0),
            )
        } else {
            egui::CentralPanel::default().frame(
                egui::Frame::default()
                    .fill(color_fondo_principal)
                    .inner_margin(egui::Margin {
                        left: 20,
                        right: 20,
                        top: 16,
                        bottom: 24,
                    }),
            )
        };
        central_panel.show(ui, |ui| match self.ui.ruta_actual {
            AppRoute::LandingPage => mostrar_landing_page(ui, self),
            AppRoute::TutorialCargo => mostrar_tutorial_cargo(ui, self),
            AppRoute::Comenzando => {
                mostrar_comenzando(ui, self);
            }
            AppRoute::LibTiposDatos => mostrar_referencia_tipos(ui, self),
            AppRoute::TutorialCompilacion => {
                crate::views::pilares::pipeline::mostrar_tutorial_compilacion(ui, self)
            }
            AppRoute::TutorialTiposDatos => {
                crate::views::tipos_compuestos::mostrar_tutorial_tipos_compuestos(ui, self)
            }
            AppRoute::TutorialControlFlujo => mostrar_tutorial_control_flujo(ui, self),
            AppRoute::TutorialFunciones => mostrar_tutorial_funciones(ui, self),
            AppRoute::TutorialIteradores => mostrar_tutorial_iteradores(ui, self),
            AppRoute::TutorialOwnership | AppRoute::TutorialStrings | AppRoute::TutorialMemoria => {
                mostrar_tutorial_strings_ownership(ui, self)
            }
            AppRoute::TutorialModulos => crate::views::modulos::mostrar_tutorial_modulos(ui, self),
            AppRoute::TutorialStructs => mostrar_tutorial_structs(ui, self),
            AppRoute::TutorialEnums => mostrar_tutorial_enums(ui, self),
            AppRoute::TutorialColecciones => {
                crate::views::tipos_compuestos::mostrar_tutorial_tipos_compuestos(ui, self)
            }
            AppRoute::TutorialErrores => mostrar_tutorial_errores(ui, self),
            AppRoute::TutorialTraits => mostrar_tutorial_traits(ui, self),
            AppRoute::TutorialGenericos => mostrar_tutorial_genericos(ui, self),
            AppRoute::DashboardGraficos => mostrar_graficos(ui, self),
            AppRoute::Playground => mostrar_editor(ui, self),
            AppRoute::PlaygroundNube => mostrar_editor_nube(ui, self),
        });

        // --- PROCESAR ATAJOS DE TECLADO GLOBALES ---
        let mut ejecutar_cargo_run_flag = false;
        ui.ctx().input_mut(|i| {
            // Filtrar eventos brutos de Escape para evitar que eframe o el viewport soliciten cambios de pantalla completa
            i.events.retain(|event| {
                !matches!(
                    event,
                    egui::Event::Key {
                        key: egui::Key::Escape,
                        pressed: true,
                        ..
                    }
                )
            });

            // Ctrl + B -> Toggle Sidebar Izquierdo
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::B,
            )) {
                self.ui.mostrar_sidebar = !self.ui.mostrar_sidebar;
            }

            // Ctrl + H -> Toggle Top Nav
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::H,
            )) {
                self.ui.mostrar_nav_superior = !self.ui.mostrar_nav_superior;
            }

            // Ctrl + J -> Toggle Status Bar (Footer)
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::J,
            )) {
                self.ui.mostrar_status_bar = !self.ui.mostrar_status_bar;
            }

            // Ctrl + T -> Toggle Terminal
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::T,
            )) {
                let abrir_terminal = !self.ui.show_terminal_modal;
                self.ui.show_terminal_modal = abrir_terminal;
            }
            // Ctrl + I -> Toggle Info Output Modal
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::I,
            )) {
                let is_open = self.ui.show_cargo_output_modal.load(Ordering::Relaxed);
                self.ui
                    .show_cargo_output_modal
                    .store(!is_open, Ordering::Relaxed);
            }
            // Esc o Ctrl + W -> Cerrar ventanas modales flotantes sin alterar el modo pantalla completa
            let esc_pressed = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::Escape,
            ));
            let ctrl_w_pressed = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::W,
            ));
            if esc_pressed || ctrl_w_pressed {
                self.ui.show_terminal_modal = false;
                self.ui
                    .show_cargo_output_modal
                    .store(false, Ordering::Relaxed);
                self.ui.show_settings_modal = false;
                self.ui.show_railroad_modal = None;
                self.project.created_project_name = None;
            }
            // Ctrl + S -> Guardar proyecto activo
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::S,
            )) {
                self.guardar_proyecto_activo();
            }
            // Ctrl + Enter o F5 -> Ejecutar cargo run en segundo plano y abrir modal centrado
            let ctrl_enter = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::Enter,
            ));
            let f5_pressed = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::F5,
            ));
            if ctrl_enter || f5_pressed {
                ejecutar_cargo_run_flag = true;
            }
        });

        if ejecutar_cargo_run_flag {
            self.guardar_proyecto_activo();
            ejecutar_cargo_run_proyecto(self, ui.ctx());
        }

        mostrar_modal_comparacion_compiladores(ui.ctx(), self);
        mostrar_modal_salida_cargo(ui.ctx(), self);
        mostrar_modal_terminal(ui.ctx(), self);
        mostrar_modal_settings(ui.ctx(), self);
        mostrar_modal_railroad_let(ui.ctx(), self);
        crate::components::modals::mostrar_modal_codigo(ui.ctx(), self);

        // Request continuous repaint for animations
        ui.ctx().request_repaint();
    }
}
