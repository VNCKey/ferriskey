use eframe::egui;
use std::sync::Arc;

/// Loader de SVG personalizado que inicializa `usvg` y `resvg`
/// con las fuentes del sistema operativo cargadas en memoria (`fontdb.load_system_fonts()`).
///
/// Esto permite que cualquier archivo SVG con etiquetas estándar `<text font-family="...">`
/// se pinte directamente con las fuentes disponibles (DejaVu Sans, Ubuntu, Roboto, etc.) sin necesidad
/// de vectorizar textos manualmente a trazos `<path>`.
pub struct SystemFontSvgLoader {
    options: usvg::Options<'static>,
}

impl SystemFontSvgLoader {
    pub fn new() -> Self {
        let mut fontdb = usvg::fontdb::Database::new();
        fontdb.load_system_fonts();
        fontdb.set_sans_serif_family("DejaVu Sans");
        fontdb.set_monospace_family("DejaVu Sans Mono");
        fontdb.set_serif_family("DejaVu Serif");

        tracing::info!(
            fuentes = fontdb.len(),
            "🔤 Base de datos de fuentes del sistema cargada en el loader SVG"
        );

        let options = usvg::Options {
            fontdb: Arc::new(fontdb),
            ..usvg::Options::default()
        };

        Self { options }
    }

    pub fn install(ctx: &egui::Context) {
        ctx.add_image_loader(Arc::new(Self::new()));
    }
}

impl egui::load::ImageLoader for SystemFontSvgLoader {
    fn id(&self) -> &str {
        egui::generate_loader_id!(SystemFontSvgLoader)
    }

    fn load(
        &self,
        ctx: &egui::Context,
        uri: &str,
        size_hint: egui::load::SizeHint,
    ) -> egui::load::ImageLoadResult {
        if !uri.ends_with(".svg") && !uri.starts_with("bytes://") {
            return Err(egui::load::LoadError::NotSupported);
        }

        // Obtener bytes del archivo o recurso
        let bytes = match ctx.try_load_bytes(uri) {
            Ok(egui::load::BytesPoll::Ready { bytes, .. }) => bytes,
            Ok(egui::load::BytesPoll::Pending { .. }) => {
                return Ok(egui::load::ImagePoll::Pending { size: None });
            }
            Err(err) => return Err(err),
        };

        // Parsear SVG con usvg usando las fuentes del sistema
        let tree = match usvg::Tree::from_data(&bytes, &self.options) {
            Ok(tree) => tree,
            Err(_) => return Err(egui::load::LoadError::NotSupported),
        };

        let svg_size = tree.size();
        let (mut width, mut height) = (svg_size.width(), svg_size.height());

        // Ajustar al tamaño solicitado según size_hint
        match size_hint {
            egui::load::SizeHint::Size {
                width: target_w,
                height: target_h,
                ..
            } => {
                if target_w > 0 && target_h > 0 {
                    width = target_w as f32;
                    height = target_h as f32;
                }
            }
            egui::load::SizeHint::Width(w) => {
                if w > 0 {
                    let ratio = w as f32 / svg_size.width();
                    width = w as f32;
                    height = svg_size.height() * ratio;
                }
            }
            egui::load::SizeHint::Height(h) if h > 0 => {
                let ratio = h as f32 / svg_size.height();
                width = svg_size.width() * ratio;
                height = h as f32;
            }
            _ => {}
        }

        let pixmap_size =
            match resvg::tiny_skia::IntSize::from_wh(width.round() as u32, height.round() as u32) {
                Some(size) => size,
                None => return Err(egui::load::LoadError::Loading("Tamaño SVG inválido".into())),
            };

        let mut pixmap =
            match resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()) {
                Some(pixmap) => pixmap,
                None => {
                    return Err(egui::load::LoadError::Loading(
                        "No se pudo crear pixmap".into(),
                    ));
                }
            };

        let transform = resvg::tiny_skia::Transform::from_scale(
            pixmap_size.width() as f32 / svg_size.width(),
            pixmap_size.height() as f32 / svg_size.height(),
        );

        resvg::render(&tree, transform, &mut pixmap.as_mut());

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [pixmap_size.width() as usize, pixmap_size.height() as usize],
            pixmap.data(),
        );

        Ok(egui::load::ImagePoll::Ready {
            image: Arc::new(color_image),
        })
    }

    fn forget(&self, _uri: &str) {}

    fn forget_all(&self) {}

    fn byte_size(&self) -> usize {
        0
    }
}
