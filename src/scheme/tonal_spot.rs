use crate::dynamic_color::dynamic_scheme::DynamicScheme;
use crate::dynamic_color::variant::Variant;
use crate::hct::Hct;
use crate::palettes::tonal::TonalPalette;
use crate::utils::math::sanitize_degrees_double;

pub struct SchemeTonalSpot {
    pub scheme: DynamicScheme,
}

impl SchemeTonalSpot {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                None,
                Variant::TonalSpot,
                is_dark,
                contrast_level,
                TonalPalette::of(source_color_hct.get_hue(), 36.0),
                TonalPalette::of(source_color_hct.get_hue(), 16.0),
                TonalPalette::of(
                    sanitize_degrees_double(source_color_hct.get_hue() + 60.0),
                    24.0,
                ),
                TonalPalette::of(source_color_hct.get_hue(), 6.0),
                TonalPalette::of(source_color_hct.get_hue(), 8.0),
                None,
            ),
        }
    }
}
