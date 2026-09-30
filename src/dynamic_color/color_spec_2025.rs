//! `ColorSpec` implementation for the 2025 spec.

use super::{
    Context, ContrastCurve, DeltaConstraint, DynamicColor, DynamicScheme, Platform, Role, SchemePalette, SpecVersion, ToneCache, ToneDeltaPair, TonePolarity,
    Variant,
    color_spec::{ColorSpec, Entry, SpecTable, coerce_in, contrast_curve, dual_background_tone, spec_table},
    color_spec_2021,
};
use crate::{contrast::ratio_of_tones, hct::Hct, palette::TonalPalette};

#[derive(Debug, Clone, Copy, Default)]
pub struct ColorSpec2025;

#[inline]
const fn phone(scheme: &DynamicScheme) -> bool {
    matches!(scheme.platform, Platform::Phone)
}

#[inline]
const fn neutral_is_yellow(scheme: &DynamicScheme) -> bool {
    Hct::is_yellow(scheme.neutral_palette.hue())
}

const fn surface_background(context: Context<'_>) -> Option<DynamicColor<'static>> {
    Some(DynamicColor::Material(if !phone(context.scheme()) {
        Role::SurfaceContainerHigh
    } else if context.scheme().is_dark {
        Role::SurfaceBright
    } else {
        Role::SurfaceDim
    }))
}

const fn phone_surface_background(context: Context<'_>) -> Option<DynamicColor<'static>> {
    if phone(context.scheme()) {
        Some(DynamicColor::Material(if context.scheme().is_dark {
            Role::SurfaceBright
        } else {
            Role::SurfaceDim
        }))
    } else {
        None
    }
}

const fn container_curve(context: Context<'_>) -> Option<ContrastCurve> {
    if phone(context.scheme()) && context.scheme().contrast_level > 0.0 {
        Some(contrast_curve(1.5))
    } else {
        None
    }
}

#[inline]
const fn phone_or(scheme: &DynamicScheme, phone_contrast: f64, watch_contrast: f64) -> Option<ContrastCurve> {
    Some(contrast_curve(if phone(scheme) { phone_contrast } else { watch_contrast }))
}

const fn on_neutral_chroma(context: Context<'_>) -> f64 {
    let scheme = context.scheme();

    if !phone(scheme) {
        return 1.0;
    }

    match scheme.variant {
        Variant::Neutral => 2.2,
        Variant::TonalSpot => 1.7,
        Variant::Expressive if neutral_is_yellow(scheme) => {
            if scheme.is_dark {
                3.0
            } else {
                2.3
            }
        }
        Variant::Expressive => 1.6,
        _ => 1.0,
    }
}

#[inline]
const fn light_surface(scheme: &DynamicScheme, yellow: f64, vibrant: f64, default: f64) -> f64 {
    if neutral_is_yellow(scheme) {
        yellow
    } else if matches!(scheme.variant, Variant::Vibrant) {
        vibrant
    } else {
        default
    }
}

#[inline]
const fn surface_chroma(scheme: &DynamicScheme, neutral: f64, tonal_spot: f64, expressive_yellow: f64, expressive: f64, vibrant: f64) -> f64 {
    match scheme.variant {
        Variant::Neutral => neutral,
        Variant::TonalSpot => tonal_spot,
        Variant::Expressive => {
            if neutral_is_yellow(scheme) {
                expressive_yellow
            } else {
                expressive
            }
        }
        Variant::Vibrant => vibrant,
        _ => 1.0,
    }
}

const fn farther_pair(a: Role, b: Role, delta: f64, polarity: TonePolarity) -> ToneDeltaPair<'static> {
    ToneDeltaPair::new(DynamicColor::Material(a), DynamicColor::Material(b), delta, polarity).with_constraint(DeltaConstraint::Farther)
}

const fn fixed_dim_pair(dim: Role, fixed: Role) -> ToneDeltaPair<'static> {
    ToneDeltaPair::new(DynamicColor::Material(dim), DynamicColor::Material(fixed), 5.0, TonePolarity::Darker)
}

static DECLARED: SpecTable = TABLE;

