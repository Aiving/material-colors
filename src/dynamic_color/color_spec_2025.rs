#[cfg(not(feature = "std"))] use alloc::{vec, vec::Vec};
#[cfg(feature = "std")] use std::vec;

use super::{DynamicColor, DynamicScheme, Variant, dynamic_scheme::Platform};
use crate::{
    contrast::{darker, lighter, ratio_of_tones},
    dynamic_color::{
        ContrastCurve, ExtendedColorData, ToneDeltaPair, TonePolarity,
        color_spec::{ColorSpec, SpecVersion},
        color_spec_2021::ColorSpec2021,
        tone_delta_pair::DeltaConstraint,
    },
    hct::Hct,
    palette::TonalPalette,
};

fn get_expressive_neutral_hue(source_color_hct: Hct) -> f64 {
    DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 71.0, 124.0, 253.0, 278.0, 300.0, 360.0], &[
        10.0, 0.0, 10.0, 0.0, 10.0, 0.0,
    ])
}

fn get_expressive_neutral_chroma(source_color_hct: Hct, is_dark: bool, platform: Platform) -> f64 {
    let neutral_hue = get_expressive_neutral_hue(source_color_hct);

    if platform == Platform::Phone {
        if is_dark {
            if Hct::is_yellow(neutral_hue) { 6.0 } else { 14.0 }
        } else {
            18.0
        }
    } else {
        12.0
    }
}

fn get_vibrant_neutral_hue(source_color_hct: Hct) -> f64 {
    DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 38.0, 105.0, 140.0, 333.0, 360.0], &[
        -14.0, 10.0, -14.0, 10.0, -14.0,
    ])
}

fn get_vibrant_neutral_chroma(source_color_hct: Hct, platform: Platform) -> f64 {
    let neutral_hue = get_vibrant_neutral_hue(source_color_hct);

    if platform == Platform::Phone || Hct::is_blue(neutral_hue) {
        28.0
    } else {
        20.0
    }
}

fn t_max_c(
    palette: &TonalPalette,
    lower_bound: f64,       // = 0.0,
    upper_bound: f64,       // = 100.0,
    chroma_multiplier: f64, // = 1.0,
) -> f64 {
    let answer = find_best_tone_for_chroma(palette.hue(), palette.chroma() * chroma_multiplier, 100.0, true);

    answer.clamp(lower_bound, upper_bound)
}

fn t_min_c(
    palette: &TonalPalette,
    lower_bound: f64, // = 0.0,
    upper_bound: f64, // = 100.0,
) -> f64 {
    let answer = find_best_tone_for_chroma(palette.hue(), palette.chroma(), 0.0, false);

    answer.clamp(lower_bound, upper_bound)
}

fn find_best_tone_for_chroma(hue: f64, chroma: f64, tone: f64, by_decreasing_tone: bool) -> f64 {
    let mut tone = tone;
    let mut answer = tone;
    let mut best_candidate = Hct::from(hue, chroma, answer);

    while best_candidate.get_chroma() < chroma {
        if !(0.0..=100.0).contains(&tone) {
            break;
        }

        tone += if by_decreasing_tone { -1.0 } else { 1.0 };

        let new_candidate = Hct::from(hue, chroma, tone);

        if best_candidate.get_chroma() < new_candidate.get_chroma() {
            best_candidate = new_candidate;

            answer = tone;
        }
    }

    answer
}

fn get_contrast_curve(default_contrast: f64) -> Option<ContrastCurve> {
    Some(match default_contrast {
        1.5 => ContrastCurve {
            low: 1.5,
            normal: 1.5,
            medium: 3.0,
            high: 5.5,
        },
        3.0 => ContrastCurve {
            low: 3.0,
            normal: 3.0,
            medium: 4.5,
            high: 7.0,
        },
        4.5 => ContrastCurve {
            low: 4.5,
            normal: 4.5,
            medium: 7.0,
            high: 11.0,
        },
        6.0 => ContrastCurve {
            low: 6.0,
            normal: 6.0,
            medium: 7.0,
            high: 11.0,
        },
        7.0 => ContrastCurve {
            low: 7.0,
            normal: 7.0,
            medium: 11.0,
            high: 21.0,
        },
        9.0 => ContrastCurve {
            low: 9.0,
            normal: 9.0,
            medium: 11.0,
            high: 21.0,
        },
        11.0 => ContrastCurve {
            low: 11.0,
            normal: 11.0,
            medium: 21.0,
            high: 21.0,
        },
        21.0 => ContrastCurve {
            low: 21.0,
            normal: 21.0,
            medium: 21.0,
            high: 21.0,
        },
        _ => ContrastCurve {
            low: default_contrast,
            normal: default_contrast,
            medium: 7.0,
            high: 21.0,
        },
    })
}

pub struct ColorSpec2025;

impl ColorSpec2025 {
    pub const CONTENT_ACCENT_TONE_DELTA: f64 = 15.0;

    pub const fn highest_surface(_: Option<ExtendedColorData>, scheme: &DynamicScheme) -> Option<&'static DynamicColor> {
        Some(if scheme.is_dark { Self::surface_bright() } else { Self::surface_dim() })
    }
}

