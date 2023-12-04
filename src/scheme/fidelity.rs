use crate::dislike::fix_if_disliked;
use crate::dynamic_color::dynamic_scheme::DynamicScheme;
use crate::dynamic_color::variant::Variant;
use crate::hct::Hct;
use crate::palettes::tonal::TonalPalette;
use crate::temperature::TemperatureCache;

pub struct SchemeFidelity {
    pub scheme: DynamicScheme,
}

impl SchemeFidelity {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                None,
                Variant::Fidelity,
                is_dark,
                contrast_level,
                TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma()),
                TonalPalette::of(
                    source_color_hct.get_hue(),
                    (source_color_hct.get_chroma() - 32.0).max(source_color_hct.get_chroma() * 0.5),
                ),
                TonalPalette::from_hct(fix_if_disliked(
                    TemperatureCache::new(source_color_hct).complement(),
                )),
                TonalPalette::of(
                    source_color_hct.get_hue(),
                    source_color_hct.get_chroma() / 8.0,
                ),
                TonalPalette::of(
                    source_color_hct.get_hue(),
                    (source_color_hct.get_chroma() / 8.0) + 4.0,
                ),
                None,
            ),
        }
    }
}
