#![allow(deprecated)]

use core::fmt;

use super::TonalPalette;
use crate::{color::Rgb, hct::Cam16};

/// An intermediate concept between the key color for a UI theme, and a full
/// color scheme. 5 tonal palettes are generated, all except one use the same
/// hue as the key color, and all vary in chroma.
#[derive(Debug, Hash, PartialEq, Eq)]
#[deprecated(
    since = "0.5.0",
    note = "use `DynamicScheme` for color scheme generation. Use `CorePalettes` for core palettes container class"
)]
pub struct CorePalette {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
    pub error: TonalPalette,
}

impl CorePalette {
    pub fn new(
        primary: TonalPalette,
        secondary: TonalPalette,
        tertiary: TonalPalette,
        neutral: TonalPalette,
        neutral_variant: TonalPalette,
        error: Option<TonalPalette>,
    ) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
            error: error.unwrap_or_else(|| TonalPalette::of(25.0, 84.0)),
        }
    }

    /// Create a [`CorePalette`] from a source Rgb color.
    pub fn of(rgb: Rgb) -> Self {
        let cam = Cam16::from(rgb);
        let (hue, chroma) = (cam.hue, cam.chroma);

        Self::new(
            TonalPalette::of(hue, 48.0_f64.max(chroma)),
            TonalPalette::of(hue, 16.0),
            TonalPalette::of(hue + 60.0, 24.0),
            TonalPalette::of(hue, 4.0),
            TonalPalette::of(hue, 8.0),
            None,
        )
    }

    /// Create a content [`CorePalette`] from a source Rgb color.
    pub fn content_of(rgb: Rgb) -> Self {
        let cam = Cam16::from(rgb);
        let (hue, chroma) = (cam.hue, cam.chroma);

        Self::new(
            TonalPalette::of(hue, chroma),
            TonalPalette::of(hue, chroma / 3.0),
            TonalPalette::of(hue + 60.0, chroma / 2.0),
            TonalPalette::of(hue, (chroma / 12.0).min(4.0)),
            TonalPalette::of(hue, (chroma / 6.0).min(8.0)),
            None,
        )
    }
}

impl fmt::Display for CorePalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "primary{} secondary{} tertiary{} neutral{} neutral_variant{}",
            self.primary, self.secondary, self.tertiary, self.neutral, self.neutral_variant
        )
    }
}

/// Comprises foundational palettes to build a color scheme. Generated from a
/// source color, these palettes will then be part of a [`DynamicScheme`]
/// together with appearance preferences.
///
/// [`DynamicScheme`]: [crate::dynamic_color::dynamic_scheme::DynamicScheme]
pub struct CorePalettes {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
}
