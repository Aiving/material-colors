use crate::{
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
};

pub struct SchemeMonochrome {
    pub scheme: DynamicScheme,
}

impl SchemeMonochrome {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::Monochrome,
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
            Palette::Primary
            | Palette::Secondary
            | Palette::Tertiary
            | Palette::Neutral
            | Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue(), 0.0),
            Palette::Error => TonalPalette::of(25.0, 84.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeMonochrome;
    use crate::{color::Argb, dynamic_color::MaterialDynamicColors};
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_key_colors() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(
            scheme.primary_palette_key_color(),
            Argb::from_u32(0xff777777)
        );
        assert_eq!(
            scheme.secondary_palette_key_color(),
            Argb::from_u32(0xff777777)
        );
        assert_eq!(
            scheme.tertiary_palette_key_color(),
            Argb::from_u32(0xff777777)
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
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff747474));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff000000));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff000000));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffd9d9d9));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff3b3b3b));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff3b3b3b));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff7a7a7a));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfff9f9f9));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfff9f9f9));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfff9f9f9));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff919191));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff3a3a3a));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffd4d4d4));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffd4d4d4));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff848484));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff000000));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff000000));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff848484));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff000000));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff000000));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131313));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131313));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff131313));
    }

    #[test]
    fn test_dark_theme_monochrome_spec() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_approx_eq!(
            f64,
            MaterialDynamicColors::primary().get_hct(&scheme).get_tone(),
            100.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_primary()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::primary_container()
                .get_hct(&scheme)
                .get_tone(),
            85.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_primary_container()
                .get_hct(&scheme)
                .get_tone(),
            0.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::secondary()
                .get_hct(&scheme)
                .get_tone(),
            80.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::secondary_container()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_container()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::tertiary()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::tertiary_container()
                .get_hct(&scheme)
                .get_tone(),
            60.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary_container()
                .get_hct(&scheme)
                .get_tone(),
            0.0,
            epsilon = 0.3
        );
    }

    fn _test_light_theme_monochrome_spec() {
        let scheme =
            SchemeMonochrome::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_approx_eq!(
            f64,
            MaterialDynamicColors::primary().get_hct(&scheme).get_tone(),
            0.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_primary()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::primary_container()
                .get_hct(&scheme)
                .get_tone(),
            25.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_primary_container()
                .get_hct(&scheme)
                .get_tone(),
            100.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::secondary()
                .get_hct(&scheme)
                .get_tone(),
            40.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary()
                .get_hct(&scheme)
                .get_tone(),
            100.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::secondary_container()
                .get_hct(&scheme)
                .get_tone(),
            85.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_container()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::tertiary()
                .get_hct(&scheme)
                .get_tone(),
            25.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::tertiary_container()
                .get_hct(&scheme)
                .get_tone(),
            49.0,
            epsilon = 0.3
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary_container()
                .get_hct(&scheme)
                .get_tone(),
            100.0,
            epsilon = 0.3
        );
    }
}
