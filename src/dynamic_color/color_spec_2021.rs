#[cfg(not(feature = "std"))] use alloc::{vec, vec::Vec};
#[cfg(feature = "std")] use std::{vec, vec::Vec};

use super::{DynamicColor, DynamicScheme, Variant, dynamic_scheme::Platform};
use crate::{
    contrast::{darker, lighter, ratio_of_tones},
    dislike::fix_if_disliked,
    dynamic_color::{ContrastCurve, ExtendedColorData, ToneDeltaPair, TonePolarity, color_spec::ColorSpec, tone_delta_pair::DeltaConstraint},
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

    pub const fn highest_surface(_: Option<ExtendedColorData>, scheme: &DynamicScheme) -> Option<&'static DynamicColor> {
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

    pub const fn primary_palette_key_color() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "primary_palette_key_color",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| scheme.primary_palette.key_color().get_tone(),
        );

        &COLOR
    }

    pub const fn secondary_palette_key_color() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "secondary_palette_key_color",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| scheme.secondary_palette.key_color().get_tone(),
        );

        &COLOR
    }

    pub const fn tertiary_palette_key_color() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "tertiary_palette_key_color",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| scheme.tertiary_palette.key_color().get_tone(),
        );

        &COLOR
    }

    pub const fn neutral_palette_key_color() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "neutral_palette_key_color",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| scheme.neutral_palette.key_color().get_tone(),
        );

        &COLOR
    }

    pub const fn neutral_variant_palette_key_color() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "neutral_variant_palette_key_color",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| scheme.neutral_variant_palette.key_color().get_tone(),
        );

        &COLOR
    }

    pub const fn error_palette_key_color() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "error_palette_key_color",
            |_, scheme| &scheme.error_palette,
            |_, scheme| scheme.error_palette.key_color().get_tone(),
        );

        &COLOR
    }

    pub const fn background() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "background",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| if scheme.is_dark { 6.0 } else { 98.0 },
        );

        &COLOR
    }

    pub const fn on_background() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_background",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::background()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 3.0,
                medium: 4.5,
                high: 7.0,
            })
        });

        &COLOR
    }

    pub const fn surface() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| if scheme.is_dark { 6.0 } else { 98.0 },
        );

        &COLOR
    }

    pub const fn surface_dim() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_dim",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
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
        );

        &COLOR
    }

    pub const fn surface_bright() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_bright",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
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
        );

        &COLOR
    }

    pub const fn surface_container_lowest() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_container_lowest",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
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
        );

        &COLOR
    }

    pub const fn surface_container_low() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_container_low",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
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
        );

        &COLOR
    }

    pub const fn surface_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_container",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
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
        );

        &COLOR
    }

    pub const fn surface_container_high() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_container_high",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
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
        );

        &COLOR
    }

    pub const fn surface_container_highest() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_container_highest",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
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
        );

        &COLOR
    }

    pub const fn on_surface() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn surface_variant() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_variant",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| if scheme.is_dark { 30.0 } else { 90.0 },
        );

        &COLOR
    }

    pub const fn on_surface_variant() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_surface_variant",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| if scheme.is_dark { 80.0 } else { 30.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
    }

    pub const fn inverse_surface() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "inverse_surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| if scheme.is_dark { 90.0 } else { 20.0 },
        );

        &COLOR
    }

    pub const fn inverse_on_surface() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "inverse_on_surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| if scheme.is_dark { 20.0 } else { 95.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::inverse_surface()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn outline() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "outline",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| if scheme.is_dark { 60.0 } else { 50.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.5,
                normal: 3.0,
                medium: 4.5,
                high: 7.0,
            })
        });

        &COLOR
    }

    pub const fn outline_variant() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "outline_variant",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| if scheme.is_dark { 30.0 } else { 80.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        });

        &COLOR
    }

    pub const fn shadow() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color("shadow", |_, scheme| &scheme.neutral_palette, |_, _| 0.0);

        &COLOR
    }

    pub const fn scrim() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color("scrim", |_, scheme| &scheme.neutral_palette, |_, _| 0.0);

        &COLOR
    }

    pub const fn surface_tint() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "surface_tint",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        );

        &COLOR
    }

    pub const fn primary() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "primary",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 100.0 } else { 0.0 }
                } else if scheme.is_dark {
                    80.0
                } else {
                    40.0
                }
            },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::primary_container(),
                ColorSpec2021::primary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        });

        &COLOR
    }

    pub const fn on_primary() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_primary",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 10.0 } else { 90.0 }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2021::primary()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn primary_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "primary_container",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
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
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::primary_container(),
                ColorSpec2021::primary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        });

        &COLOR
    }

    pub const fn on_primary_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_primary_container",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(ColorSpec2021::primary_container().tone(scheme), 4.5)
                } else if is_monochrome(scheme) {
                    if scheme.is_dark { 0.0 } else { 100.0 }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2021::primary_container()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
    }

    pub const fn inverse_primary() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "inverse_primary",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| if scheme.is_dark { 40.0 } else { 80.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::inverse_surface()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        });

        &COLOR
    }

    pub const fn secondary() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "secondary",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::secondary_container(),
                ColorSpec2021::secondary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        });

        &COLOR
    }

    pub const fn on_secondary() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_secondary",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 10.0 } else { 100.0 }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2021::secondary()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn secondary_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "secondary_container",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                let initial_tone = if scheme.is_dark { 30.0 } else { 90.0 };

                if is_monochrome(scheme) {
                    if scheme.is_dark { 30.0 } else { 90.0 }
                } else if !is_fidelity(scheme) {
                    initial_tone
                } else {
                    ColorSpec2021::_find_desired_chroma_by_tone(
                        scheme.secondary_palette.hue(),
                        scheme.secondary_palette.chroma(),
                        initial_tone,
                        !scheme.is_dark,
                    )
                }
            },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::secondary_container(),
                ColorSpec2021::secondary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        });

        &COLOR
    }

    pub const fn on_secondary_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_secondary_container",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(ColorSpec2021::secondary_container().tone(scheme), 4.5)
                } else if scheme.is_dark {
                    90.0
                } else if is_monochrome(scheme) {
                    30.0
                } else {
                    10.0
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2021::secondary_container()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
    }

    pub const fn tertiary() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "tertiary",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 90.0 } else { 25.0 }
                } else if scheme.is_dark {
                    80.0
                } else {
                    40.0
                }
            },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 7.0,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::tertiary_container(),
                ColorSpec2021::tertiary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        });

        &COLOR
    }

    pub const fn on_tertiary() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 10.0 } else { 90.0 }
                } else if scheme.is_dark {
                    20.0
                } else {
                    100.0
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2021::tertiary()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn tertiary_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "tertiary_container",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if is_monochrome(scheme) {
                    if scheme.is_dark { 60.0 } else { 49.0 }
                } else if !is_fidelity(scheme) {
                    if scheme.is_dark { 30.0 } else { 90.0 }
                } else {
                    fix_if_disliked(scheme.tertiary_palette.get_hct(scheme.source_color_hct.get_tone())).get_tone()
                }
            },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::tertiary_container(),
                ColorSpec2021::tertiary(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        });

        &COLOR
    }

    pub const fn on_tertiary_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary_container",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if is_fidelity(scheme) {
                    DynamicColor::foreground_tone(ColorSpec2021::tertiary_container().tone(scheme), 4.5)
                } else if is_monochrome(scheme) {
                    if scheme.is_dark { 0.0 } else { 100.0 }
                } else if scheme.is_dark {
                    90.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2021::tertiary_container()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
    }

    pub const fn error() -> &'static DynamicColor {
        static COLOR: DynamicColor =
            DynamicColor::background_color("error", |_, scheme| &scheme.error_palette, |_, scheme| if scheme.is_dark { 80.0 } else { 40.0 })
                .with_background(ColorSpec2021::highest_surface)
                .with_contrast_curve(|_, _| {
                    Some(ContrastCurve {
                        low: 3.0,
                        normal: 4.5,
                        medium: 7.0,
                        high: 7.0,
                    })
                })
                .with_tone_delta_pair(|_, _| {
                    Some(ToneDeltaPair::new(
                        ColorSpec2021::error_container(),
                        ColorSpec2021::error(),
                        10.0,
                        TonePolarity::RelativeLighter,
                        false,
                        DeltaConstraint::Nearer,
                    ))
                });

        &COLOR
    }

    pub const fn on_error() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_error",
            |_, scheme| &scheme.error_palette,
            |_, scheme| if scheme.is_dark { 20.0 } else { 100.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::error()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn error_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "error_container",
            |_, scheme| &scheme.error_palette,
            |_, scheme| if scheme.is_dark { 30.0 } else { 90.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::error_container(),
                ColorSpec2021::error(),
                10.0,
                TonePolarity::RelativeLighter,
                false,
                DeltaConstraint::Nearer,
            ))
        });

        &COLOR
    }

    pub const fn on_error_container() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_error_container",
            |_, scheme| &scheme.error_palette,
            |_, scheme| {
                if scheme.is_dark {
                    90.0
                } else if is_monochrome(scheme) {
                    10.0
                } else {
                    30.0
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2021::error_container()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
    }

    pub const fn primary_fixed() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "primary_fixed",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| if is_monochrome(scheme) { 40.0 } else { 90.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::primary_fixed(),
                ColorSpec2021::primary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        });

        &COLOR
    }

    pub const fn primary_fixed_dim() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "primary_fixed_dim",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| if is_monochrome(scheme) { 30.0 } else { 80.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::primary_fixed(),
                ColorSpec2021::primary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        });

        &COLOR
    }

    pub const fn on_primary_fixed() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_primary_fixed",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| if is_monochrome(scheme) { 100.0 } else { 10.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::primary_fixed_dim()))
        .with_second_background(|_, _| Some(ColorSpec2021::primary_fixed()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn on_primary_fixed_variant() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_primary_fixed_variant",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| if is_monochrome(scheme) { 90.0 } else { 30.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::primary_fixed_dim()))
        .with_second_background(|_, _| Some(ColorSpec2021::primary_fixed()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
    }

    pub const fn secondary_fixed() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "secondary_fixed",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| if is_monochrome(scheme) { 80.0 } else { 90.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::secondary_fixed(),
                ColorSpec2021::secondary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        });

        &COLOR
    }

    pub const fn secondary_fixed_dim() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "secondary_fixed_dim",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| if is_monochrome(scheme) { 70.0 } else { 80.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::secondary_fixed(),
                ColorSpec2021::secondary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        });

        &COLOR
    }

    pub const fn on_secondary_fixed() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color("on_secondary_fixed", |_, scheme| &scheme.secondary_palette, |_, _| 10.0)
            .with_background(|_, _| Some(ColorSpec2021::secondary_fixed_dim()))
            .with_second_background(|_, _| Some(ColorSpec2021::secondary_fixed()))
            .with_contrast_curve(|_, _| {
                Some(ContrastCurve {
                    low: 4.5,
                    normal: 7.0,
                    medium: 11.0,
                    high: 21.0,
                })
            });

        &COLOR
    }

    pub const fn on_secondary_fixed_variant() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_secondary_fixed_variant",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| if is_monochrome(scheme) { 25.0 } else { 30.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::secondary_fixed_dim()))
        .with_second_background(|_, _| Some(ColorSpec2021::secondary_fixed()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
    }

    pub const fn tertiary_fixed() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "tertiary_fixed",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| if is_monochrome(scheme) { 40.0 } else { 90.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::tertiary_fixed(),
                ColorSpec2021::tertiary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        });

        &COLOR
    }

    pub const fn tertiary_fixed_dim() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "tertiary_fixed_dim",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| if is_monochrome(scheme) { 30.0 } else { 80.0 },
        )
        .with_background(ColorSpec2021::highest_surface)
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 1.0,
                normal: 1.0,
                medium: 3.0,
                high: 4.5,
            })
        })
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2021::tertiary_fixed(),
                ColorSpec2021::tertiary_fixed_dim(),
                10.0,
                TonePolarity::Lighter,
                true,
                DeltaConstraint::Exact,
            ))
        });

        &COLOR
    }

    pub const fn on_tertiary_fixed() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary_fixed",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| if is_monochrome(scheme) { 100.0 } else { 10.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::tertiary_fixed_dim()))
        .with_second_background(|_, _| Some(ColorSpec2021::tertiary_fixed()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 4.5,
                normal: 7.0,
                medium: 11.0,
                high: 21.0,
            })
        });

        &COLOR
    }

    pub const fn on_tertiary_fixed_variant() -> &'static DynamicColor {
        static COLOR: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary_fixed_variant",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| if is_monochrome(scheme) { 90.0 } else { 30.0 },
        )
        .with_background(|_, _| Some(ColorSpec2021::tertiary_fixed_dim()))
        .with_second_background(|_, _| Some(ColorSpec2021::tertiary_fixed()))
        .with_contrast_curve(|_, _| {
            Some(ContrastCurve {
                low: 3.0,
                normal: 4.5,
                medium: 7.0,
                high: 11.0,
            })
        });

        &COLOR
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
                    let mut availables: Vec<f64> = vec![];

                    if let Some(light_option) = light_option {
                        availables.push(light_option);
                    }

                    if let Some(dark_option) = dark_option {
                        availables.push(dark_option);
                    }

                    let prefers_light = DynamicColor::tone_prefers_light_foreground(bg_tone1) || DynamicColor::tone_prefers_light_foreground(bg_tone2);

                    if prefers_light {
                        return light_option.unwrap_or(100.0);
                    }

                    if availables.len() == 1 {
                        return availables[0];
                    }

                    return dark_option.unwrap_or(0.0);
                }
            }

            answer
        }
    }

    pub const fn get_primary_palette(
        _variant: Variant,
        _source_color_hct: Hct,
        _is_dark: bool,
        _platform: Platform,
        _contrast_level: f64,
    ) -> TonalPalette {
        todo!()
    }

    pub const fn get_secondary_palette(
        _variant: Variant,
        _source_color_hct: Hct,
        _is_dark: bool,
        _platform: Platform,
        _contrast_level: f64,
    ) -> TonalPalette {
        todo!()
    }

    pub const fn get_tertiary_palette(
        _variant: Variant,
        _source_color_hct: Hct,
        _is_dark: bool,
        _platform: Platform,
        _contrast_level: f64,
    ) -> TonalPalette {
        todo!()
    }

    pub const fn get_neutral_palette(
        _variant: Variant,
        _source_color_hct: Hct,
        _is_dark: bool,
        _platform: Platform,
        _contrast_level: f64,
    ) -> TonalPalette {
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

    pub const fn get_error_palette(
        _variant: Variant,
        _source_color_hct: Hct,
        _is_dark: bool,
        _platform: Platform,
        _contrast_level: f64,
    ) -> TonalPalette {
        todo!()
    }
}