pub const TABLE: SpecTable = spec_table! {
    // Surfaces.
    Background: alias(Surface),
    OnBackground: alias(OnSurface) {
        tone: |scheme| if scheme.platform == Platform::Watch { 100.0 } else { scheme.tone(DynamicColor::Material(Role::OnSurface)) },
    },

    Surface: background(Neutral) {
        tone: |scheme| {
            if !phone(&scheme) {
                0.0
            } else if scheme.is_dark {
                4.0
            } else {
                light_surface(&scheme, 99.0, 97.0, 98.0)
            }
        },
    },

    SurfaceDim: background(Neutral) {
        tone: |scheme| if scheme.is_dark { 4.0 } else { light_surface(&scheme, 90.0, 85.0, 87.0) },
        chroma_multiplier: |scheme| if scheme.is_dark { 1.0 } else { surface_chroma(&scheme, 2.5, 1.7, 2.7, 1.75, 1.36) },
    },

    SurfaceBright: background(Neutral) {
        tone: |scheme| if scheme.is_dark { 18.0 } else { light_surface(&scheme, 99.0, 97.0, 98.0) },
        chroma_multiplier: |scheme| if scheme.is_dark { surface_chroma(&scheme, 2.5, 1.7, 2.7, 1.75, 1.36) } else { 1.0 },
    },

    SurfaceContainerLowest: background(Neutral) { tone: |scheme| if scheme.is_dark { 0.0 } else { 100.0 } },
    SurfaceContainerLow: background(Neutral) {
        tone: |scheme| {
            if !phone(&scheme) {
                15.0
            } else if scheme.is_dark {
                6.0
            } else {
                light_surface(&scheme, 98.0, 95.0, 96.0)
            }
        },
        chroma_multiplier: |scheme| if phone(&scheme) { surface_chroma(&scheme, 1.3, 1.25, 1.3, 1.15, 1.08) } else { 1.0 },
    },

    SurfaceContainer: background(Neutral) {
        tone: |scheme| {
            if !phone(&scheme) {
                20.0
            } else if scheme.is_dark {
                9.0
            } else {
                light_surface(&scheme, 96.0, 92.0, 94.0)
            }
        },
        chroma_multiplier: |scheme| if phone(&scheme) { surface_chroma(&scheme, 1.6, 1.4, 1.6, 1.3, 1.15) } else { 1.0 },
    },

    SurfaceContainerHigh: background(Neutral) {
        tone: |scheme| {
            if !phone(&scheme) {
                25.0
            } else if scheme.is_dark {
                12.0
            } else {
                light_surface(&scheme, 94.0, 90.0, 92.0)
            }
        },
        chroma_multiplier: |scheme| if phone(&scheme) { surface_chroma(&scheme, 1.9, 1.5, 1.95, 1.45, 1.22) } else { 1.0 },
    },

    SurfaceContainerHighest: background(Neutral) {
        tone: |scheme| if scheme.is_dark { 15.0 } else { light_surface(&scheme, 92.0, 88.0, 90.0) },
        chroma_multiplier: |scheme| surface_chroma(&scheme, 2.2, 1.7, 2.3, 1.6, 1.29),
    },

    OnSurface: foreground(Neutral) {
        tone: |scheme| {
            if scheme.variant == Variant::Vibrant {
                scheme.t_max_c_scaled(SchemePalette::Neutral, 0.0, 100.0, 1.1)
            } else {
                surface_background(scheme).map_or(50.0, |background| scheme.tone(background))
            }
        },
        chroma_multiplier: on_neutral_chroma,
        background: surface_background,
        contrast_curve: |scheme| Some(contrast_curve(if scheme.is_dark && phone(&scheme) { 11.0 } else { 9.0 })),
    },

    SurfaceVariant: alias(SurfaceContainerHighest),
    OnSurfaceVariant: foreground(Neutral) {
        chroma_multiplier: on_neutral_chroma,
        background: surface_background,
        contrast_curve: |scheme| {
            Some(contrast_curve(if !phone(&scheme) {
                7.0
            } else if scheme.is_dark {
                6.0
            } else {
                4.5
            }))
        },
    },

    InverseSurface: background(Neutral) { tone: |scheme| if scheme.is_dark { 98.0 } else { 4.0 } },
    InverseOnSurface: foreground(Neutral) {
        background: |_| Some(DynamicColor::Material(Role::InverseSurface)),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    Outline: foreground(Neutral) {
        chroma_multiplier: on_neutral_chroma,
        background: surface_background,
        contrast_curve: |scheme| phone_or(&scheme, 3.0, 4.5),
    },

    OutlineVariant: foreground(Neutral) {
        chroma_multiplier: on_neutral_chroma,
        background: surface_background,
        contrast_curve: |scheme| phone_or(&scheme, 1.5, 3.0),
    },

    SurfaceTint: alias(Primary),

    // Primaries.
    Primary: background(Primary) {
        tone: |scheme| {
            let hue = scheme.primary_palette.hue();

            match scheme.variant {
                Variant::Neutral => {
                    if !phone(&scheme) {
                        90.0
                    } else if scheme.is_dark {
                        80.0
                    } else {
                        40.0
                    }
                }
                Variant::TonalSpot => {
                    if !phone(&scheme) {
                        scheme.t_max_c(SchemePalette::Primary, 0.0, 90.0)
                    } else if scheme.is_dark {
                        80.0
                    } else {
                        scheme.t_max_c(SchemePalette::Primary, 0.0, 100.0)
                    }
                }
                _ if !phone(&scheme) => scheme.t_max_c(SchemePalette::Primary, 0.0, 100.0),
                Variant::Expressive => {
                    let upper = if scheme.is_dark {
                        if Hct::is_cyan(hue) { 88.0 } else { 98.0 }
                    } else if Hct::is_yellow(hue) {
                        25.0
                    } else {
                        98.0
                    };

                    scheme.t_max_c(SchemePalette::Primary, 0.0, upper)
                }
                _ => scheme.t_max_c(SchemePalette::Primary, 0.0, if Hct::is_cyan(hue) { 88.0 } else { 98.0 }),
            }
        },
        background: surface_background,
        contrast_curve: |scheme| phone_or(&scheme, 4.5, 7.0),
        tone_delta_pair: |scheme| phone(&scheme).then_some(farther_pair(Role::PrimaryContainer, Role::Primary, 5.0, TonePolarity::RelativeLighter)),
    },

    PrimaryDim: background(Primary) {
        tone: |scheme| match scheme.variant {
            Variant::Neutral => 85.0,
            Variant::TonalSpot => scheme.t_max_c(SchemePalette::Primary, 0.0, 90.0),
            _ => scheme.t_max_c(SchemePalette::Primary, 0.0, 100.0),
        },
        background: |_| Some(DynamicColor::Material(Role::SurfaceContainerHigh)),
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |_| Some(farther_pair(Role::PrimaryDim, Role::Primary, 5.0, TonePolarity::Darker)),
    },

    OnPrimary: foreground(Primary) {
        background: |scheme| Some(DynamicColor::Material(if phone(&scheme) { Role::Primary } else { Role::PrimaryDim })),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    PrimaryContainer: background(Primary) {
        tone: |scheme| {
            let hue = scheme.primary_palette.hue();

            if !phone(&scheme) {
                return 30.0;
            }

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
                        scheme.t_min_c(SchemePalette::Primary, 35.0, 93.0)
                    } else {
                        scheme.t_max_c(SchemePalette::Primary, 0.0, 90.0)
                    }
                }
                Variant::Expressive => {
                    if scheme.is_dark {
                        scheme.t_min_c(SchemePalette::Primary, 30.0, 93.0)
                    } else {
                        scheme.t_max_c(SchemePalette::Primary, 78.0, if Hct::is_cyan(hue) { 88.0 } else { 90.0 })
                    }
                }
                _ => {
                    if scheme.is_dark {
                        scheme.t_min_c(SchemePalette::Primary, 66.0, 93.0)
                    } else {
                        scheme.t_max_c(SchemePalette::Primary, 66.0, if Hct::is_cyan(hue) { 88.0 } else { 93.0 })
                    }
                }
            }
        },
        background: phone_surface_background,
        tone_delta_pair: |scheme| (!phone(&scheme)).then_some(farther_pair(Role::PrimaryContainer, Role::PrimaryDim, 10.0, TonePolarity::Darker)),
        contrast_curve: container_curve,
    },

    OnPrimaryContainer: foreground(Primary) {
        background: |_| Some(DynamicColor::Material(Role::PrimaryContainer)),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    InversePrimary: foreground(Primary) {
        tone: |scheme| scheme.t_max_c(SchemePalette::Primary, 0.0, 100.0),
        background: |_| Some(DynamicColor::Material(Role::InverseSurface)),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    // Secondaries.
    Secondary: background(Secondary) {
        tone: |scheme| {
            if !phone(&scheme) {
                return if scheme.variant == Variant::Neutral { 90.0 } else { scheme.t_max_c(SchemePalette::Secondary, 0.0, 90.0) };
            }

            match scheme.variant {
                Variant::Neutral => {
                    if scheme.is_dark {
                        scheme.t_min_c(SchemePalette::Secondary, 0.0, 98.0)
                    } else {
                        scheme.t_max_c(SchemePalette::Secondary, 0.0, 100.0)
                    }
                }
                Variant::Vibrant => scheme.t_max_c(SchemePalette::Secondary, 0.0, if scheme.is_dark { 90.0 } else { 98.0 }),
                _ => {
                    if scheme.is_dark {
                        80.0
                    } else {
                        scheme.t_max_c(SchemePalette::Secondary, 0.0, 100.0)
                    }
                }
            }
        },
        background: surface_background,
        contrast_curve: |scheme| phone_or(&scheme, 4.5, 7.0),
        tone_delta_pair: |scheme| {
            phone(&scheme).then_some(farther_pair(Role::SecondaryContainer, Role::Secondary, 5.0, TonePolarity::RelativeLighter))
        },
    },

    SecondaryDim: background(Secondary) {
        tone: |scheme| if scheme.variant == Variant::Neutral { 85.0 } else { scheme.t_max_c(SchemePalette::Secondary, 0.0, 90.0) },
        background: |_| Some(DynamicColor::Material(Role::SurfaceContainerHigh)),
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |_| Some(farther_pair(Role::SecondaryDim, Role::Secondary, 5.0, TonePolarity::Darker)),
    },

    OnSecondary: foreground(Secondary) {
        background: |scheme| Some(DynamicColor::Material(if phone(&scheme) { Role::Secondary } else { Role::SecondaryDim })),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    SecondaryContainer: background(Secondary) {
        tone: |scheme| {
            if !phone(&scheme) {
                return 30.0;
            }

            match scheme.variant {
                Variant::Vibrant => {
                    if scheme.is_dark {
                        scheme.t_min_c(SchemePalette::Secondary, 30.0, 40.0)
                    } else {
                        scheme.t_max_c(SchemePalette::Secondary, 84.0, 90.0)
                    }
                }
                Variant::Expressive => {
                    if scheme.is_dark {
                        15.0
                    } else {
                        scheme.t_max_c(SchemePalette::Secondary, 90.0, 95.0)
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
        },
        background: phone_surface_background,
        tone_delta_pair: |scheme| (!phone(&scheme)).then_some(farther_pair(Role::SecondaryContainer, Role::SecondaryDim, 10.0, TonePolarity::Darker)),
        contrast_curve: container_curve,
    },

    OnSecondaryContainer: foreground(Secondary) {
        background: |_| Some(DynamicColor::Material(Role::SecondaryContainer)),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    // Tertiaries.
    Tertiary: background(Tertiary) {
        tone: |scheme| {
            if !phone(&scheme) {
                return scheme.t_max_c(SchemePalette::Tertiary, 0.0, if scheme.variant == Variant::TonalSpot { 90.0 } else { 100.0 });
            }

            match scheme.variant {
                Variant::Expressive | Variant::Vibrant => {
                    let upper = if Hct::is_cyan(scheme.tertiary_palette.hue()) {
                        88.0
                    } else if scheme.is_dark {
                        98.0
                    } else {
                        100.0
                    };

                    scheme.t_max_c(SchemePalette::Tertiary, 0.0, upper)
                }
                _ => scheme.t_max_c(SchemePalette::Tertiary, 0.0, if scheme.is_dark { 98.0 } else { 100.0 }),
            }
        },
        background: surface_background,
        contrast_curve: |scheme| phone_or(&scheme, 4.5, 7.0),
        tone_delta_pair: |scheme| phone(&scheme).then_some(farther_pair(Role::TertiaryContainer, Role::Tertiary, 5.0, TonePolarity::RelativeLighter)),
    },

    TertiaryDim: background(Tertiary) {
        tone: |scheme| scheme.t_max_c(SchemePalette::Tertiary, 0.0, if scheme.variant == Variant::TonalSpot { 90.0 } else { 100.0 }),
        background: |_| Some(DynamicColor::Material(Role::SurfaceContainerHigh)),
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |_| Some(farther_pair(Role::TertiaryDim, Role::Tertiary, 5.0, TonePolarity::Darker)),
    },

    OnTertiary: foreground(Tertiary) {
        background: |scheme| Some(DynamicColor::Material(if phone(&scheme) { Role::Tertiary } else { Role::TertiaryDim })),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    TertiaryContainer: background(Tertiary) {
        tone: |scheme| {
            if !phone(&scheme) {
                return scheme.t_max_c(SchemePalette::Tertiary, 0.0, if scheme.variant == Variant::TonalSpot { 90.0 } else { 100.0 });
            }

            match scheme.variant {
                Variant::Neutral => scheme.t_max_c(SchemePalette::Tertiary, 0.0, if scheme.is_dark { 93.0 } else { 96.0 }),
                Variant::TonalSpot => scheme.t_max_c(SchemePalette::Tertiary, 0.0, if scheme.is_dark { 93.0 } else { 100.0 }),
                Variant::Expressive => {
                    let upper = if Hct::is_cyan(scheme.tertiary_palette.hue()) {
                        88.0
                    } else if scheme.is_dark {
                        93.0
                    } else {
                        100.0
                    };

                    scheme.t_max_c(SchemePalette::Tertiary, 75.0, upper)
                }
                _ => {
                    if scheme.is_dark {
                        scheme.t_max_c(SchemePalette::Tertiary, 0.0, 93.0)
                    } else {
                        scheme.t_max_c(SchemePalette::Tertiary, 72.0, 100.0)
                    }
                }
            }
        },
        background: phone_surface_background,
        tone_delta_pair: |scheme| (!phone(&scheme)).then_some(farther_pair(Role::TertiaryContainer, Role::TertiaryDim, 10.0, TonePolarity::Darker)),
        contrast_curve: container_curve,
    },

    OnTertiaryContainer: foreground(Tertiary) {
        background: |_| Some(DynamicColor::Material(Role::TertiaryContainer)),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    // Errors.
    Error: background(Error) {
        tone: |scheme| {
            if !phone(&scheme) {
                scheme.t_min_c(SchemePalette::Error, 0.0, 100.0)
            } else if scheme.is_dark {
                scheme.t_min_c(SchemePalette::Error, 0.0, 98.0)
            } else {
                scheme.t_max_c(SchemePalette::Error, 0.0, 100.0)
            }
        },
        background: surface_background,
        contrast_curve: |scheme| phone_or(&scheme, 4.5, 7.0),
        tone_delta_pair: |scheme| phone(&scheme).then_some(farther_pair(Role::ErrorContainer, Role::Error, 5.0, TonePolarity::RelativeLighter)),
    },

    ErrorDim: background(Error) {
        tone: |scheme| scheme.t_min_c(SchemePalette::Error, 0.0, 100.0),
        background: |_| Some(DynamicColor::Material(Role::SurfaceContainerHigh)),
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |_| Some(farther_pair(Role::ErrorDim, Role::Error, 5.0, TonePolarity::Darker)),
    },

    OnError: foreground(Error) {
        background: |scheme| Some(DynamicColor::Material(if phone(&scheme) { Role::Error } else { Role::ErrorDim })),
        contrast_curve: |scheme| phone_or(&scheme, 6.0, 7.0),
    },

    ErrorContainer: background(Error) {
        tone: |scheme| {
            if !phone(&scheme) {
                30.0
            } else if scheme.is_dark {
                scheme.t_min_c(SchemePalette::Error, 30.0, 93.0)
            } else {
                scheme.t_max_c(SchemePalette::Error, 0.0, 90.0)
            }
        },
        background: phone_surface_background,
        tone_delta_pair: |scheme| (!phone(&scheme)).then_some(farther_pair(Role::ErrorContainer, Role::ErrorDim, 10.0, TonePolarity::Darker)),
        contrast_curve: container_curve,
    },

    OnErrorContainer: foreground(Error) {
        background: |_| Some(DynamicColor::Material(Role::ErrorContainer)),
        contrast_curve: |scheme| phone_or(&scheme, 4.5, 7.0),
    },

    // Primary fixed colors.
    PrimaryFixed: background(Primary) {
        tone: |scheme| scheme.in_light_standard(|light_scheme| light_scheme.tone(DynamicColor::Material(Role::PrimaryContainer))),
        background: phone_surface_background,
        contrast_curve: container_curve,
    },

    PrimaryFixedDim: background(Primary) {
        tone: |scheme| scheme.tone(DynamicColor::Material(Role::PrimaryFixed)),
        tone_delta_pair: |_| Some(fixed_dim_pair(Role::PrimaryFixedDim, Role::PrimaryFixed)),
    },

    OnPrimaryFixed: foreground(Primary) {
        background: |_| Some(DynamicColor::Material(Role::PrimaryFixedDim)),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    OnPrimaryFixedVariant: foreground(Primary) {
        background: |_| Some(DynamicColor::Material(Role::PrimaryFixedDim)),
        contrast_curve: |_| Some(contrast_curve(4.5)),
    },

    // Secondary fixed colors.
    SecondaryFixed: background(Secondary) {
        tone: |scheme| scheme.in_light_standard(|light_scheme| light_scheme.tone(DynamicColor::Material(Role::SecondaryContainer))),
        background: phone_surface_background,
        contrast_curve: container_curve,
    },

    SecondaryFixedDim: background(Secondary) {
        tone: |scheme| scheme.tone(DynamicColor::Material(Role::SecondaryFixed)),
        tone_delta_pair: |_| Some(fixed_dim_pair(Role::SecondaryFixedDim, Role::SecondaryFixed)),
    },

    OnSecondaryFixed: foreground(Secondary) {
        background: |_| Some(DynamicColor::Material(Role::SecondaryFixedDim)),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    OnSecondaryFixedVariant: foreground(Secondary) {
        background: |_| Some(DynamicColor::Material(Role::SecondaryFixedDim)),
        contrast_curve: |_| Some(contrast_curve(4.5)),
    },

    // Tertiary fixed colors.
    TertiaryFixed: background(Tertiary) {
        tone: |scheme| scheme.in_light_standard(|light_scheme| light_scheme.tone(DynamicColor::Material(Role::TertiaryContainer))),
        background: phone_surface_background,
        contrast_curve: container_curve,
    },

    TertiaryFixedDim: background(Tertiary) {
        tone: |scheme| scheme.tone(DynamicColor::Material(Role::TertiaryFixed)),
        tone_delta_pair: |_| Some(fixed_dim_pair(Role::TertiaryFixedDim, Role::TertiaryFixed)),
    },

    OnTertiaryFixed: foreground(Tertiary) {
        background: |_| Some(DynamicColor::Material(Role::TertiaryFixedDim)),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    OnTertiaryFixedVariant: foreground(Tertiary) {
        background: |_| Some(DynamicColor::Material(Role::TertiaryFixedDim)),
        contrast_curve: |_| Some(contrast_curve(4.5)),
    },
};

#[inline]
fn avoid_awkward_background(color: &DynamicColor<'_>, is_background: bool, tone: f64) -> f64 {
    if !is_background || color.is_fixed_dim() {
        tone
    } else if tone >= 57.0 {
        coerce_in(tone, 65.0, 100.0)
    } else {
        coerce_in(tone, 0.0, 49.0)
    }
}

pub(super) fn hct(context: Context<'_>, color: DynamicColor<'_>, tone: f64) -> Hct {
    let view = color.view(context.spec_version);
    let palette = view.palette(context);
    let chroma_multiplier = view.chroma_multiplier(context).unwrap_or(1.0);

    if chroma_multiplier == 1.0 {
        return palette.get_hct(tone);
    }

    let chroma = palette.chroma() * chroma_multiplier;

    if tone == 99.0 && Hct::is_yellow(palette.hue()) {
        return TonalPalette::from_hue_and_chroma(palette.hue(), chroma).get_hct(tone);
    }

    Hct::from(palette.hue(), chroma, tone)
}

pub(super) fn tone(context: Context<'_>, color: DynamicColor<'_>) -> f64 {
    let contrast_level = context.contrast_level;
    let view = color.view(context.spec_version);

    // Case 0: tone delta pair.
    if let Some(pair) = view.tone_delta_pair(context) {
        let absolute_delta = match pair.polarity {
            TonePolarity::Darker => -pair.delta,
            TonePolarity::RelativeLighter if context.is_dark => -pair.delta,
            TonePolarity::RelativeDarker if !context.is_dark => -pair.delta,
            _ => pair.delta,
        };

        let am_role_a = color.same_as(&pair.role_a);
        let (self_role, reference_role) = if am_role_a { (pair.role_a, pair.role_b) } else { (pair.role_b, pair.role_a) };
        let reference_tone = context.tone(reference_role);
        let relative_delta = if am_role_a { absolute_delta } else { -absolute_delta };
        let target = reference_tone + relative_delta;
        let mut self_tone = match pair.constraint {
            // The unadjusted self tone is irrelevant for EXACT; skip computing it.
            DeltaConstraint::Exact => coerce_in(target, 0.0, 100.0),
            DeltaConstraint::Nearer => {
                let self_tone = context.raw_tone(self_role);
                let (low, high) = if relative_delta > 0.0 {
                    (reference_tone, target)
                } else {
                    (target, reference_tone)
                };

                coerce_in(coerce_in(self_tone, low, high), 0.0, 100.0)
            }
            DeltaConstraint::Farther => {
                let self_tone = context.raw_tone(self_role);

                if relative_delta > 0.0 {
                    coerce_in(self_tone, target, 100.0)
                } else {
                    coerce_in(self_tone, 0.0, target)
                }
            }
        };

        if let Some(curve) = view.contrast_curve(context)
            && let Some(background) = view.background(context)
        {
            let bg_tone = context.tone(background);
            let self_contrast = curve.get(contrast_level);

            if !(ratio_of_tones(bg_tone, self_tone) >= self_contrast && contrast_level >= 0.0) {
                self_tone = DynamicColor::foreground_tone(bg_tone, self_contrast);
            }
        }

        return avoid_awkward_background(&color, view.is_background(), self_tone);
    }

    // Case 1: No tone delta pair; just solve for itself.
    let answer = view.raw_tone(context);
    let Some(curve) = view.contrast_curve(context) else {
        return answer;
    };

    let Some(background) = view.background(context) else {
        return answer; // No adjustment for colors with no background.
    };

    let bg_tone = context.tone(background);
    let desired_ratio = curve.get(contrast_level);

    // Recalculate the tone from desired contrast ratio if the current ratio is
    // not enough or desired contrast level is decreasing (<0).
    let answer = if ratio_of_tones(bg_tone, answer) >= desired_ratio && contrast_level >= 0.0 {
        answer
    } else {
        DynamicColor::foreground_tone(bg_tone, desired_ratio)
    };

    let answer = avoid_awkward_background(&color, view.is_background(), answer);

    // Case 2: Adjust for dual backgrounds.
    view.second_background(context)
        .map_or(answer, |second| dual_background_tone(answer, bg_tone, context.tone(second), desired_ratio))
}

pub(super) fn primary_palette(variant: Variant, source: Hct, is_dark: bool, platform: Platform) -> Option<TonalPalette> {
    let hue = source.get_hue();
    let phone = platform == Platform::Phone;

    Some(match variant {
        Variant::Neutral => TonalPalette::from_hue_and_chroma(hue, match (phone, Hct::is_blue(hue)) {
            (true, false) => 8.0,
            (false, true) => 16.0,
            _ => 12.0,
        }),
        Variant::TonalSpot => TonalPalette::from_hue_and_chroma(hue, if phone && is_dark { 26.0 } else { 32.0 }),
        Variant::Expressive => TonalPalette::from_hue_and_chroma(
            hue,
            if !phone {
                40.0
            } else if is_dark {
                36.0
            } else {
                48.0
            },
        ),
        Variant::Vibrant => TonalPalette::from_hue_and_chroma(hue, if phone { 74.0 } else { 56.0 }),
        _ => return color_spec_2021::primary_palette(variant, source),
    })
}

pub(super) fn secondary_palette(variant: Variant, source: Hct, is_dark: bool, platform: Platform) -> Option<TonalPalette> {
    let hue = source.get_hue();
    let phone = platform == Platform::Phone;

    Some(match variant {
        Variant::Neutral => TonalPalette::from_hue_and_chroma(hue, match (phone, Hct::is_blue(hue)) {
            (true, false) => 4.0,
            (false, true) => 10.0,
            _ => 6.0,
        }),
        Variant::TonalSpot => TonalPalette::from_hue_and_chroma(hue, 16.0),
        Variant::Expressive => TonalPalette::from_hue_and_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0], &[
                -160.0, 155.0, -100.0, 96.0, -96.0, -156.0, -165.0, -160.0,
            ]),
            if phone && is_dark { 16.0 } else { 24.0 },
        ),
        // Same rotation table as the vibrant neutral hue.
        Variant::Vibrant => TonalPalette::from_hue_and_chroma(vibrant_neutral_hue(hue), if phone { 56.0 } else { 36.0 }),
        _ => return color_spec_2021::secondary_palette(variant, source),
    })
}

pub(super) fn tertiary_palette(variant: Variant, source: Hct, platform: Platform) -> Option<TonalPalette> {
    let hue = source.get_hue();
    let phone = platform == Platform::Phone;

    Some(match variant {
        Variant::Neutral => TonalPalette::from_hue_and_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 38.0, 105.0, 161.0, 204.0, 278.0, 333.0, 360.0], &[
                -32.0, 26.0, 10.0, -39.0, 24.0, -15.0, -32.0,
            ]),
            if phone { 20.0 } else { 36.0 },
        ),
        Variant::TonalSpot => TonalPalette::from_hue_and_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 20.0, 71.0, 161.0, 333.0, 360.0], &[-40.0, 48.0, -32.0, 40.0, -32.0]),
            if phone { 28.0 } else { 32.0 },
        ),
        Variant::Expressive => TonalPalette::from_hue_and_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0], &[
                -165.0, 160.0, -105.0, 101.0, -101.0, -160.0, -170.0, -165.0,
            ]),
            48.0,
        ),
        Variant::Vibrant => TonalPalette::from_hue_and_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 38.0, 71.0, 105.0, 140.0, 161.0, 253.0, 333.0, 360.0], &[
                -72.0, 35.0, 24.0, -24.0, 62.0, 50.0, 62.0, -72.0,
            ]),
            56.0,
        ),
        _ => return color_spec_2021::tertiary_palette(variant, source),
    })
}

