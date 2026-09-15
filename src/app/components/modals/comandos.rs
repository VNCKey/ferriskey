use crate::app::AppState;
use eframe::egui;

pub const COMANDOS_TALLER: &[(&str, &str)] = &[
    (
        "rustup show",
        "Muestra el toolchain activo y los destinos instalados.",
    ),
    ("rustc --version", "Comprueba la versión del compilador."),
    ("cargo --version", "Comprueba la versión de Cargo."),
    ("cargo new mi_proyecto", "Crea un nuevo proyecto binario."),
    (
        "cargo check",
        "Comprueba el código rápidamente sin generar el ejecutable final.",
    ),
    ("cargo run", "Compila y ejecuta el programa."),
    (
        "cargo build",
        "Construye el proyecto y deja el resultado en target/.",
    ),
    ("cargo test", "Compila y ejecuta las pruebas."),
    ("cargo fmt", "Aplica el formato estándar de Rust."),
    (
        "cargo clippy",
        "Analiza el código y propone mejoras idiomáticas.",
    ),
    (
        "cargo doc --open",
        "Genera la documentación y la abre en el navegador.",
    ),
    (
        "cargo expand",
        "Expande macros; requiere instalar cargo-expand.",
    ),
];

#[allow(dead_code)]
pub fn mostrar_modal_comandos(ctx: &egui::Context, state: &mut AppState) {
    let mut abierto = state.ui.show_commands_modal;
    let mut comando_elegido = None;
    egui::Window::new("⌨ Referencia rápida de comandos")
        .open(&mut abierto)
        .collapsible(false)
        .resizable(true)
        .default_width(650.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height(460.0).show(ui, |ui| {
                egui::Grid::new("comandos_taller_grid")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        for (comando, descripcion) in COMANDOS_TALLER {
                            ui.code(*comando);
                            ui.label(*descripcion);
                            if ui.small_button("Usar").clicked() {
                                comando_elegido = Some(*comando);
                            }
                            ui.end_row();
                        }
                    });
            });
            ui.separator();
            ui.small("cargo expand no forma parte de Cargo: se instala con `cargo install cargo-expand`.");
        });
    state.ui.show_commands_modal = abierto;
    if let Some(comando) = comando_elegido {
        state.terminal.term_input = comando.to_owned();
    }
}
