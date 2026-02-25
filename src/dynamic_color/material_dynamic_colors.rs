use super::{ContrastCurve, DynamicColor, DynamicScheme, ToneDeltaPair, TonePolarity, Variant};
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{dislike::fix_if_disliked, hct::Hct};

const fn is_fidelity(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Fidelity | Variant::Content)
}

const fn is_monochrome(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Monochrome)
}

/// Tokens, or named colors, in the Material Design system.
pub struct MaterialDynamicColors;

impl MaterialDynamicColors {
    pub const CONTENT_ACCENT_TONE_DELTA: f64 = 15.0;

    const fn highest_surface(scheme: &DynamicScheme) -> DynamicColor {
        if scheme.is_dark { Self::surface_bright() } else { Self::surface_dim() }
    }

    pub const fn primary_palette_key_color() -> DynamicColor {
        DynamicColor::foreground(
            "primary_palette_key_color",
            |scheme| &scheme.primary_palette,
            |scheme| scheme.primary_palette.key_color().get_tone(),
        )
    }

    pub const fn secondary_palette_key_color() -> DynamicColor {
        DynamicColor::foreground(
            "secondary_palette_key_color",
            |scheme| &scheme.secondary_palette,
            |scheme| scheme.secondary_palette.key_color().get_tone(),
        )
    }

    pub const fn tertiary_palette_key_color() -> DynamicColor {
        DynamicColor::foreground(
            "tertiary_palette_key_color",
            |scheme| &scheme.tertiary_palette,
            |scheme| scheme.tertiary_palette.key_color().get_tone(),
        )
    }

    pub const fn neutral_palette_key_color() -> DynamicColor {
        DynamicColor::foreground(
            "neutral_palette_key_color",
            |scheme| &scheme.neutral_palette,
            |scheme| scheme.neutral_palette.key_color().get_tone(),
        )
    }

    pub const fn neutral_variant_palette_key_color() -> DynamicColor {
        DynamicColor::foreground(
            "neutral_variant_palette_key_color",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| scheme.neutral_variant_palette.key_color().get_tone(),
        )
    }

    pub const fn background() -> DynamicColor {
        DynamicColor::background("background", |scheme| &scheme.neutral_palette, |scheme| if scheme.is_dark { 6.0 } else { 98.0 })
    }

