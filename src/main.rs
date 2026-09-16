mod app;
mod application;
mod domain;
mod errors;
mod infrastructure;
mod utils;

pub use app::components;
pub use app::content;
pub use app::routes;
pub use app::state;
pub use app::views;
pub use infrastructure::binary_analysis;
pub use infrastructure::execution;
pub use utils::platform;
pub use utils::svg_loader;

use app::AppState;
use eframe::egui;
use tracing_subscriber::EnvFilter;

fn main() -> eframe::Result {
    // Configura el registro de información, advertencias y errores.
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();

    tracing::info!("Iniciando FerrisKey Desktop");

    #[cfg(target_os = "linux")]
    {
        if let Err(err) = gtk::init() {
            tracing::warn!("No se pudo inicializar GTK: {err}");
        }
    }

    // Muestra los errores críticos en la terminal antes de delegarlos al hook original.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        eprintln!("\nError crítico en FerrisKey:");
        eprintln!("{panic_info}");
        eprintln!("Usa RUST_BACKTRACE=1 para ver la traza completa.\n");
        default_hook(panic_info);
    }));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("FerrisKey - El Ecosistema Interactivo de Rust"),
        ..Default::default()
    };

    eframe::run_native(
        "FerrisKey - El Ecosistema Interactivo de Rust",
        options,
        Box::new(|cc| {
            // Instala los loaders estándar de imágenes.
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // Instala el loader SVG con soporte para fuentes del sistema.
            svg_loader::SystemFontSvgLoader::install(&cc.egui_ctx);
            tracing::info!("Loaders de imágenes y SVG instalados");
            let mut app = AppState::default();
            app.restaurar_sesion();
            Ok(Box::new(app))
        }),
    )
}
