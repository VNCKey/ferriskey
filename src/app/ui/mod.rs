#![allow(dead_code)]

pub mod badge;
pub mod button;
pub mod callout;
pub mod card;
pub mod code;
pub mod modal;
pub mod section;
pub mod table;
pub mod theme;

#[allow(unused_imports)]
pub use badge::{badge, tag_chip, BadgeStyle};
#[allow(unused_imports)]
pub use button::{btn_action_orange_small, btn_action_small, btn_primary, btn_secondary};
#[allow(unused_imports)]
pub use callout::{callout, callout_ferris, callout_info, callout_tip, callout_warning, CalloutType};
#[allow(unused_imports)]
pub use card::{
    card, card_overview, compact_interactive_card, crate_item_card, diagram_card,
    formatear_numero, formatear_numero_compacto, taxonomy_item_card, CrateCardBadge,
};
#[allow(unused_imports)]
pub use code::{
    code_block, code_box, highlighted_code, highlighted_code_block, inline_code_chip,
    inline_code_chip_color, inline_highlighted_code, table_code_snippet, CodePresentation,
};
#[allow(unused_imports)]
pub use modal::{mostrar_modal_diagrama, parse_svg_dimensions};
#[allow(unused_imports)]
pub use section::{codelab_cta_banner, codelab_notice, divider, section_heading, session_intro, session_title};
#[allow(unused_imports)]
pub use table::{centered_grid, cell_centered, cell_centered_horizontal, texto_con_chips_inline, EducationalTable, TableBody, tabla_metodos};
#[allow(unused_imports)]
pub use theme::{Colors, Spacing, Typography};
