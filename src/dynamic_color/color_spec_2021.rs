use super::{DynamicColor, DynamicScheme, Variant, dynamic_scheme::Platform};
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    contrast::{darker, lighter, ratio_of_tones},
    dislike::fix_if_disliked,
    dynamic_color::{ContrastCurve, ToneDeltaPair, TonePolarity, color_spec::ColorSpec, tone_delta_pair::DeltaConstraint},
    hct::Hct,
    palette::TonalPalette,
};

const fn is_fidelity(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Fidelity | Variant::Content)
}

const fn is_monochrome(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Monochrome)
}

pub struct ColorSpec2021;

impl ColorSpec2021 {
    pub const CONTENT_ACCENT_TONE_DELTA: f64 = 15.0;

    pub const fn highest_surface(scheme: &DynamicScheme) -> Option<DynamicColor> {
        Some(if scheme.is_dark { Self::surface_bright() } else { Self::surface_dim() })
    }

    pub(super) fn _find_desired_chroma_by_tone(hue: f64, chroma: f64, tone: f64, by_decreasing_tone: bool) -> f64 {
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

    pub const fn primary_palette_key_color() -> DynamicColor {
        DynamicColor::foreground_color(
            "primary_palette_key_color",
            |scheme| &scheme.primary_palette,
            |scheme| scheme.primary_palette.key_color().get_tone(),
        )
    }

    pub const fn secondary_palette_key_color() -> DynamicColor {
        DynamicColor::foreground_color(
            "secondary_palette_key_color",
            |scheme| &scheme.secondary_palette,
            |scheme| scheme.secondary_palette.key_color().get_tone(),
        )
    }

    pub const fn tertiary_palette_key_color() -> DynamicColor {
        DynamicColor::foreground_color(
            "tertiary_palette_key_color",
            |scheme| &scheme.tertiary_palette,
            |scheme| scheme.tertiary_palette.key_color().get_tone(),
        )
    }

    pub const fn neutral_palette_key_color() -> DynamicColor {
        DynamicColor::foreground_color(
            "neutral_palette_key_color",
            |scheme| &scheme.neutral_palette,
            |scheme| scheme.neutral_palette.key_color().get_tone(),
        )
    }

    pub const fn neutral_variant_palette_key_color() -> DynamicColor {
        DynamicColor::foreground_color(
            "neutral_variant_palette_key_color",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| scheme.neutral_variant_palette.key_color().get_tone(),
        )
    }

    pub const fn error_palette_key_color() -> DynamicColor {
        DynamicColor::foreground_color(
            "error_palette_key_color",
            |scheme| &scheme.error_palette,
            |scheme| scheme.error_palette.key_color().get_tone(),
        )
    }

    pub const fn background() -> DynamicColor {
        DynamicColor::background_color("background", |scheme| &scheme.neutral_palette, |scheme| if scheme.is_dark { 6.0 } else { 98.0 })
    }

    pub const fn on_background() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_background",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        )
        .with_background(|_| Some(Self::background()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 3.0,
                medium: 4.5,
                high: 7.0,
            })
        })
    }

    pub const fn surface() -> DynamicColor {
        DynamicColor::background_color("surface", |scheme| &scheme.neutral_palette, |scheme| if scheme.is_dark { 6.0 } else { 98.0 })
    }

    pub const fn surface_dim() -> DynamicColor {
        DynamicColor::background_color(
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
        DynamicColor::background_color(
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
        DynamicColor::background_color(
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
        DynamicColor::background_color(
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
        DynamicColor::background_color(
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
        DynamicColor::background_color(
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
        DynamicColor::background_color(
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
        DynamicColor::foreground_color(
            "on_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        })
    }

    pub const fn surface_variant() -> DynamicColor {
        DynamicColor::background_color(
            "surface_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 90.0 },
        )
    }

    pub const fn on_surface_variant() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_surface_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 30.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn inverse_surface() -> DynamicColor {
        DynamicColor::background_color(
            "inverse_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 90.0 } else { 20.0 },
        )
    }

    pub const fn inverse_on_surface() -> DynamicColor {
        DynamicColor::foreground_color(
            "inverse_on_surface",
            |scheme| &scheme.neutral_palette,
            |scheme| if scheme.is_dark { 20.0 } else { 95.0 },
        )
        .with_background(|_| Some(Self::inverse_surface()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        })
    }

    pub const fn outline() -> DynamicColor {
        DynamicColor::foreground_color(
            "outline",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 60.0 } else { 50.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.5,
                normal: 3.0,
                medium: 4.5,
                high: 7.0,
            })
        })
    }

    pub const fn outline_variant() -> DynamicColor {
        DynamicColor::foreground_color(
            "outline_variant",
            |scheme| &scheme.neutral_variant_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
    }

    pub const fn shadow() -> DynamicColor {
        DynamicColor::foreground_color("shadow", |scheme| &scheme.neutral_palette, |_| 0.0)
    }

    pub const fn scrim() -> DynamicColor {
        DynamicColor::foreground_color("scrim", |scheme| &scheme.neutral_palette, |_| 0.0)
    }

    pub const fn surface_tint() -> DynamicColor {
        DynamicColor::background_color(
            "surface_tint",
            |scheme| &scheme.primary_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        )
    }

    pub const fn primary() -> DynamicColor {
        DynamicColor::background_color(
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
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::primary_container(),
                Self::primary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        })
    }

    pub const fn on_primary() -> DynamicColor {
        DynamicColor::foreground_color(
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
        .with_background(|_| Some(Self::primary()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        })
    }

    pub const fn primary_container() -> DynamicColor {
        DynamicColor::background_color(
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
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::primary_container(),
                Self::primary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        })
    }

    pub const fn on_primary_container() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_primary_container",
            |scheme| &scheme.primary_palette,
            |scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(Self::primary_container().tone(scheme), 4.5)
                } else if is_monochrome(scheme) {
                    if scheme.is_dark { 0.0 } else { 100.0 }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_| Some(Self::primary_container()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn inverse_primary() -> DynamicColor {
        DynamicColor::foreground_color(
            "inverse_primary",
            |scheme| &scheme.primary_palette,
            |scheme| if scheme.is_dark { 40.0 } else { 80.0 },
        )
        .with_background(|_| Some(Self::inverse_surface()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        })
    }

    pub const fn secondary() -> DynamicColor {
        DynamicColor::background_color(
            "secondary",
            |scheme| &scheme.secondary_palette,
            |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::secondary_container(),
                Self::secondary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        })
    }

    pub const fn on_secondary() -> DynamicColor {
        DynamicColor::foreground_color(
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
        .with_background(|_| Some(Self::secondary()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        })
    }

    pub const fn secondary_container() -> DynamicColor {
        DynamicColor::background_color(
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
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::secondary_container(),
                Self::secondary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        })
    }

    pub const fn on_secondary_container() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_secondary_container",
            |scheme| &scheme.secondary_palette,
            |scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(Self::secondary_container().tone(scheme), 4.5)
                } else if scheme.is_dark {
                    90.0
                } else if is_monochrome(scheme) {
                    30.0
                } else {
                    10.0
                }
            },
        )
        .with_background(|_| Some(Self::secondary_container()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn tertiary() -> DynamicColor {
        DynamicColor::background_color(
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
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::tertiary_container(),
                Self::tertiary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        })
    }

    pub const fn on_tertiary() -> DynamicColor {
        DynamicColor::foreground_color(
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
        .with_background(|_| Some(Self::tertiary()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        })
    }

    pub const fn tertiary_container() -> DynamicColor {
        DynamicColor::background_color(
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
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::tertiary_container(),
                Self::tertiary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        })
    }

    pub const fn on_tertiary_container() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_tertiary_container",
            |scheme| &scheme.tertiary_palette,
            |scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(Self::tertiary_container().tone(scheme), 4.5)
                } else if is_monochrome(scheme) {
                    if scheme.is_dark { 0.0 } else { 100.0 }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_| Some(Self::tertiary_container()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn error() -> DynamicColor {
        DynamicColor::background_color("error", |scheme| &scheme.error_palette, |scheme| if scheme.is_dark { 80.0 } else { 40.0 })
            .with_background(Self::highest_surface)
            .with_contrast_curve(|_| {
                Some(ContrastCurve {
                    low: 3.0,
                    normal: 4.5,
                    medium: 7.0,
                    high: 7.0,
                })
            })
            .with_tone_delta_pair(|_| {
                Some(ToneDeltaPair::new(
                    Self::error_container(),
                    Self::error(),
                    10.0,
                    TonePolarity::RelativeLighter,
                    false,
                    DeltaConstraint::Nearer,
                ))
            })
    }

    pub const fn on_error() -> DynamicColor {
        DynamicColor::foreground_color("on_error", |scheme| &scheme.error_palette, |scheme| if scheme.is_dark { 20.0 } else { 100.0 })
            .with_background(|_| Some(Self::error()))
            .with_contrast_curve(|_| {
                Some(ContrastCurve {
                    low: 4.5,
                    normal: 7.0,
                    medium: 11.0,
                    high: 21.0,
                })
            })
    }

    pub const fn error_container() -> DynamicColor {
        DynamicColor::background_color(
            "error_container",
            |scheme| &scheme.error_palette,
            |scheme| if scheme.is_dark { 30.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::error_container(),
                Self::error(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        })
    }

    pub const fn on_error_container() -> DynamicColor {
        DynamicColor::foreground_color(
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
        .with_background(|_| Some(Self::error_container()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn primary_fixed() -> DynamicColor {
        DynamicColor::background_color(
            "primary_fixed",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 40.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::primary_fixed(),
                Self::primary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        })
    }

    pub const fn primary_fixed_dim() -> DynamicColor {
        DynamicColor::background_color(
            "primary_fixed_dim",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 30.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::primary_fixed(),
                Self::primary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        })
    }

    pub const fn on_primary_fixed() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_primary_fixed",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 100.0 } else { 10.0 },
        )
        .with_background(|_| Some(Self::primary_fixed_dim()))
        .with_second_background(|_| Some(Self::primary_fixed()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        })
    }

    pub const fn on_primary_fixed_variant() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_primary_fixed_variant",
            |scheme| &scheme.primary_palette,
            |scheme| if is_monochrome(scheme) { 90.0 } else { 30.0 },
        )
        .with_background(|_| Some(Self::primary_fixed_dim()))
        .with_second_background(|_| Some(Self::primary_fixed()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn secondary_fixed() -> DynamicColor {
        DynamicColor::background_color(
            "secondary_fixed",
            |scheme| &scheme.secondary_palette,
            |scheme| if is_monochrome(scheme) { 80.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::secondary_fixed(),
                Self::secondary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        })
    }

    pub const fn secondary_fixed_dim() -> DynamicColor {
        DynamicColor::background_color(
            "secondary_fixed_dim",
            |scheme| &scheme.secondary_palette,
            |scheme| if is_monochrome(scheme) { 70.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::secondary_fixed(),
                Self::secondary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        })
    }

    pub const fn on_secondary_fixed() -> DynamicColor {
        DynamicColor::foreground_color("on_secondary_fixed", |scheme| &scheme.secondary_palette, |_| 10.0)
            .with_background(|_| Some(Self::secondary_fixed_dim()))
            .with_second_background(|_| Some(Self::secondary_fixed()))
            .with_contrast_curve(|_| {
                Some(ContrastCurve {
                    low: 4.5,
                    normal: 7.0,
                    medium: 11.0,
                    high: 21.0,
                })
            })
    }

    pub const fn on_secondary_fixed_variant() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_secondary_fixed_variant",
            |scheme| &scheme.secondary_palette,
            |scheme| if is_monochrome(scheme) { 25.0 } else { 30.0 },
        )
        .with_background(|_| Some(Self::secondary_fixed_dim()))
        .with_second_background(|_| Some(Self::secondary_fixed()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn tertiary_fixed() -> DynamicColor {
        DynamicColor::background_color(
            "tertiary_fixed",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 40.0 } else { 90.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::tertiary_fixed(),
                Self::tertiary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        })
    }

    pub const fn tertiary_fixed_dim() -> DynamicColor {
        DynamicColor::background_color(
            "tertiary_fixed_dim",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 30.0 } else { 80.0 },
        )
        .with_background(Self::highest_surface)
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_| {
            Some(ToneDeltaPair::new(
                Self::tertiary_fixed(),
                Self::tertiary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        })
    }

    pub const fn on_tertiary_fixed() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_tertiary_fixed",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 100.0 } else { 10.0 },
        )
        .with_background(|_| Some(Self::tertiary_fixed_dim()))
        .with_second_background(|_| Some(Self::tertiary_fixed()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        })
    }

    pub const fn on_tertiary_fixed_variant() -> DynamicColor {
        DynamicColor::foreground_color(
            "on_tertiary_fixed_variant",
            |scheme| &scheme.tertiary_palette,
            |scheme| if is_monochrome(scheme) { 90.0 } else { 30.0 },
        )
        .with_background(|_| Some(Self::tertiary_fixed_dim()))
        .with_second_background(|_| Some(Self::tertiary_fixed()))
        .with_contrast_curve(|_| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        })
    }

    pub const fn primary_dim() -> Option<&'static DynamicColor> {
        None
    }

    pub const fn secondary_dim() -> Option<&'static DynamicColor> {
        None
    }

    pub const fn tertiary_dim() -> Option<&'static DynamicColor> {
        None
    }

    pub const fn error_dim() -> Option<&'static DynamicColor> {
        None
    }

    pub fn get_hct(&self, scheme: &DynamicScheme, color: &DynamicColor) -> Hct {
        color.palette(scheme).get_hct(self.get_tone(scheme, color))
    }

    pub fn get_tone(&self, scheme: &DynamicScheme, color: &DynamicColor) -> f64 {
        let decreasing_contrast = scheme.contrast_level < 0.0;

        // Case 1: dual foreground, pair of colors with delta constraint.
        let pair = color.tone_delta_pair(scheme);

        if let Some(pair) = pair {
            let role_a = pair.subject;
            let role_b = pair.basis;
            let delta = pair.delta;
            let polarity = pair.polarity;
            let stay_together = pair.stay_together;

            let bg = color.background(scheme).unwrap();
            let bg_tone = bg.get_tone(scheme);

            let a_is_nearer = pair.constraint == DeltaConstraint::Nearer
                || (polarity == TonePolarity::Lighter && !scheme.is_dark)
                || (polarity == TonePolarity::Darker && scheme.is_dark);
            let nearer = if a_is_nearer { &role_a } else { &role_b };
            let farther = if a_is_nearer { &role_b } else { &role_a };
            let am_nearer = color.name == nearer.name;
            let expansion_dir = if scheme.is_dark { 1.0 } else { -1.0 };

            // 1st round: solve to min, each
            let n_contrast = nearer.contrast_curve(scheme).unwrap().get(scheme.contrast_level);
            let f_contrast = farther.contrast_curve(scheme).unwrap().get(scheme.contrast_level);

            // If a color is good enough, it is not adjusted.
            // Initial and adjusted tones for `nearer`
            let n_initial_tone = nearer.tone(scheme);
            let mut n_tone = if decreasing_contrast {
                DynamicColor::foreground_tone(bg_tone, n_contrast)
            } else if ratio_of_tones(bg_tone, n_initial_tone) >= n_contrast {
                n_initial_tone
            } else {
                DynamicColor::foreground_tone(bg_tone, n_contrast)
            };
            // Initial and adjusted tones for `farther`
            let f_initial_tone = farther.tone(scheme);
            let mut f_tone = if decreasing_contrast {
                DynamicColor::foreground_tone(bg_tone, f_contrast)
            } else if ratio_of_tones(bg_tone, f_initial_tone) >= f_contrast {
                f_initial_tone
            } else {
                DynamicColor::foreground_tone(bg_tone, f_contrast)
            };

            if (f_tone - n_tone) * expansion_dir >= delta {
                // Good! Tones satisfy the constraint; no change needed.
            } else {
                // 2nd round: expand farther to match delta.
                f_tone = delta.mul_add(expansion_dir, n_tone).clamp(0.0, 100.0);

                if (f_tone - n_tone) * expansion_dir >= delta {
                    // Good! Tones now satisfy the constraint; no change needed.
                } else {
                    // 3rd round: contract nearer to match delta.
                    n_tone = delta.mul_add(-expansion_dir, f_tone).clamp(0.0, 100.0);
                }
            }

            // Avoids the 50-59 awkward zone.
            if (50.0..60.0).contains(&n_tone) {
                // If `nearer` is in the awkward zone, move it away, together with
                // `farther`.
                if expansion_dir > 0.0 {
                    n_tone = 60.0;

                    f_tone = f_tone.max(delta.mul_add(expansion_dir, n_tone));
                } else {
                    n_tone = 49.0;

                    f_tone = f_tone.min(delta.mul_add(expansion_dir, n_tone));
                }
            } else if (50.0..60.0).contains(&f_tone) {
                if stay_together {
                    // Fixes both, to avoid two colors on opposite sides of the "awkward
                    // zone".
                    if expansion_dir > 0.0 {
                        n_tone = 60.0;

                        f_tone = f_tone.max(delta.mul_add(expansion_dir, n_tone));
                    } else {
                        n_tone = 49.0;

                        f_tone = f_tone.min(delta.mul_add(expansion_dir, n_tone));
                    }
                } else {
                    // Not required to stay together; fixes just one.
                    if expansion_dir > 0.0 {
                        f_tone = 60.0;
                    } else {
                        f_tone = 49.0;
                    }
                }
            }

            // Returns `nTone` if this color is `nearer`, otherwise `fTone`.
            if am_nearer { n_tone } else { f_tone }
        } else {
            // Case 2: No contrast pair; just solve for itself.
            let mut answer = color.tone(scheme);

            if let Some(background) = color.background(scheme) {
                let bg_tone = background.get_tone(scheme);

                let desired_ratio = color.contrast_curve(scheme).unwrap().get(scheme.contrast_level);

                if ratio_of_tones(bg_tone, answer) >= desired_ratio {
                    // Don't "improve" what's good enough.
                } else {
                    // Rough improvement.
                    answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
                }

                if decreasing_contrast {
                    answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
                }

                if color.is_background && (50.0..60.0).contains(&answer) {
                    // Must adjust
                    if ratio_of_tones(49.0, bg_tone) >= desired_ratio {
                        answer = 49.0;
                    } else {
                        answer = 60.0;
                    }
                }

                if let Some(second_background) = color.second_background(scheme) {
                    // Case 3: Adjust for dual backgrounds.

                    let bg_tone1 = color.background(scheme).unwrap().get_tone(scheme);
                    let bg_tone2 = second_background.get_tone(scheme);

                    let upper = bg_tone1.max(bg_tone2);
                    let lower = bg_tone1.min(bg_tone2);

                    if ratio_of_tones(upper, answer) >= desired_ratio && ratio_of_tones(lower, answer) >= desired_ratio {
                        return answer;
                    }

                    // The darkest light tone that satisfies the desired ratio,
                    // or -1 if such ratio cannot be reached.
                    let light_option = lighter(upper, desired_ratio);

                    // The lightest dark tone that satisfies the desired ratio,
                    // or -1 if such ratio cannot be reached.
                    let dark_option = darker(lower, desired_ratio);

                    // Tones suitable for the foreground.
                    let first_available = light_option.or(dark_option);
                    let second_available = light_option.and(dark_option);

                    if DynamicColor::tone_prefers_light_foreground(bg_tone1) || DynamicColor::tone_prefers_light_foreground(bg_tone2) {
                        light_option.unwrap_or(100.0)
                    } else if let Some(first) = first_available
                        && second_available.is_none()
                    {
                        first
                    } else {
                        dark_option.unwrap_or(0.0)
                    }
                } else {
                    answer
                }
            } else {
                answer
            }
        }
    }

    pub const fn get_primary_palette(_variant: Variant, _source_color_hct: Hct, _is_dark: bool, _platform: Platform, _contrast_level: f64) -> TonalPalette {
        todo!()
    }

    pub const fn get_secondary_palette(_variant: Variant, _source_color_hct: Hct, _is_dark: bool, _platform: Platform, _contrast_level: f64) -> TonalPalette {
        todo!()
    }

    pub const fn get_tertiary_palette(_variant: Variant, _source_color_hct: Hct, _is_dark: bool, _platform: Platform, _contrast_level: f64) -> TonalPalette {
        todo!()
    }

    pub const fn get_neutral_palette(_variant: Variant, _source_color_hct: Hct, _is_dark: bool, _platform: Platform, _contrast_level: f64) -> TonalPalette {
        todo!()
    }

    pub const fn get_neutral_variant_palette(
        _variant: Variant,
        _source_color_hct: Hct,
        _is_dark: bool,
        _platform: Platform,
        _contrast_level: f64,
    ) -> TonalPalette {
        todo!()
    }

    pub const fn get_error_palette(_variant: Variant, _source_color_hct: Hct, _is_dark: bool, _platform: Platform, _contrast_level: f64) -> TonalPalette {
        todo!()
    }
}

