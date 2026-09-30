use crate::app::ui::*;
use crate::app::AppState;
use eframe::egui;

/// Ruta informativa para mostrar el siguiente recorrido de aprendizaje sin
/// mezclar contenidos avanzados con las sesiones fundamentales.
pub fn mostrar_proximamente(ui: &mut egui::Ui, _state: &mut AppState) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            session_title(ui, "Próximamente");
            session_intro(
                ui,
                "FerrisKey continuará creciendo después del curso principal. Estas son las rutas que estamos preparando para conectar Rust con patrones, Tokio y el desarrollo backend.",
            );

            ui.add_space(18.0);
            section_heading(ui, "Siguiente recorrido");
            texto_con_chips_inline(
                ui,
                "Primero reforzaremos las formas idiomáticas de Rust. Después aplicaremos esos conocimientos en concurrencia, APIs, bases de datos y servicios preparados para producción.",
                Colors::TEXT_PRIMARY,
                Colors::CYAN_ACCENT,
            );

            ui.add_space(18.0);
            ui.columns(2, |columns| {
                card_overview(
                    &mut columns[0],
                    "Rust Idioms & Conversions",
                    "From, Into, TryFrom, TryInto, AsRef, Borrow, Deref, mem::take y decisiones idiomáticas para diseñar APIs claras.",
                );
                columns[0].add_space(8.0);
                card_overview(
                    &mut columns[0],
                    "Patterns & Design",
                    "Newtype, Builder, RAII Guards, Command, State, Strategy, Observer, Facade y Repository, siempre después de conocer sus fundamentos.",
                );

                card_overview(
                    &mut columns[1],
                    "Tokio & Async avanzado",
                    "JoinHandle, JoinSet, canales, timeouts, Semaphore, cancelación, backpressure, Worker Pool y graceful shutdown.",
                );
                columns[1].add_space(8.0);
                card_overview(
                    &mut columns[1],
                    "Backend con Axum",
                    "Routers, handlers, extractors, State, errores, middleware, validación y diseño de APIs HTTP.",
                );
            });

            ui.add_space(18.0);
            section_heading(ui, "Después");
            ui.columns(2, |columns| {
                card_overview(
                    &mut columns[0],
                    "Datos y servicios",
                    "Serde, JSON, reqwest, SQLx, SQLite, PostgreSQL, migraciones, transacciones y Repository Pattern.",
                );
                card_overview(
                    &mut columns[1],
                    "Producción y seguridad",
                    "Testing asíncrono, tracing, configuración, autenticación, autorización, retries, observabilidad y despliegue.",
                );
            });

            ui.add_space(18.0);
            section_heading(ui, "Una ruta, no una avalancha");
            texto_con_chips_inline(
                ui,
                "Estos temas se publicarán progresivamente. FerrisKey mantendrá separadas las bases de Rust, los patrones y el backend para que cada concepto aparezca cuando el estudiante ya tenga las herramientas necesarias para entenderlo.",
                Colors::TEXT_PRIMARY,
                Colors::CYAN_ACCENT,
            );
            ui.add_space(20.0);
        });
}
