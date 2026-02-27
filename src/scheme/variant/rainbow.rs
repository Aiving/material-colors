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
            Palette::Tertiary => TonalPalette::of(sanitize_degrees_double(source_color_hct.get_hue() + 60.0), 24.0),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral | Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue(), 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeRainbow;
    use crate::color::Rgb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_palette_key_color(), Rgb::from_u32(0x696FC4));
        assert_eq!(scheme.secondary_palette_key_color(), Rgb::from_u32(0x75758B));
        assert_eq!(scheme.tertiary_palette_key_color(), Rgb::from_u32(0x936B84));
        assert_eq!(scheme.neutral_palette_key_color(), Rgb::from_u32(0x777777));
        assert_eq!(scheme.neutral_variant_palette_key_color(), Rgb::from_u32(0x777777));
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x676DC1));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x5056A9));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x1B2074));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xD5D6FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xE0E0FF));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x3A4092));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xFBCBE7));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xFFD8EE));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x613E55));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x6C72C7));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x383E8F));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xFFFFFF));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xF9F9F9));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xF9F9F9));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xF9F9F9));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.secondary(), Rgb::from_u32(0x5C5D72));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.secondary_container(), Rgb::from_u32(0xE1E0F9));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x8389E0));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0xBEC2FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0xF0EEFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x2A3082));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x383E8F));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xBABDFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x767CD2));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xE0E0FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x00003D));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xA17891));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xFFD8EE));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x1B0315));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x131313));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x131313));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x131313));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.secondary(), Rgb::from_u32(0xC5C4DD));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary_container() {
        let scheme = SchemeRainbow::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.secondary_container(), Rgb::from_u32(0x444559));
    }
}
