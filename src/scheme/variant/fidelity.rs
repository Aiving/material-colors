use crate::{
    dislike::fix_if_disliked,
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
    temperature::TemperatureCache,
};

pub struct SchemeFidelity {
    pub scheme: DynamicScheme,
}

impl SchemeFidelity {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                Variant::Fidelity,
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
            Palette::Tertiary => TonalPalette::from_hct(fix_if_disliked(TemperatureCache::new(*source_color_hct).complement())),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral => TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma() / 8.0),
            Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue(), (source_color_hct.get_chroma() / 8.0) + 4.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SchemeFidelity;
    use crate::color::Argb;

    #[test]
    fn test_key_colors() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;

        assert_eq!(scheme.primary_palette_key_color(), Argb::from_u32(0xFF080CFF));
        assert_eq!(scheme.secondary_palette_key_color(), Argb::from_u32(0xFF656DD3));
        assert_eq!(scheme.tertiary_palette_key_color(), Argb::from_u32(0xFF9D0002));
        assert_eq!(scheme.neutral_palette_key_color(), Argb::from_u32(0xFF767684));
        // assert_eq!(
        //     scheme.neutral_variant_palette_key_color(),
        //     Argb::from_u32(0xff757589)
        // );
    }

    #[test]
    fn test_light_theme_min_contrast_primary() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF5660FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF0001BB));
    }

    #[test]
    fn test_light_theme_max_contrast_primary() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF00019F));
    }

    #[test]
    fn test_light_theme_min_contrast_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFFD5D6FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF0000FF));
    }

    #[test]
    fn test_light_theme_max_contrast_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF0000F6));
    }

    #[test]
    fn test_light_theme_min_contrast_tertiary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFFFFCDC6));
    }

    #[test]
    fn test_light_theme_standard_contrast_tertiary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFF9D0002));
    }

    #[test]
    fn test_light_theme_max_contrast_tertiary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFF980002));
    }

    #[test]
    fn test_light_theme_min_contrast_objectionable_tertiary_container_lightens() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF850096).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFFEBD982));
    }

    #[test]
    fn test_light_theme_standard_contrast_objectionable_tertiary_container_lightens() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF850096).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFFBCAC5A));
    }

    #[test]
    fn test_light_theme_max_contrast_objectionable_tertiary_container_darkens() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF850096).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.tertiary_container(), Argb::from_u32(0xFF544900));
    }

    #[test]
    fn test_light_theme_min_contrast_on_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFF5E68FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFFB3B7FF));
    }

    #[test]
    fn test_light_theme_max_contrast_on_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFFFFFFFF));
    }

    #[test]
    fn test_light_theme_min_contrast_surface() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFFFBF8FF));
    }

    #[test]
    fn test_light_theme_standard_contrast_surface() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFFFBF8FF));
    }

    #[test]
    fn test_light_theme_max_contrast_surface() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), false, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFFFBF8FF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFF7C84FF));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFFBEC2FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary(), Argb::from_u32(0xFFF0EEFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF0001C9));
    }

    #[test]
    fn test_dark_theme_standard_contrast_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFF0000FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.primary_container(), Argb::from_u32(0xFFBABDFF));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFF6B75FF));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFFB3B7FF));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_primary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_primary_container(), Argb::from_u32(0xFF00003D));
    }

    #[test]
    fn test_dark_theme_min_contrast_on_tertiary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xFFEF4635));
    }

    #[test]
    fn test_dark_theme_standard_contrast_on_tertiary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xFFFFA598));
    }

    #[test]
    fn test_dark_theme_max_contrast_on_tertiary_container() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.on_tertiary_container(), Argb::from_u32(0xFF220000));
    }

    #[test]
    fn test_dark_theme_min_contrast_surface() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(-1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFF12121D));
    }

    #[test]
    fn test_dark_theme_standard_contrast_surface() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(0.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFF12121D));
    }

    #[test]
    fn test_dark_theme_max_contrast_surface() {
        let scheme = SchemeFidelity::new(Argb::from_u32(0xFF0000FF).into(), true, Some(1.0)).scheme;
        assert_eq!(scheme.surface(), Argb::from_u32(0xFF12121D));
    }
}