impl ColorSpec2025 {
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
        ColorSpec2021::neutral_variant_palette_key_color()
    }

    pub const fn error_palette_key_color() -> &'static DynamicColor {
        ColorSpec2021::error_palette_key_color()
    }

    pub const fn background() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = ColorSpec2025::surface().with_name("background");
        static COLOR: DynamicColor = ColorSpec2021::background().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_background() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = ColorSpec2025::on_surface().with_name("on_background").with_tone(|_, scheme| {
            if scheme.platform == Platform::Watch {
                100.0
            } else {
                ColorSpec2025::on_surface().get_tone(scheme)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_background().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        4.0
                    } else if Hct::is_yellow(scheme.neutral_palette.hue()) {
                        99.0
                    } else if scheme.variant == Variant::Vibrant {
                        97.0
                    } else {
                        98.0
                    }
                } else {
                    0.0
                }
            },
        );

        static COLOR: DynamicColor = ColorSpec2021::surface().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_dim() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface_dim",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.is_dark {
                    4.0
                } else if Hct::is_yellow(scheme.neutral_palette.hue()) {
                    90.0
                } else if scheme.variant == Variant::Vibrant {
                    85.0
                } else {
                    87.0
                }
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.is_dark {
                1.0
            } else {
                match scheme.variant {
                    Variant::Neutral => 2.5,
                    Variant::TonalSpot => 1.7,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            2.7
                        } else {
                            1.75
                        }
                    }
                    Variant::Vibrant => 1.36,
                    _ => 1.0,
                }
            })
        });

        static COLOR: DynamicColor = ColorSpec2021::surface_dim().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_bright() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface_bright",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.is_dark {
                    18.0
                } else if Hct::is_yellow(scheme.neutral_palette.hue()) {
                    99.0
                } else if scheme.variant == Variant::Vibrant {
                    97.0
                } else {
                    98.0
                }
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.is_dark {
                match scheme.variant {
                    Variant::Neutral => 2.5,
                    Variant::TonalSpot => 1.7,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            2.7
                        } else {
                            1.75
                        }
                    }
                    Variant::Vibrant => 1.36,
                    _ => 1.0,
                }
            } else {
                1.0
            })
        });

        static COLOR: DynamicColor = ColorSpec2021::surface_bright().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_container_lowest() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface_container_lowest",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.is_dark { 0.0 } else { 100.0 }
            },
        );

        static COLOR: DynamicColor = ColorSpec2021::surface_container_lowest().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_container_low() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface_container_low",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        6.0
                    } else if Hct::is_yellow(scheme.neutral_palette.hue()) {
                        98.0
                    } else if scheme.variant == Variant::Vibrant {
                        95.0
                    } else {
                        96.0
                    }
                } else {
                    15.0
                }
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                match scheme.variant {
                    Variant::Neutral => 1.3,
                    Variant::TonalSpot => 1.25,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            1.3
                        } else {
                            1.15
                        }
                    }
                    Variant::Vibrant => 1.08,
                    _ => 1.0,
                }
            } else {
                1.0
            })
        });

        static COLOR: DynamicColor = ColorSpec2021::surface_container_low().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface_container",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        9.0
                    } else if Hct::is_yellow(scheme.neutral_palette.hue()) {
                        96.0
                    } else if scheme.variant == Variant::Vibrant {
                        92.0
                    } else {
                        94.0
                    }
                } else {
                    20.0
                }
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                match scheme.variant {
                    Variant::Neutral => 1.6,
                    Variant::TonalSpot => 1.4,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            1.6
                        } else {
                            1.3
                        }
                    }
                    Variant::Vibrant => 1.15,
                    _ => 1.0,
                }
            } else {
                1.0
            })
        });

        static COLOR: DynamicColor = ColorSpec2021::surface_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_container_high() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface_container_high",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        12.0
                    } else if Hct::is_yellow(scheme.neutral_palette.hue()) {
                        94.0
                    } else if scheme.variant == Variant::Vibrant {
                        90.0
                    } else {
                        92.0
                    }
                } else {
                    25.0
                }
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                match scheme.variant {
                    Variant::Neutral => 1.9,
                    Variant::TonalSpot => 1.5,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            1.95
                        } else {
                            1.45
                        }
                    }
                    Variant::Vibrant => 1.22,
                    _ => 1.0,
                }
            } else {
                1.0
            })
        });

        static COLOR: DynamicColor = ColorSpec2021::surface_container_high().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_container_highest() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "surface_container_highest",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.is_dark {
                    15.0
                } else if Hct::is_yellow(scheme.neutral_palette.hue()) {
                    92.0
                } else if scheme.variant == Variant::Vibrant {
                    88.0
                } else {
                    90.0
                }
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(match scheme.variant {
                Variant::Neutral => 2.2,
                Variant::TonalSpot => 1.7,
                Variant::Expressive => {
                    if Hct::is_yellow(scheme.neutral_palette.hue()) {
                        2.3
                    } else {
                        1.6
                    }
                }
                Variant::Vibrant => 1.29,
                _ => 1.0,
            })
        });

        static COLOR: DynamicColor = ColorSpec2021::surface_container_highest().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_surface() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| {
                if scheme.variant == Variant::Vibrant {
                    t_max_c(&scheme.neutral_palette, 0.0, 100.0, 1.1)
                } else {
                    if scheme.platform == Platform::Phone {
                        if scheme.is_dark {
                            ColorSpec2025::surface_bright()
                        } else {
                            ColorSpec2025::surface_dim()
                        }
                    } else {
                        ColorSpec2025::surface_container_high()
                    }
                    .get_tone(scheme)
                }
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                match scheme.variant {
                    Variant::Neutral => 2.2,
                    Variant::TonalSpot => 1.7,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            if scheme.is_dark { 3.0 } else { 2.3 }
                        } else {
                            1.6
                        }
                    }
                    _ => 1.0,
                }
            } else {
                1.0
            })
        })
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.is_dark && scheme.platform == Platform::Phone {
                get_contrast_curve(11.0)
            } else {
                get_contrast_curve(9.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_surface().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn surface_variant() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = ColorSpec2025::surface_container_highest().with_name("surface_variant");
        static COLOR: DynamicColor = ColorSpec2021::surface_variant().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_surface_variant() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_surface_variant",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        ColorSpec2025::surface_bright()
                    } else {
                        ColorSpec2025::surface_dim()
                    }
                } else {
                    ColorSpec2025::surface_container_high()
                }
                .get_tone(scheme)
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                match scheme.variant {
                    Variant::Neutral => 2.2,
                    Variant::TonalSpot => 1.7,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            if scheme.is_dark { 3.0 } else { 2.3 }
                        } else {
                            1.6
                        }
                    }
                    _ => 1.0,
                }
            } else {
                1.0
            })
        })
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                if scheme.is_dark { get_contrast_curve(6.0) } else { get_contrast_curve(4.5) }
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_surface_variant().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn inverse_surface() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "inverse_surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| if scheme.is_dark { 98.0 } else { 4.0 },
        );

        static COLOR: DynamicColor = ColorSpec2021::inverse_surface().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn inverse_on_surface() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "inverse_on_surface",
            |_, scheme| &scheme.neutral_palette,
            |_, scheme| ColorSpec2025::inverse_surface().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::inverse_surface()))
        .with_contrast_curve(|_, _| get_contrast_curve(7.0));

        static COLOR: DynamicColor = ColorSpec2021::inverse_on_surface().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn outline() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "outline",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        ColorSpec2025::surface_bright()
                    } else {
                        ColorSpec2025::surface_dim()
                    }
                } else {
                    ColorSpec2025::surface_container_high()
                }
                .get_tone(scheme)
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                match scheme.variant {
                    Variant::Neutral => 2.2,
                    Variant::TonalSpot => 1.7,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            if scheme.is_dark { 3.0 } else { 2.3 }
                        } else {
                            1.6
                        }
                    }
                    _ => 1.0,
                }
            } else {
                1.0
            })
        })
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(3.0)
            } else {
                get_contrast_curve(4.5)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::outline().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn outline_variant() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "outline_variant",
            |_, scheme| &scheme.neutral_variant_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        ColorSpec2025::surface_bright()
                    } else {
                        ColorSpec2025::surface_dim()
                    }
                } else {
                    ColorSpec2025::surface_container_high()
                }
                .get_tone(scheme)
            },
        )
        .with_chroma_multiplier(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                match scheme.variant {
                    Variant::Neutral => 2.2,
                    Variant::TonalSpot => 1.7,
                    Variant::Expressive => {
                        if Hct::is_yellow(scheme.neutral_palette.hue()) {
                            if scheme.is_dark { 3.0 } else { 2.3 }
                        } else {
                            1.6
                        }
                    }
                    _ => 1.0,
                }
            } else {
                1.0
            })
        })
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(1.5)
            } else {
                get_contrast_curve(3.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::outline_variant().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn shadow() -> &'static DynamicColor {
        ColorSpec2021::shadow()
    }

    pub const fn scrim() -> &'static DynamicColor {
        ColorSpec2021::scrim()
    }

    pub const fn surface_tint() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = ColorSpec2025::primary().with_name("surface_tint");
        static COLOR: DynamicColor = ColorSpec2021::surface_tint().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn primary() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "primary",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
                match scheme.variant {
                    Variant::Neutral => {
                        if scheme.platform == Platform::Phone {
                            if scheme.is_dark { 80.0 } else { 40.0 }
                        } else {
                            90.0
                        }
                    }
                    Variant::TonalSpot => {
                        if scheme.platform == Platform::Phone {
                            if scheme.is_dark {
                                80.0
                            } else {
                                t_max_c(&scheme.primary_palette, 0.0, 100.0, 1.0)
                            }
                        } else {
                            t_max_c(&scheme.primary_palette, 0.0, 90.0, 1.0)
                        }
                    }
                    Variant::Expressive => {
                        if scheme.platform == Platform::Phone {
                            t_max_c(
                                &scheme.primary_palette,
                                0.0,
                                if Hct::is_yellow(scheme.primary_palette.hue()) {
                                    25.0
                                } else if Hct::is_cyan(scheme.primary_palette.hue()) {
                                    88.0
                                } else {
                                    98.0
                                },
                                1.0,
                            )
                        } else {
                            // WATCH
                            t_max_c(&scheme.primary_palette, 0.0, 100.0, 1.0)
                        }
                    }
                    _ => {
                        // VIBRANT
                        if scheme.platform == Platform::Phone {
                            t_max_c(
                                &scheme.primary_palette,
                                0.0,
                                if Hct::is_cyan(scheme.primary_palette.hue()) { 88.0 } else { 98.0 },
                                1.0,
                            )
                        } else {
                            // WATCH
                            t_max_c(&scheme.primary_palette, 0.0, 100.0, 1.0)
                        }
                    }
                }
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(4.5)
            } else {
                get_contrast_curve(7.0)
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::primary_container(),
                    ColorSpec2025::primary(),
                    5.0,
                    TonePolarity::RelativeLighter,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::primary().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn primary_dim() -> Option<&'static DynamicColor> {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "primary_dim",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| match scheme.variant {
                Variant::Neutral => 85.0,
                Variant::TonalSpot => t_max_c(&scheme.primary_palette, 0.0, 90.0, 1.0),
                _ => t_max_c(&scheme.primary_palette, 0.0, 100.0, 1.0),
            },
        )
        .with_background(|_, _| Some(ColorSpec2025::surface_container_high()))
        .with_contrast_curve(|_, _| get_contrast_curve(4.5))
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                unsafe { ColorSpec2025::primary_dim().unwrap_unchecked() },
                ColorSpec2025::primary(),
                5.0,
                TonePolarity::Darker,
                true,
                DeltaConstraint::Farther,
            ))
        });

        Some(&COLOR)
    }

    pub const fn on_primary() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_primary",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    ColorSpec2025::primary()
                } else {
                    unsafe { ColorSpec2025::primary_dim().unwrap_unchecked() }
                }
                .get_tone(scheme)
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                ColorSpec2025::primary()
            } else {
                unsafe { ColorSpec2025::primary_dim().unwrap_unchecked() }
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_primary().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn primary_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "primary_container",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Watch {
                    30.0
                } else {
                    match scheme.variant {
                        Variant::Neutral => {
                            if scheme.is_dark {
                                30.0
                            } else {
                                90.0
                            }
                        }
                        Variant::TonalSpot => {
                            if scheme.is_dark {
                                t_min_c(&scheme.primary_palette, 35.0, 93.0)
                            } else {
                                t_max_c(&scheme.primary_palette, 0.0, 90.0, 1.0)
                            }
                        }
                        Variant::Expressive => {
                            if scheme.is_dark {
                                t_max_c(&scheme.primary_palette, 30.0, 93.0, 1.0)
                            } else {
                                t_max_c(
                                    &scheme.primary_palette,
                                    78.0,
                                    if Hct::is_cyan(scheme.primary_palette.hue()) { 88.0 } else { 90.0 },
                                    1.0,
                                )
                            }
                        }
                        _ => {
                            // VIBRANT
                            if scheme.is_dark {
                                t_min_c(&scheme.primary_palette, 66.0, 93.0)
                            } else {
                                t_max_c(
                                    &scheme.primary_palette,
                                    66.0,
                                    if Hct::is_cyan(scheme.primary_palette.hue()) { 88.0 } else { 93.0 },
                                    1.0,
                                )
                            }
                        }
                    }
                }
            },
        )
        .with_background(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                })
            } else {
                None
            }
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone && scheme.contrast_level > 0.0 {
                get_contrast_curve(1.5)
            } else {
                None
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Watch {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::primary_container(),
                    unsafe { ColorSpec2025::primary_dim().unwrap_unchecked() },
                    10.0,
                    TonePolarity::Darker,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::primary_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_primary_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_primary_container",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| ColorSpec2025::primary_container().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::primary_container()))
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_primary_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn inverse_primary() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "inverse_primary",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| t_max_c(&scheme.primary_palette, 0.0, 100.0, 1.0),
        )
        .with_background(|_, _| Some(ColorSpec2025::inverse_surface()))
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::inverse_primary().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn secondary() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "secondary",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Watch {
                    if scheme.variant == Variant::Neutral {
                        90.0
                    } else {
                        t_max_c(&scheme.secondary_palette, 0.0, 90.0, 1.0)
                    }
                } else {
                    match scheme.variant {
                        Variant::Neutral => {
                            if scheme.is_dark {
                                t_min_c(&scheme.secondary_palette, 0.0, 98.0)
                            } else {
                                t_max_c(&scheme.secondary_palette, 0.0, 100.0, 1.0)
                            }
                        }
                        Variant::Vibrant => t_max_c(&scheme.secondary_palette, 0.0, if scheme.is_dark { 90.0 } else { 98.0 }, 1.0),
                        _ => {
                            // EXPRESSIVE and TONAL_SPOT
                            if scheme.is_dark {
                                80.0
                            } else {
                                t_max_c(&scheme.secondary_palette, 0.0, 100.0, 1.0)
                            }
                        }
                    }
                }
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(4.5)
            } else {
                get_contrast_curve(7.0)
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::secondary_container(),
                    ColorSpec2025::secondary(),
                    5.0,
                    TonePolarity::RelativeLighter,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::secondary().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn secondary_dim() -> Option<&'static DynamicColor> {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "secondary_dim",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                if scheme.variant == Variant::Neutral {
                    85.0
                } else {
                    t_max_c(&scheme.secondary_palette, 0.0, 90.0, 1.0)
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2025::surface_container_high()))
        .with_contrast_curve(|_, _| get_contrast_curve(4.5))
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                unsafe { ColorSpec2025::secondary_dim().unwrap_unchecked() },
                ColorSpec2025::secondary(),
                5.0,
                TonePolarity::Darker,
                true,
                DeltaConstraint::Farther,
            ))
        });

        Some(&COLOR)
    }

    pub const fn on_secondary() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_secondary",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    ColorSpec2025::secondary()
                } else {
                    unsafe { ColorSpec2025::secondary_dim().unwrap_unchecked() }
                }
                .get_tone(scheme)
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                ColorSpec2025::secondary()
            } else {
                unsafe { ColorSpec2025::secondary_dim().unwrap_unchecked() }
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_secondary().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn secondary_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "secondary_container",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Watch {
                    30.0
                } else {
                    match scheme.variant {
                        Variant::Vibrant => {
                            if scheme.is_dark {
                                t_min_c(&scheme.secondary_palette, 30.0, 40.0)
                            } else {
                                t_max_c(&scheme.secondary_palette, 84.0, 90.0, 1.0)
                            }
                        }
                        Variant::Expressive => {
                            if scheme.is_dark {
                                15.0
                            } else {
                                t_max_c(&scheme.secondary_palette, 90.0, 95.0, 1.0)
                            }
                        }
                        _ => {
                            if scheme.is_dark {
                                25.0
                            } else {
                                90.0
                            }
                        }
                    }
                }
            },
        )
        .with_background(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                })
            } else {
                None
            }
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone && scheme.contrast_level > 0.0 {
                get_contrast_curve(1.5)
            } else {
                None
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Watch {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::secondary_container(),
                    unsafe { ColorSpec2025::secondary_dim().unwrap_unchecked() },
                    10.0,
                    TonePolarity::Darker,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::secondary_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_secondary_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_secondary_container",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| ColorSpec2025::secondary_container().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::secondary_container()))
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_secondary_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn tertiary() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "tertiary",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Watch {
                    if scheme.variant == Variant::TonalSpot {
                        t_max_c(&scheme.tertiary_palette, 0.0, 90.0, 1.0)
                    } else {
                        t_max_c(&scheme.tertiary_palette, 0.0, 100.0, 1.0)
                    }
                } else {
                    match scheme.variant {
                        Variant::Expressive | Variant::Vibrant => t_max_c(
                            &scheme.tertiary_palette,
                            0.0,
                            if Hct::is_cyan(scheme.tertiary_palette.hue()) {
                                88.0
                            } else if scheme.is_dark {
                                98.0
                            } else {
                                100.0
                            },
                            1.0,
                        ),
                        _ => {
                            // NEUTRAL and TONAL_SPOT
                            if scheme.is_dark {
                                t_max_c(&scheme.tertiary_palette, 0.0, 98.0, 1.0)
                            } else {
                                t_max_c(&scheme.tertiary_palette, 0.0, 100.0, 1.0)
                            }
                        }
                    }
                }
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(4.5)
            } else {
                get_contrast_curve(7.0)
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::tertiary_container(),
                    ColorSpec2025::tertiary(),
                    5.0,
                    TonePolarity::RelativeLighter,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::tertiary().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn tertiary_dim() -> Option<&'static DynamicColor> {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "tertiary_dim",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if scheme.variant == Variant::TonalSpot {
                    t_max_c(&scheme.tertiary_palette, 0.0, 90.0, 1.0)
                } else {
                    t_max_c(&scheme.tertiary_palette, 0.0, 100.0, 1.0)
                }
            },
        )
        .with_background(|_, _| Some(ColorSpec2025::surface_container_high()))
        .with_contrast_curve(|_, _| get_contrast_curve(4.5))
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                unsafe { ColorSpec2025::tertiary_dim().unwrap_unchecked() },
                ColorSpec2025::tertiary(),
                5.0,
                TonePolarity::Darker,
                true,
                DeltaConstraint::Farther,
            ))
        });

        Some(&COLOR)
    }

    pub const fn on_tertiary() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    ColorSpec2025::tertiary()
                } else {
                    unsafe { ColorSpec2025::tertiary_dim().unwrap_unchecked() }
                }
                .get_tone(scheme)
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                ColorSpec2025::tertiary()
            } else {
                unsafe { ColorSpec2025::tertiary_dim().unwrap_unchecked() }
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_tertiary().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn tertiary_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "tertiary_container",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                if scheme.platform == Platform::Watch {
                    if scheme.variant == Variant::TonalSpot {
                        t_max_c(&scheme.tertiary_palette, 0.0, 90.0, 1.0)
                    } else {
                        t_max_c(&scheme.tertiary_palette, 0.0, 100.0, 1.0)
                    }
                } else {
                    match scheme.variant {
                        Variant::Neutral => {
                            if scheme.is_dark {
                                t_max_c(&scheme.tertiary_palette, 0.0, 93.0, 1.0)
                            } else {
                                t_max_c(&scheme.tertiary_palette, 0.0, 96.0, 1.0)
                            }
                        }
                        Variant::TonalSpot => t_max_c(&scheme.tertiary_palette, 0.0, if scheme.is_dark { 93.0 } else { 100.0 }, 1.0),
                        Variant::Expressive => t_max_c(
                            &scheme.tertiary_palette,
                            75.0,
                            if Hct::is_cyan(scheme.tertiary_palette.hue()) {
                                88.0
                            } else if scheme.is_dark {
                                93.0
                            } else {
                                100.0
                            },
                            1.0,
                        ),
                        _ => {
                            // VIBRANT
                            if scheme.is_dark {
                                t_max_c(&scheme.tertiary_palette, 0.0, 93.0, 1.0)
                            } else {
                                t_max_c(&scheme.tertiary_palette, 72.0, 100.0, 1.0)
                            }
                        }
                    }
                }
            },
        )
        .with_background(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                })
            } else {
                None
            }
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone && scheme.contrast_level > 0.0 {
                get_contrast_curve(1.5)
            } else {
                None
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Watch {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::tertiary_container(),
                    unsafe { ColorSpec2025::tertiary_dim().unwrap_unchecked() },
                    10.0,
                    TonePolarity::Darker,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::tertiary_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_tertiary_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary_container",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| ColorSpec2025::tertiary_container().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::tertiary_container()))
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_tertiary_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn error() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "error",
            |_, scheme| &scheme.error_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    if scheme.is_dark {
                        t_min_c(&scheme.error_palette, 0.0, 98.0)
                    } else {
                        t_max_c(&scheme.error_palette, 0.0, 100.0, 1.0)
                    }
                } else {
                    t_min_c(&scheme.error_palette, 0.0, 100.0)
                }
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                }
            } else {
                ColorSpec2025::surface_container_high()
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(4.5)
            } else {
                get_contrast_curve(7.0)
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::error_container(),
                    ColorSpec2025::error(),
                    5.0,
                    TonePolarity::RelativeLighter,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::error().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn error_dim() -> Option<&'static DynamicColor> {
        static COLOR: DynamicColor = DynamicColor::background_color(
            "error_dim",
            |_, scheme| &scheme.error_palette,
            |_, scheme| t_min_c(&scheme.error_palette, 0.0, 100.0),
        )
        .with_background(|_, _| Some(ColorSpec2025::surface_container_high()))
        .with_contrast_curve(|_, _| get_contrast_curve(4.5))
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                unsafe { ColorSpec2025::error_dim().unwrap_unchecked() },
                ColorSpec2025::error(),
                5.0,
                TonePolarity::Darker,
                true,
                DeltaConstraint::Farther,
            ))
        });

        Some(&COLOR)
    }

    pub const fn on_error() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_error",
            |_, scheme| &scheme.error_palette,
            |_, scheme| {
                if scheme.platform == Platform::Phone {
                    ColorSpec2025::error()
                } else {
                    unsafe { ColorSpec2025::error_dim().unwrap_unchecked() }
                }
                .get_tone(scheme)
            },
        )
        .with_background(|_, scheme| {
            Some(if scheme.platform == Platform::Phone {
                ColorSpec2025::error()
            } else {
                unsafe { ColorSpec2025::error_dim().unwrap_unchecked() }
            })
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(6.0)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_error().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn error_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "error_container",
            |_, scheme| &scheme.error_palette,
            |_, scheme| {
                if scheme.platform == Platform::Watch {
                    30.0
                } else if scheme.is_dark {
                    t_min_c(&scheme.error_palette, 30.0, 93.0)
                } else {
                    t_max_c(&scheme.error_palette, 0.0, 90.0, 1.0)
                }
            },
        )
        .with_background(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                })
            } else {
                None
            }
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone && scheme.contrast_level > 0.0 {
                get_contrast_curve(1.5)
            } else {
                None
            }
        })
        .with_tone_delta_pair(|_, scheme| {
            if scheme.platform == Platform::Watch {
                Some(ToneDeltaPair::new(
                    ColorSpec2025::error_container(),
                    unsafe { ColorSpec2025::error_dim().unwrap_unchecked() },
                    10.0,
                    TonePolarity::Darker,
                    true,
                    DeltaConstraint::Farther,
                ))
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::error_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_error_container() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_error_container",
            |_, scheme| &scheme.error_palette,
            |_, scheme| ColorSpec2025::error_container().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::error_container()))
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone {
                get_contrast_curve(4.5)
            } else {
                get_contrast_curve(7.0)
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::on_error_container().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn primary_fixed() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "primary_fixed",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| {
                let mut temp_scheme = scheme.clone();

                temp_scheme.is_dark = false;
                temp_scheme.contrast_level = 0.0;

                ColorSpec2025::primary_container().get_tone(&temp_scheme)
            },
        )
        .with_background(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                })
            } else {
                None
            }
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone && scheme.contrast_level > 0.0 {
                get_contrast_curve(1.5)
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::primary_fixed().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn primary_fixed_dim() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "primary_fixed_dim",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| ColorSpec2025::primary_fixed().get_tone(scheme),
        )
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2025::primary_fixed_dim(),
                ColorSpec2025::primary_fixed(),
                5.0,
                TonePolarity::Darker,
                true,
                DeltaConstraint::Exact,
            ))
        });

        static COLOR: DynamicColor = ColorSpec2021::primary_fixed_dim().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_primary_fixed() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_primary_fixed",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| ColorSpec2025::primary_fixed_dim().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::primary_fixed_dim()))
        .with_contrast_curve(|_, _scheme| get_contrast_curve(7.0));

        static COLOR: DynamicColor = ColorSpec2021::on_primary_fixed().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_primary_fixed_variant() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_primary_fixed_variant",
            |_, scheme| &scheme.primary_palette,
            |_, scheme| ColorSpec2025::primary_fixed_dim().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::primary_fixed_dim()))
        .with_contrast_curve(|_, _scheme| get_contrast_curve(4.5));

        static COLOR: DynamicColor = ColorSpec2021::on_primary_fixed_variant().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn secondary_fixed() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "secondary_fixed",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| {
                let mut temp_scheme = scheme.clone();

                temp_scheme.is_dark = false;
                temp_scheme.contrast_level = 0.0;

                ColorSpec2025::secondary_container().get_tone(&temp_scheme)
            },
        )
        .with_background(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                })
            } else {
                None
            }
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone && scheme.contrast_level > 0.0 {
                get_contrast_curve(1.5)
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::secondary_fixed().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn secondary_fixed_dim() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "secondary_fixed_dim",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| ColorSpec2025::secondary_fixed().get_tone(scheme),
        )
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2025::secondary_fixed_dim(),
                ColorSpec2025::secondary_fixed(),
                5.0,
                TonePolarity::Darker,
                true,
                DeltaConstraint::Exact,
            ))
        });

        static COLOR: DynamicColor = ColorSpec2021::secondary_fixed_dim().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_secondary_fixed() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_secondary_fixed",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| ColorSpec2025::secondary_fixed_dim().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::secondary_fixed_dim()))
        .with_contrast_curve(|_, _scheme| get_contrast_curve(7.0));

        static COLOR: DynamicColor = ColorSpec2021::on_secondary_fixed().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_secondary_fixed_variant() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_secondary_fixed_variant",
            |_, scheme| &scheme.secondary_palette,
            |_, scheme| ColorSpec2025::secondary_fixed_dim().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::secondary_fixed_dim()))
        .with_contrast_curve(|_, _scheme| get_contrast_curve(4.5));

        static COLOR: DynamicColor = ColorSpec2021::on_secondary_fixed_variant().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn tertiary_fixed() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "tertiary_fixed",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| {
                let mut temp_scheme = scheme.clone();

                temp_scheme.is_dark = false;
                temp_scheme.contrast_level = 0.0;

                ColorSpec2025::tertiary_container().get_tone(&temp_scheme)
            },
        )
        .with_background(|_, scheme| {
            if scheme.platform == Platform::Phone {
                Some(if scheme.is_dark {
                    ColorSpec2025::surface_bright()
                } else {
                    ColorSpec2025::surface_dim()
                })
            } else {
                None
            }
        })
        .with_contrast_curve(|_, scheme| {
            if scheme.platform == Platform::Phone && scheme.contrast_level > 0.0 {
                get_contrast_curve(1.5)
            } else {
                None
            }
        });

        static COLOR: DynamicColor = ColorSpec2021::tertiary_fixed().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn tertiary_fixed_dim() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::background_color(
            "tertiary_fixed_dim",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| ColorSpec2025::tertiary_fixed().get_tone(scheme),
        )
        .with_tone_delta_pair(|_, _| {
            Some(ToneDeltaPair::new(
                ColorSpec2025::tertiary_fixed_dim(),
                ColorSpec2025::tertiary_fixed(),
                5.0,
                TonePolarity::Darker,
                true,
                DeltaConstraint::Exact,
            ))
        });

        static COLOR: DynamicColor = ColorSpec2021::tertiary_fixed_dim().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_tertiary_fixed() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary_fixed",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| ColorSpec2025::tertiary_fixed_dim().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::tertiary_fixed_dim()))
        .with_contrast_curve(|_, _scheme| get_contrast_curve(7.0));

        static COLOR: DynamicColor = ColorSpec2021::on_tertiary_fixed().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub const fn on_tertiary_fixed_variant() -> &'static DynamicColor {
        static COLOR_2025: DynamicColor = DynamicColor::foreground_color(
            "on_tertiary_fixed_variant",
            |_, scheme| &scheme.tertiary_palette,
            |_, scheme| ColorSpec2025::tertiary_fixed_dim().get_tone(scheme),
        )
        .with_background(|_, _| Some(ColorSpec2025::tertiary_fixed_dim()))
        .with_contrast_curve(|_, _scheme| get_contrast_curve(4.5));

        static COLOR: DynamicColor = ColorSpec2021::on_tertiary_fixed_variant().extend_spec_version(SpecVersion::Spec2025, &COLOR_2025);

        &COLOR
    }

    pub fn get_hct(&self, scheme: &DynamicScheme, color: &DynamicColor) -> Hct {
        let palette = color.palette(scheme);
        let tone = self.get_tone(scheme, color);
        let chroma_multiplier = color.chroma_multiplier(scheme).unwrap_or(1.0);

        if chroma_multiplier == 1.0 {
            return palette.get_hct(tone);
        }

        let chroma = palette.chroma() * chroma_multiplier;

        if tone == 99.0 && Hct::is_yellow(palette.hue()) {
            return TonalPalette::from_hue_and_chroma(palette.hue(), chroma).get_hct(tone);
        }

        Hct::from(palette.hue(), chroma, tone)
    }

    pub fn get_tone(&self, scheme: &DynamicScheme, color: &DynamicColor) -> f64 {
        let tone_delta_pair = color.tone_delta_pair(scheme);

        // Case 0: tone delta pair.
        if let Some(tone_delta_pair) = tone_delta_pair {
            let role_a = tone_delta_pair.subject;
            let role_b = tone_delta_pair.basis;
            let polarity = tone_delta_pair.polarity;
            let constraint = tone_delta_pair.constraint;
            let absolute_delta = if polarity == TonePolarity::Darker
                || (polarity == TonePolarity::RelativeLighter && scheme.is_dark)
                || (polarity == TonePolarity::RelativeDarker && !scheme.is_dark)
            {
                -tone_delta_pair.delta
            } else {
                tone_delta_pair.delta
            };

            let am_role_a = color.name == role_a.name;
            let self_role = if am_role_a { role_a } else { role_b };
            let reference_role = if am_role_a { role_b } else { role_a };
            let mut self_tone = self_role.tone(scheme);
            let reference_tone = reference_role.get_tone(scheme);
            let relative_delta = absolute_delta * (if am_role_a { 1.0 } else { -1.0 });

            match constraint {
                DeltaConstraint::Exact => self_tone = (reference_tone + relative_delta).clamp(0.0, 100.0),
                DeltaConstraint::Nearer => {
                    if relative_delta > 0.0 {
                        self_tone = self_tone.clamp(reference_tone, reference_tone + relative_delta).clamp(0.0, 100.0);
                    } else {
                        self_tone = self_tone.clamp(reference_tone + relative_delta, reference_tone).clamp(0.0, 100.0);
                    }
                }
                DeltaConstraint::Farther => {
                    if relative_delta > 0.0 {
                        self_tone = self_tone.clamp(reference_tone + relative_delta, 100.0);
                    } else {
                        self_tone = self_tone.clamp(0.0, reference_tone + relative_delta);
                    }
                }
            }

            let background = color.background(scheme);
            let contrast_curve = color.contrast_curve(scheme);

            if let (Some(background), Some(contrast_curve)) = (background, contrast_curve) {
                let bg_tone = background.get_tone(scheme);
                let self_contrast = contrast_curve.get(scheme.contrast_level);

                self_tone = if ratio_of_tones(bg_tone, self_tone) >= self_contrast && scheme.contrast_level >= 0.0 {
                    self_tone
                } else {
                    DynamicColor::foreground_tone(bg_tone, self_contrast)
                };
            }

            // This can avoid the awkward tones for background colors including the access
            // fixed colors. Accent fixed dim colors should not be adjusted.
            if color.is_background && !color.name.ends_with("_fixed_dim") {
                self_tone = if self_tone >= 57.0 {
                    self_tone.clamp(65.0, 100.0)
                } else {
                    self_tone.clamp(0.0, 49.0)
                }
            }

            self_tone
        } else {
            // Case 1: No tone delta pair; just solve for itself.
            let mut answer = color.tone(scheme);
            let background = color.background(scheme);
            let contrast_curve = color.contrast_curve(scheme);

            if let (Some(background), Some(contrast_curve)) = (background, contrast_curve) {
                let bg_tone = background.get_tone(scheme);
                let desired_ratio = contrast_curve.get(scheme.contrast_level);

                // Recalculate the tone from desired contrast ratio if the current
                // contrast ratio is not enough or desired contrast level is decreasing
                // (<0).
                answer = if ratio_of_tones(bg_tone, answer) >= desired_ratio && scheme.contrast_level >= 0.0 {
                    answer
                } else {
                    DynamicColor::foreground_tone(bg_tone, desired_ratio)
                };

                // This can avoid the awkward tones for background colors including the access
                // fixed colors. Accent fixed dim colors should not be adjusted.
                if color.is_background && !color.name.ends_with("_fixed_dim") {
                    answer = if answer >= 57.0 { answer.clamp(65.0, 100.0) } else { answer.clamp(0.0, 49.0) }
                }

                let second_background = color.second_background(scheme);

                if let Some(second_background) = second_background {
                    // Case 2: Adjust for dual backgrounds.
                    let bg_tone1 = background.get_tone(scheme);
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
                    let mut availables = vec![];

                    if let Some(light_option) = light_option {
                        availables.push(light_option);
                    }

                    if let Some(dark_option) = dark_option {
                        availables.push(dark_option);
                    }

                    if DynamicColor::tone_prefers_light_foreground(bg_tone1) || DynamicColor::tone_prefers_light_foreground(bg_tone2) {
                        light_option.unwrap_or(100.0)
                    } else if availables.len() == 1 {
                        availables[0]
                    } else {
                        dark_option.unwrap_or(0.0)
                    }
                } else {
                    answer
                }
            } else {
                answer // No adjustment for colors with no background.
            }
        }
    }

    pub fn get_primary_palette(variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        match variant {
            Variant::Neutral => TonalPalette::from_hue_and_chroma(
                source_color_hct.get_hue(),
                if platform == Platform::Phone {
                    if Hct::is_blue(source_color_hct.get_hue()) { 12.0 } else { 8.0 }
                } else if Hct::is_blue(source_color_hct.get_hue()) {
                    16.0
                } else {
                    12.0
                },
            ),
            Variant::TonalSpot => {
                TonalPalette::from_hue_and_chroma(source_color_hct.get_hue(), if platform == Platform::Phone && is_dark { 26.0 } else { 32.0 })
            }
            Variant::Expressive => TonalPalette::from_hue_and_chroma(
                source_color_hct.get_hue(),
                if platform == Platform::Phone {
                    if is_dark { 36.0 } else { 48.0 }
                } else {
                    40.0
                },
            ),
            Variant::Vibrant => TonalPalette::from_hue_and_chroma(source_color_hct.get_hue(), if platform == Platform::Phone { 74.0 } else { 56.0 }),
            _ => ColorSpec2021::get_primary_palette(variant, source_color_hct, is_dark, platform, contrast_level),
        }
    }

    pub fn get_secondary_palette(variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        match variant {
            Variant::Neutral => TonalPalette::from_hue_and_chroma(
                source_color_hct.get_hue(),
                if platform == Platform::Phone {
                    if Hct::is_blue(source_color_hct.get_hue()) { 6.0 } else { 4.0 }
                } else if Hct::is_blue(source_color_hct.get_hue()) {
                    10.0
                } else {
                    6.0
                },
            ),
            Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.get_hue(), 16.0),
            Variant::Expressive => TonalPalette::from_hue_and_chroma(
                DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0], &[
                    -160.0, 155.0, -100.0, 96.0, -96.0, -156.0, -165.0, -160.0,
                ]),
                if platform == Platform::Phone {
                    if is_dark { 16.0 } else { 24.0 }
                } else {
                    24.0
                },
            ),
            Variant::Vibrant => TonalPalette::from_hue_and_chroma(
                DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 38.0, 105.0, 140.0, 333.0, 360.0], &[
                    -14.0, 10.0, -14.0, 10.0, -14.0,
                ]),
                if platform == Platform::Phone { 56.0 } else { 36.0 },
            ),
            _ => ColorSpec2021::get_secondary_palette(variant, source_color_hct, is_dark, platform, contrast_level),
        }
    }

    pub fn get_tertiary_palette(variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        match variant {
            Variant::Neutral => TonalPalette::from_hue_and_chroma(
                DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 38.0, 105.0, 161.0, 204.0, 278.0, 333.0, 360.0], &[
                    -32.0, 26.0, 10.0, -39.0, 24.0, -15.0, -32.0,
                ]),
                if platform == Platform::Phone { 20.0 } else { 36.0 },
            ),
            Variant::TonalSpot => TonalPalette::from_hue_and_chroma(
                DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 20.0, 71.0, 161.0, 333.0, 360.0], &[
                    -40.0, 48.0, -32.0, 40.0, -32.0,
                ]),
                if platform == Platform::Phone { 28.0 } else { 32.0 },
            ),
            Variant::Expressive => TonalPalette::from_hue_and_chroma(
                DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0], &[
                    -165.0, 160.0, -105.0, 101.0, -101.0, -160.0, -170.0, -165.0,
                ]),
                48.0,
            ),
            Variant::Vibrant => TonalPalette::from_hue_and_chroma(
                DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 38.0, 71.0, 105.0, 140.0, 161.0, 253.0, 333.0, 360.0], &[
                    -72.0, 35.0, 24.0, -24.0, 62.0, 50.0, 62.0, -72.0,
                ]),
                56.0,
            ),
            _ => ColorSpec2021::get_tertiary_palette(variant, source_color_hct, is_dark, platform, contrast_level),
        }
    }

    pub fn get_neutral_palette(variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        match variant {
            Variant::Neutral => TonalPalette::from_hue_and_chroma(source_color_hct.get_hue(), if platform == Platform::Phone { 1.4 } else { 6.0 }),
            Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.get_hue(), if platform == Platform::Phone { 5.0 } else { 10.0 }),
            Variant::Expressive => TonalPalette::from_hue_and_chroma(
                get_expressive_neutral_hue(source_color_hct),
                get_expressive_neutral_chroma(source_color_hct, is_dark, platform),
            ),
            Variant::Vibrant => TonalPalette::from_hue_and_chroma(
                get_vibrant_neutral_hue(source_color_hct),
                get_vibrant_neutral_chroma(source_color_hct, platform),
            ),
            _ => ColorSpec2021::get_neutral_palette(variant, source_color_hct, is_dark, platform, contrast_level),
        }
    }

    pub fn get_neutral_variant_palette(variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        match variant {
            Variant::Neutral => TonalPalette::from_hue_and_chroma(source_color_hct.get_hue(), (if platform == Platform::Phone { 1.4 } else { 6.0 }) * 2.2),
            Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.get_hue(), (if platform == Platform::Phone { 5.0 } else { 10.0 }) * 1.7),
            Variant::Expressive => {
                let expressive_neutral_hue = get_expressive_neutral_hue(source_color_hct);
                let expressive_neutral_chroma = get_expressive_neutral_chroma(source_color_hct, is_dark, platform);

                TonalPalette::from_hue_and_chroma(
                    expressive_neutral_hue,
                    expressive_neutral_chroma * if (105.0..125.0).contains(&expressive_neutral_hue) { 1.6 } else { 2.3 },
                )
            }
            Variant::Vibrant => {
                let vibrant_neutral_hue = get_vibrant_neutral_hue(source_color_hct);
                let vibrant_neutral_chroma = get_vibrant_neutral_chroma(source_color_hct, platform);

                TonalPalette::from_hue_and_chroma(vibrant_neutral_hue, vibrant_neutral_chroma * 1.29)
            }
            _ => ColorSpec2021::get_neutral_variant_palette(variant, source_color_hct, is_dark, platform, contrast_level),
        }
    }

    pub fn get_error_palette(variant: Variant, source_color_hct: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> TonalPalette {
        let error_hue = DynamicScheme::get_rotated_hue(source_color_hct.get_hue(), &[0.0, 3.0, 13.0, 23.0, 33.0, 43.0, 153.0, 273.0, 360.0], &[
            12.0, 22.0, 32.0, 12.0, 22.0, 32.0, 22.0, 12.0,
        ]);

        match variant {
            Variant::Neutral => TonalPalette::from_hue_and_chroma(error_hue, if platform == Platform::Phone { 50.0 } else { 40.0 }),
            Variant::TonalSpot => TonalPalette::from_hue_and_chroma(error_hue, if platform == Platform::Phone { 60.0 } else { 48.0 }),
            Variant::Expressive => TonalPalette::from_hue_and_chroma(error_hue, if platform == Platform::Phone { 64.0 } else { 48.0 }),
            Variant::Vibrant => TonalPalette::from_hue_and_chroma(error_hue, if platform == Platform::Phone { 80.0 } else { 60.0 }),
            _ => ColorSpec2021::get_error_palette(variant, source_color_hct, is_dark, platform, contrast_level),
        }
    }
}

impl ColorSpec for ColorSpec2025 {
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
        const { Self::primary_dim() }
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
        const { Self::secondary_dim() }
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
        const { Self::tertiary_dim() }
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
        const { Self::error_dim() }
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
