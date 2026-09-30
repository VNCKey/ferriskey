use crate::app::AppState;
use crate::app::ui::*;
use eframe::egui;

type Reto = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

pub fn retos_cargo_workspaces() -> Vec<Reto> {
    vec![
        (
            "Package Structure",
            "Cargo & Workspaces",
            "Separar librería y binarios",
            "Un paquete puede tener una librería reutilizable en `src/lib.rs` y uno o varios ejecutables en `src/main.rs` o `src/bin/`.",
            "Relaciona cada archivo con el tipo de código que debe contener y el comando de Cargo que lo ejecuta.",
            "src/
├── lib.rs
├── main.rs
└── bin/
    └── cli.rs",
        ),
        (
            "Features",
            "Cargo & Workspaces",
            "Activar código opcional",
            "Las `Features` permiten elegir funcionalidades durante la compilación. Una dependencia opcional solo se incluye cuando una Feature la activa.",
            "Declara una Feature llamada `json` y relaciona la configuración con `#[cfg(feature = \"json\")]`.",
            "[features]
default = []
json = []

#[cfg(feature = \"json\")]
fn habilitar_json() {
    // Funcionalidad opcional
}",
        ),
        (
            "Workspace Members",
            "Cargo & Workspaces",
            "Agrupar varios paquetes",
            "Un `Workspace` reúne varios paquetes relacionados bajo un `Cargo.toml` raíz. Los miembros comparten `target/` y normalmente un `Cargo.lock`.",
            "Crea un Workspace con un crate de dominio y otro de aplicación dentro de `crates/`.",
            "[workspace]
resolver = \"2\"
members = [
    \"crates/dominio\",
    \"crates/api\",
]",
        ),
        (
            "Local Dependencies",
            "Cargo & Workspaces",
            "Conectar paquetes propios",
            "Un paquete puede depender de otro miembro con una dependencia local `path`. Así la aplicación utiliza el código del crate de dominio sin publicarlo todavía.",
            "Añade una dependencia local desde `api` hacia `dominio` y deja claro qué paquete ofrece la API reutilizable.",
            "[dependencies]
dominio = { path = \"../dominio\" }",
        ),
        (
            "Questions",
            "Evaluation",
            "5 Questions",
            "Esta evaluación repasa la estructura de paquetes, Features, Workspaces y dependencias locales.",
            "Responde correctamente las cinco preguntas para completar la sesión.",
            "",
        ),
    ]
}

pub fn mostrar_tutorial_cargo_workspaces(ui: &mut egui::Ui, state: &mut AppState) {
    if state.lessons.cargo_workspaces_tab == 5 {
        mostrar_code_lab_workspaces(ui, state);
        return;
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(10.0);
            match state.lessons.cargo_workspaces_tab {
                0 => mostrar_overview(ui),
                1 => mostrar_package_structure(ui, state),
                2 => mostrar_features(ui, state),
                3 => mostrar_workspaces(ui, state),
                4 => mostrar_local_dependencies(ui, state),
                _ => mostrar_overview(ui),
            }
            ui.add_space(20.0);
        });
}

