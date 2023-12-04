use crate::dynamic_color::dynamic_scheme::DynamicScheme;
use crate::dynamic_color::variant::Variant;
use crate::hct::Hct;
use crate::palettes::tonal::TonalPalette;

pub struct SchemeMonochrome {
    pub scheme: DynamicScheme,
}

impl SchemeMonochrome {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                None,
                Variant::Monochrome,
                is_dark,
                contrast_level,
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                None,
            ),
        }
    }
}