impl ColorSpec for ColorSpec2021 {
    fn primary_palette_key_color(&self) -> DynamicColor {
        const { Self::primary_palette_key_color() }
    }

    fn secondary_palette_key_color(&self) -> DynamicColor {
        const { Self::secondary_palette_key_color() }
    }

    fn tertiary_palette_key_color(&self) -> DynamicColor {
        const { Self::tertiary_palette_key_color() }
    }

    fn neutral_palette_key_color(&self) -> DynamicColor {
        const { Self::neutral_palette_key_color() }
    }

    fn neutral_variant_palette_key_color(&self) -> DynamicColor {
        const { Self::neutral_variant_palette_key_color() }
    }

    fn error_palette_key_color(&self) -> DynamicColor {
        const { Self::error_palette_key_color() }
    }

    fn background(&self) -> DynamicColor {
        const { Self::background() }
    }

    fn on_background(&self) -> DynamicColor {
        const { Self::on_background() }
    }

    fn surface(&self) -> DynamicColor {
        const { Self::surface() }
    }

    fn surface_dim(&self) -> DynamicColor {
        const { Self::surface_dim() }
    }

    fn surface_bright(&self) -> DynamicColor {
        const { Self::surface_bright() }
    }

    fn surface_container_lowest(&self) -> DynamicColor {
        const { Self::surface_container_lowest() }
    }

