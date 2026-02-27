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
            Palette::Neutral | Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue(), 2.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeNeutral;
    use crate::color::Rgb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_palette_key_color(), Rgb::from_u32(0x767685));
        assert_eq!(scheme.secondary_palette_key_color(), Rgb::from_u32(0x777680));
        assert_eq!(scheme.tertiary_palette_key_color(), Rgb::from_u32(0x75758B));
        assert_eq!(scheme.neutral_palette_key_color(), Rgb::from_u32(0x787678));
        assert_eq!(scheme.neutral_variant_palette_key_color(), Rgb::from_u32(0x787678));
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x737383));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x5D5D6C));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x2B2B38));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xD9D7E9));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xE2E1F3));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x484856));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x797888));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x454654));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xFFFFFF));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xFCF8FA));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xFCF8FA));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xFCF8FA));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x908F9F));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0xC6C5D6));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0xF0EEFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x393947));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x454654));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xC2C1D2));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x838393));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xE2E1F3));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x090A16));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x828299));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xE1E0F9));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x080A1B));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x131315));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x131315));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeNeutral::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x131315));
    }
}
