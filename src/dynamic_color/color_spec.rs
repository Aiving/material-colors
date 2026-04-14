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
    fn primary_palette_key_color(&self) -> &'static DynamicColor;
    fn secondary_palette_key_color(&self) -> &'static DynamicColor;
    fn tertiary_palette_key_color(&self) -> &'static DynamicColor;
    fn neutral_palette_key_color(&self) -> &'static DynamicColor;
    fn neutral_variant_palette_key_color(&self) -> &'static DynamicColor;
    fn error_palette_key_color(&self) -> &'static DynamicColor;

    fn background(&self) -> &'static DynamicColor;
    fn on_background(&self) -> &'static DynamicColor;
    fn surface(&self) -> &'static DynamicColor;
    fn surface_dim(&self) -> &'static DynamicColor;
    fn surface_bright(&self) -> &'static DynamicColor;
    fn surface_container_lowest(&self) -> &'static DynamicColor;
    fn surface_container_low(&self) -> &'static DynamicColor;
    fn surface_container(&self) -> &'static DynamicColor;
    fn surface_container_high(&self) -> &'static DynamicColor;
    fn surface_container_highest(&self) -> &'static DynamicColor;
    fn on_surface(&self) -> &'static DynamicColor;
    fn surface_variant(&self) -> &'static DynamicColor;
    fn on_surface_variant(&self) -> &'static DynamicColor;
    fn inverse_surface(&self) -> &'static DynamicColor;
    fn inverse_on_surface(&self) -> &'static DynamicColor;
    fn outline(&self) -> &'static DynamicColor;
    fn outline_variant(&self) -> &'static DynamicColor;
    fn shadow(&self) -> &'static DynamicColor;
    fn scrim(&self) -> &'static DynamicColor;
    fn surface_tint(&self) -> &'static DynamicColor;

    fn primary(&self) -> &'static DynamicColor;
    fn primary_dim(&self) -> Option<&'static DynamicColor>;
    fn on_primary(&self) -> &'static DynamicColor;
    fn primary_container(&self) -> &'static DynamicColor;
    fn on_primary_container(&self) -> &'static DynamicColor;
    fn inverse_primary(&self) -> &'static DynamicColor;

    fn secondary(&self) -> &'static DynamicColor;
    fn secondary_dim(&self) -> Option<&'static DynamicColor>;
    fn on_secondary(&self) -> &'static DynamicColor;
    fn secondary_container(&self) -> &'static DynamicColor;
    fn on_secondary_container(&self) -> &'static DynamicColor;

    fn tertiary(&self) -> &'static DynamicColor;
    fn tertiary_dim(&self) -> Option<&'static DynamicColor>;
    fn on_tertiary(&self) -> &'static DynamicColor;
    fn tertiary_container(&self) -> &'static DynamicColor;
    fn on_tertiary_container(&self) -> &'static DynamicColor;

    fn error(&self) -> &'static DynamicColor;
    fn error_dim(&self) -> Option<&'static DynamicColor>;
    fn on_error(&self) -> &'static DynamicColor;
    fn error_container(&self) -> &'static DynamicColor;
    fn on_error_container(&self) -> &'static DynamicColor;

    fn primary_fixed(&self) -> &'static DynamicColor;
    fn primary_fixed_dim(&self) -> &'static DynamicColor;
    fn on_primary_fixed(&self) -> &'static DynamicColor;
    fn on_primary_fixed_variant(&self) -> &'static DynamicColor;

    fn secondary_fixed(&self) -> &'static DynamicColor;
    fn secondary_fixed_dim(&self) -> &'static DynamicColor;
    fn on_secondary_fixed(&self) -> &'static DynamicColor;
    fn on_secondary_fixed_variant(&self) -> &'static DynamicColor;

    fn tertiary_fixed(&self) -> &'static DynamicColor;
    fn tertiary_fixed_dim(&self) -> &'static DynamicColor;
    fn on_tertiary_fixed(&self) -> &'static DynamicColor;
    fn on_tertiary_fixed_variant(&self) -> &'static DynamicColor;

    fn get_hct(&self, scheme: &DynamicScheme, color: &DynamicColor) -> Hct;
    fn get_tone(&self, scheme: &DynamicScheme, color: &DynamicColor) -> f64;

    fn get_primary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_secondary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_tertiary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_neutral_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_neutral_variant_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
    fn get_error_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette;
}
