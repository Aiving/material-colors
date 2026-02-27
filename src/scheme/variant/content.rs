use crate::{
    dislike::fix_if_disliked,
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
    temperature::TemperatureCache,
};

pub struct SchemeContent {
    pub scheme: DynamicScheme,
}

impl SchemeContent {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::Content,
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
            Palette::Primary => TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma()),
            Palette::Secondary => TonalPalette::of(
                source_color_hct.get_hue(),
                (source_color_hct.get_chroma() - 32.0).max(source_color_hct.get_chroma() * 0.5),
            ),
            Palette::Tertiary => TonalPalette::from_hct(fix_if_disliked(
                *TemperatureCache::new(*source_color_hct).analogous(Some(3), Some(6)).last().unwrap(),
            )),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral => TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma() / 8.0),
            Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma() / 8.0 + 4.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeContent;
    use crate::color::Rgb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_palette_key_color(), Rgb::from_u32(0x080CFF));
        assert_eq!(scheme.secondary_palette_key_color(), Rgb::from_u32(0x656DD3));
        assert_eq!(scheme.tertiary_palette_key_color(), Rgb::from_u32(0x81009F));
        assert_eq!(scheme.neutral_palette_key_color(), Rgb::from_u32(0x767684));
        // assert_eq!(
        //     scheme.neutral_variant_palette_key_color(),
        //     Rgb::from_u32(0xff757589)
        // );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x5660FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x0001BB));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x00019F));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xD5D6FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000FF));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000F6));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xFAC9FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x81009F));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x7D009A));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x5E68FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xB3B7FF));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xFFFFFF));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0x7C84FF));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0xBEC2FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Rgb::from_u32(0xF0EEFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0001C9));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Rgb::from_u32(0xBABDFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x6B75FF));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xB3B7FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x00003D));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xC254DE));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xF09FFF));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x1A0022));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x12121D));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x12121D));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Rgb::from_u32(0x12121D));
    }

    #[test]
    fn test_light_theme_min_contrast_objectionabe_tertiary_container_lightens() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x850096).into(), false, Some(-1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xFFCCD7));
    }

    #[test]
    fn test_light_theme_standard_contrast_objectionabe_tertiary_container_lightens() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x850096).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x980249));
    }

    #[test]
    fn test_light_theme_max_contrast_objectionabe_tertiary_container_darkens() {
        let scheme = SchemeContent::new(Rgb::from_u32(0x850096).into(), false, Some(1.0)).scheme;

        assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x930046));
    }
}
