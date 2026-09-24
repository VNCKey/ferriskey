use eframe::egui::{Color32, FontId, Margin, CornerRadius, Stroke};

/// Paleta oficial de colores de FerrisKey
pub struct Colors;

impl Colors {
    // Acentos Principales
    pub const ORANGE_RUST: Color32 = Color32::from_rgb(255, 160, 50);
    pub const CYAN_ACCENT: Color32 = Color32::from_rgb(100, 200, 255);
    pub const GREEN_ACCENT: Color32 = Color32::from_rgb(180, 220, 180);

    // Textos
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(205, 215, 230);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(160, 175, 195);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(120, 135, 155);
    pub const TEXT_WHITE: Color32 = Color32::WHITE;

    // Fondos
    pub const BG_APP: Color32 = Color32::from_rgb(13, 15, 19);
    pub const BG_CARD: Color32 = Color32::from_rgb(14, 18, 26);
    pub const BG_CARD_DARK: Color32 = Color32::from_rgb(10, 14, 20);
    pub const BG_CODE_INLINE: Color32 = Color32::from_rgb(20, 25, 35);

    // Bordes
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(45, 60, 90);
    pub const BORDER_ORANGE_GLOW: Color32 = Color32::from_rgba_premultiplied(75, 47, 15, 75);
    pub const BORDER_CYAN_GLOW: Color32 = Color32::from_rgba_premultiplied(29, 58, 75, 75);

    // Estados Semánticos
    pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
    pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);
    pub const INFO: Color32 = Color32::from_rgb(59, 130, 246);
}

/// Jerarquía tipográfica unificada
pub struct Typography;

impl Typography {
    pub fn session_title() -> FontId {
        FontId::proportional(24.0)
    }
    pub fn heading() -> FontId {
        FontId::proportional(18.0)
    }
    pub fn card_title() -> FontId {
        FontId::proportional(15.0)
    }
    pub fn body() -> FontId {
        FontId::proportional(13.5)
    }
    pub fn body_small() -> FontId {
        FontId::proportional(12.5)
    }
    pub fn code() -> FontId {
        FontId::monospace(12.5)
    }
    pub fn code_small() -> FontId {
        FontId::monospace(11.5)
    }
}

/// Constantes de geometría y espaciado
pub struct Spacing;

impl Spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;

    pub const ROUND_SM: u8 = 4;
    pub const ROUND_MD: u8 = 8;
    pub const ROUND_LG: u8 = 12;

    pub fn card_margin() -> Margin {
        Margin::same(12)
    }

    pub fn card_rounding() -> CornerRadius {
        CornerRadius::same(Self::ROUND_MD)
    }

    pub fn card_stroke() -> Stroke {
        Stroke::new(1.0, Colors::BORDER_SUBTLE)
    }

    pub fn card_stroke_orange() -> Stroke {
        Stroke::new(1.0, Colors::BORDER_ORANGE_GLOW)
    }
}
