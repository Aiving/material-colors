use crate::{
    dynamic_color::{DynamicScheme, Variant},
    hct::Hct,
    palette::{Palette, TonalPalette},
    utils::math::sanitize_degrees_double,
};

/// A Dynamic Color theme that is intentionally detached from the input color.
pub struct SchemeExpressive {
    pub scheme: DynamicScheme,
}

impl SchemeExpressive {
    /// Hues used at breakpoints such that designers can specify a hue rotation
    /// that occurs at a given break point.
    pub const HUES: [f64; 9] = [0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0];

    /// Hue rotations of the Secondary [`TonalPalette`], corresponding to the
    /// breakpoints in [hues].
    pub const SECONDARY_ROTATIONS: [f64; 9] =
        [45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0];

    /// Hue rotations of the Tertiary [`TonalPalette`], corresponding to the
    /// breakpoints in [hues].
    pub const TERTIARY_ROTATIONS: [f64; 9] =
        [120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0];

    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                None,
                Variant::Expressive,
                is_dark,
                contrast_level,
                Self::palette(&source_color_hct, &Palette::Primary),
                Self::palette(&source_color_hct, &Palette::Secondary),
                Self::palette(&source_color_hct, &Palette::Tertiary),
                Self::palette(&source_color_hct, &Palette::Neutral),
                Self::palette(&source_color_hct, &Palette::NeutralVariant),
                None,
            ),
        }
    }

    pub fn palette(source_color_hct: &Hct, variant: &Palette) -> TonalPalette {
        match variant {
            Palette::Primary => TonalPalette::of(
                sanitize_degrees_double(source_color_hct.get_hue() + 240.0),
                40.0,
            ),
            Palette::Secondary => TonalPalette::of(
                DynamicScheme::get_rotated_hue(
                    source_color_hct.get_hue(),
                    &Self::HUES,
                    &Self::SECONDARY_ROTATIONS,
                ),
                24.0,
            ),
            Palette::Tertiary => TonalPalette::of(
                DynamicScheme::get_rotated_hue(
                    source_color_hct.get_hue(),
                    &Self::HUES,
                    &Self::TERTIARY_ROTATIONS,
                ),
                32.0,
            ),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral => TonalPalette::of(source_color_hct.get_hue() + 15.0, 8.0),
            Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue() + 15.0, 12.0),
        }
    }
}
