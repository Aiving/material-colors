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
                source_color_hct,
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
                source_color_hct.get_chroma() / 8.0 + 4.0,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeContent;
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(
            scheme.primary_palette_key_color(),
            Argb::from_u32(0xff080cff)
        );
        assert_eq!(
            scheme.secondary_palette_key_color(),
            Argb::from_u32(0xff656dd3)
        );
        assert_eq!(
            scheme.tertiary_palette_key_color(),
            Argb::from_u32(0xff81009f)
        );
        assert_eq!(
            scheme.neutral_palette_key_color(),
            Argb::from_u32(0xff767684)
        );
        // assert_eq!(
        //     scheme.neutral_variant_palette_key_color(),
        //     Argb::from_u32(0xff757589)
        // );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme =
            SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff5660ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff0001bb));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff00019f));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme =
            SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffd5d6ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff0000ff));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff0000f6));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme =
            SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xfffac9ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xff81009f));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xff7d009a));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff5e68ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffb3b7ff));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme =
            SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff7c84ff));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xffbec2ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xfff0eeff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff0001c9));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff0000ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffbabdff));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff6b75ff));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffb3b7ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff00003d));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xffc254de));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xfff09fff));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff1a0022));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff12121d));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff12121d));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff12121d));
    }

    #[test]
    fn test_light_theme_min_contrast_objectionabe_tertiary_container_lightens() {
        let scheme =
            SchemeContent::new(Argb::from_u32(0xff850096).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xffffccd7));
    }

    #[test]
    fn test_light_theme_standard_contrast_objectionabe_tertiary_container_lightens() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff850096).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xff980249));
    }

    #[test]
    fn test_light_theme_max_contrast_objectionabe_tertiary_container_darkens() {
        let scheme = SchemeContent::new(Argb::from_u32(0xff850096).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xff930046));
    }
}
