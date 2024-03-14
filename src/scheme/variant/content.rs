use crate::{
    dislike::fix_if_disliked,
    dynamic_color::{DynamicScheme, Variant},
    temperature::TemperatureCache,
    Hct, TonalPalette,
};

pub struct SchemeContent {
    pub scheme: DynamicScheme,
}

impl SchemeContent {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                Some(source_color_hct),
                Variant::Content,
                is_dark,
                contrast_level,
                TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma()),
                TonalPalette::of(
                    source_color_hct.get_hue(),
                    (source_color_hct.get_chroma() - 32.0).max(source_color_hct.get_chroma() * 0.5),
                ),
                TonalPalette::from_hct(fix_if_disliked(
                    *TemperatureCache::new(source_color_hct)
                        .analogous(Some(3), Some(6))
                        .last()
                        .unwrap(),
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
