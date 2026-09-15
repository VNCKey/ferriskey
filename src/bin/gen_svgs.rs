use railroad::*;

fn extract_attr(tag: &str, attr_name: &str) -> f32 {
    let pattern = format!("{}=\"", attr_name);
    if let Some(pos) = tag.find(&pattern) {
        let sub = &tag[pos + pattern.len()..];
        if let Some(end) = sub.find('"') {
            return sub[..end].parse().unwrap_or(0.0);
        }
    }
    0.0
}

/// Envuelve nodos con `SimpleStart` / `SimpleEnd` (bolita + línea conectada).
/// Antes se inyectaban `<circle>` sueltos y quedaban desconectados del riel.
fn seq_con_extremos(mut nodos: Vec<Box<dyn Node>>) -> Sequence<Box<dyn Node>> {
    let mut all = Vec::with_capacity(nodos.len() + 2);
    all.push(Box::new(SimpleStart) as Box<dyn Node>);
    all.append(&mut nodos);
    all.push(Box::new(SimpleEnd) as Box<dyn Node>);
    Sequence::new(all)
}

/// Quita las flechas automáticas que `railroad` inserta en tramos horizontales/verticales
/// largos (>50px). Esas flechas se dibujan *encima* del riel (`l ±5 ±5`) y se ven como
/// líneas superpuestas.
///
/// Forma típica (horizontal LTR, h=100):
/// ` m -47 0 l -5 -5 m 0 10 l 5 -5 m 47 0`
fn quitar_flechas_railroad(svg: &str) -> String {
    // Núcleo del chevron (sin los `m` de ida/vuelta al centro del tramo).
    const CORES: &[&str] = &[
        " l -5 -5 m 0 10 l 5 -5",
        " l 5 -5 m 0 10 l -5 -5",
        " l -5 5 m 0 -10 l 5 5",
        " l 5 5 m 0 -10 l -5 5",
        " l -5 -5 m 10 0 l -5 5",
        " l -5 5 m 10 0 l -5 -5",
        " l 5 -5 m -10 0 l 5 5",
        " l 5 5 m -10 0 l 5 -5",
    ];

    let mut s = svg.to_string();
    loop {
        let mut removed = false;
        for core in CORES {
            if let Some(core_at) = s.find(core) {
                let after_core = core_at + core.len();
                // ` m dx dy` de vuelta al riel (después del chevron)
                let end = consume_rel_move(&s, after_core).unwrap_or(after_core);
                // ` m dx dy` de ida al centro (antes del chevron)
                let start = {
                    let before = &s[..core_at];
                    if let Some(m_at) = before.rfind(" m ") {
                        if consume_rel_move(&s, m_at) == Some(core_at) {
                            m_at
                        } else {
                            core_at
                        }
                    } else {
                        core_at
                    }
                };
                s.replace_range(start..end, "");
                removed = true;
                break;
            }
        }
        if !removed {
            break;
        }
    }
    s
}

/// Triángulo relleno apuntando a la derecha, centrado en el riel del bypass.
/// No se dibuja encima de un trazo (a diferencia de las flechas nativas de railroad).
fn svg_flecha_bypass(mx: f32, my: f32) -> String {
    // Punta a la derecha; base un poco a la izquierda del centro del tramo.
    format!(
        "<polygon points=\"{tip_x:.1},{my:.1} {bx:.1},{ty:.1} {bx:.1},{by:.1}\" fill=\"#64c8ff\" stroke=\"#1e2638\" stroke-width=\"0.8\"/>",
        tip_x = mx + 5.5,
        my = my,
        bx = mx - 4.0,
        ty = my - 4.5,
        by = my + 4.5,
    )
}

