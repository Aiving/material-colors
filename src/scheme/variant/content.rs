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

#[cfg(test)]
mod tests {
    use crate::color::Argb;
    use crate::dynamic_color::MaterialDynamicColors;

    use super::SchemeContent;

    #[test]
    fn test_light_theme_min_contrast_objectionabe_tertiary_container_lightens() {
        let scheme =
            SchemeContent::new(Argb::from_u32(0xff850096).into(), false, Some(-1.0)).scheme;

        assert_eq!(
            MaterialDynamicColors::tertiary_container().get_argb(&scheme),
            Argb::from_u32(0xffffccd7)
        );
    }

    #[test]
    fn test_light_theme_standard_contrast_objectionabe_tertiary_container_lightens() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff850096).into(), false, Some(0.0)).scheme;

        assert_eq!(
            MaterialDynamicColors::tertiary_container().get_argb(&scheme),
            Argb::from_u32(0xff980249)
        );
    }

    #[test]
    fn test_light_theme_max_contrast_objectionabe_tertiary_container_darkens() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff850096).into(), false, Some(1.0)).scheme;

        assert_eq!(
            MaterialDynamicColors::tertiary_container().get_argb(&scheme),
            Argb::from_u32(0xff930046)
        );
    }
}