    pub const fn on_background() -> DynamicColor {
        DynamicColor::foreground(
            "on_background",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        )
        .with_background(|_| Self::background())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 3.0,
            medium: 4.5,
            high: 7.0,
        })
    }

    pub const fn surface() -> DynamicColor {
        DynamicColor::background("surface", |scheme| &scheme.neutral_palette, |scheme| if scheme.is_dark { 6.0 } else { 98.0 })
    }

    pub const fn surface_dim() -> DynamicColor {
        DynamicColor::background(
            "surface_dim",
            |scheme| &scheme.neutral_palette,
            |scheme| {
                if scheme.is_dark {
                    6.0
                } else {
                    ContrastCurve {
                        low: 87.0,
                        normal: 87.0,
                        medium: 80.0,
                        high: 75.0,
                    }
                    .get(scheme.contrast_level)
                }
            },
        )
    }

    pub const fn surface_bright() -> DynamicColor {
        DynamicColor::background(
            "surface_bright",
            |scheme| &scheme.neutral_palette,
            |scheme| {
                if scheme.is_dark {
                    ContrastCurve {
                        low: 24.0,
                        normal: 24.0,
                        medium: 29.0,
                        high: 34.0,
                    }
                    .get(scheme.contrast_level)
                } else {
                    98.0
                }
            },
        )
    }

    pub const fn surface_container_lowest() -> DynamicColor {
        DynamicColor::background(
            "surface_container_lowest",
            |scheme| &scheme.neutral_palette,
            |scheme| {
                if scheme.is_dark {
                    ContrastCurve {
                        low: 4.0,
                        normal: 4.0,
                        medium: 2.0,
                        high: 0.0,
                    }
                    .get(scheme.contrast_level)
                } else {
                    100.0
                }
            },
        )
    }

    pub const fn surface_container_low() -> DynamicColor {
        DynamicColor::background(
            "surface_container_low",
            |scheme| &scheme.neutral_palette,
            |scheme| {
                if scheme.is_dark {
                    ContrastCurve {
                        low: 10.0,
                        normal: 10.0,
                        medium: 11.0,
                        high: 12.0,
                    }
                    .get(scheme.contrast_level)
                } else {
                    ContrastCurve {
                        low: 96.0,
                        normal: 96.0,
                        medium: 96.0,
                        high: 95.0,
                    }
                    .get(scheme.contrast_level)
                }
            },
        )
    }

    pub const fn surface_container() -> DynamicColor {
        DynamicColor::background(
            "surface_container",
            |scheme| &scheme.neutral_palette,
            |scheme| {
                if scheme.is_dark {
                    ContrastCurve {
                        low: 12.0,
                        normal: 12.0,
                        medium: 16.0,
                        high: 20.0,
                    }
                    .get(scheme.contrast_level)
                } else {
                    ContrastCurve {
                        low: 94.0,
                        normal: 94.0,
                        medium: 92.0,
                        high: 90.0,
                    }
                    .get(scheme.contrast_level)
                }
            },
        )
    }

    pub const fn surface_container_high() -> DynamicColor {
        DynamicColor::background(
            "surface_container_high",
            |scheme| &scheme.neutral_palette,
            |scheme| {
                if scheme.is_dark {
                    ContrastCurve {
                        low: 17.0,
                        normal: 17.0,
                        medium: 21.0,
                        high: 25.0,
                    }
                    .get(scheme.contrast_level)
                } else {
                    ContrastCurve {
                        low: 92.0,
                        normal: 92.0,
                        medium: 88.0,
                        high: 85.0,
                    }
                    .get(scheme.contrast_level)
                }
            },
        )
    }

    pub const fn surface_container_highest() -> DynamicColor {
        DynamicColor::background(
            "surface_container_highest",
            |scheme| &scheme.neutral_palette,
            |scheme| {
                if scheme.is_dark {
                    ContrastCurve {
                        low: 22.0,
                        normal: 22.0,
                        medium: 26.0,
                        high: 30.0,
                    }
                    .get(scheme.contrast_level)
                } else {
                    ContrastCurve {
                        low: 90.0,
                        normal: 90.0,
                        medium: 84.0,
                        high: 80.0,
                    }
                    .get(scheme.contrast_level)
                }
            },
        )
    }

    pub const fn on_surface() -> DynamicColor {
        DynamicColor::foreground(
            "on_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 4.5,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        })
    }

    pub const fn surface_variant() -> DynamicColor {
        DynamicColor::background(
            "surface_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 90.0 },
        )
    }

    pub const fn on_surface_variant() -> DynamicColor {
        DynamicColor::foreground(
            "on_surface_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 30.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    pub const fn inverse_surface() -> DynamicColor {
        DynamicColor::background(
            "inverse_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 20.0 },
        )
    }

    pub const fn inverse_on_surface() -> DynamicColor {
        DynamicColor::foreground(
            "inverse_on_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 20.0 } else { 95.0 },
        )
        .with_background(|_| Self::inverse_surface())
        .with_contrast_curve(ContrastCurve {
            low: 4.5,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        })
    }

    pub const fn outline() -> DynamicColor {
        DynamicColor::foreground(
            "outline",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 60.0 } else { 50.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.5,
            normal: 3.0,
            medium: 4.5,
            high: 7.0,
        })
    }

    pub const fn outline_variant() -> DynamicColor {
        DynamicColor::foreground(
            "outline_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
    }

    pub const fn shadow() -> DynamicColor {
        DynamicColor::foreground("shadow", |scheme| &scheme.neutral_palette, |_| 0.0)
    }

    pub const fn scrim() -> DynamicColor {
        DynamicColor::foreground("scrim", |scheme| &scheme.neutral_palette, |_| 0.0)
    }

    pub const fn surface_tint() -> DynamicColor {
        DynamicColor::background(
            "surface_tint",
            |scheme| &scheme.primary_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        )
    }

    pub const fn primary() -> DynamicColor {
        DynamicColor::background(
            "primary",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 100.0 } else { 0.0 }
                } else if scheme.is_dark {
                    80.0
                } else {
                    40.0
                }
            },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 7.0,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::primary_container(), Self::primary(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_primary() -> DynamicColor {
        DynamicColor::foreground(
            "on_primary",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 10.0 } else { 90.0 }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
        )
        .with_background(|_| Self::primary())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        })
    }

    pub const fn primary_container() -> DynamicColor {
        DynamicColor::background(
            "primary_container",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if is_fidelity(scheme) {
                    scheme.source_color_hct.get_tone()
                } else if is_monochrome(scheme) {
                    if scheme.is_dark { 85.0 } else { 25.0 }
                } else if scheme.is_dark {
                    30.0
                } else {
                    90.0
                }
            },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::primary_container(), Self::primary(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_primary_container() -> DynamicColor {
        DynamicColor::foreground(
            "on_primary_container",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(Self::primary_container().get_tone(scheme), 4.5)
                } else if is_monochrome(scheme) {
                    if scheme.is_dark { 0.0 } else { 100.0 }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_| Self::primary_container())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    pub const fn inverse_primary() -> DynamicColor {
        DynamicColor::foreground(
            "inverse_primary",
            |scheme| &scheme.primary_palette,
            |scheme| if scheme.is_dark { 40.0 } else { 80.0 },
        )
        .with_background(|_| Self::inverse_surface())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 7.0,
        })
    }

    pub const fn secondary() -> DynamicColor {
        DynamicColor::background(
            "secondary",
            |scheme| &scheme.secondary_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 7.0,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::secondary_container(), Self::secondary(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_secondary() -> DynamicColor {
        DynamicColor::foreground(
            "on_secondary",
            |scheme| &scheme.secondary_palette,
            |scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 10.0 } else { 100.0 }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
        )
        .with_background(|_| Self::secondary())
        .with_contrast_curve(ContrastCurve {
            low: 4.5,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        })
    }

    pub const fn secondary_container() -> DynamicColor {
        DynamicColor::background(
            "secondary_container",
            |scheme| &scheme.secondary_palette,
            |scheme| {
                let initial_tone = if scheme.is_dark { 30.0 } else { 90.0 };

                if is_monochrome(scheme) {
                    if scheme.is_dark { 30.0 } else { 90.0 }
                } else if !is_fidelity(scheme) {
                    initial_tone
                } else {
                    Self::_find_desired_chroma_by_tone(scheme.secondary_palette.hue(), scheme.secondary_palette.chroma(), initial_tone, !scheme.is_dark)
                }
            },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::secondary_container(), Self::secondary(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_secondary_container() -> DynamicColor {
        DynamicColor::foreground(
            "on_secondary_container",
            |scheme| &scheme.secondary_palette,
            |scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone((Self::secondary_container().tone)(scheme), 4.5)
                } else if scheme.is_dark {
                    90.0
                } else if is_monochrome(scheme) {
                    30.0
                } else {
                    10.0
                }
            },
        )
        .with_background(|_| Self::secondary_container())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    pub const fn tertiary() -> DynamicColor {
        DynamicColor::background(
            "tertiary",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 90.0 } else { 25.0 }
                } else if scheme.is_dark {
                    80.0
                } else {
                    40.0
                }
            },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 7.0,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::tertiary_container(), Self::tertiary(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_tertiary() -> DynamicColor {
        DynamicColor::foreground(
            "on_tertiary",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 10.0 } else { 90.0 }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
        )
        .with_background(|_| Self::tertiary())
        .with_contrast_curve(ContrastCurve {
            low: 4.5,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        })
    }

    pub const fn tertiary_container() -> DynamicColor {
        DynamicColor::background(
            "tertiary_container",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 60.0 } else { 49.0 }
                } else if !is_fidelity(scheme) {
                    if scheme.is_dark { 30.0 } else { 90.0 }
                } else {
                    fix_if_disliked(scheme.tertiary_palette.get_hct(scheme.source_color_hct.get_tone())).get_tone()
                }
            },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::tertiary_container(), Self::tertiary(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_tertiary_container() -> DynamicColor {
        DynamicColor::foreground(
            "on_tertiary_container",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(Self::tertiary_container().get_tone(scheme), 4.5)
                } else if is_monochrome(scheme) {
                    if scheme.is_dark { 0.0 } else { 100.0 }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_| Self::tertiary_container())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    pub const fn error() -> DynamicColor {
        DynamicColor::background("error", |scheme| &scheme.error_palette, |scheme| if scheme.is_dark { 80.0 } else { 40.0 })
            .with_background(Self::highest_surface)
            .with_contrast_curve(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
            .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::error_container(), Self::error(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_error() -> DynamicColor {
        DynamicColor::foreground("on_error", |scheme| &scheme.error_palette, |scheme| if scheme.is_dark { 20.0 } else { 100.0 })
            .with_background(|_| Self::error())
            .with_contrast_curve(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
    }

    pub const fn error_container() -> DynamicColor {
        DynamicColor::background(
            "error_container",
            |scheme| &scheme.error_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::error_container(), Self::error(), 10.0, TonePolarity::Nearer, false))
    }

    pub const fn on_error_container() -> DynamicColor {
        DynamicColor::foreground(
            "on_error_container",
            |scheme| &scheme.error_palette,
            |scheme| {
                if scheme.is_dark {
                    90.0
                } else if is_monochrome(scheme) {
                    10.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_| Self::error_container())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    pub const fn primary_fixed() -> DynamicColor {
        DynamicColor::background(
            "primary_fixed",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 40.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::primary_fixed(), Self::primary_fixed_dim(), 10.0, TonePolarity::Lighter, true))
    }

    pub const fn primary_fixed_dim() -> DynamicColor {
        DynamicColor::background(
            "primary_fixed_dim",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 30.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::primary_fixed(), Self::primary_fixed_dim(), 10.0, TonePolarity::Lighter, true))
    }

    pub const fn on_primary_fixed() -> DynamicColor {
        DynamicColor::foreground(
            "on_primary_fixed",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 100.0 } else { 10.0 },
        )
        .with_background(|_| Self::primary_fixed_dim())
        .with_second_background(|_| Self::primary_fixed())
        .with_contrast_curve(ContrastCurve {
            low: 4.5,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        })
    }

    pub const fn on_primary_fixed_variant() -> DynamicColor {
        DynamicColor::foreground(
            "on_primary_fixed_variant",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 90.0 } else { 30.0 },
        )
        .with_background(|_| Self::primary_fixed_dim())
        .with_second_background(|_| Self::primary_fixed())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    pub const fn secondary_fixed() -> DynamicColor {
        DynamicColor::background(
            "secondary_fixed",
            |scheme| &scheme.secondary_palette,
            |scheme| if is_monochrome(scheme) { 80.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::secondary_fixed(), Self::secondary_fixed_dim(), 10.0, TonePolarity::Lighter, true))
    }

    pub const fn secondary_fixed_dim() -> DynamicColor {
        DynamicColor::background(
            "secondary_fixed_dim",
            |scheme| &scheme.secondary_palette,
            |scheme| if is_monochrome(scheme) { 70.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::secondary_fixed(), Self::secondary_fixed_dim(), 10.0, TonePolarity::Lighter, true))
    }

    pub const fn on_secondary_fixed() -> DynamicColor {
        DynamicColor::foreground("on_secondary_fixed", |scheme| &scheme.secondary_palette, |_| 10.0)
            .with_background(|_| Self::secondary_fixed_dim())
            .with_second_background(|_| Self::secondary_fixed())
            .with_contrast_curve(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
    }

    pub const fn on_secondary_fixed_variant() -> DynamicColor {
        DynamicColor::foreground(
            "on_secondary_fixed_variant",
            |scheme| &scheme.secondary_palette,
            |scheme| if is_monochrome(scheme) { 25.0 } else { 30.0 },
        )
        .with_background(|_| Self::secondary_fixed_dim())
        .with_second_background(|_| Self::secondary_fixed())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    pub const fn tertiary_fixed() -> DynamicColor {
        DynamicColor::background(
            "tertiary_fixed",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 40.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::tertiary_fixed(), Self::tertiary_fixed_dim(), 10.0, TonePolarity::Lighter, true))
    }

    pub const fn tertiary_fixed_dim() -> DynamicColor {
        DynamicColor::background(
            "tertiary_fixed_dim",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 30.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(ContrastCurve {
            low: 1.0,
            normal: 1.0,
            medium: 3.0,
            high: 4.5,
        })
        .with_tone_delta_pair(|_| ToneDeltaPair::new(Self::tertiary_fixed(), Self::tertiary_fixed_dim(), 10.0, TonePolarity::Lighter, true))
    }

    pub const fn on_tertiary_fixed() -> DynamicColor {
        DynamicColor::foreground(
            "on_tertiary_fixed",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 100.0 } else { 10.0 },
        )
        .with_background(|_| Self::tertiary_fixed_dim())
        .with_second_background(|_| Self::tertiary_fixed())
        .with_contrast_curve(ContrastCurve {
            low: 4.5,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        })
    }

    pub const fn on_tertiary_fixed_variant() -> DynamicColor {
        DynamicColor::foreground(
            "on_tertiary_fixed_variant",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 90.0 } else { 30.0 },
        )
        .with_background(|_| Self::tertiary_fixed_dim())
        .with_second_background(|_| Self::tertiary_fixed())
        .with_contrast_curve(ContrastCurve {
            low: 3.0,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        })
    }

    fn _find_desired_chroma_by_tone(hue: f64, chroma: f64, tone: f64, by_decreasing_tone: bool) -> f64 {
        let mut answer = tone;

        let mut closest_to_chroma = Hct::from(hue, chroma, tone);

        if closest_to_chroma.get_chroma() < chroma {
            let mut chroma_peak = closest_to_chroma.get_chroma();

            while closest_to_chroma.get_chroma() < chroma {
                answer += if by_decreasing_tone { -1.0 } else { 1.0 };

                let potential_solution = Hct::from(hue, chroma, answer);

                if chroma_peak > potential_solution.get_chroma() {
                    break;
                }

                if (potential_solution.get_chroma() - chroma).abs() < 0.4 {
                    break;
                }

                let (potential_delta, current_delta) = (
                    (potential_solution.get_chroma() - chroma).abs(),
                    (closest_to_chroma.get_chroma() - chroma).abs(),
                );

                if potential_delta < current_delta {
                    closest_to_chroma = potential_solution;
                }

                chroma_peak = chroma_peak.max(potential_solution.get_chroma());
            }
        }

        answer
    }
}