/// Busca tramos horizontales largos (bypass de Optional/Repeat, h≥50) y devuelve
/// el punto medio de cada uno para colocar una flecha limpia.
fn puntos_flecha_en_path_d(d: &str) -> Vec<(f32, f32)> {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut arrows = Vec::new();
    let chars: Vec<char> = d.chars().collect();
    let mut i = 0;

    fn read_num(chars: &[char], i: &mut usize) -> Option<f32> {
        while *i < chars.len() && chars[*i].is_whitespace() {
            *i += 1;
        }
        if *i >= chars.len() {
            return None;
        }
        let start = *i;
        if chars[*i] == '-' || chars[*i] == '+' {
            *i += 1;
        }
        let d0 = *i;
        while *i < chars.len() && (chars[*i].is_ascii_digit() || chars[*i] == '.') {
            *i += 1;
        }
        if *i == d0 {
            return None;
        }
        let s: String = chars[start..*i].iter().collect();
        s.parse().ok()
    }

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() || c == ',' {
            i += 1;
            continue;
        }
        // comando SVG
        if c.is_ascii_alphabetic() {
            let cmd = c;
            i += 1;
            match cmd {
                'M' | 'L' => {
                    // railroad emite un solo par por comando
                    if let (Some(nx), Some(ny)) =
                        (read_num(&chars, &mut i), read_num(&chars, &mut i))
                    {
                        x = nx;
                        y = ny;
                    }
                }
                'm' | 'l' => {
                    if let (Some(dx), Some(dy)) =
                        (read_num(&chars, &mut i), read_num(&chars, &mut i))
                    {
                        x += dx;
                        y += dy;
                    }
                }
                'H' => {
                    if let Some(nx) = read_num(&chars, &mut i) {
                        x = nx;
                    }
                }
                'h' => {
                    if let Some(dh) = read_num(&chars, &mut i) {
                        // Flecha solo en tramos largos hacia adelante (bypass LTR)
                        if dh >= 50.0 {
                            let mx = x + dh / 2.0;
                            let my = y;
                            arrows.push((mx, my));
                        }
                        x += dh;
                    }
                }
                'V' => {
                    if let Some(ny) = read_num(&chars, &mut i) {
                        y = ny;
                    }
                }
                'v' => {
                    if let Some(dv) = read_num(&chars, &mut i) {
                        y += dv;
                    }
                }
                'a' | 'A' => {
                    // a rx ry xrot large sweep x y  (rel / abs end)
                    let _rx = read_num(&chars, &mut i);
                    let _ry = read_num(&chars, &mut i);
                    let _rot = read_num(&chars, &mut i);
                    let _large = read_num(&chars, &mut i);
                    let _sweep = read_num(&chars, &mut i);
                    if let (Some(ex), Some(ey)) =
                        (read_num(&chars, &mut i), read_num(&chars, &mut i))
                    {
                        if cmd == 'a' {
                            x += ex;
                            y += ey;
                        } else {
                            x = ex;
                            y = ey;
                        }
                    }
                }
                'z' | 'Z' => {}
                _ => {
                    // comando desconocido: saltar números
                    while read_num(&chars, &mut i).is_some() {}
                }
            }
        } else {
            i += 1;
        }
    }
    arrows
}

/// Tras quitar las flechas nativas, inserta triángulos limpios en los bypass largos.
fn inyectar_flechas_bypass_limpias(svg: &str) -> String {
    let mut out = String::with_capacity(svg.len() + 256);
    let mut rest = svg;
    while let Some(p_rel) = rest.find("<path ") {
        out.push_str(&rest[..p_rel]);
        let after = &rest[p_rel..];
        let end_rel = after.find('>').unwrap_or(after.len() - 1);
        let path_tag = &after[..=end_rel];
        out.push_str(path_tag);

        // Extraer d="..."
        if let Some(d_pos) = path_tag.find("d=\"") {
            let d_rest = &path_tag[d_pos + 3..];
            if let Some(d_end) = d_rest.find('"') {
                let d = &d_rest[..d_end];
                for (mx, my) in puntos_flecha_en_path_d(d) {
                    out.push_str(&svg_flecha_bypass(mx, my));
                }
            }
        }

        rest = &after[end_rel + 1..];
    }
    out.push_str(rest);
    out
}

/// Si `svg[pos..]` empieza por ` m <int> <int>`, devuelve el índice posterior.
fn consume_rel_move(svg: &str, pos: usize) -> Option<usize> {
    let rest = svg.get(pos..)?.strip_prefix(" m ")?;
    let mut i = 0;
    let b = rest.as_bytes();
    // int1
    if i < b.len() && b[i] == b'-' {
        i += 1;
    }
    let d0 = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == d0 {
        return None;
    }
    if i >= b.len() || b[i] != b' ' {
        return None;
    }
    i += 1;
    // int2
    if i < b.len() && b[i] == b'-' {
        i += 1;
    }
    let d1 = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == d1 {
        return None;
    }
    Some(pos + 3 + i)
}

