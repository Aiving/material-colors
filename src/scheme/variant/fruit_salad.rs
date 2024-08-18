use crate::{
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
    utils::math::sanitize_degrees_double,
};

pub struct SchemeFruitSalad {
    pub scheme: DynamicScheme,
}

impl SchemeFruitSalad {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::FruitSalad,
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
                sanitize_degrees_double(source_color_hct.get_hue() - 50.0),
                48.0,
            ),
            Palette::Secondary => TonalPalette::of(
                sanitize_degrees_double(source_color_hct.get_hue() - 50.0),
                36.0,
            ),
            Palette::Tertiary => TonalPalette::of(source_color_hct.get_hue(), 36.0),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral => TonalPalette::of(source_color_hct.get_hue(), 10.0),
            Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue(), 16.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeFruitSalad;
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(
            scheme.primary_palette_key_color(),
            Argb::from_u32(0xff0393c3)
        );

        assert_eq!(
            scheme.secondary_palette_key_color(),
            Argb::from_u32(0xff3a7e9e)
        );

        assert_eq!(
            scheme.tertiary_palette_key_color(),
            Argb::from_u32(0xff6e72ac)
        );

        assert_eq!(
            scheme.neutral_palette_key_color(),
            Argb::from_u32(0xff777682)
        );

        // assert_eq!(
        //     scheme.neutral_variant_palette_key_color(),
        //     Argb::from_u32(0xff75758b)
        // );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.primary(), Argb::from_u32(0xff007ea7));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary(), Argb::from_u32(0xff006688));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.primary(), Argb::from_u32(0xff003042));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffaae0ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_container(), Argb::from_u32(0xffc2e8ff));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff004f6b));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xffd5d6ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xffe0e0ff));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xff40447b));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff0083ae));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff004d67));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffffffff));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.surface(), Argb::from_u32(0xfffbf8ff));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.secondary(), Argb::from_u32(0xff196584));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.secondary_container(), Argb::from_u32(0xffc2e8ff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.primary(), Argb::from_u32(0xff1e9bcb));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.primary(), Argb::from_u32(0xff76d1ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.primary(), Argb::from_u32(0xffe0f3ff));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff003f56));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF004D67));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.primary_container(), Argb::from_u32(0xff68ceff));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff008ebc));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xffc2e8ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xff000d15));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff7b7fbb));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xffe0e0ff));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xff00003c));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.surface(), Argb::from_u32(0xff12131c));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.surface(), Argb::from_u32(0xff12131c));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.surface(), Argb::from_u32(0xff12131c));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.secondary(), Argb::from_u32(0xff8ecff2));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary_container() {
        let scheme =
            SchemeFruitSalad::new(Argb::from_u32(0xff0000ff).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.secondary_container(), Argb::from_u32(0xff004d67));
    }
}
