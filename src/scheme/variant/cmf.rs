use crate::{
    dynamic_color::{DynamicScheme, Platform, SpecVersion, Variant, dynamic_scheme::DEFAULT_PLATFORM},
    hct::Hct,
    palette::{Palette, TonalPalette},
};

/// A Dynamic Color theme with 2 source colors.
///
/// Only defined for spec 2026, so there is no spec parameter.
pub struct SchemeCmf {
    pub scheme: DynamicScheme,
}

impl SchemeCmf {
    pub const VARIANT: Variant = Variant::Cmf;

    /// One source color, phone.
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self::with_sources(source_color_hct, None, is_dark, contrast_level, DEFAULT_PLATFORM)
    }

    /// One or two source colors.
    pub fn with_sources(
        source_color_hct: Hct,
        secondary_source_color_hct: Option<Hct>,
        is_dark: bool,
        contrast_level: Option<f64>,
        platform: Platform,
    ) -> Self {
        let palette = |palette: Palette| Self::palette_with_secondary(&source_color_hct, secondary_source_color_hct.as_ref(), &palette);

        Self {
            scheme: DynamicScheme {
                source_color_hct,
                secondary_source_color_hct,
                variant: Self::VARIANT,
                is_dark,
                contrast_level: contrast_level.unwrap_or(0.0),
                platform,
                spec_version: SpecVersion::Spec2026,
                primary_palette: palette(Palette::Primary),
                secondary_palette: palette(Palette::Secondary),
                tertiary_palette: palette(Palette::Tertiary),
                neutral_palette: palette(Palette::Neutral),
                neutral_variant_palette: palette(Palette::NeutralVariant),
                error_palette: palette(Palette::Error),
            },
        }
    }

    /// One palette for a single source color (used by
    /// `TonalPalette::by_variant`).
    pub fn palette(source_color_hct: &Hct, palette: &Palette) -> TonalPalette {
        Self::palette_with_secondary(source_color_hct, None, palette)
    }

    pub fn palette_with_secondary(source_color_hct: &Hct, secondary_source_color_hct: Option<&Hct>, palette: &Palette) -> TonalPalette {
        let (hue, chroma) = (source_color_hct.get_hue(), source_color_hct.get_chroma());

        match palette {
            Palette::Primary => TonalPalette::from_hue_and_chroma(hue, chroma),
            Palette::Secondary => TonalPalette::from_hue_and_chroma(hue, chroma * 0.5),
            Palette::Tertiary => tertiary_palette(source_color_hct, secondary_source_color_hct),
            Palette::Neutral | Palette::NeutralVariant => TonalPalette::from_hue_and_chroma(hue, chroma * 0.2),
            Palette::Error => {
                let tertiary_hue = tertiary_palette(source_color_hct, secondary_source_color_hct).hue();

                TonalPalette::from_hue_and_chroma(error_hue(hue, tertiary_hue), chroma.max(50.0))
            }
        }
    }
}

/// The second source's palette if it differs from the first (`Hct` equality
/// compares RGB values), else a softer copy of the first.
fn tertiary_palette(source: &Hct, secondary: Option<&Hct>) -> TonalPalette {
    let secondary = secondary.unwrap_or(source);

    if source == secondary {
        TonalPalette::from_hue_and_chroma(source.get_hue(), source.get_chroma() * 0.75)
    } else {
        TonalPalette::from_hue_and_chroma(secondary.get_hue(), secondary.get_chroma())
    }
}

/// Error hue chosen to stand apart from the primary and tertiary hues.
fn error_hue(primary_hue: f64, tertiary_hue: f64) -> f64 {
    let t = tertiary_hue;

    if primary_hue <= 8.0 {
        if t <= 24.0 {
            28.0
        } else if t <= 32.0 {
            16.0
        } else {
            20.0
        }
    } else if primary_hue <= 16.0 {
        if t <= 24.0 {
            32.0
        } else if t <= 32.0 {
            20.0
        } else {
            24.0
        }
    } else if primary_hue <= 20.0 {
        if t <= 28.0 {
            32.0
        } else if t <= 32.0 {
            24.0
        } else {
            28.0
        }
    } else if primary_hue <= 28.0 {
        if t <= 24.0 { 32.0 } else { 16.0 }
    } else if primary_hue <= 32.0 {
        if t <= 20.0 {
            24.0
        } else if t <= 28.0 {
            16.0
        } else {
            20.0
        }
    } else if primary_hue <= 40.0 {
        if t > 20.0 && t <= 28.0 { 16.0 } else { 24.0 }
    } else if primary_hue <= 152.0 {
        if t > 24.0 && t <= 36.0 { 20.0 } else { 32.0 }
    } else if primary_hue <= 272.0 {
        if t > 20.0 && t <= 28.0 { 16.0 } else { 24.0 }
    } else if t > 12.0 && t <= 28.0 {
        32.0
    } else {
        16.0
    }
}
