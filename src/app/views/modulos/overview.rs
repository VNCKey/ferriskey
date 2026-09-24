use eframe::egui::{self, RichText};
use crate::app::ui::*;

pub fn mostrar(ui: &mut egui::Ui) {
    session_title(ui, "Modules & Visibility");
    session_intro(
        ui,
        "Esta sesión explica cómo estructurar y organizar proyectos en Rust mediante árboles de módulos, cómo controlar la visibilidad para proteger la privacidad de los datos, y cómo diseñar interfaces públicas limpias utilizando re-exportaciones.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("Primero aprenderemos a declarar módulos con")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "mod");
        ui.label(
            RichText::new("para construir el árbol jerárquico del crate. Después veremos las reglas de privacidad por defecto y cómo usar")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "pub");
        ui.label(
            RichText::new("y")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "pub(crate)");
        ui.label(
            RichText::new("para controlar el acceso seguro a funciones y tipos.")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("A continuación estudiaremos la navegación de rutas con")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "crate::");
        ui.label(RichText::new(",").font(Typography::body()).color(Colors::TEXT_PRIMARY));
        inline_code_chip(ui, "self::");
        ui.label(RichText::new("y").font(Typography::body()).color(Colors::TEXT_PRIMARY));
        inline_code_chip(ui, "super::");
        ui.label(
            RichText::new(", simplificando imports mediante")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "use");
        ui.label(
            RichText::new(". Finalmente veremos cómo diseñar APIs públicas ergonómicas aplicando el patrón Facade con")
                .font(Typography::body())
                .color(Colors::TEXT_PRIMARY),
        );
        inline_code_chip(ui, "pub use");
        ui.label(RichText::new(".").font(Typography::body()).color(Colors::TEXT_PRIMARY));
    });

    ui.add_space(18.0);
    section_heading(ui, "Pilares Conceptuales");

    card_overview(
        ui,
        "1. Declaration & Module Tree (mod)",
        "Un módulo agrupa código relacionado dentro de un espacio de nombres propio. Todo crate comienza en una raíz (main.rs o lib.rs) y ramifica su funcionalidad en submódulos, evitando colisiones de nombres y facilitando la navegación del código.",
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "2. Visibility & Encapsulation (pub · pub(crate))",
        "En Rust, todos los ítems (funciones, structs, enums, campos) son privados por defecto dentro de su módulo contenedor. Usar pub expone un ítem públicamente, mientras que pub(crate) lo hace visible para cualquier archivo del mismo crate sin exponerlo a usuarios externos.",
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "3. Paths & Imports (crate · super · self · use)",
        "Las rutas indican el camino para acceder a cualquier elemento en el árbol de módulos. crate:: comienza en la raíz del proyecto, self:: se refiere al módulo actual y super:: sube un nivel al módulo padre. La palabra clave use crea accesos directos convenientes.",
    );
    ui.add_space(8.0);

    card_overview(
        ui,
        "4. Public API Design & Re-export (pub use / Facade Pattern)",
        "Permite organizar internamente el código en carpetas y submódulos profundos, mientras que en la raíz del crate re-exportas únicamente los tipos y funciones principales con pub use, ofreciendo una experiencia limpia y ergonómica a los consumidores de tu librería.",
    );
}