fn mostrar_overview(ui: &mut egui::Ui) {
    session_title(ui, "Cargo & Workspaces");
    session_intro(
        ui,
        "Esta sesión explica cómo Cargo organiza paquetes, binarios, librerías y proyectos formados por varios crates.",
    );
    codelab_notice(ui);

    section_heading(ui, "Recorrido de la sesión");
    texto_con_chips_inline(
        ui,
        "Primero ubicaremos `src/lib.rs`, `src/main.rs` y `src/bin/`. Después veremos cómo activar código opcional con `Features`, agrupar paquetes con `Workspace` y conectar crates propios mediante `path`.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );

    ui.add_space(18.0);
    section_heading(ui, "Pilares conceptuales");
    ui.columns(2, |columns| {
        card_overview(
            &mut columns[0],
            "Package",
            "La unidad que Cargo compila y describe mediante un `Cargo.toml`.",
        );
        columns[0].add_space(8.0);
        card_overview(
            &mut columns[0],
            "Binary y Library",
            "Un binario inicia la aplicación; una librería concentra código reutilizable.",
        );

        card_overview(
            &mut columns[1],
            "Features",
            "Interruptores de compilación para incluir solo la funcionalidad necesaria.",
        );
        columns[1].add_space(8.0);
        card_overview(
            &mut columns[1],
            "Workspace",
            "Una raíz común para varios paquetes que evolucionan juntos.",
        );
    });

    ui.add_space(18.0);
    section_heading(ui, "Idea central");
    texto_con_chips_inline(
        ui,
        "Una buena estructura de Cargo separa responsabilidades y hace explícito qué código es reutilizable, qué programa se ejecuta y qué funcionalidad se compila.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_package_structure(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Package Structure");
    session_intro(
        ui,
        "Cargo utiliza convenciones de archivos para encontrar la librería, el binario principal y los binarios adicionales.",
    );
    section_heading(ui, "Una estructura mixta");
    texto_con_chips_inline(
        ui,
        "`src/lib.rs` contiene el núcleo reutilizable. `src/main.rs` es el binario principal y cada archivo de `src/bin/` puede convertirse en otro ejecutable.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "mi_proyecto/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    └── bin/
        └── cli.rs",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "text",
    );
    ui.add_space(16.0);
    section_heading(ui, "Comandos relacionados");
    texto_con_chips_inline(
        ui,
        "Usa `cargo run` para el binario principal y `cargo run --bin cli` para un binario adicional.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_features(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Cargo Features");
    session_intro(
        ui,
        "Una Feature permite activar una parte opcional del proyecto sin obligar a todos los usuarios a compilarla.",
    );
    section_heading(ui, "Configurar y consultar");
    texto_con_chips_inline(
        ui,
        "La tabla `[features]` vive en `Cargo.toml`. En el código, `#[cfg(feature = \"json\")]` incluye un elemento solo cuando esa Feature está activa.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "[features]
default = []
json = []

#[cfg(feature = \"json\")]
fn habilitar_json() {
    // Funcionalidad opcional
}",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "toml",
    );
    ui.add_space(16.0);
    section_heading(ui, "Activar una Feature");
    texto_con_chips_inline(
        ui,
        "Puedes compilar con `cargo build --features json`. La Feature no cambia el lenguaje: decide qué partes entran en esa compilación.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_workspaces(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Workspaces");
    session_intro(
        ui,
        "Un Workspace reúne varios paquetes relacionados bajo una única raíz de Cargo.",
    );
    section_heading(ui, "La raíz del proyecto");
    texto_con_chips_inline(
        ui,
        "El `Cargo.toml` raíz declara los `members`. Los paquetes mantienen su propio `Cargo.toml`, pero comparten la organización del Workspace.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "[workspace]
resolver = \"2\"
members = [
    \"crates/dominio\",
    \"crates/api\",
    \"crates/cli\",
]",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "toml",
    );
    ui.add_space(16.0);
    section_heading(ui, "Qué se comparte");
    texto_con_chips_inline(
        ui,
        "Los miembros comparten `target/` y normalmente `Cargo.lock`, lo que evita repetir compilaciones y versiones dentro del mismo proyecto.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn mostrar_local_dependencies(ui: &mut egui::Ui, state: &AppState) {
    session_title(ui, "Local Dependencies");
    session_intro(
        ui,
        "Los paquetes de un Workspace pueden depender de otros paquetes propios mediante una ruta local.",
    );
    section_heading(ui, "Conectar el código");
    texto_con_chips_inline(
        ui,
        "Una dependencia con `path` apunta a la carpeta del crate vecino. La aplicación importa la API pública que el crate de dominio expone.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
    ui.add_space(12.0);
    highlighted_code_block(
        ui,
        "[package]
name = \"api\"

[dependencies]
dominio = { path = \"../dominio\" }",
        &state.editor.syntax_set,
        &state.editor.theme_set.themes["base16-ocean.dark"],
        "toml",
    );
    ui.add_space(16.0);
    section_heading(ui, "Regla práctica");
    texto_con_chips_inline(
        ui,
        "El crate consumidor debe depender de la API, no de los archivos internos del otro paquete. Esa frontera mantiene el diseño claro.",
        Colors::TEXT_PRIMARY,
        Colors::CYAN_ACCENT,
    );
}

fn preguntas_cargo_workspaces() -> [SessionQuestion; 5] {
    [
        SessionQuestion {
            prompt: "1. ¿Qué archivo suele contener el núcleo reutilizable?",
            options: ["src/lib.rs", "Cargo.lock", "target/"],
            correct: 0,
        },
        SessionQuestion {
            prompt: "2. ¿Qué permite una Feature?",
            options: [
                "Activar código opcional",
                "Eliminar Cargo.toml",
                "Cambiar el sistema operativo",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "3. ¿Qué declara el Cargo.toml raíz de un Workspace?",
            options: [
                "Sus members",
                "Los píxeles de la interfaz",
                "La memoria del Stack",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "4. ¿Qué indica una dependencia path?",
            options: [
                "La ruta a otro crate local",
                "Una URL obligatoria",
                "Un nombre de función",
            ],
            correct: 0,
        },
        SessionQuestion {
            prompt: "5. ¿Qué comando activa una Feature llamada json?",
            options: [
                "cargo build --features json",
                "cargo json",
                "cargo run --feature",
            ],
            correct: 0,
        },
    ]
}

fn mostrar_code_lab_workspaces(ui: &mut egui::Ui, state: &mut AppState) {
    let retos = retos_cargo_workspaces();
    let total_retos = retos.len();
    let shell_state = crate::views::pilares::anatomy::mostrar_code_lab_shell(
        ui,
        state,
        crate::views::pilares::anatomy::CodeLabConfig {
            project_selector_id: "combo_proyectos_workspaces_codelab",
            separator_id: "workspaces_editor_toolbar_sep_y",
            terminal_panel_id: "workspaces_terminal_panel",
            editor_scroll_id: "workspaces_codelab_editor",
            drawer_key: "workspaces",
            drawer_open: state.ui.mostrar_explorer_drawer,
            drawer_tab: state.ui.codelab_drawer_tab,
            output_open: state.ui.mostrar_console_drawer,
            navigation_step: state.lessons.session_codelab_reto,
            navigation_total: total_retos,
        },
        |ui, state, orange, cyan| {
            let current = state
                .lessons
                .session_codelab_reto
                .min(total_retos.saturating_sub(1));
            let (titulo, categoria, subtitulo, explicacion, paso_practico, codigo_ejemplo) =
                retos[current];
            if titulo == "Questions" {
                mostrar_evaluacion(
                    ui,
                    &mut state.lessons.cargo_workspaces_preguntas_respuestas,
                    &preguntas_cargo_workspaces(),
                    &["Package", "Features", "Workspace", "path"],
                    "Comprueba si puedes elegir la estructura de Cargo adecuada para un proyecto pequeño o multipaquete.",
                    orange,
                );
            } else {
                crate::views::pilares::anatomy::mostrar_reto_codelab_item(
                    ui,
                    state,
                    current + 1,
                    titulo,
                    categoria,
                    subtitulo,
                    explicacion,
                    paso_practico,
                    codigo_ejemplo,
                    orange,
                    cyan,
                );
            }
        },
    );

    state.ui.mostrar_explorer_drawer = shell_state.drawer_open;
    state.ui.codelab_drawer_tab = shell_state.drawer_tab;
    state.ui.mostrar_console_drawer = shell_state.output_open;
    crate::views::pilares::anatomy::aplicar_navegacion_codelab(
        &mut state.lessons.session_codelab_reto,
        shell_state.navigation_delta,
        total_retos,
    );
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut AppState) {
    let tabs = [
        ("Overview", 0),
        ("Package", 1),
        ("Features", 2),
        ("Workspaces", 3),
        ("Dependencies", 4),
    ];
    let active = state.lessons.cargo_workspaces_tab;
    crate::components::navigation::mostrar_nav_superior_sesion(
        ui,
        state,
        "Cargo & Workspaces",
        &tabs,
        5,
        active,
        |st, idx| st.lessons.cargo_workspaces_tab = idx,
    );
}
