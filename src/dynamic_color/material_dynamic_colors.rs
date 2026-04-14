// use super::{ContrastCurve, DynamicColor, DynamicScheme, ToneDeltaPair,
// TonePolarity, Variant}; #[cfg(all(not(feature = "std"), feature = "libm"))]
// #[allow(unused_imports)]
// use crate::utils::no_std::FloatExt;
// use crate::{dislike::fix_if_disliked, dynamic_color::ExtendedColorData,
// hct::Hct};

use crate::dynamic_color::{DynamicColor, color_spec_2021::ColorSpec2021};

/// Tokens, or named colors, in the Material Design system.
pub struct MaterialDynamicColors;

impl MaterialDynamicColors {
    pub const fn primary_palette_key_color() -> &'static DynamicColor {
        ColorSpec2021::primary_palette_key_color()
    }

    pub const fn secondary_palette_key_color() -> &'static DynamicColor {
        ColorSpec2021::secondary_palette_key_color()
    }

    pub const fn tertiary_palette_key_color() -> &'static DynamicColor {
        ColorSpec2021::tertiary_palette_key_color()
    }

    pub const fn neutral_palette_key_color() -> &'static DynamicColor {
        ColorSpec2021::neutral_palette_key_color()
    }

    pub const fn neutral_variant_palette_key_color() -> &'static DynamicColor {
        ColorSpec2021::neutral_palette_key_color()
    }

    pub const fn background() -> &'static DynamicColor {
        ColorSpec2021::background()
    }

    pub const fn on_background() -> &'static DynamicColor {
        ColorSpec2021::on_background()
    }

    pub const fn surface() -> &'static DynamicColor {
        ColorSpec2021::surface()
    }

    pub const fn surface_dim() -> &'static DynamicColor {
        ColorSpec2021::surface_dim()
    }

    pub const fn surface_bright() -> &'static DynamicColor {
        ColorSpec2021::surface_bright()
    }

    pub const fn surface_container_lowest() -> &'static DynamicColor {
        ColorSpec2021::surface_container_lowest()
    }

    pub const fn surface_container_low() -> &'static DynamicColor {
        ColorSpec2021::surface_container_low()
    }

    pub const fn surface_container() -> &'static DynamicColor {
        ColorSpec2021::surface_container()
    }

    pub const fn surface_container_high() -> &'static DynamicColor {
        ColorSpec2021::surface_container_high()
    }

    pub const fn surface_container_highest() -> &'static DynamicColor {
        ColorSpec2021::surface_container_highest()
    }

    pub const fn on_surface() -> &'static DynamicColor {
        ColorSpec2021::on_surface()
    }

    pub const fn surface_variant() -> &'static DynamicColor {
        ColorSpec2021::surface_variant()
    }

    pub const fn on_surface_variant() -> &'static DynamicColor {
        ColorSpec2021::on_surface_variant()
    }

    pub const fn inverse_surface() -> &'static DynamicColor {
        ColorSpec2021::inverse_surface()
    }

    pub const fn inverse_on_surface() -> &'static DynamicColor {
        ColorSpec2021::inverse_on_surface()
    }

    pub const fn outline() -> &'static DynamicColor {
        ColorSpec2021::outline()
    }

    pub const fn outline_variant() -> &'static DynamicColor {
        ColorSpec2021::outline_variant()
    }

    pub const fn shadow() -> &'static DynamicColor {
        ColorSpec2021::shadow()
    }

    pub const fn scrim() -> &'static DynamicColor {
        ColorSpec2021::scrim()
    }

    pub const fn surface_tint() -> &'static DynamicColor {
        ColorSpec2021::surface_tint()
    }

    pub const fn primary() -> &'static DynamicColor {
        ColorSpec2021::primary()
    }

    pub const fn on_primary() -> &'static DynamicColor {
        ColorSpec2021::on_primary()
    }

    pub const fn primary_container() -> &'static DynamicColor {
        ColorSpec2021::primary_container()
    }

    pub const fn on_primary_container() -> &'static DynamicColor {
        ColorSpec2021::on_primary_container()
    }

    pub const fn inverse_primary() -> &'static DynamicColor {
        ColorSpec2021::inverse_primary()
    }

    pub const fn secondary() -> &'static DynamicColor {
        ColorSpec2021::secondary()
    }

    pub const fn on_secondary() -> &'static DynamicColor {
        ColorSpec2021::on_secondary()
    }

    pub const fn secondary_container() -> &'static DynamicColor {
        ColorSpec2021::secondary_container()
    }

    pub const fn on_secondary_container() -> &'static DynamicColor {
        ColorSpec2021::on_secondary_container()
    }

    pub const fn tertiary() -> &'static DynamicColor {
        ColorSpec2021::tertiary()
    }

    pub const fn on_tertiary() -> &'static DynamicColor {
        ColorSpec2021::on_tertiary()
    }

    pub const fn tertiary_container() -> &'static DynamicColor {
        ColorSpec2021::tertiary_container()
    }

    pub const fn on_tertiary_container() -> &'static DynamicColor {
        ColorSpec2021::on_tertiary_container()
    }

    pub const fn error() -> &'static DynamicColor {
        ColorSpec2021::error()
    }

    pub const fn on_error() -> &'static DynamicColor {
        ColorSpec2021::on_error()
    }

    pub const fn error_container() -> &'static DynamicColor {
        ColorSpec2021::error_container()
    }

    pub const fn on_error_container() -> &'static DynamicColor {
        ColorSpec2021::on_error_container()
    }

    pub const fn primary_fixed() -> &'static DynamicColor {
        ColorSpec2021::primary_fixed()
    }

    pub const fn primary_fixed_dim() -> &'static DynamicColor {
        ColorSpec2021::primary_fixed_dim()
    }

    pub const fn on_primary_fixed() -> &'static DynamicColor {
        ColorSpec2021::on_primary_fixed()
    }

    pub const fn on_primary_fixed_variant() -> &'static DynamicColor {
        ColorSpec2021::on_primary_fixed_variant()
    }

    pub const fn secondary_fixed() -> &'static DynamicColor {
        ColorSpec2021::secondary_fixed()
    }

    pub const fn secondary_fixed_dim() -> &'static DynamicColor {
        ColorSpec2021::secondary_fixed_dim()
    }

    pub const fn on_secondary_fixed() -> &'static DynamicColor {
        ColorSpec2021::on_secondary_fixed()
    }

    pub const fn on_secondary_fixed_variant() -> &'static DynamicColor {
        ColorSpec2021::on_secondary_fixed_variant()
    }

    pub const fn tertiary_fixed() -> &'static DynamicColor {
        ColorSpec2021::tertiary_fixed()
    }

    pub const fn tertiary_fixed_dim() -> &'static DynamicColor {
        ColorSpec2021::tertiary_fixed_dim()
    }

    pub const fn on_tertiary_fixed() -> &'static DynamicColor {
        ColorSpec2021::on_tertiary_fixed()
    }

    pub const fn on_tertiary_fixed_variant() -> &'static DynamicColor {
        ColorSpec2021::on_tertiary_fixed_variant()
    }
}
