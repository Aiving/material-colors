use crate::{
    dynamic_color::{DynamicColor, DynamicScheme, Variant, dynamic_scheme::Platform},
    hct::Hct,
    palette::TonalPalette,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecVersion {
    Spec2021,
    Spec2025,
    Spec2026,
}

pub trait ColorSpec {
    fn primary_palette_key_color(&self) -> DynamicColor;
    fn secondary_palette_key_color(&self) -> DynamicColor;
    fn tertiary_palette_key_color(&self) -> DynamicColor;
    fn neutral_palette_key_color(&self) -> DynamicColor;
    fn neutral_variant_palette_key_color(&self) -> DynamicColor;
    fn error_palette_key_color(&self) -> DynamicColor;

    fn background(&self) -> DynamicColor;
    fn on_background(&self) -> DynamicColor;
    fn surface(&self) -> DynamicColor;
    fn surface_dim(&self) -> DynamicColor;
    fn surface_bright(&self) -> DynamicColor;
    fn surface_container_lowest(&self) -> DynamicColor;
    fn surface_container_low(&self) -> DynamicColor;
    fn surface_container(&self) -> DynamicColor;
    fn surface_container_high(&self) -> DynamicColor;
    fn surface_container_highest(&self) -> DynamicColor;
    fn on_surface(&self) -> DynamicColor;
    fn surface_variant(&self) -> DynamicColor;
    fn on_surface_variant(&self) -> DynamicColor;
    fn inverse_surface(&self) -> DynamicColor;
    fn inverse_on_surface(&self) -> DynamicColor;
    fn outline(&self) -> DynamicColor;
    fn outline_variant(&self) -> DynamicColor;
    fn shadow(&self) -> DynamicColor;
    fn scrim(&self) -> DynamicColor;
    fn surface_tint(&self) -> DynamicColor;

    fn primary(&self) -> DynamicColor;
    fn primary_dim(&self) -> Option<DynamicColor>;
    fn on_primary(&self) -> DynamicColor;
    fn primary_container(&self) -> DynamicColor;
    fn on_primary_container(&self) -> DynamicColor;
    fn inverse_primary(&self) -> DynamicColor;

    fn secondary(&self) -> DynamicColor;
    fn secondary_dim(&self) -> Option<DynamicColor>;
    fn on_secondary(&self) -> DynamicColor;
    fn secondary_container(&self) -> DynamicColor;
    fn on_secondary_container(&self) -> DynamicColor;

    fn tertiary(&self) -> DynamicColor;
    fn tertiary_dim(&self) -> Option<DynamicColor>;
    fn on_tertiary(&self) -> DynamicColor;
    fn tertiary_container(&self) -> DynamicColor;
    fn on_tertiary_container(&self) -> DynamicColor;

    fn error(&self) -> DynamicColor;
    fn error_dim(&self) -> Option<DynamicColor>;
    fn on_error(&self) -> DynamicColor;
    fn error_container(&self) -> DynamicColor;
    fn on_error_container(&self) -> DynamicColor;

    fn primary_fixed(&self) -> DynamicColor;
    fn primary_fixed_dim(&self) -> DynamicColor;
    fn on_primary_fixed(&self) -> DynamicColor;
    fn on_primary_fixed_variant(&self) -> DynamicColor;

    fn secondary_fixed(&self) -> DynamicColor;
    fn secondary_fixed_dim(&self) -> DynamicColor;
    fn on_secondary_fixed(&self) -> DynamicColor;
    fn on_secondary_fixed_variant(&self) -> DynamicColor;

    fn tertiary_fixed(&self) -> DynamicColor;
    fn tertiary_fixed_dim(&self) -> DynamicColor;
    fn on_tertiary_fixed(&self) -> DynamicColor;
    fn on_tertiary_fixed_variant(&self) -> DynamicColor;

    fn get_hct(&self, scheme: &DynamicScheme, color: &DynamicColor) -> Hct;
    fn get_tone(&self, scheme: &DynamicScheme, color: &DynamicColor) -> f64;

    fn get_primary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_secondary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_tertiary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_neutral_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_neutral_variant_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_error_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
}
