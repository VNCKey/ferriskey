use crate::app::AppState;
use eframe::egui;
use crate::app::ui::*;

pub fn mostrar_teoria_modulos(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Module File Structure & Best Practices");
    session_intro(
        ui,
        "En Rust, el sistema de archivos no determina automáticamente la existencia de un módulo: es el código fuente en la raíz (main.rs o lib.rs) el que explícitamente construye el árbol de módulos mediante declaraciones 'mod nombre;'.",
    );
    ui.add_space(8.0);

    section_heading(ui, "Estructura de Archivos Recomendada (Rust 2018+)");

    highlighted_code_block(
        ui,
        "src/\n├── main.rs          # Raíz del crate binario (declara: mod red; mod auth;)\n├── red.rs           # Módulo 'red' (declara: pub mod http; pub mod tcp;)\n├── red/\n│   ├── http.rs      # Submódulo: crate::red::http\n│   └── tcp.rs       # Submódulo: crate::red::tcp\n└── auth.rs          # Módulo simple: crate::auth",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "rs",
    );
    ui.add_space(16.0);

    section_heading(ui, "Principios y Buenas Prácticas");

    card_overview(
        ui,
        "Estructura Moderna vs Tradicional (mod.rs)",
        "En Rust tradicional (2015), cada carpeta requería un archivo obligatorio llamado 'mod.rs'. En Rust moderno (2018, 2021 y 2024), la convención recomendada es colocar 'red.rs' junto a la carpeta 'red/'. Esto evita tener docenas de archivos llamados 'mod.rs' abiertos simultáneamente en tu editor de código.",
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "Principio de Menor Privilegio (Mínima Exposición)",
        "Mantén funciones, campos de structs y módulos privados por defecto. Expón únicamente con 'pub' los tipos que formen parte de tu contrato de API. Si varios módulos internos necesitan comunicarse sin exponerse al exterior, utiliza 'pub(crate)'.",
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "Fachada Limpia con 'pub use'",
        "No obligues a los usuarios de tu librería o a otras capas de tu aplicación a recordar rutas largas como 'use crate::infraestructura::repositorios::usuario::UsuarioRepository;'. Utiliza 'pub use' en 'src/lib.rs' para re-exportar una API compacta como 'use crate::UsuarioRepository;'.",
    );
    ui.add_space(16.0);

    codelab_cta_banner(ui);
}