    fn surface_container_low(&self) -> DynamicColor {
        const { Self::surface_container_low() }
    }

    fn surface_container(&self) -> DynamicColor {
        const { Self::surface_container() }
    }

    fn surface_container_high(&self) -> DynamicColor {
        const { Self::surface_container_high() }
    }

    fn surface_container_highest(&self) -> DynamicColor {
        const { Self::surface_container_highest() }
    }

    fn on_surface(&self) -> DynamicColor {
        const { Self::on_surface() }
    }

    fn surface_variant(&self) -> DynamicColor {
        const { Self::surface_variant() }
    }

    fn on_surface_variant(&self) -> DynamicColor {
        const { Self::on_surface_variant() }
    }

    fn inverse_surface(&self) -> DynamicColor {
        const { Self::inverse_surface() }
    }

    fn inverse_on_surface(&self) -> DynamicColor {
        const { Self::inverse_on_surface() }
    }

    fn outline(&self) -> DynamicColor {
        const { Self::outline() }
    }

    fn outline_variant(&self) -> DynamicColor {
        const { Self::outline_variant() }
    }

    fn shadow(&self) -> DynamicColor {
        const { Self::shadow() }
    }

    fn scrim(&self) -> DynamicColor {
        const { Self::scrim() }
    }

    fn surface_tint(&self) -> DynamicColor {
        const { Self::surface_tint() }
    }

