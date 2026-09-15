use eframe::egui;

#[derive(Clone, Debug)]
pub enum FileTreeNode {
    Directory {
        name: String,
        children: Vec<FileTreeNode>,
    },
    File {
        name: String,
        rel_path: String,
    },
}

pub(super) struct FileTreeContext<'a> {
    pub(super) selected_file: &'a mut Option<String>,
    pub(super) project_dir: Option<&'a std::path::Path>,
    pub(super) code_target: &'a mut String,
    pub(super) close_popup: &'a mut bool,
    pub(super) alpha: u8,
    pub(super) combo_id: &'a str,
}

pub fn build_file_tree(paths: &[String]) -> Vec<FileTreeNode> {
    let mut root_children: Vec<FileTreeNode> = Vec::new();

    for path_str in paths {
        let parts: Vec<&str> = path_str.split('/').collect();
        insert_into_file_tree(&mut root_children, &parts, path_str);
    }

    sort_file_tree(&mut root_children);
    root_children
}

fn insert_into_file_tree(nodes: &mut Vec<FileTreeNode>, parts: &[&str], full_path: &str) {
    if parts.is_empty() {
        return;
    }
    if parts.len() == 1 {
        nodes.push(FileTreeNode::File {
            name: parts[0].to_string(),
            rel_path: full_path.to_string(),
        });
    } else {
        let dir_name = parts[0];
        let rest = &parts[1..];
        if let Some(existing) = nodes.iter_mut().find(|n| match n {
            FileTreeNode::Directory { name, .. } => name == dir_name,
            _ => false,
        }) {
            if let FileTreeNode::Directory { children, .. } = existing {
                insert_into_file_tree(children, rest, full_path);
            }
        } else {
            let mut children = Vec::new();
            insert_into_file_tree(&mut children, rest, full_path);
            nodes.push(FileTreeNode::Directory {
                name: dir_name.to_string(),
                children,
            });
        }
    }
}

fn sort_file_tree(nodes: &mut [FileTreeNode]) {
    nodes.sort_by(|a, b| match (a, b) {
        (FileTreeNode::Directory { name: na, .. }, FileTreeNode::Directory { name: nb, .. }) => {
            na.cmp(nb)
        }
        (FileTreeNode::Directory { .. }, FileTreeNode::File { .. }) => std::cmp::Ordering::Less,
        (FileTreeNode::File { .. }, FileTreeNode::Directory { .. }) => std::cmp::Ordering::Greater,
        (FileTreeNode::File { name: na, .. }, FileTreeNode::File { name: nb, .. }) => na.cmp(nb),
    });
    for node in nodes.iter_mut() {
        if let FileTreeNode::Directory { children, .. } = node {
            sort_file_tree(children);
        }
    }
}

pub(super) fn render_file_tree(
    ui: &mut egui::Ui,
    nodes: &[FileTreeNode],
    context: &mut FileTreeContext<'_>,
    depth: usize,
) {
    for node in nodes {
        match node {
            FileTreeNode::Directory { name, children } => {
                let dir_id = ui.make_persistent_id(format!(
                    "{}_tree_dir_{}_{}",
                    context.combo_id, depth, name
                ));
                let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(dir_id).unwrap_or(true));
                let alpha = context.alpha;

                ui.horizontal(|ui| {
                    if depth > 0 {
                        ui.add_space((depth as f32) * 12.0);
                    }
                    let btn_text = egui::RichText::new(format!("{}/", name))
                        .size(11.5)
                        .strong()
                        .color(egui::Color32::from_rgba_unmultiplied(140, 180, 220, alpha));

                    if ui.add(egui::Button::new(btn_text).frame(false)).clicked() {
                        is_open = !is_open;
                        ui.data_mut(|d| d.insert_temp(dir_id, is_open));
                    }
                });

                if is_open {
                    render_file_tree(ui, children, context, depth + 1);
                }
            }
            FileTreeNode::File { name, rel_path } => {
                let is_sel = context.selected_file.as_ref() == Some(rel_path);
                let alpha = context.alpha;
                ui.horizontal(|ui| {
                    if depth > 0 {
                        ui.add_space((depth as f32) * 12.0);
                    }
                    let txt_color = if is_sel {
                        egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha)
                    } else {
                        egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha)
                    };
                    let txt_file = egui::RichText::new(name).size(11.0).color(txt_color);

                    if ui.selectable_label(is_sel, txt_file).clicked() {
                        *context.selected_file = Some(rel_path.clone());
                        if let Some(proj_dir) = context.project_dir {
                            let target_file = proj_dir.join(rel_path);
                            if let Ok(content) = std::fs::read_to_string(&target_file) {
                                *context.code_target = content;
                            }
                        }
                        *context.close_popup = true;
                    }
                });
            }
        }
    }
}
