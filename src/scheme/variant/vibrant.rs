use crate::{
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
};

/// A Dynamic Color theme that is intentionally detached from the input color.
pub struct SchemeVibrant {
    pub scheme: DynamicScheme,
}

impl SchemeVibrant {
    /// Hues used at breakpoints such that designers can specify a hue rotation
    /// that occurs at a given break point.
    const HUES: [f64; 9] = [0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0];

    /// Hue rotations of the Secondary [`TonalPalette`], corresponding to the
    /// breakpoints in `hues`.
    const SECONDARY_ROTATIONS: [f64; 9] = [18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0];

    /// Hue rotations of the Tertiary [`TonalPalette`], corresponding to the
    /// breakpoints in `hues`.
    const TERTIARY_ROTATIONS: [f64; 9] = [35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0];

    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::Vibrant,
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
            Palette::Primary => TonalPalette::of(source_color_hct.get_hue(), 200.0),
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
            Palette::Neutral | Palette::NeutralVariant => {
                TonalPalette::of(source_color_hct.get_hue(), 10.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeVibrant;
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(
            scheme.primary_palette_key_color(),
            Argb::from_u32(0xff080cff)
        );
        assert_eq!(
            scheme.secondary_palette_key_color(),
            Argb::from_u32(0xff7b7296)
        );
        assert_eq!(
            scheme.tertiary_palette_key_color(),
            Argb::from_u32(0xff886c9d)
        );
        assert_eq!(
            scheme.neutral_palette_key_color(),
            Argb::from_u32(0xff777682)
        );
        // assert_eq!(
        //     scheme.neutral_variant_palette_key_color(),
        //     Argb::from_u32(0xff767685)
        // );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme =
            SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff5660ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff343dff));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff00019f));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme =
            SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffd5d6ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffe0e0ff));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff0000f6));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff5e68ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff0000ef));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme =
            SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xff7c84ff));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xffbec2ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xfff0eeff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff0001c9));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff0000ef));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffbabdff));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff6b75ff));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffe0e0ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff00003d));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff9679ab));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xfff2daff));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff16002a));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff12131c));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff12131c));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeVibrant::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xff12131c));
    }
}