fn expressive_neutral_hue(hue: f64) -> f64 {
    DynamicScheme::get_rotated_hue(hue, &[0.0, 71.0, 124.0, 253.0, 278.0, 300.0, 360.0], &[10.0, 0.0, 10.0, 0.0, 10.0, 0.0])
}

fn expressive_neutral_chroma(neutral_hue: f64, is_dark: bool, platform: Platform) -> f64 {
    if platform != Platform::Phone {
        12.0
    } else if !is_dark {
        18.0
    } else if Hct::is_yellow(neutral_hue) {
        6.0
    } else {
        14.0
    }
}

fn vibrant_neutral_hue(hue: f64) -> f64 {
    DynamicScheme::get_rotated_hue(hue, &[0.0, 38.0, 105.0, 140.0, 333.0, 360.0], &[-14.0, 10.0, -14.0, 10.0, -14.0])
}

fn vibrant_neutral_chroma(neutral_hue: f64, platform: Platform) -> f64 {
    if platform == Platform::Phone || Hct::is_blue(neutral_hue) {
        28.0
    } else {
        20.0
    }
}

pub(super) fn neutral_palette(variant: Variant, source: Hct, is_dark: bool, platform: Platform) -> Option<TonalPalette> {
    let hue = source.get_hue();
    let phone = platform == Platform::Phone;

    Some(match variant {
        Variant::Neutral => TonalPalette::from_hue_and_chroma(hue, if phone { 1.4 } else { 6.0 }),
        Variant::TonalSpot => TonalPalette::from_hue_and_chroma(hue, if phone { 5.0 } else { 10.0 }),
        Variant::Expressive => {
            let neutral_hue = expressive_neutral_hue(hue);

            TonalPalette::from_hue_and_chroma(neutral_hue, expressive_neutral_chroma(neutral_hue, is_dark, platform))
        }
        Variant::Vibrant => {
            let neutral_hue = vibrant_neutral_hue(hue);

            TonalPalette::from_hue_and_chroma(neutral_hue, vibrant_neutral_chroma(neutral_hue, platform))
        }
        _ => return color_spec_2021::neutral_palette(variant, source),
    })
}

