use crate::{
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
    utils::math::sanitize_degrees_double,
};

/// A Dynamic Color theme that is intentionally detached from the input color.
pub struct SchemeExpressive {
    pub scheme: DynamicScheme,
}

impl SchemeExpressive {
    /// Hues used at breakpoints such that designers can specify a hue rotation
    /// that occurs at a given break point.
    const HUES: [f64; 9] = [0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0];

    /// Hue rotations of the Secondary [`TonalPalette`], corresponding to the
    /// breakpoints in `hues`.
    const SECONDARY_ROTATIONS: [f64; 9] = [45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0];

    /// Hue rotations of the Tertiary [`TonalPalette`], corresponding to the
    /// breakpoints in `hues`.
    const TERTIARY_ROTATIONS: [f64; 9] = [120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0];

    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::Expressive,
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
            Palette::Primary => TonalPalette::of(
                sanitize_degrees_double(source_color_hct.get_hue() + 240.0),
                40.0,
            ),
            Palette::Secondary => TonalPalette::of(
                DynamicScheme::get_rotated_hue(
                    source_color_hct.get_hue(),
                    &Self::HUES,
                    &Self::SECONDARY_ROTATIONS,
                ),
                24.0,
            ),
            Palette::Tertiary => TonalPalette::of(
                DynamicScheme::get_rotated_hue(
                    source_color_hct.get_hue(),
                    &Self::HUES,
                    &Self::TERTIARY_ROTATIONS,
                ),
                32.0,
            ),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral => TonalPalette::of(source_color_hct.get_hue() + 15.0, 8.0),
            Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue() + 15.0, 12.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeExpressive;
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(
            scheme.primary_palette_key_color(),
            Argb::from_u32(0xff35855f)
        );
        assert_eq!(
            scheme.secondary_palette_key_color(),
            Argb::from_u32(0xff8c6d8c)
        );
        assert_eq!(
            scheme.tertiary_palette_key_color(),
            Argb::from_u32(0xff806ea1)
        );
        assert_eq!(
            scheme.neutral_palette_key_color(),
            Argb::from_u32(0xff79757f)
        );
        // assert_eq!(
        //     scheme.neutral_variant_palette_key_color(),
        //     Argb::from_u32(0xff7a7585)
        // );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff32835d));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff146c48));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff00341f));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff99eabd));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffa2f4c6));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff005436));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff388862));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff005234));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffdf7ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffdf7ff));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffdf7ff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff51a078));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff87d7ab));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xffbbffd7));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff00432a));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff005234));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff83d3a8));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff43936c));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffa2f4c6));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff000e06));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff14121a));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff14121a));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme =
            SchemeExpressive::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff14121a));
    }
}
