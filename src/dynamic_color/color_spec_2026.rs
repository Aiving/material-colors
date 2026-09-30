//! `ColorSpec` implementation for the 2026 spec.
//!
//! This spec only declares color definitions; roles it doesn't declare, the
//! tone/HCT algorithm and the scheme palettes all come from 2025.
//!
//! Only `Variant::Cmf` keeps spec 2026 (`DynamicScheme::fallback_spec_version`
//! downgrades every other variant), so the `else 0.0` branches below are dead
//! in practice. They are kept for fidelity.

use super::{
    Context, ContrastCurve, DeltaConstraint, DynamicColor, DynamicScheme, Platform, Role, SchemePalette, SpecVersion, ToneDeltaPair, TonePolarity, Variant,
    color_spec::{ColorSpec, Entry, SpecTable, coerce_in, contrast_curve, highest_surface_fn, spec_table},
    color_spec_2025,
};
use crate::{hct::Hct, palette::TonalPalette};

#[derive(Debug, Clone, Copy, Default)]
pub struct ColorSpec2026;

#[inline]
const fn cmf(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Cmf)
}

#[inline]
const fn cmf_tone(scheme: &DynamicScheme, dark: f64, light: f64) -> f64 {
    if !cmf(scheme) {
        0.0
    } else if scheme.is_dark {
        dark
    } else {
        light
    }
}

#[inline]
const fn cmf_chroma(scheme: &DynamicScheme, multiplier: f64) -> f64 {
    if cmf(scheme) { multiplier } else { 0.0 }
}

const fn container_curve(context: Context<'_>) -> Option<ContrastCurve> {
    if context.scheme().contrast_level > 0.0 {
        Some(contrast_curve(1.5))
    } else {
        None
    }
}

const fn container_pair(scheme: &DynamicScheme, container: Role, role: Role) -> Option<ToneDeltaPair<'static>> {
    if matches!(scheme.platform, Platform::Phone) {
        Some(
            ToneDeltaPair::new(
                DynamicColor::Material(container),
                DynamicColor::Material(role),
                5.0,
                TonePolarity::RelativeLighter,
            )
            .with_constraint(DeltaConstraint::Farther),
        )
    } else {
        None
    }
}

const fn fixed_dim_pair(dim: Role, fixed: Role) -> ToneDeltaPair<'static> {
    ToneDeltaPair::new(DynamicColor::Material(dim), DynamicColor::Material(fixed), 5.0, TonePolarity::Darker)
}

