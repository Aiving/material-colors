use crate::{
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
    utils::math::sanitize_degrees_double,
};

pub struct SchemeRainbow {
    pub scheme: DynamicScheme,
}

impl SchemeRainbow {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::Rainbow,
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
            Palette::Primary => TonalPalette::of(source_color_hct.get_hue(), 48.0),
            Palette::Secondary => TonalPalette::of(source_color_hct.get_hue(), 16.0),
            Palette::Tertiary => TonalPalette::of(
                sanitize_degrees_double(source_color_hct.get_hue() + 60.0),
                24.0,
            ),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral | Palette::NeutralVariant => {
                TonalPalette::of(source_color_hct.get_hue(), 0.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeRainbow;
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(
            scheme.primary_palette_key_color(),
            Argb::from_u32(0xff696fc4)
        );
        assert_eq!(
            scheme.secondary_palette_key_color(),
            Argb::from_u32(0xff75758b)
        );
        assert_eq!(
            scheme.tertiary_palette_key_color(),
            Argb::from_u32(0xff936b84)
        );
        assert_eq!(
            scheme.neutral_palette_key_color(),
            Argb::from_u32(0xff777777)
        );
        assert_eq!(
            scheme.neutral_variant_palette_key_color(),
            Argb::from_u32(0xff777777)
        );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme =
            SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff676dc1));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff5056a9));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff1b2074));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme =
            SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffd5d6ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffe0e0ff));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff3a4092));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme =
            SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xfffbcbe7));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xffffd8ee));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xff613e55));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff6c72c7));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff383e8f));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme =
            SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfff9f9f9));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfff9f9f9));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfff9f9f9));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.secondary(), Argb::from_u32(0xff5c5d72));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.secondary_container(), Argb::from_u32(0xffe1e0f9));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff8389e0));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xffbec2ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xfff0eeff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff2a3082));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff383e8f));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffbabdff));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff767cd2));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffe0e0ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff00003d));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xffa17891));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xffffd8ee));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff1b0315));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131313));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131313));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131313));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.secondary(), Argb::from_u32(0xffc5c4dd));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.secondary_container(), Argb::from_u32(0xff444559));
    }
}
