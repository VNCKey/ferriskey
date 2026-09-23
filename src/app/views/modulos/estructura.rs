use eframe::egui;
use crate::views::pilares::anatomy::codigo_inline_chip;

fn titulo_seccion(ui: &mut egui::Ui, titulo: &str, color: egui::Color32) {
    ui.heading(
        egui::RichText::new(titulo)
            .size(18.0)
            .strong()
            .color(color),
    );
    ui.add_space(8.0);
}

fn card_estructura(
    ui: &mut egui::Ui,
    titulo: &str,
    descripcion: &str,
    naranja: egui::Color32,
    texto: egui::Color32,
) {
    let mut card = egui::Frame::new();
    card.fill = egui::Color32::from_rgb(14, 18, 26);
    card.inner_margin = egui::Margin::same(12);
    card.corner_radius = egui::CornerRadius::same(8);
    card.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    card.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(
            egui::RichText::new(titulo)
                .size(15.0)
                .strong()
                .color(naranja),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new(descripcion)
                .size(13.0)
                .color(texto)
                .line_height(Some(19.0)),
        );
    });
}

fn diagrama_arbol_archivos(ui: &mut egui::Ui, texto_arbol: &str) {
    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(10, 14, 20);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(6);
    frame.stroke = egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(255, 160, 50, 80),
    );

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(
            egui::RichText::new(texto_arbol)
                .monospace()
                .size(13.0)
                .color(egui::Color32::from_rgb(100, 200, 255)),
        );
    });
}

pub fn mostrar_teoria_modulos(ui: &mut egui::Ui) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let texto = egui::Color32::from_rgb(205, 215, 230);

    ui.heading(
        egui::RichText::new("Module File Structure & Best Practices")
            .size(22.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new(
            "En Rust, el sistema de archivos no determina automáticamente la existencia de un módulo: es el código fuente en la raíz (main.rs o lib.rs) el que explícitamente construye el árbol de módulos mediante declaraciones 'mod nombre;'.",
        )
        .size(14.0)
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(16.0);

    titulo_seccion(ui, "Estructura de Archivos Recomendada (Rust 2018+)", naranja);

    diagrama_arbol_archivos(
        ui,
        "src/\n├── main.rs          # Raíz del crate binario (declara: mod red; mod auth;)\n├── red.rs           # Módulo 'red' (declara: pub mod http; pub mod tcp;)\n├── red/\n│   ├── http.rs      # Submódulo: crate::red::http\n│   └── tcp.rs       # Submódulo: crate::red::tcp\n└── auth.rs          # Módulo simple: crate::auth",
    );
    ui.add_space(16.0);

    titulo_seccion(ui, "Principios y Buenas Prácticas", naranja);

    card_estructura(
        ui,
        "1. Estructura Moderna vs Tradicional (mod.rs)",
        "En Rust tradicional (2015), cada carpeta requería un archivo obligatorio llamado 'mod.rs'. En Rust moderno (2018, 2021 y 2024), la convención recomendada es colocar 'red.rs' junto a la carpeta 'red/'. Esto evita tener docenas de archivos llamados 'mod.rs' abiertos simultáneamente en tu editor de código.",
        naranja,
        texto,
    );
    ui.add_space(8.0);

    card_estructura(
        ui,
        "2. Principio de Menor Privilegio (Mínima Exposición)",
        "Mantén funciones, campos de structs y módulos privados por defecto. Expón únicamente con 'pub' los tipos que formen parte de tu contrato de API. Si varios módulos internos necesitan comunicarse sin exponerse al exterior, utiliza 'pub(crate)'.",
        naranja,
        texto,
    );
    ui.add_space(8.0);

    card_estructura(
        ui,
        "3. Fachada Limpia con 'pub use'",
        "No obligues a los usuarios de tu librería o a otras capas de tu aplicación a recordar rutas largas como 'use crate::infraestructura::repositorios::usuario::UsuarioRepository;'. Utiliza 'pub use' en 'src/lib.rs' para re-exportar una API compacta como 'use crate::UsuarioRepository;'.",
        naranja,
        texto,
    );
    ui.add_space(16.0);

    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("👉").size(16.0));
        ui.label(
            egui::RichText::new(
                "Para poner en práctica todos estos conceptos con proyectos reales y compilación en vivo, dirígete a la pestaña",
            )
            .size(13.5)
            .color(texto),
        );
        codigo_inline_chip(ui, "Code Lab");
        ui.label(
            egui::RichText::new("en la esquina superior derecha.")
                .size(13.5)
                .color(texto),
        );
    });
}
