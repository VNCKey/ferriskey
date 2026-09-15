fn main() {
    let svg_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("diagramas/pipe.svg");
    let svg_str = std::fs::read_to_string(&svg_path).expect("No se pudo leer pipe.svg");

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };

    match usvg::Tree::from_str(&svg_str, &opt) {
        Ok(tree) => {
            let write_opt = usvg::WriteOptions::default();
            let vectorized_svg = tree.to_string(&write_opt);
            std::fs::write(svg_path, vectorized_svg).expect("No se pudo escribir pipe.svg");
            println!(
                "✅ pipe.svg vectorizado exitosamente: todos los textos ahora son trazos <path> nativos."
            );
        }
        Err(e) => {
            eprintln!("❌ Error parseando pipe.svg con usvg: {:?}", e);
        }
    }
}