    fn primary(&self) -> DynamicColor {
        const { Self::primary() }
    }

    fn primary_dim(&self) -> Option<DynamicColor> {
        None
    }

    fn on_primary(&self) -> DynamicColor {
        const { Self::on_primary() }
    }

    fn primary_container(&self) -> DynamicColor {
        const { Self::primary_container() }
    }

    fn on_primary_container(&self) -> DynamicColor {
        const { Self::on_primary_container() }
    }

    fn inverse_primary(&self) -> DynamicColor {
        const { Self::inverse_primary() }
    }

    fn secondary(&self) -> DynamicColor {
        const { Self::secondary() }
    }

    fn secondary_dim(&self) -> Option<DynamicColor> {
        None
    }

    fn on_secondary(&self) -> DynamicColor {
        const { Self::on_secondary() }
    }

    fn secondary_container(&self) -> DynamicColor {
        const { Self::secondary_container() }
    }

    fn on_secondary_container(&self) -> DynamicColor {
        const { Self::on_secondary_container() }
    }

    fn tertiary(&self) -> DynamicColor {
        const { Self::tertiary() }
    }

    fn tertiary_dim(&self) -> Option<DynamicColor> {
        None
    }

    fn on_tertiary(&self) -> DynamicColor {
        const { Self::on_tertiary() }
    }

