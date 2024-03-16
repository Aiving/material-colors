use crate::{
    dislike::fix_if_disliked,
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
    temperature::TemperatureCache,
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
                Self::palette(&source_color_hct, &Palette::Primary),
                Self::palette(&source_color_hct, &Palette::Secondary),
                Self::palette(&source_color_hct, &Palette::Tertiary),
                Self::palette(&source_color_hct, &Palette::Neutral),
                Self::palette(&source_color_hct, &Palette::NeutralVariant),
                None,
            ),
        }
    }

    pub fn palette(source_color_hct: &Hct, variant: &Palette) -> TonalPalette {
        match variant {
            Palette::Primary => {
                TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma())
            }
            Palette::Secondary => TonalPalette::of(
                source_color_hct.get_hue(),
                (source_color_hct.get_chroma() - 32.0).max(source_color_hct.get_chroma() * 0.5),
            ),
            Palette::Tertiary => TonalPalette::from_hct(fix_if_disliked(
                *TemperatureCache::new(*source_color_hct)
                    .analogous(Some(3), Some(6))
                    .last()
                    .unwrap(),
            )),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral => TonalPalette::of(
                source_color_hct.get_hue(),
                source_color_hct.get_chroma() / 8.0,
            ),
            Palette::NeutralVariant => TonalPalette::of(
                source_color_hct.get_hue(),
                (source_color_hct.get_chroma() / 8.0) + 4.0,
            ),
        }
    }
}
