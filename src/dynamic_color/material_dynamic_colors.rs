use super::{ContrastCurve, DynamicColor, DynamicScheme, ToneDeltaPair, TonePolarity, Variant};
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{dislike::fix_if_disliked, hct::Hct};

const fn _is_fidelity(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Fidelity) || matches!(scheme.variant, Variant::Content)
}

const fn _is_monochrome(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Monochrome)
}

/// Tokens, or named colors, in the Material Design system.
pub struct MaterialDynamicColors;

impl MaterialDynamicColors {
    pub const CONTENT_ACCENT_TONE_DELTA: f64 = 15.0;

    fn highest_surface(scheme: &DynamicScheme) -> DynamicColor {
        if scheme.is_dark {
            Self::surface_bright()
        } else {
            Self::surface_dim()
        }
    }

    pub fn primary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            "primary_palette_key_color",
            |scheme| &scheme.primary_palette,
            |scheme| scheme.primary_palette.key_color().get_tone(),
        )
    }

    pub fn secondary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            "secondary_palette_key_color",
            |scheme| &scheme.secondary_palette,
            |scheme| scheme.secondary_palette.key_color().get_tone(),
        )
    }

    pub fn tertiary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            "tertiary_palette_key_color",
            |scheme| &scheme.tertiary_palette,
            |scheme| scheme.tertiary_palette.key_color().get_tone(),
        )
    }

    pub fn neutral_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            "neutral_palette_key_color",
            |scheme| &scheme.neutral_palette,
            |scheme| scheme.neutral_palette.key_color().get_tone(),
        )
    }

    pub fn neutral_variant_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            "neutral_variant_palette_key_color",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| scheme.neutral_variant_palette.key_color().get_tone(),
        )
    }

    pub fn background() -> DynamicColor {
        DynamicColor::new(
            "background",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 6.0 } else { 98.0 },
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_background() -> DynamicColor {
        DynamicColor::new(
            "on_background",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
            false,
            Some(|_scheme| Self::background()),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 3.0,
                medium: 4.5,
                high: 7.0,
            }),
            None,
        )
    }

    pub fn surface() -> DynamicColor {
        DynamicColor::new(
            "surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 6.0 } else { 98.0 },
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_dim() -> DynamicColor {
        DynamicColor::new(
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
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_bright() -> DynamicColor {
        DynamicColor::new(
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
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container_lowest() -> DynamicColor {
        DynamicColor::new(
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
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container_low() -> DynamicColor {
        DynamicColor::new(
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
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container() -> DynamicColor {
        DynamicColor::new(
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
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container_high() -> DynamicColor {
        DynamicColor::new(
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
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container_highest() -> DynamicColor {
        DynamicColor::new(
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
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_surface() -> DynamicColor {
        DynamicColor::new(
            "on_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
            false,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn surface_variant() -> DynamicColor {
        DynamicColor::new(
            "surface_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 90.0 },
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_surface_variant() -> DynamicColor {
        DynamicColor::new(
            "on_surface_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 30.0 },
            false,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    pub fn inverse_surface() -> DynamicColor {
        DynamicColor::new(
            "inverse_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 20.0 },
            false,
            None,
            None,
            None,
            None,
        )
    }

    pub fn inverse_on_surface() -> DynamicColor {
        DynamicColor::new(
            "inverse_on_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 20.0 } else { 95.0 },
            false,
            Some(|_scheme| Self::inverse_surface()),
            None,
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn outline() -> DynamicColor {
        DynamicColor::new(
            "outline",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 60.0 } else { 50.0 },
            false,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.5,
                normal: 3.0,
                medium: 4.5,
                high: 7.0,
            }),
            None,
        )
    }

    pub fn outline_variant() -> DynamicColor {
        DynamicColor::new(
            "outline_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 80.0 },
            false,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            None,
        )
    }

    pub fn shadow() -> DynamicColor {
        DynamicColor::new(
            "shadow",
            |scheme| &scheme.neutral_palette,
            |_scheme| 0.0,
            false,
            None,
            None,
            None,
            None,
        )
    }

    pub fn scrim() -> DynamicColor {
        DynamicColor::new(
            "scrim",
            |scheme| &scheme.neutral_palette,
            |_scheme| 0.0,
            false,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_tint() -> DynamicColor {
        DynamicColor::new(
            "surface_tint",
            |scheme| &scheme.primary_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
            true,
            None,
            None,
            None,
            None,
        )
    }

    pub fn primary() -> DynamicColor {
        DynamicColor::new(
            "primary",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        100.0
                    } else {
                        0.0
                    }
                } else if scheme.is_dark {
                    80.0
                } else {
                    40.0
                }
            },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::primary_container(),
                    Self::primary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_primary() -> DynamicColor {
        DynamicColor::new(
            "on_primary",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        10.0
                    } else {
                        90.0
                    }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
            false,
            Some(|_scheme| Self::primary()),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn primary_container() -> DynamicColor {
        DynamicColor::new(
            "primary_container",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if _is_fidelity(scheme) {
                    scheme.source_color_hct.get_tone()
                } else if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        85.0
                    } else {
                        25.0
                    }
                } else if scheme.is_dark {
                    30.0
                } else {
                    90.0
                }
            },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::primary_container(),
                    Self::primary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_primary_container() -> DynamicColor {
        DynamicColor::new(
            "on_primary_container",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if _is_fidelity(scheme) {
                    DynamicColor::foreground_tone(Self::primary_container().get_tone(scheme), 4.5)
                } else if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        0.0
                    } else {
                        100.0
                    }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
            false,
            Some(|_scheme| Self::primary_container()),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    pub fn inverse_primary() -> DynamicColor {
        DynamicColor::new(
            "inverse_primary",
            |scheme| &scheme.primary_palette,
            |scheme| if scheme.is_dark { 40.0 } else { 80.0 },
            false,
            Some(|_scheme| Self::inverse_surface()),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            }),
            None,
        )
    }

    pub fn secondary() -> DynamicColor {
        DynamicColor::new(
            "secondary",
            |scheme| &scheme.secondary_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::secondary_container(),
                    Self::secondary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_secondary() -> DynamicColor {
        DynamicColor::new(
            "on_secondary",
            |scheme| &scheme.secondary_palette,
            |scheme| {
                if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        10.0
                    } else {
                        100.0
                    }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
            false,
            Some(|_scheme| Self::secondary()),
            None,
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn secondary_container() -> DynamicColor {
        DynamicColor::new(
            "secondary_container",
            |scheme| &scheme.secondary_palette,
            |scheme| {
                let initial_tone = if scheme.is_dark { 30.0 } else { 90.0 };

                if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        30.0
                    } else {
                        90.0
                    }
                } else if !_is_fidelity(scheme) {
                    initial_tone
                } else {
                    Self::_find_desired_chroma_by_tone(
                        scheme.secondary_palette.hue(),
                        scheme.secondary_palette.chroma(),
                        initial_tone,
                        !scheme.is_dark,
                    )
                }
            },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::secondary_container(),
                    Self::secondary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_secondary_container() -> DynamicColor {
        DynamicColor::new(
            "on_secondary_container",
            |scheme| &scheme.secondary_palette,
            |scheme| {
                if _is_fidelity(scheme) {
                    DynamicColor::foreground_tone((Self::secondary_container().tone)(scheme), 4.5)
                } else if scheme.is_dark {
                    90.0
                } else if _is_monochrome(scheme) {
                    30.0
                } else {
                    10.0
                }
            },
            false,
            Some(|_scheme| Self::secondary_container()),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    pub fn tertiary() -> DynamicColor {
        DynamicColor::new(
            "tertiary",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        90.0
                    } else {
                        25.0
                    }
                } else if scheme.is_dark {
                    80.0
                } else {
                    40.0
                }
            },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::tertiary_container(),
                    Self::tertiary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_tertiary() -> DynamicColor {
        DynamicColor::new(
            "on_tertiary",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        10.0
                    } else {
                        90.0
                    }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
            false,
            Some(|_scheme| Self::tertiary()),
            None,
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn tertiary_container() -> DynamicColor {
        DynamicColor::new(
            "tertiary_container",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        60.0
                    } else {
                        49.0
                    }
                } else if !_is_fidelity(scheme) {
                    if scheme.is_dark {
                        30.0
                    } else {
                        90.0
                    }
                } else {
                    fix_if_disliked(
                        scheme
                            .tertiary_palette
                            .get_hct(scheme.source_color_hct.get_tone()),
                    )
                    .get_tone()
                }
            },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::tertiary_container(),
                    Self::tertiary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_tertiary_container() -> DynamicColor {
        DynamicColor::new(
            "on_tertiary_container",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if _is_fidelity(scheme) {
                    DynamicColor::foreground_tone(Self::tertiary_container().get_tone(scheme), 4.5)
                } else if _is_monochrome(scheme) {
                    if scheme.is_dark {
                        0.0
                    } else {
                        100.0
                    }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
            false,
            Some(|_scheme| Self::tertiary_container()),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    pub fn error() -> DynamicColor {
        DynamicColor::new(
            "error",
            |scheme| &scheme.error_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::error_container(),
                    Self::error(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_error() -> DynamicColor {
        DynamicColor::new(
            "on_error",
            |scheme| &scheme.error_palette,
            |scheme| if scheme.is_dark { 20.0 } else { 100.0 },
            false,
            Some(|_scheme| Self::error()),
            None,
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn error_container() -> DynamicColor {
        DynamicColor::new(
            "error_container",
            |scheme| &scheme.error_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 90.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::error_container(),
                    Self::error(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            }),
        )
    }

    pub fn on_error_container() -> DynamicColor {
        DynamicColor::new(
            "on_error_container",
            |scheme| &scheme.error_palette,
            |scheme| {
                if scheme.is_dark {
                    90.0
                } else if _is_monochrome(scheme) {
                    10.0
                } else {
                    30.0
                }
            },
            false,
            Some(|_scheme| Self::error_container()),
            None,
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    pub fn primary_fixed() -> DynamicColor {
        DynamicColor::new(
            "primary_fixed",
            |scheme| &scheme.primary_palette,
            |scheme| if _is_monochrome(scheme) { 40.0 } else { 90.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::primary_fixed(),
                    Self::primary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            }),
        )
    }

    pub fn primary_fixed_dim() -> DynamicColor {
        DynamicColor::new(
            "primary_fixed_dim",
            |scheme| &scheme.primary_palette,
            |scheme| if _is_monochrome(scheme) { 30.0 } else { 80.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::primary_fixed(),
                    Self::primary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            }),
        )
    }

    pub fn on_primary_fixed() -> DynamicColor {
        DynamicColor::new(
            "on_primary_fixed",
            |scheme| &scheme.primary_palette,
            |scheme| if _is_monochrome(scheme) { 100.0 } else { 10.0 },
            false,
            Some(|_scheme| Self::primary_fixed_dim()),
            Some(|_scheme| Self::primary_fixed()),
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn on_primary_fixed_variant() -> DynamicColor {
        DynamicColor::new(
            "on_primary_fixed_variant",
            |scheme| &scheme.primary_palette,
            |scheme| if _is_monochrome(scheme) { 90.0 } else { 30.0 },
            false,
            Some(|_scheme| Self::primary_fixed_dim()),
            Some(|_scheme| Self::primary_fixed()),
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    pub fn secondary_fixed() -> DynamicColor {
        DynamicColor::new(
            "secondary_fixed",
            |scheme| &scheme.secondary_palette,
            |scheme| if _is_monochrome(scheme) { 80.0 } else { 90.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::secondary_fixed(),
                    Self::secondary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            }),
        )
    }

    pub fn secondary_fixed_dim() -> DynamicColor {
        DynamicColor::new(
            "secondary_fixed_dim",
            |scheme| &scheme.secondary_palette,
            |scheme| if _is_monochrome(scheme) { 70.0 } else { 80.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::secondary_fixed(),
                    Self::secondary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            }),
        )
    }

    pub fn on_secondary_fixed() -> DynamicColor {
        DynamicColor::new(
            "on_secondary_fixed",
            |scheme| &scheme.secondary_palette,
            |_scheme| 10.0,
            false,
            Some(|_scheme| Self::secondary_fixed_dim()),
            Some(|_scheme| Self::secondary_fixed()),
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn on_secondary_fixed_variant() -> DynamicColor {
        DynamicColor::new(
            "on_secondary_fixed_variant",
            |scheme| &scheme.secondary_palette,
            |scheme| if _is_monochrome(scheme) { 25.0 } else { 30.0 },
            false,
            Some(|_scheme| Self::secondary_fixed_dim()),
            Some(|_scheme| Self::secondary_fixed()),
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    pub fn tertiary_fixed() -> DynamicColor {
        DynamicColor::new(
            "tertiary_fixed",
            |scheme| &scheme.tertiary_palette,
            |scheme| if _is_monochrome(scheme) { 40.0 } else { 90.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::tertiary_fixed(),
                    Self::tertiary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            }),
        )
    }

    pub fn tertiary_fixed_dim() -> DynamicColor {
        DynamicColor::new(
            "tertiary_fixed_dim",
            |scheme| &scheme.tertiary_palette,
            |scheme| if _is_monochrome(scheme) { 30.0 } else { 80.0 },
            true,
            Some(Self::highest_surface),
            None,
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            }),
            Some(|_scheme| {
                ToneDeltaPair::new(
                    Self::tertiary_fixed(),
                    Self::tertiary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            }),
        )
    }

    pub fn on_tertiary_fixed() -> DynamicColor {
        DynamicColor::new(
            "on_tertiary_fixed",
            |scheme| &scheme.tertiary_palette,
            |scheme| if _is_monochrome(scheme) { 100.0 } else { 10.0 },
            false,
            Some(|_scheme| Self::tertiary_fixed_dim()),
            Some(|_scheme| Self::tertiary_fixed()),
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            }),
            None,
        )
    }

    pub fn on_tertiary_fixed_variant() -> DynamicColor {
        DynamicColor::new(
            "on_tertiary_fixed_variant",
            |scheme| &scheme.tertiary_palette,
            |scheme| if _is_monochrome(scheme) { 90.0 } else { 30.0 },
            false,
            Some(|_scheme| Self::tertiary_fixed_dim()),
            Some(|_scheme| Self::tertiary_fixed()),
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            }),
            None,
        )
    }

    fn _find_desired_chroma_by_tone(
        hue: f64,
        chroma: f64,
        tone: f64,
        by_decreasing_tone: bool,
    ) -> f64 {
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
