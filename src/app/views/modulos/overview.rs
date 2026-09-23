use eframe::egui;
use crate::views::pilares::anatomy::codigo_inline_chip;

fn titulo_overview(ui: &mut egui::Ui, titulo: &str, color: egui::Color32) {
    ui.heading(
        egui::RichText::new(titulo)
            .size(18.0)
            .strong()
            .color(color),
    );
    ui.add_space(8.0);
}

fn card_overview(
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

pub fn mostrar(ui: &mut egui::Ui) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let texto = egui::Color32::from_rgb(205, 215, 230);

    ui.heading(
        egui::RichText::new("Modules & Visibility")
            .size(24.0)
            .strong()
            .color(naranja),
    );
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new(
            "Esta sesión explica cómo estructurar y organizar proyectos en Rust mediante árboles de módulos, cómo controlar la visibilidad para proteger la privacidad de los datos, y cómo diseñar interfaces públicas limpias utilizando re-exportaciones.",
        )
        .size(14.5)
        .color(texto)
        .line_height(Some(21.0)),
    );
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new(
            "El Code Lab contiene los ejemplos, ejercicios y retos prácticos interactivos de cada tema. Este Overview funciona como el mapa conceptual general de la sesión.",
        )
        .size(13.5)
        .color(texto)
        .line_height(Some(20.0)),
    );
    ui.add_space(18.0);

    titulo_overview(ui, "Recorrido de la sesión", naranja);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Primero aprenderemos a declarar módulos con").size(13.5).color(texto));
        codigo_inline_chip(ui, "mod");
        ui.label(egui::RichText::new("para construir el árbol jerárquico del crate. Después veremos las reglas de privacidad por defecto y cómo usar").size(13.5).color(texto));
        codigo_inline_chip(ui, "pub");
        ui.label(egui::RichText::new("y").size(13.5).color(texto));
        codigo_inline_chip(ui, "pub(crate)");
        ui.label(egui::RichText::new("para controlar el acceso seguro a funciones y tipos.").size(13.5).color(texto));
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("A continuación estudiaremos la navegación de rutas con").size(13.5).color(texto));
        codigo_inline_chip(ui, "crate::");
        ui.label(egui::RichText::new(",").size(13.5).color(texto));
        codigo_inline_chip(ui, "self::");
        ui.label(egui::RichText::new("y").size(13.5).color(texto));
        codigo_inline_chip(ui, "super::");
        ui.label(egui::RichText::new(", simplificando imports mediante").size(13.5).color(texto));
        codigo_inline_chip(ui, "use");
        ui.label(egui::RichText::new(". Finalmente veremos cómo diseñar APIs públicas ergonómicas aplicando el patrón Facade con").size(13.5).color(texto));
        codigo_inline_chip(ui, "pub use");
        ui.label(egui::RichText::new(".").size(13.5).color(texto));
    });

    ui.add_space(18.0);
    titulo_overview(ui, "Pilares Conceptuales", naranja);

    card_overview(
        ui,
        "1. Declaration & Module Tree (mod)",
        "Un módulo agrupa código relacionado dentro de un espacio de nombres propio. Todo crate comienza en una raíz (main.rs o lib.rs) y ramifica su funcionalidad en submódulos, evitando colisiones de nombres y facilitando la navegación del código.",
        naranja,
        texto,
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "2. Visibility & Encapsulation (pub · pub(crate))",
        "En Rust, todos los ítems (funciones, structs, enums, campos) son privados por defecto dentro de su módulo contenedor. Usar pub expone un ítem públicamente, mientras que pub(crate) lo hace visible para cualquier archivo del mismo crate sin exponerlo a usuarios externos.",
        naranja,
        texto,
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "3. Paths & Imports (crate · super · self · use)",
        "Las rutas indican el camino para acceder a cualquier elemento en el árbol de módulos. crate:: comienza en la raíz del proyecto, self:: se refiere al módulo actual y super:: sube un nivel al módulo padre. La palabra clave use crea accesos directos convenientes.",
        naranja,
        texto,
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "4. Public API Design & Re-export (pub use / Facade Pattern)",
        "Permite organizar internamente el código en carpetas y submódulos profundos, mientras que en la raíz del crate re-exportas únicamente los tipos y funciones principales con pub use, ofreciendo una experiencia limpia y ergonómica a los consumidores de tu librería.",
        naranja,
        texto,
    );
}
