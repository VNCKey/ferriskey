use eframe::egui;

use crate::app::AppState;
use crate::components::educational_table::mostrar_tabla_educativa;

pub fn mostrar_project_anatomy(ui: &mut egui::Ui, _state: &mut AppState) {
    let orange = egui::Color32::from_rgb(255, 180, 80);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let text = egui::Color32::from_rgb(205, 215, 230);

    egui::ScrollArea::vertical().show(ui, |ui| {
        // --- ENCABEZADO PRINCIPAL ---
        ui.heading(
            egui::RichText::new("Anatomía de un Proyecto Cargo & Toolchains")
                .size(22.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(
                "Guía completa sobre la estructura de archivos, manifiestos, crates binarias/librerías y configuración de toolchains con Rustup.",
            )
            .size(15.0)
            .color(text),
        );
        ui.add_space(20.0);

        // ==========================================
        // SECCIÓN 1: PROJECT SETUP & VERIFICACIÓN
        // ==========================================
        ui.heading(egui::RichText::new("1. Project Setup").size(18.0).strong().color(orange));
        ui.add_space(6.0);
        ui.label("Comprueba que Rust, Cargo y el toolchain estén disponibles correctamente en el sistema:");
        ui.add_space(8.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_environment_commands", |ui| {
            ui.label(egui::RichText::new("Comando").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Qué permite comprobar").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (command, purpose) in [
                ("rustc --version", "Versión del compilador de Rust"),
                ("cargo --version", "Versión del gestor de paquetes Cargo disponible"),
            ] {
                ui.label(egui::RichText::new(command).monospace().color(cyan));
                ui.label(purpose);
                ui.end_row();
            }
        });

        ui.add_space(24.0);
        ui.separator();
        ui.add_space(20.0);

        // ==========================================
        // SECCIÓN 2: BINARY CRATE VS LIBRARY CRATE
        // ==========================================
        ui.heading(egui::RichText::new("2. Binary Crate vs Library Crate").size(18.0).strong().color(orange));
        ui.add_space(6.0);
        ui.label("En Rust, los paquetes pueden compilarse como ejecutables independientes o como librerías reutilizables:");
        ui.add_space(8.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_binary_library_grid", |ui| {
            ui.label(egui::RichText::new("Tipo").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Creación").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Archivo principal").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Propósito").strong().color(egui::Color32::WHITE));
            ui.end_row();

            ui.label(egui::RichText::new("Binario").strong().color(orange));
            ui.label(egui::RichText::new("cargo new mi_programa").monospace().color(cyan));
            ui.label(egui::RichText::new("src/main.rs").monospace().color(egui::Color32::WHITE));
            ui.label("Crear un programa ejecutable con punto de entrada fn main()");
            ui.end_row();

            ui.label(egui::RichText::new("Librería").strong().color(orange));
            ui.label(egui::RichText::new("cargo new mi_libreria --lib").monospace().color(cyan));
            ui.label(egui::RichText::new("src/lib.rs").monospace().color(egui::Color32::WHITE));
            ui.label("Compartir funciones, tipos, structs y módulos para otros proyectos");
            ui.end_row();
        });
        ui.add_space(6.0);
        ui.label(egui::RichText::new("Nota: Un paquete puede contener una librería (src/lib.rs) y uno o varios binarios (src/bin/*.rs o src/main.rs).").color(egui::Color32::from_rgb(170, 185, 205)));

        ui.add_space(24.0);
        ui.separator();
        ui.add_space(20.0);

        // ==========================================
        // SECCIÓN 3: ESTRUCTURA DE ARCHIVOS DE CARGO
        // ==========================================
        ui.heading(egui::RichText::new("3. Estructura y Archivos del Proyecto").size(18.0).strong().color(orange));
        ui.add_space(6.0);
        ui.label("Convenciones de archivos y carpetas que Cargo reconoce automáticamente:");
        ui.add_space(8.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_file_roles", |ui| {
            ui.label(egui::RichText::new("Archivo / Carpeta").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Función y Rol en el Proyecto").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (file, role) in [
                ("Cargo.toml", "Manifiesto del proyecto: metadatos, dependencias, edición y perfiles."),
                ("Cargo.lock", "Guarda el árbol exacto y determinista de versiones resueltas por Cargo."),
                ("src/main.rs", "Punto de entrada principal para un proyecto binario (ejecutable)."),
                ("src/lib.rs", "Punto de entrada principal para una librería compartible."),
                ("src/bin/*.rs", "Binarios adicionales independientes dentro del mismo paquete."),
                ("tests/*.rs", "Tests de integración (prueban la librería como un usuario externo)."),
                ("examples/*.rs", "Ejemplos prácticos ejecutables con 'cargo run --example <nombre>'."),
                ("benches/*.rs", "Pruebas de rendimiento y benchmarks para optimización."),
                ("build.rs", "Script de compilación ejecutado antes de compilar la crate."),
                ("target/", "Carpeta generada por Cargo con los artefactos de compilación."),
            ] {
                ui.label(egui::RichText::new(file).monospace().color(cyan));
                ui.label(role);
                ui.end_row();
            }
        });

        ui.add_space(16.0);
        ui.label(egui::RichText::new("Metadatos clave de [package] en Cargo.toml").strong().color(orange));
        ui.add_space(6.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_cargo_package_metadata", |ui| {
            ui.label(egui::RichText::new("Campo").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Qué representa").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (field, example, purpose) in [
                ("name", "\"mi_proyecto\"", "Nombre único del paquete y de la crate principal"),
                ("version", "\"0.1.0\"", "Versión publicada del paquete según SemVer"),
                ("edition", "\"2024\"", "Edición de las reglas del lenguaje (2015, 2018, 2021, 2024)"),
                ("rust-version", "\"1.85\"", "Versión mínima del compilador compatible (MSRV)"),
                ("description", "\"Mi proyecto Rust\"", "Descripción breve del propósito del paquete"),
                ("license", "\"MIT OR Apache-2.0\"", "Licencia de código abierto con la que se distribuye"),
                ("repository", "\"https://...\"", "URL del repositorio de código fuente"),
            ] {
                ui.label(egui::RichText::new(field).monospace().color(cyan));
                ui.label(egui::RichText::new(example).monospace().color(egui::Color32::WHITE));
                ui.label(purpose);
                ui.end_row();
            }
        });

        ui.add_space(16.0);
        ui.label(egui::RichText::new("Anatomía del directorio target/").strong().color(orange));
        ui.add_space(6.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_target_directory", |ui| {
            ui.label(egui::RichText::new("Subcarpeta").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Contenido").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (element, purpose) in [
                ("debug/", "Binarios y artefactos sin optimizar con símbolos de depuración."),
                ("release/", "Binarios altamente optimizados generados con cargo build --release."),
                ("deps/", "Dependencias externas ya compiladas para enlace rápido."),
                ("incremental/", "Caché de compilación incremental para acelerar recompilaciones."),
                ("build/", "Archivos temporales generados por scripts build.rs."),
            ] {
                ui.label(egui::RichText::new(element).monospace().color(cyan));
                ui.label(purpose);
                ui.end_row();
            }
        });
        ui.add_space(6.0);
        ui.label(egui::RichText::new("Tip: 'target/' se ignora en git (.gitignore) y se puede limpiar por completo con 'cargo clean'.").color(egui::Color32::from_rgb(170, 185, 205)));

        ui.add_space(24.0);
        ui.separator();
        ui.add_space(20.0);

        // ==========================================
        // SECCIÓN 4: TOOLCHAIN DEL PROYECTO & RUSTUP
        // ==========================================
        ui.heading(egui::RichText::new("4. Configuración del Toolchain (rust-toolchain.toml)").size(18.0).strong().color(orange));
        ui.add_space(6.0);
        ui.label("Un proyecto puede declarar el canal y los componentes exactos que necesita en un archivo rust-toolchain.toml:");
        ui.add_space(8.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_toolchain_fields", |ui| {
            ui.label(egui::RichText::new("Campo").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Qué permite configurar").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (field, example, purpose) in [
                ("channel", "\"stable\"", "Canal o versión exacta de Rust que usará el proyecto"),
                ("components", "[\"rustfmt\", \"clippy\"]", "Herramientas adicionales que rustup debe asegurar"),
                ("targets", "[\"wasm32-unknown-unknown\"]", "Arquitecturas objetivo adicionales para compilación cruzada"),
                ("profile", "\"minimal\"", "Perfil de instalación: minimal, default o complete"),
            ] {
                ui.label(egui::RichText::new(field).monospace().color(cyan));
                ui.label(egui::RichText::new(example).monospace().color(egui::Color32::WHITE));
                ui.label(purpose);
                ui.end_row();
            }
        });

        ui.add_space(16.0);
        ui.label(egui::RichText::new("Comandos esenciales de rustup").strong().color(orange));
        ui.add_space(6.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_rustup_commands", |ui| {
            ui.label(egui::RichText::new("Comando").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Función").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (command, purpose) in [
                ("rustup show", "Muestra el toolchain activo y los componentes instalados."),
                ("rustup toolchain list", "Lista todos los toolchains instalados en la máquina."),
                ("rustup default stable", "Establece el canal stable como predeterminado global."),
                ("rustup override set nightly", "Fuerza el uso de nightly exclusivamente en el proyecto actual."),
                ("rustup update", "Actualiza todos los toolchains instalados a la última versión."),
            ] {
                ui.label(egui::RichText::new(command).monospace().color(cyan));
                ui.label(purpose);
                ui.end_row();
            }
        });

        ui.add_space(16.0);
        ui.label(egui::RichText::new("Canales de Lanzamiento de Rust").strong().color(orange));
        ui.add_space(6.0);

        mostrar_tabla_educativa(ui, "theory_anatomy_rust_channels", |ui| {
            ui.label(egui::RichText::new("Canal").strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("Uso y Características").strong().color(egui::Color32::WHITE));
            ui.end_row();

            for (channel, purpose) in [
                ("stable", "Versión recomendada y garantizada sin cambios incompatibles para producción."),
                ("beta", "Próxima versión en período de pruebas para estabilización."),
                ("nightly", "Compilación diaria con características experimentales (#![feature(...)])."),
            ] {
                ui.label(egui::RichText::new(channel).monospace().strong().color(orange));
                ui.label(purpose);
                ui.end_row();
            }
        });

        ui.add_space(30.0);
    });
}
