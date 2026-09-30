//! Tokens, or named colors, in the Material Design system.
//!
//! Every token is simply its [`Role`]; resolution against the scheme's
//! spec version happens when the color is evaluated.

use super::{DynamicColor, DynamicScheme, Role, color_spec::highest_surface};

/// Tokens, or named colors, in the Material Design system.
pub struct MaterialDynamicColors;

impl MaterialDynamicColors {
    /// All tokens, in `Role` order.
    pub const ALL: [Role; Role::COUNT] = Role::ALL;

    pub const fn highest_surface(scheme: &DynamicScheme) -> DynamicColor<'static> {
        highest_surface(scheme)
    }

    pub const fn primary_palette_key_color() -> DynamicColor<'static> {
        Role::PrimaryPaletteKeyColor.color()
    }

    pub const fn secondary_palette_key_color() -> DynamicColor<'static> {
        Role::SecondaryPaletteKeyColor.color()
    }

    pub const fn tertiary_palette_key_color() -> DynamicColor<'static> {
        Role::TertiaryPaletteKeyColor.color()
    }

    pub const fn neutral_palette_key_color() -> DynamicColor<'static> {
        Role::NeutralPaletteKeyColor.color()
    }

    pub const fn neutral_variant_palette_key_color() -> DynamicColor<'static> {
        Role::NeutralVariantPaletteKeyColor.color()
    }

    pub const fn error_palette_key_color() -> DynamicColor<'static> {
        Role::ErrorPaletteKeyColor.color()
    }

    pub const fn background() -> DynamicColor<'static> {
        Role::Background.color()
    }

    pub const fn on_background() -> DynamicColor<'static> {
        Role::OnBackground.color()
    }

    pub const fn surface() -> DynamicColor<'static> {
        Role::Surface.color()
    }

    pub const fn surface_dim() -> DynamicColor<'static> {
        Role::SurfaceDim.color()
    }

    pub const fn surface_bright() -> DynamicColor<'static> {
        Role::SurfaceBright.color()
    }

    pub const fn surface_container_lowest() -> DynamicColor<'static> {
        Role::SurfaceContainerLowest.color()
    }

    pub const fn surface_container_low() -> DynamicColor<'static> {
        Role::SurfaceContainerLow.color()
    }

    pub const fn surface_container() -> DynamicColor<'static> {
        Role::SurfaceContainer.color()
    }

    pub const fn surface_container_high() -> DynamicColor<'static> {
        Role::SurfaceContainerHigh.color()
    }

    pub const fn surface_container_highest() -> DynamicColor<'static> {
        Role::SurfaceContainerHighest.color()
    }

    pub const fn on_surface() -> DynamicColor<'static> {
        Role::OnSurface.color()
    }

    pub const fn surface_variant() -> DynamicColor<'static> {
        Role::SurfaceVariant.color()
    }

    pub const fn on_surface_variant() -> DynamicColor<'static> {
        Role::OnSurfaceVariant.color()
    }

    pub const fn inverse_surface() -> DynamicColor<'static> {
        Role::InverseSurface.color()
    }

    pub const fn inverse_on_surface() -> DynamicColor<'static> {
        Role::InverseOnSurface.color()
    }

    pub const fn outline() -> DynamicColor<'static> {
        Role::Outline.color()
    }

    pub const fn outline_variant() -> DynamicColor<'static> {
        Role::OutlineVariant.color()
    }

    pub const fn shadow() -> DynamicColor<'static> {
        Role::Shadow.color()
    }

    pub const fn scrim() -> DynamicColor<'static> {
        Role::Scrim.color()
    }

    pub const fn surface_tint() -> DynamicColor<'static> {
        Role::SurfaceTint.color()
    }

    pub const fn primary() -> DynamicColor<'static> {
        Role::Primary.color()
    }

    pub const fn primary_dim() -> DynamicColor<'static> {
        Role::PrimaryDim.color()
    }

    pub const fn on_primary() -> DynamicColor<'static> {
        Role::OnPrimary.color()
    }

    pub const fn primary_container() -> DynamicColor<'static> {
        Role::PrimaryContainer.color()
    }

    pub const fn on_primary_container() -> DynamicColor<'static> {
        Role::OnPrimaryContainer.color()
    }

    pub const fn inverse_primary() -> DynamicColor<'static> {
        Role::InversePrimary.color()
    }

    pub const fn secondary() -> DynamicColor<'static> {
        Role::Secondary.color()
    }

    pub const fn secondary_dim() -> DynamicColor<'static> {
        Role::SecondaryDim.color()
    }

    pub const fn on_secondary() -> DynamicColor<'static> {
        Role::OnSecondary.color()
    }

    pub const fn secondary_container() -> DynamicColor<'static> {
        Role::SecondaryContainer.color()
    }

    pub const fn on_secondary_container() -> DynamicColor<'static> {
        Role::OnSecondaryContainer.color()
    }

    pub const fn tertiary() -> DynamicColor<'static> {
        Role::Tertiary.color()
    }

    pub const fn tertiary_dim() -> DynamicColor<'static> {
        Role::TertiaryDim.color()
    }

    pub const fn on_tertiary() -> DynamicColor<'static> {
        Role::OnTertiary.color()
    }

    pub const fn tertiary_container() -> DynamicColor<'static> {
        Role::TertiaryContainer.color()
    }

    pub const fn on_tertiary_container() -> DynamicColor<'static> {
        Role::OnTertiaryContainer.color()
    }

    pub const fn error() -> DynamicColor<'static> {
        Role::Error.color()
    }

    pub const fn error_dim() -> DynamicColor<'static> {
        Role::ErrorDim.color()
    }

    pub const fn on_error() -> DynamicColor<'static> {
        Role::OnError.color()
    }

    pub const fn error_container() -> DynamicColor<'static> {
        Role::ErrorContainer.color()
    }

    pub const fn on_error_container() -> DynamicColor<'static> {
        Role::OnErrorContainer.color()
    }

    pub const fn primary_fixed() -> DynamicColor<'static> {
        Role::PrimaryFixed.color()
    }

    pub const fn primary_fixed_dim() -> DynamicColor<'static> {
        Role::PrimaryFixedDim.color()
    }

    pub const fn on_primary_fixed() -> DynamicColor<'static> {
        Role::OnPrimaryFixed.color()
    }

    pub const fn on_primary_fixed_variant() -> DynamicColor<'static> {
        Role::OnPrimaryFixedVariant.color()
    }

    pub const fn secondary_fixed() -> DynamicColor<'static> {
        Role::SecondaryFixed.color()
    }

    pub const fn secondary_fixed_dim() -> DynamicColor<'static> {
        Role::SecondaryFixedDim.color()
    }

    pub const fn on_secondary_fixed() -> DynamicColor<'static> {
        Role::OnSecondaryFixed.color()
    }

    pub const fn on_secondary_fixed_variant() -> DynamicColor<'static> {
        Role::OnSecondaryFixedVariant.color()
    }

    pub const fn tertiary_fixed() -> DynamicColor<'static> {
        Role::TertiaryFixed.color()
    }

    pub const fn tertiary_fixed_dim() -> DynamicColor<'static> {
        Role::TertiaryFixedDim.color()
    }

    pub const fn on_tertiary_fixed() -> DynamicColor<'static> {
        Role::OnTertiaryFixed.color()
    }

    pub const fn on_tertiary_fixed_variant() -> DynamicColor<'static> {
        Role::OnTertiaryFixedVariant.color()
    }
}