    fn tertiary_container(&self) -> DynamicColor {
        const { Self::tertiary_container() }
    }

    fn on_tertiary_container(&self) -> DynamicColor {
        const { Self::on_tertiary_container() }
    }

    fn error(&self) -> DynamicColor {
        const { Self::error() }
    }

    fn error_dim(&self) -> Option<DynamicColor> {
        None
    }

    fn on_error(&self) -> DynamicColor {
        const { Self::on_error() }
    }

    fn error_container(&self) -> DynamicColor {
        const { Self::error_container() }
    }

    fn on_error_container(&self) -> DynamicColor {
        const { Self::on_error_container() }
    }

    fn primary_fixed(&self) -> DynamicColor {
        const { Self::primary_fixed() }
    }

    fn primary_fixed_dim(&self) -> DynamicColor {
        const { Self::primary_fixed_dim() }
    }

    fn on_primary_fixed(&self) -> DynamicColor {
        const { Self::on_primary_fixed() }
    }

    fn on_primary_fixed_variant(&self) -> DynamicColor {
        const { Self::on_primary_fixed_variant() }
    }

    fn secondary_fixed(&self) -> DynamicColor {
        const { Self::secondary_fixed() }
    }

    fn secondary_fixed_dim(&self) -> DynamicColor {
        const { Self::secondary_fixed_dim() }
    }

