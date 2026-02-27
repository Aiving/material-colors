#![allow(deprecated)]

use core::fmt;

use super::TonalPalette;
use crate::{color::Rgb, hct::Cam16};

/// An intermediate concept between the key color for a UI theme, and a full
/// color scheme. 5 tonal palettes are generated, all except one use the same
/// hue as the key color, and all vary in chroma.
#[derive(Debug, Hash, PartialEq, Eq)]
#[deprecated = "Use `DynamicScheme` for color scheme generation. Use `CorePalettes` for core palettes container class"]
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

#[cfg(test)]
mod tests {
    use core::hash::{Hash, Hasher};

    use ahash::AHasher;

    use crate::{color::Rgb, palette::CorePalette};

    fn hash_value<T: Hash>(value: &T) -> u64 {
        let mut hasher = AHasher::default();

        value.hash(&mut hasher);

        hasher.finish()
    }

    #[test]
    fn test_equals_and_hash() {
        let core_palette_a = CorePalette::of(Rgb::from_u32(0x0000FF));
        let core_palette_b = CorePalette::of(Rgb::from_u32(0x0000FF));
        let core_palette_c = CorePalette::of(Rgb::from_u32(0x123456));

        assert_eq!(core_palette_a, core_palette_b);
        assert!(core_palette_b != core_palette_c);

        assert_eq!(hash_value(&core_palette_a), hash_value(&core_palette_b));
        assert!(hash_value(&core_palette_b) != hash_value(&core_palette_c));
    }

    #[test]
    fn test_of_blue() {
        let core = CorePalette::of(Rgb::from_u32(0x0000FF));

        assert_eq!(core.primary.tone(100), Rgb::from_u32(0xFFFFFF));
        assert_eq!(core.primary.tone(95), Rgb::from_u32(0xF1EFFF));
        assert_eq!(core.primary.tone(90), Rgb::from_u32(0xE0E0FF));
        assert_eq!(core.primary.tone(80), Rgb::from_u32(0xBEC2FF));
        assert_eq!(core.primary.tone(70), Rgb::from_u32(0x9DA3FF));
        assert_eq!(core.primary.tone(60), Rgb::from_u32(0x7C84FF));
        assert_eq!(core.primary.tone(50), Rgb::from_u32(0x5A64FF));
        assert_eq!(core.primary.tone(40), Rgb::from_u32(0x343DFF));
        assert_eq!(core.primary.tone(30), Rgb::from_u32(0x0000EF));
        assert_eq!(core.primary.tone(20), Rgb::from_u32(0x0001AC));
        assert_eq!(core.primary.tone(10), Rgb::from_u32(0x00006E));
        assert_eq!(core.primary.tone(0), Rgb::from_u32(0x000000));
        assert_eq!(core.secondary.tone(100), Rgb::from_u32(0xFFFFFF));
        assert_eq!(core.secondary.tone(95), Rgb::from_u32(0xF1EFFF));
        assert_eq!(core.secondary.tone(90), Rgb::from_u32(0xE1E0F9));
        assert_eq!(core.secondary.tone(80), Rgb::from_u32(0xC5C4DD));
        assert_eq!(core.secondary.tone(70), Rgb::from_u32(0xA9A9C1));
        assert_eq!(core.secondary.tone(60), Rgb::from_u32(0x8F8FA6));
        assert_eq!(core.secondary.tone(50), Rgb::from_u32(0x75758B));
        assert_eq!(core.secondary.tone(40), Rgb::from_u32(0x5C5D72));
        assert_eq!(core.secondary.tone(30), Rgb::from_u32(0x444559));
        assert_eq!(core.secondary.tone(20), Rgb::from_u32(0x2E2F42));
        assert_eq!(core.secondary.tone(10), Rgb::from_u32(0x191A2C));
        assert_eq!(core.secondary.tone(0), Rgb::from_u32(0x000000));
    }

    #[test]
    fn test_content_of_blue() {
        let core = CorePalette::content_of(Rgb::from_u32(0x0000FF));

        assert_eq!(core.primary.tone(100), Rgb::from_u32(0xFFFFFF));
        assert_eq!(core.primary.tone(95), Rgb::from_u32(0xF1EFFF));
        assert_eq!(core.primary.tone(90), Rgb::from_u32(0xE0E0FF));
        assert_eq!(core.primary.tone(80), Rgb::from_u32(0xBEC2FF));
        assert_eq!(core.primary.tone(70), Rgb::from_u32(0x9DA3FF));
        assert_eq!(core.primary.tone(60), Rgb::from_u32(0x7C84FF));
        assert_eq!(core.primary.tone(50), Rgb::from_u32(0x5A64FF));
        assert_eq!(core.primary.tone(40), Rgb::from_u32(0x343DFF));
        assert_eq!(core.primary.tone(30), Rgb::from_u32(0x0000EF));
        assert_eq!(core.primary.tone(20), Rgb::from_u32(0x0001AC));
        assert_eq!(core.primary.tone(10), Rgb::from_u32(0x00006E));
        assert_eq!(core.primary.tone(0), Rgb::from_u32(0x000000));
        assert_eq!(core.secondary.tone(100), Rgb::from_u32(0xFFFFFF));
        assert_eq!(core.secondary.tone(95), Rgb::from_u32(0xF1EFFF));
        assert_eq!(core.secondary.tone(90), Rgb::from_u32(0xE0E0FF));
        assert_eq!(core.secondary.tone(80), Rgb::from_u32(0xC1C3F4));
        assert_eq!(core.secondary.tone(70), Rgb::from_u32(0xA5A7D7));
        assert_eq!(core.secondary.tone(60), Rgb::from_u32(0x8B8DBB));
        assert_eq!(core.secondary.tone(50), Rgb::from_u32(0x7173A0));
        assert_eq!(core.secondary.tone(40), Rgb::from_u32(0x585B86));
        assert_eq!(core.secondary.tone(30), Rgb::from_u32(0x40436D));
        assert_eq!(core.secondary.tone(20), Rgb::from_u32(0x2A2D55));
        assert_eq!(core.secondary.tone(10), Rgb::from_u32(0x14173F));
        assert_eq!(core.secondary.tone(0), Rgb::from_u32(0x000000));
    }
}