/// Limpia y transforma el SVG crudo de `railroad`: tema oscuro, estilos inline
/// y padding horizontal. Las bolitas de inicio/fin vienen de SimpleStart/SimpleEnd
/// (ya unidas al riel); no se dibujan círculos flotantes aparte.
fn limpiar_svg_railroad(raw_svg: &str) -> String {
    let clean =
        quitar_flechas_railroad(&raw_svg.replace(">\n", ">").replace(">\r\n", ">").replace(
            "<rect width=\"100%\" height=\"100%\" class=\"railroad_canvas\"/>",
            "",
        ));

    // 1. Extraer viewBox original
    let mut orig_w = 500.0f32;
    let mut orig_h = 60.0f32;
    if let Some(vb_pos) = clean.find("viewBox=\"") {
        let vb_sub = &clean[vb_pos + 9..];
        if let Some(vb_close) = vb_sub.find('"') {
            let parts: Vec<&str> = vb_sub[..vb_close].split_whitespace().collect();
            if parts.len() >= 4 {
                orig_w = parts[2].parse().unwrap_or(500.0);
                orig_h = parts[3].parse().unwrap_or(60.0);
            }
        }
    }

    let padding_x = 40.0f32;
    let canvas_w = orig_w + (padding_x * 2.0);

    // 2. Re-estilar terminal / nonterminal
    let mut output = String::with_capacity(clean.len() * 2);
    let mut search_idx = 0;

    while let Some(g_rel) = clean[search_idx..].find("<g class=") {
        let g_start = search_idx + g_rel;
        let g_substr = &clean[g_start..];

        let is_terminal = g_substr.starts_with("<g class=\"terminal\"");
        let is_nonterminal = g_substr.starts_with("<g class=\"nonterminal\"");

        if (is_terminal || is_nonterminal)
            && let Some(g_end_rel) = g_substr.find("</g>")
        {
            let g_end = g_start + g_end_rel + 4;
            let group_block = &clean[g_start..g_end];

            let mut content = "";
            if let (Some(t_start), Some(t_end)) =
                (group_block.find("<text"), group_block.find("</text>"))
            {
                let text_sub = &group_block[t_start..t_end];
                if let Some(tag_close) = text_sub.find('>') {
                    content = text_sub[tag_close + 1..].trim();
                }
            }

            let mut rx = 0.0f32;
            let mut ry = 0.0f32;
            let mut rw = 0.0f32;
            let mut rh = 0.0f32;

            if let Some(r_pos) = group_block.find("<rect ") {
                let r_sub = &group_block[r_pos..];
                if let Some(r_close) = r_sub.find('>') {
                    let r_tag = &r_sub[..r_close + 1];
                    rx = extract_attr(r_tag, "x");
                    ry = extract_attr(r_tag, "y");
                    rw = extract_attr(r_tag, "width");
                    rh = extract_attr(r_tag, "height");
                }
            }

            if rw <= 0.0 {
                rw = (content.len() as f32 * 8.0) + 20.0;
            }
            if rh <= 0.0 {
                rh = 22.0;
            }

            let new_rect = if is_terminal {
                format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"10\" ry=\"10\" fill=\"#1e2638\" stroke=\"#ff9d00\" stroke-width=\"2\"/>",
                    rx, ry, rw, rh
                )
            } else {
                format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#1a2336\" stroke=\"#64c8ff\" stroke-width=\"2\"/>",
                    rx, ry, rw, rh
                )
            };

            // Texto regular y un poco más chico (sin bold) — se lee mejor al escalar el modal
            let font_size = if is_terminal { 12.0f32 } else { 11.0f32 };
            let font_color = if is_terminal { "#ffb347" } else { "#ffffff" };
            let font_weight = "normal";

            let cx = rx + rw / 2.0;
            let cy = ry + rh / 2.0 + (font_size * 0.35);

            let new_text = format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"DejaVu Sans, Arial, sans-serif\" font-size=\"{:.1}\" font-weight=\"{}\" fill=\"{}\" text-anchor=\"middle\">{}</text>",
                cx, cy, font_size, font_weight, font_color, content
            );

            let class_name = if is_terminal {
                "terminal"
            } else {
                "nonterminal"
            };
            let new_group = format!("<g class=\"{}\">{}{}</g>", class_name, new_rect, new_text);

            output.push_str(&clean[search_idx..g_start]);
            output.push_str(&new_group);
            search_idx = g_end;
            continue;
        }

        output.push_str(&clean[search_idx..g_start + 8]);
        search_idx = g_start + 8;
    }

    output.push_str(&clean[search_idx..]);

    // 3. Estilos en <path>: rieles cyan; bolitas SimpleStart/End rellenas
    //    (sus paths incluyen arcos `a5` / `A` de radio ~5).
    let mut styled_output = String::with_capacity(output.len());
    let mut path_search = 0;

    while let Some(p_rel) = output[path_search..].find("<path ") {
        let p_start = path_search + p_rel;
        styled_output.push_str(&output[path_search..p_start]);

        if let Some(p_close_rel) = output[p_start..].find('>') {
            let p_end = p_start + p_close_rel + 1;
            let path_tag = &output[p_start..p_end];

            if !path_tag.contains("stroke=") {
                // SimpleStart/SimpleEnd: path con varios arcos de radio 5
                let is_endpoint = path_tag.contains("a5 ")
                    || path_tag.contains("a 5 ")
                    || path_tag.contains("A5 ")
                    || path_tag.matches("a5,").count() >= 1
                    || path_tag.matches(" a5").count() >= 2
                    || path_tag.matches("5 5 0").count() >= 2;

                let new_path_tag = if is_endpoint {
                    path_tag.replace(
                        "<path ",
                        "<path stroke=\"#64c8ff\" stroke-width=\"2\" fill=\"#64c8ff\" ",
                    )
                } else {
                    // stroke 2 (no 2.5): en uniones con cajas se veía como doble trazo
                    path_tag.replace(
                        "<path ",
                        "<path stroke=\"#64c8ff\" stroke-width=\"2\" fill=\"none\" ",
                    )
                };
                styled_output.push_str(&new_path_tag);
            } else {
                styled_output.push_str(path_tag);
            }
            path_search = p_end;
        } else {
            styled_output.push_str(&output[p_start..p_start + 6]);
            path_search = p_start + 6;
        }
    }
    styled_output.push_str(&output[path_search..]);

    // 3b. Flechas limpias (triángulo) solo en bypass largos de Optional — no las nativas de railroad
    let styled_output = inyectar_flechas_bypass_limpias(&styled_output);

    // 4. Padding horizontal (sin círculos extra desconectados)
    let mut final_canvas = String::with_capacity(styled_output.len() + 200);
    final_canvas.push_str(&format!(
        "<svg width=\"{:.1}\" height=\"{:.1}\" viewBox=\"0 0 {:.1} {:.1}\" xmlns=\"http://www.w3.org/2000/svg\"><g transform=\"translate({:.1}, 0)\">",
        canvas_w, orig_h, canvas_w, orig_h, padding_x
    ));

    if let Some(seq_pos) = styled_output.find("<g class=\"sequence\">") {
        let content_after = &styled_output[seq_pos..];
        if let Some(end_svg) = content_after.rfind("</svg>") {
            final_canvas.push_str(&content_after[..end_svg]);
        } else {
            final_canvas.push_str(content_after);
        }
    } else if let Some(svg_pos) = styled_output.find("<svg") {
        // Fallback: cuerpo interior del svg
        if let Some(gt) = styled_output[svg_pos..].find('>') {
            let inner = &styled_output[svg_pos + gt + 1..];
            if let Some(end_svg) = inner.rfind("</svg>") {
                final_canvas.push_str(&inner[..end_svg]);
            } else {
                final_canvas.push_str(inner);
            }
        }
    } else {
        final_canvas.push_str(&styled_output);
    }

    final_canvas.push_str("</g></svg>");
    final_canvas
}

