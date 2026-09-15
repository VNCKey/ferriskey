pub mod code_viewer;
pub mod comandos;
pub mod compiladores;
pub mod railroad;
pub mod salida_cargo;
pub mod settings;
pub mod terminal;

#[allow(unused_imports)]
pub use code_viewer::mostrar_modal_codigo;
#[allow(unused_imports)]
pub use comandos::mostrar_modal_comandos;
pub use compiladores::mostrar_modal_comparacion_compiladores;
pub use railroad::mostrar_modal_railroad_let;
pub use salida_cargo::mostrar_modal_salida_cargo;
pub use settings::mostrar_modal_settings;
pub use terminal::mostrar_modal_terminal;
