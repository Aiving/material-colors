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
            Palette::Primary => TonalPalette::of(sanitize_degrees_double(source_color_hct.get_hue() - 50.0), 48.0),
            Palette::Secondary => TonalPalette::of(sanitize_degrees_double(source_color_hct.get_hue() - 50.0), 36.0),
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
    use crate::color::Rgb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_palette_key_color(), Rgb::from_u32(0x0393C3));

        assert_eq!(scheme.secondary_palette_key_color(), Rgb::from_u32(0x3A7E9E));

        assert_eq!(scheme.tertiary_palette_key_color(), Rgb::from_u32(0x6E72AC));

        assert_eq!(scheme.neutral_palette_key_color(), Rgb::from_u32(0x777682));

        // assert_eq!(
        //     scheme.neutral_variant_palette_key_color(),
        //     Rgb::from_u32(0xff75758b)
        // );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.primary(), Rgb::from_u32(0x007EA7));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary(), Rgb::from_u32(0x006688));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.primary(), Rgb::from_u32(0x003042));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xAAE0FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xC2E8FF));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x004F6B));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xD5D6FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xE0E0FF));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x40447B));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x0083AE));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x004D67));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xFFFFFF));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.secondary(), Rgb::from_u32(0x196584));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.secondary_container(), Rgb::from_u32(0xC2E8FF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.primary(), Rgb::from_u32(0x1E9BCB));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.primary(), Rgb::from_u32(0x76D1FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.primary(), Rgb::from_u32(0xE0F3FF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x003F56));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x004D67));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x68CEFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x008EBC));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xC2E8FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x000D15));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x7B7FBB));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xE0E0FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x00003C));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

        assert_eq!(scheme.surface(), Rgb::from_u32(0x12131C));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.surface(), Rgb::from_u32(0x12131C));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

        assert_eq!(scheme.surface(), Rgb::from_u32(0x12131C));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.secondary(), Rgb::from_u32(0x8ECFF2));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary_container() {
        let scheme = SchemeFruitSalad::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

        assert_eq!(scheme.secondary_container(), Rgb::from_u32(0x004D67));
    }
}