pub(super) fn neutral_variant_palette(variant: Variant, source: Hct, is_dark: bool, platform: Platform) -> Option<TonalPalette> {
    let hue = source.get_hue();
    let phone = platform == Platform::Phone;

    Some(match variant {
        Variant::Neutral => TonalPalette::from_hue_and_chroma(hue, (if phone { 1.4 } else { 6.0 }) * 2.2),
        Variant::TonalSpot => TonalPalette::from_hue_and_chroma(hue, (if phone { 5.0 } else { 10.0 }) * 1.7),
        Variant::Expressive => {
            let neutral_hue = expressive_neutral_hue(hue);

            TonalPalette::from_hue_and_chroma(
                neutral_hue,
                expressive_neutral_chroma(neutral_hue, is_dark, platform) * if (105.0..125.0).contains(&neutral_hue) { 1.6 } else { 2.3 },
            )
        }
        Variant::Vibrant => {
            let neutral_hue = vibrant_neutral_hue(hue);

            TonalPalette::from_hue_and_chroma(neutral_hue, vibrant_neutral_chroma(neutral_hue, platform) * 1.29)
        }
        _ => return color_spec_2021::neutral_variant_palette(variant, source),
    })
}

pub(super) fn error_palette(variant: Variant, source: Hct, platform: Platform) -> Option<TonalPalette> {
    // Absolute hue from a lookup (piecewise), not a rotation.
    let error_hue = DynamicScheme::get_piecewise_value(source.get_hue(), &[0.0, 3.0, 13.0, 23.0, 33.0, 43.0, 153.0, 273.0, 360.0], &[
        12.0, 22.0, 32.0, 12.0, 22.0, 32.0, 22.0, 12.0,
    ]);

    let phone = platform == Platform::Phone;

    Some(match variant {
        Variant::Neutral => TonalPalette::from_hue_and_chroma(error_hue, if phone { 50.0 } else { 40.0 }),
        Variant::TonalSpot => TonalPalette::from_hue_and_chroma(error_hue, if phone { 60.0 } else { 48.0 }),
        Variant::Expressive => TonalPalette::from_hue_and_chroma(error_hue, if phone { 64.0 } else { 48.0 }),
        Variant::Vibrant => TonalPalette::from_hue_and_chroma(error_hue, if phone { 80.0 } else { 60.0 }),
        _ => color_spec_2021::error_palette(),
    })
}

impl ColorSpec for ColorSpec2025 {
    fn spec_version(&self) -> SpecVersion {
        SpecVersion::Spec2025
    }

    fn declared(&self, role: Role) -> Option<Entry> {
        DECLARED[role]
    }

    fn get_hct(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> Hct {
        let cache = ToneCache::new();
        let context = Context::new(scheme, &cache);

        hct(context, color, tone(context, color))
    }

    fn get_tone(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> f64 {
        tone(Context::new(scheme, &ToneCache::new()), color)
    }

    fn get_primary_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        primary_palette(variant, source, is_dark, platform)
    }

    fn get_secondary_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        secondary_palette(variant, source, is_dark, platform)
    }

    fn get_tertiary_palette(&self, variant: Variant, source: Hct, _: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        tertiary_palette(variant, source, platform)
    }

    fn get_neutral_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        neutral_palette(variant, source, is_dark, platform)
    }

    fn get_neutral_variant_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        neutral_variant_palette(variant, source, is_dark, platform)
    }

    fn get_error_palette(&self, variant: Variant, source: Hct, _: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        error_palette(variant, source, platform)
    }
}