#[inline]
fn fixed_background(scheme: Context<'_>, fixed: Role, fixed_dim: Role) -> Option<DynamicColor<'static>> {
    Some(DynamicColor::Material(if scheme.tone(DynamicColor::Material(fixed)) > 57.0 {
        fixed_dim
    } else {
        fixed
    }))
}

#[inline]
const fn source_container_tone(tone: f64, dark_min: f64) -> f64 {
    if tone > 55.0 {
        coerce_in(tone, 61.0, 90.0)
    } else {
        coerce_in(tone, dark_min, 49.0)
    }
}

static DECLARED: SpecTable = TABLE;

pub const TABLE: SpecTable = spec_table! {
    // Surfaces.
    Surface: background(Neutral) { tone: |scheme| cmf_tone(&scheme, 4.0, 98.0) },
    SurfaceDim: background(Neutral) {
        tone: |scheme| cmf_tone(&scheme, 4.0, 87.0),
        chroma_multiplier: |scheme| cmf_chroma(&scheme, if scheme.is_dark { 1.0 } else { 1.7 }),
    },

    SurfaceBright: background(Neutral) {
        tone: |scheme| cmf_tone(&scheme, 18.0, 98.0),
        chroma_multiplier: |scheme| cmf_chroma(&scheme, if scheme.is_dark { 1.7 } else { 1.0 }),
    },

    SurfaceContainerLowest: background(Neutral) { tone: |scheme| cmf_tone(&scheme, 0.0, 100.0) },
    SurfaceContainerLow: background(Neutral) {
        tone: |scheme| cmf_tone(&scheme, 6.0, 96.0),
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.25),
    },

    SurfaceContainer: background(Neutral) {
        tone: |scheme| cmf_tone(&scheme, 9.0, 94.0),
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.4),
    },

    SurfaceContainerHigh: background(Neutral) {
        tone: |scheme| cmf_tone(&scheme, 12.0, 92.0),
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.5),
    },

    SurfaceContainerHighest: background(Neutral) {
        tone: |scheme| cmf_tone(&scheme, 15.0, 90.0),
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.7),
    },

    OnSurface: foreground(Neutral) {
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.7),
        background: highest_surface_fn,
        contrast_curve: |scheme| Some(contrast_curve(if scheme.is_dark { 11.0 } else { 9.0 })),
    },

    OnSurfaceVariant: foreground(Neutral) {
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.7),
        background: highest_surface_fn,
        contrast_curve: |scheme| Some(contrast_curve(if scheme.is_dark { 6.0 } else { 4.5 })),
    },

    Outline: foreground(Neutral) {
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.7),
        background: highest_surface_fn,
        contrast_curve: |_| Some(contrast_curve(3.0)),
    },

    OutlineVariant: foreground(Neutral) {
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.7),
        background: highest_surface_fn,
        contrast_curve: |_| Some(contrast_curve(1.5)),
    },

    InverseSurface: background(Neutral) {
        tone: |scheme| if scheme.is_dark { 98.0 } else { 4.0 },
        chroma_multiplier: |scheme| cmf_chroma(&scheme, 1.7),
    },

    InverseOnSurface: foreground(Neutral) {
        background: |_| Some(DynamicColor::Material(Role::InverseSurface)),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    // Primaries.
    Primary: background(Primary) {
        tone: |scheme| {
            if scheme.source_color_hct.get_chroma() <= 12.0 {
                if scheme.is_dark { 80.0 } else { 40.0 }
            } else {
                scheme.source_color_hct.get_tone()
            }
        },
        background: highest_surface_fn,
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |scheme| container_pair(&scheme, Role::PrimaryContainer, Role::Primary),
    },

    PrimaryDim: alias(Primary),
    OnPrimary: foreground(Primary) {
        background: |_| Some(DynamicColor::Material(Role::Primary)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },

    PrimaryContainer: background(Primary) {
        tone: |scheme| {
            let source = scheme.source_color_hct;

            if !scheme.is_dark && source.get_chroma() <= 12.0 { 90.0 } else { source_container_tone(source.get_tone(), 30.0) }
        },
        background: highest_surface_fn,
        contrast_curve: container_curve,
    },

    OnPrimaryContainer: foreground(Primary) {
        background: |_| Some(DynamicColor::Material(Role::PrimaryContainer)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },

    PrimaryFixed: background(Primary) {
        tone: |scheme| scheme.in_light_standard(|light_scheme| light_scheme.tone(DynamicColor::Material(Role::PrimaryContainer))),
        background: highest_surface_fn,
        contrast_curve: container_curve,
    },

    PrimaryFixedDim: background(Primary) {
        tone: |scheme| scheme.tone(DynamicColor::Material(Role::PrimaryFixed)),
        background: highest_surface_fn,
        tone_delta_pair: |_| Some(fixed_dim_pair(Role::PrimaryFixedDim, Role::PrimaryFixed)),
        contrast_curve: container_curve,
    },

    OnPrimaryFixed: foreground(Primary) {
        background: |scheme| fixed_background(scheme, Role::PrimaryFixed, Role::PrimaryFixedDim),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    OnPrimaryFixedVariant: foreground(Primary) {
        background: |scheme| fixed_background(scheme, Role::PrimaryFixed, Role::PrimaryFixedDim),
        contrast_curve: |_| Some(contrast_curve(4.5)),
    },

    // Secondaries.
    Secondary: background(Secondary) {
        tone: |scheme| {
            if scheme.is_dark {
                scheme.t_min_c(SchemePalette::Secondary, 0.0, 100.0)
            } else {
                scheme.t_max_c(SchemePalette::Secondary, 0.0, 100.0)
            }
        },
        background: highest_surface_fn,
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |scheme| container_pair(&scheme, Role::SecondaryContainer, Role::Secondary),
    },

    SecondaryDim: alias(Secondary),
    OnSecondary: foreground(Secondary) {
        background: |_| Some(DynamicColor::Material(Role::Secondary)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },

    SecondaryContainer: background(Secondary) {
        tone: |scheme| {
            if scheme.is_dark {
                scheme.t_min_c(SchemePalette::Secondary, 20.0, 49.0)
            } else {
                scheme.t_max_c(SchemePalette::Secondary, 61.0, 90.0)
            }
        },
        background: highest_surface_fn,
        contrast_curve: container_curve,
    },

    OnSecondaryContainer: foreground(Secondary) {
        background: |_| Some(DynamicColor::Material(Role::SecondaryContainer)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },

    SecondaryFixed: background(Secondary) {
        tone: |scheme| scheme.in_light_standard(|light_scheme| light_scheme.tone(DynamicColor::Material(Role::SecondaryContainer))),
        background: highest_surface_fn,
        contrast_curve: container_curve,
    },

    SecondaryFixedDim: background(Secondary) {
        tone: |scheme| scheme.tone(DynamicColor::Material(Role::SecondaryFixed)),
        background: highest_surface_fn,
        tone_delta_pair: |_| Some(fixed_dim_pair(Role::SecondaryFixedDim, Role::SecondaryFixed)),
        contrast_curve: container_curve,
    },

    OnSecondaryFixed: foreground(Secondary) {
        background: |scheme| fixed_background(scheme, Role::SecondaryFixed, Role::SecondaryFixedDim),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    OnSecondaryFixedVariant: foreground(Secondary) {
        background: |scheme| fixed_background(scheme, Role::SecondaryFixed, Role::SecondaryFixedDim),
        contrast_curve: |_| Some(contrast_curve(4.5)),
    },

    // Tertiaries.
    Tertiary: background(Tertiary) {
        tone: |scheme| scheme.secondary_source_or_primary().get_tone(),
        background: highest_surface_fn,
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |scheme| container_pair(&scheme, Role::TertiaryContainer, Role::Tertiary),
    },

    TertiaryDim: alias(Tertiary),
    OnTertiary: foreground(Tertiary) {
        background: |_| Some(DynamicColor::Material(Role::Tertiary)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },

    TertiaryContainer: background(Tertiary) {
        tone: |scheme| source_container_tone(scheme.secondary_source_or_primary().get_tone(), 20.0),
        background: highest_surface_fn,
        contrast_curve: container_curve,
    },

    OnTertiaryContainer: foreground(Tertiary) {
        background: |_| Some(DynamicColor::Material(Role::TertiaryContainer)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },

    TertiaryFixed: background(Tertiary) {
        tone: |scheme| scheme.in_light_standard(|light_scheme| light_scheme.tone(DynamicColor::Material(Role::TertiaryContainer))),
        background: highest_surface_fn,
        contrast_curve: container_curve,
    },

    TertiaryFixedDim: background(Tertiary) {
        tone: |scheme| scheme.tone(DynamicColor::Material(Role::TertiaryFixed)),
        background: highest_surface_fn,
        tone_delta_pair: |_| Some(fixed_dim_pair(Role::TertiaryFixedDim, Role::TertiaryFixed)),
        contrast_curve: container_curve,
    },

    OnTertiaryFixed: foreground(Tertiary) {
        background: |scheme| fixed_background(scheme, Role::TertiaryFixed, Role::TertiaryFixedDim),
        contrast_curve: |_| Some(contrast_curve(7.0)),
    },

    OnTertiaryFixedVariant: foreground(Tertiary) {
        background: |scheme| fixed_background(scheme, Role::TertiaryFixed, Role::TertiaryFixedDim),
        contrast_curve: |_| Some(contrast_curve(4.5)),
    },

    // Errors.
    Error: background(Error) {
        tone: |scheme| scheme.t_max_c(SchemePalette::Error, 0.0, 100.0),
        background: highest_surface_fn,
        contrast_curve: |_| Some(contrast_curve(4.5)),
        tone_delta_pair: |scheme| container_pair(&scheme, Role::ErrorContainer, Role::Error),
    },

    ErrorDim: alias(Error),
    OnError: foreground(Error) {
        background: |_| Some(DynamicColor::Material(Role::Error)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },

    ErrorContainer: background(Error) {
        tone: |scheme| {
            if scheme.is_dark {
                scheme.t_min_c(SchemePalette::Error, 0.0, 100.0)
            } else {
                scheme.t_max_c(SchemePalette::Error, 0.0, 100.0)
            }
        },
        background: highest_surface_fn,
        contrast_curve: container_curve,
    },

    OnErrorContainer: foreground(Error) {
        background: |_| Some(DynamicColor::Material(Role::ErrorContainer)),
        contrast_curve: |_| Some(contrast_curve(6.0)),
    },
};

impl ColorSpec for ColorSpec2026 {
    fn spec_version(&self) -> SpecVersion {
        SpecVersion::Spec2026
    }

    fn declared(&self, role: Role) -> Option<Entry> {
        DECLARED[role]
    }

    fn get_hct(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> Hct {
        color_spec_2025::ColorSpec2025.get_hct(scheme, color)
    }

    fn get_tone(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> f64 {
        color_spec_2025::ColorSpec2025.get_tone(scheme, color)
    }

    fn get_primary_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        color_spec_2025::primary_palette(variant, source, is_dark, platform)
    }

    fn get_secondary_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        color_spec_2025::secondary_palette(variant, source, is_dark, platform)
    }

    fn get_tertiary_palette(&self, variant: Variant, source: Hct, _: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        color_spec_2025::tertiary_palette(variant, source, platform)
    }

    fn get_neutral_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        color_spec_2025::neutral_palette(variant, source, is_dark, platform)
    }

    fn get_neutral_variant_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        color_spec_2025::neutral_variant_palette(variant, source, is_dark, platform)
    }

    fn get_error_palette(&self, variant: Variant, source: Hct, _: bool, platform: Platform, _: f64) -> Option<TonalPalette> {
        color_spec_2025::error_palette(variant, source, platform)
    }
}
