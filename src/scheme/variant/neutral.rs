use crate::{
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
};

pub struct SchemeNeutral {
    pub scheme: DynamicScheme,
}

impl SchemeNeutral {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::Neutral,
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
            Palette::Primary => TonalPalette::of(source_color_hct.get_hue(), 12.0),
            Palette::Secondary => TonalPalette::of(source_color_hct.get_hue(), 8.0),
            Palette::Tertiary => TonalPalette::of(source_color_hct.get_hue(), 16.0),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral | Palette::NeutralVariant => {
                TonalPalette::of(source_color_hct.get_hue(), 2.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeNeutral;
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(
            scheme.primary_palette_key_color(),
            Argb::from_u32(0xff767685)
        );
        assert_eq!(
            scheme.secondary_palette_key_color(),
            Argb::from_u32(0xff777680)
        );
        assert_eq!(
            scheme.tertiary_palette_key_color(),
            Argb::from_u32(0xff75758b)
        );
        assert_eq!(
            scheme.neutral_palette_key_color(),
            Argb::from_u32(0xff787678)
        );
        assert_eq!(
            scheme.neutral_variant_palette_key_color(),
            Argb::from_u32(0xff787678)
        );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme =
            SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff737383));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff5d5d6c));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff2b2b38));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme =
            SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffd9d7e9));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffe2e1f3));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff484856));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff797888));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff454654));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme =
            SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffcf8fa));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffcf8fa));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffcf8fa));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff908f9f));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xffc6c5d6));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xfff0eeff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff393947));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff454654));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffc2c1d2));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff838393));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffe2e1f3));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff090a16));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff828299));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xffe1e0f9));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff080a1b));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131315));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131315));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeNeutral::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131315));
    }
}
