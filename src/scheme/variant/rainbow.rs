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
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_palette_key_color(), Argb::from_u32(0xFF696FC4));
        assert_eq!(scheme.secondary_palette_key_color(), Argb::from_u32(0xFF75758B));
        assert_eq!(scheme.tertiary_palette_key_color(), Argb::from_u32(0xFF936B84));
        assert_eq!(scheme.neutral_palette_key_color(), Argb::from_u32(0xFF777777));
        assert_eq!(scheme.neutral_variant_palette_key_color(), Argb::from_u32(0xFF777777));
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF676DC1));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF5056A9));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF1B2074));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFFD5D6FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFFE0E0FF));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF3A4092));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFFFBCBE7));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFFFFD8EE));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFF613E55));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFF6C72C7));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFF383E8F));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFFFFFFFF));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFFF9F9F9));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFFF9F9F9));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFFF9F9F9));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.secondary(), Argb::from_u32(0xFF5C5D72));
    }

    #[test]
    fn test_light_theme_standard_contrast_secondary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.secondary_container(), Argb::from_u32(0xFFE1E0F9));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF8389E0));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFFBEC2FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFFF0EEFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF2A3082));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF383E8F));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFFBABDFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFF767CD2));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFFE0E0FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFF00003D));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xFFA17891));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xFFFFD8EE));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xFF1B0315));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFF131313));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFF131313));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFF131313));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.secondary(), Argb::from_u32(0xFFC5C4DD));
    }

    #[test]
    fn test_dark_theme_standard_contrast_secondary_container() {
        let scheme = SchemeRainbow::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.secondary_container(), Argb::from_u32(0xFF444559));
    }
}