impl ColorSpec for ColorSpec2021 {
    fn primary_palette_key_color(&self) -> &'static DynamicColor {
        const { Self::primary_palette_key_color() }
    }

    fn secondary_palette_key_color(&self) -> &'static DynamicColor {
        const { Self::secondary_palette_key_color() }
    }

    fn tertiary_palette_key_color(&self) -> &'static DynamicColor {
        const { Self::tertiary_palette_key_color() }
    }

    fn neutral_palette_key_color(&self) -> &'static DynamicColor {
        const { Self::neutral_palette_key_color() }
    }

    fn neutral_variant_palette_key_color(&self) -> &'static DynamicColor {
        const { Self::neutral_variant_palette_key_color() }
    }

    fn error_palette_key_color(&self) -> &'static DynamicColor {
        const { Self::error_palette_key_color() }
    }

    fn background(&self) -> &'static DynamicColor {
        const { Self::background() }
    }

    fn on_background(&self) -> &'static DynamicColor {
        const { Self::on_background() }
    }

    fn surface(&self) -> &'static DynamicColor {
        const { Self::surface() }
    }

    fn surface_dim(&self) -> &'static DynamicColor {
        const { Self::surface_dim() }
    }

    fn surface_bright(&self) -> &'static DynamicColor {
        const { Self::surface_bright() }
    }

    fn surface_container_lowest(&self) -> &'static DynamicColor {
        const { Self::surface_container_lowest() }
    }

    fn surface_container_low(&self) -> &'static DynamicColor {
        const { Self::surface_container_low() }
    }

    fn surface_container(&self) -> &'static DynamicColor {
        const { Self::surface_container() }
    }

    fn surface_container_high(&self) -> &'static DynamicColor {
        const { Self::surface_container_high() }
    }

    fn surface_container_highest(&self) -> &'static DynamicColor {
        const { Self::surface_container_highest() }
    }

    fn on_surface(&self) -> &'static DynamicColor {
        const { Self::on_surface() }
    }

    fn surface_variant(&self) -> &'static DynamicColor {
        const { Self::surface_variant() }
    }

    fn on_surface_variant(&self) -> &'static DynamicColor {
        const { Self::on_surface_variant() }
    }

    fn inverse_surface(&self) -> &'static DynamicColor {
        const { Self::inverse_surface() }
    }

    fn inverse_on_surface(&self) -> &'static DynamicColor {
        const { Self::inverse_on_surface() }
    }

    fn outline(&self) -> &'static DynamicColor {
        const { Self::outline() }
    }

    fn outline_variant(&self) -> &'static DynamicColor {
        const { Self::outline_variant() }
    }

    fn shadow(&self) -> &'static DynamicColor {
        const { Self::shadow() }
    }

    fn scrim(&self) -> &'static DynamicColor {
        const { Self::scrim() }
    }

    fn surface_tint(&self) -> &'static DynamicColor {
        const { Self::surface_tint() }
    }

    fn primary(&self) -> &'static DynamicColor {
        const { Self::primary() }
    }

    fn primary_dim(&self) -> Option<&'static DynamicColor> {
        None
    }

    fn on_primary(&self) -> &'static DynamicColor {
        const { Self::on_primary() }
    }

    fn primary_container(&self) -> &'static DynamicColor {
        const { Self::primary_container() }
    }

    fn on_primary_container(&self) -> &'static DynamicColor {
        const { Self::on_primary_container() }
    }

    fn inverse_primary(&self) -> &'static DynamicColor {
        const { Self::inverse_primary() }
    }

    fn secondary(&self) -> &'static DynamicColor {
        const { Self::secondary() }
    }

    fn secondary_dim(&self) -> Option<&'static DynamicColor> {
        None
    }

    fn on_secondary(&self) -> &'static DynamicColor {
        const { Self::on_secondary() }
    }

    fn secondary_container(&self) -> &'static DynamicColor {
        const { Self::secondary_container() }
    }

    fn on_secondary_container(&self) -> &'static DynamicColor {
        const { Self::on_secondary_container() }
    }

    fn tertiary(&self) -> &'static DynamicColor {
        const { Self::tertiary() }
    }

    fn tertiary_dim(&self) -> Option<&'static DynamicColor> {
        None
    }

    fn on_tertiary(&self) -> &'static DynamicColor {
        const { Self::on_tertiary() }
    }

    fn tertiary_container(&self) -> &'static DynamicColor {
        const { Self::tertiary_container() }
    }

    fn on_tertiary_container(&self) -> &'static DynamicColor {
        const { Self::on_tertiary_container() }
    }

    fn error(&self) -> &'static DynamicColor {
        const { Self::error() }
    }

    fn error_dim(&self) -> Option<&'static DynamicColor> {
        None
    }

    fn on_error(&self) -> &'static DynamicColor {
        const { Self::on_error() }
    }

    fn error_container(&self) -> &'static DynamicColor {
        const { Self::error_container() }
    }

    fn on_error_container(&self) -> &'static DynamicColor {
        const { Self::on_error_container() }
    }

    fn primary_fixed(&self) -> &'static DynamicColor {
        const { Self::primary_fixed() }
    }

    fn primary_fixed_dim(&self) -> &'static DynamicColor {
        const { Self::primary_fixed_dim() }
    }

    fn on_primary_fixed(&self) -> &'static DynamicColor {
        const { Self::on_primary_fixed() }
    }

    fn on_primary_fixed_variant(&self) -> &'static DynamicColor {
        const { Self::on_primary_fixed_variant() }
    }

    fn secondary_fixed(&self) -> &'static DynamicColor {
        const { Self::secondary_fixed() }
    }

    fn secondary_fixed_dim(&self) -> &'static DynamicColor {
        const { Self::secondary_fixed_dim() }
    }

    fn on_secondary_fixed(&self) -> &'static DynamicColor {
        const { Self::on_secondary_fixed() }
    }

    fn on_secondary_fixed_variant(&self) -> &'static DynamicColor {
        const { Self::on_secondary_fixed_variant() }
    }

    fn tertiary_fixed(&self) -> &'static DynamicColor {
        const { Self::tertiary_fixed() }
    }

    fn tertiary_fixed_dim(&self) -> &'static DynamicColor {
        const { Self::tertiary_fixed_dim() }
    }

    fn on_tertiary_fixed(&self) -> &'static DynamicColor {
        const { Self::on_tertiary_fixed() }
    }

    fn on_tertiary_fixed_variant(&self) -> &'static DynamicColor {
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