/// Convierte los elementos <text> del SVG en trazados de vectores (<path fill="...">)
/// eliminando cualquier fondo negro inyectado para dejar el canvas completamente transparente.
fn convertir_texto_a_vectores(svg_str: &str) -> String {
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };

    if let Ok(tree) = usvg::Tree::from_str(svg_str, &opt) {
        let write_opt = usvg::WriteOptions::default();
        let raw_out = tree.to_string(&write_opt);
        let mut transparent_out = raw_out.replace(
            "<rect width=\"100%\" height=\"100%\" class=\"railroad_canvas\"/>",
            "",
        );

        if let Some(p_start) = transparent_out.find("<path fill=\"#000000\" stroke=\"none\"")
            && let Some(p_end) = transparent_out[p_start..].find("/>")
        {
            transparent_out.replace_range(p_start..p_start + p_end + 2, "");
        }

        return transparent_out;
    }
    svg_str.to_string()
}

fn main() {
    println!("Generando diagramas SVG con SimpleStart/SimpleEnd (bolitas conectadas al riel)...");

    let diagram_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("diagramas");
    let out = |name: &str, svg: String| {
        let path = diagram_dir.join(name);
        std::fs::write(&path, svg).unwrap();
        println!("  wrote {}", path.display());
    };

    // 1. Diagrama Inmutable: let ident [: tipo] = expr;
    let dia_immut = Diagram::new(seq_con_extremos(vec![
        Box::new(Terminal::new("let".to_string())),
        Box::new(NonTerminal::new("ident".to_string())),
        Box::new(Optional::new(Sequence::new(vec![
            Box::new(Terminal::new(":".to_string())) as Box<dyn Node>,
            Box::new(NonTerminal::new("tipo".to_string())),
        ]))),
        Box::new(Terminal::new("=".to_string())),
        Box::new(NonTerminal::new("expr".to_string())),
        Box::new(Terminal::new(";".to_string())),
    ]));
    out(
        "diagrama_let_immut.svg",
        convertir_texto_a_vectores(&limpiar_svg_railroad(&dia_immut.to_string())),
    );

    // 2. Diagrama Mutable: let mut ident [: tipo] = expr;
    let dia_mut = Diagram::new(seq_con_extremos(vec![
        Box::new(Terminal::new("let".to_string())),
        Box::new(Terminal::new("mut".to_string())),
        Box::new(NonTerminal::new("ident".to_string())),
        Box::new(Optional::new(Sequence::new(vec![
            Box::new(Terminal::new(":".to_string())) as Box<dyn Node>,
            Box::new(NonTerminal::new("tipo".to_string())),
        ]))),
        Box::new(Terminal::new("=".to_string())),
        Box::new(NonTerminal::new("expr".to_string())),
        Box::new(Terminal::new(";".to_string())),
    ]));
    out(
        "diagrama_let_mut.svg",
        convertir_texto_a_vectores(&limpiar_svg_railroad(&dia_mut.to_string())),
    );

    // 3. Diagrama fn main(): fn main([parámetros]) { [bloque de código] }
    let dia_fn = Diagram::new(seq_con_extremos(vec![
        Box::new(Terminal::new("fn".to_string())),
        Box::new(Terminal::new("main".to_string())),
        Box::new(Terminal::new("(".to_string())),
        Box::new(Optional::new(NonTerminal::new("parámetros".to_string()))),
        Box::new(Terminal::new(")".to_string())),
        Box::new(Terminal::new("{".to_string())),
        Box::new(Optional::new(NonTerminal::new(
            "bloque de código".to_string(),
        ))),
        Box::new(Terminal::new("}".to_string())),
    ]));
    out(
        "diagrama_fn_main.svg",
        convertir_texto_a_vectores(&limpiar_svg_railroad(&dia_fn.to_string())),
    );

    // 4. Diagrama Librería (src/lib.rs)
    let dia_lib = Diagram::new(seq_con_extremos(vec![
        Box::new(Optional::new(Terminal::new("pub".to_string()))),
        Box::new(NonTerminal::new("fn / struct / mod".to_string())),
        Box::new(NonTerminal::new("ident".to_string())),
        Box::new(Terminal::new("{".to_string())),
        Box::new(Optional::new(NonTerminal::new(
            "cuerpo librería".to_string(),
        ))),
        Box::new(Terminal::new("}".to_string())),
    ]));
    out(
        "diagrama_lib.svg",
        convertir_texto_a_vectores(&limpiar_svg_railroad(&dia_lib.to_string())),
    );

    // 5. Compile Time
    let dia_compile = Diagram::new(seq_con_extremos(vec![
        Box::new(NonTerminal::new("código .rs".to_string())),
        Box::new(Terminal::new("cargo build".to_string())),
        Box::new(Terminal::new("rustc".to_string())),
        Box::new(NonTerminal::new("Borrow Checker & Tipos".to_string())),
        Box::new(Terminal::new("Binario".to_string())),
    ]));
    out(
        "diagrama_compile_time.svg",
        convertir_texto_a_vectores(&limpiar_svg_railroad(&dia_compile.to_string())),
    );

    // 6. Run Time
    let dia_run = Diagram::new(seq_con_extremos(vec![
        Box::new(Terminal::new("./ejecutable".to_string())),
        Box::new(Terminal::new("CPU & OS".to_string())),
        Box::new(NonTerminal::new("Stack & Heap".to_string())),
        Box::new(Terminal::new("fn main()".to_string())),
        Box::new(NonTerminal::new("Lógica & Resultados".to_string())),
    ]));
    out(
        "diagrama_run_time.svg",
        convertir_texto_a_vectores(&limpiar_svg_railroad(&dia_run.to_string())),
    );

    println!("¡Diagramas SVG con bolitas conectadas generados con éxito!");
}
