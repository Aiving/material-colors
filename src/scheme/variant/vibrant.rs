use crate::{
    dynamic_color::{DynamicScheme, Variant},
    Hct, TonalPalette,
};

/// A Dynamic Color theme that is intentionally detached from the input color.
pub struct SchemeVibrant {
    pub scheme: DynamicScheme,
}

impl SchemeVibrant {
    /// Hues used at breakpoints such that designers can specify a hue rotation
    /// that occurs at a given break point.
    pub const HUES: [f64; 9] = [0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0];

    /// Hue rotations of the Secondary [TonalPalette], corresponding to the
    /// breakpoints in [hues].
    pub const SECONDARY_ROTATIONS: [f64; 9] =
        [18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0];

    /// Hue rotations of the Tertiary [TonalPalette], corresponding to the
    /// breakpoints in [hues].
    pub const TERTIARY_ROTATIONS: [f64; 9] = [35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0];

    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                None,
                Variant::Vibrant,
                is_dark,
                contrast_level,
                TonalPalette::of(source_color_hct.get_hue(), 200.0),
                TonalPalette::of(
                    DynamicScheme::get_rotated_hue(
                        source_color_hct,
                        &Self::HUES,
                        &Self::SECONDARY_ROTATIONS,
                    ),
                    24.0,
                ),
                TonalPalette::of(
                    DynamicScheme::get_rotated_hue(
                        source_color_hct,
                        &Self::HUES,
                        &Self::TERTIARY_ROTATIONS,
                    ),
                    32.0,
                ),
                TonalPalette::of(source_color_hct.get_hue(), 10.0),
                TonalPalette::of(source_color_hct.get_hue(), 12.0),
                None,
            ),
        }
    }
}