    fn on_secondary_fixed(&self) -> DynamicColor {
        const { Self::on_secondary_fixed() }
    }

    fn on_secondary_fixed_variant(&self) -> DynamicColor {
        const { Self::on_secondary_fixed_variant() }
    }

    fn tertiary_fixed(&self) -> DynamicColor {
        const { Self::tertiary_fixed() }
    }

    fn tertiary_fixed_dim(&self) -> DynamicColor {
        const { Self::tertiary_fixed_dim() }
    }

    fn on_tertiary_fixed(&self) -> DynamicColor {
        const { Self::on_tertiary_fixed() }
    }

    fn on_tertiary_fixed_variant(&self) -> DynamicColor {
        const { Self::on_tertiary_fixed_variant() }
    }

    fn get_hct(&self, scheme: &DynamicScheme, color: &DynamicColor) -> Hct {
        (self as &Self).get_hct(scheme, color)
    }

    fn get_tone(&self, scheme: &DynamicScheme, color: &DynamicColor) -> f64 {
        (self as &Self).get_tone(scheme, color)
    }

    fn get_primary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        Self::get_primary_palette(variant, source_color_hct, is_dark, platform, contrast_level)
    }

    fn get_secondary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        Self::get_secondary_palette(variant, source_color_hct, is_dark, platform, contrast_level)
    }

    fn get_tertiary_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        Self::get_tertiary_palette(variant, source_color_hct, is_dark, platform, contrast_level)
    }

    fn get_neutral_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        Self::get_neutral_palette(variant, source_color_hct, is_dark, platform, contrast_level)
    }

    fn get_neutral_variant_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        Self::get_neutral_variant_palette(variant, source_color_hct, is_dark, platform, contrast_level)
    }

    fn get_error_palette(&self, variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        Self::get_error_palette(variant, source_color_hct, is_dark, platform, contrast_level)
    }
}
