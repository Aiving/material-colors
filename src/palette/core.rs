#![allow(deprecated)]

use super::TonalPalette;
use crate::{color::Argb, hct::Cam16};
use core::fmt;

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

    /// Create a [`CorePalette`] from a source Argb color.
    pub fn of(argb: Argb) -> Self {
        let cam = Cam16::from(argb);
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

    /// Create a content [`CorePalette`] from a source Argb color.
    pub fn content_of(argb: Argb) -> Self {
        let cam = Cam16::from(argb);
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
/// source color, these palettes will then be part of a [`DynamicScheme`] together
/// with appearance preferences.
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
    use crate::{color::Argb, palette::CorePalette};
    use ahash::AHasher;
    use core::hash::{Hash, Hasher};

    fn hash_value<T: Hash>(value: &T) -> u64 {
        let mut hasher = AHasher::default();

        value.hash(&mut hasher);

        hasher.finish()
    }

    #[test]
    fn test_equals_and_hash() {
        let core_palette_a = CorePalette::of(Argb::from_u32(0xff0000ff));
        let core_palette_b = CorePalette::of(Argb::from_u32(0xff0000ff));
        let core_palette_c = CorePalette::of(Argb::from_u32(0xff123456));

        assert_eq!(core_palette_a, core_palette_b);
        assert!(core_palette_b != core_palette_c);

        assert_eq!(hash_value(&core_palette_a), hash_value(&core_palette_b));
        assert!(hash_value(&core_palette_b) != hash_value(&core_palette_c));
    }

    #[test]
    fn test_of_blue() {
        let core = CorePalette::of(Argb::from_u32(0xff0000ff));

        assert_eq!(core.primary.tone(100), Argb::from_u32(0xffffffff));
        assert_eq!(core.primary.tone(95), Argb::from_u32(0xfff1efff));
        assert_eq!(core.primary.tone(90), Argb::from_u32(0xffe0e0ff));
        assert_eq!(core.primary.tone(80), Argb::from_u32(0xffbec2ff));
        assert_eq!(core.primary.tone(70), Argb::from_u32(0xff9da3ff));
        assert_eq!(core.primary.tone(60), Argb::from_u32(0xff7c84ff));
        assert_eq!(core.primary.tone(50), Argb::from_u32(0xff5a64ff));
        assert_eq!(core.primary.tone(40), Argb::from_u32(0xff343dff));
        assert_eq!(core.primary.tone(30), Argb::from_u32(0xff0000ef));
        assert_eq!(core.primary.tone(20), Argb::from_u32(0xff0001ac));
        assert_eq!(core.primary.tone(10), Argb::from_u32(0xff00006e));
        assert_eq!(core.primary.tone(0), Argb::from_u32(0xff000000));
        assert_eq!(core.secondary.tone(100), Argb::from_u32(0xffffffff));
        assert_eq!(core.secondary.tone(95), Argb::from_u32(0xfff1efff));
        assert_eq!(core.secondary.tone(90), Argb::from_u32(0xffe1e0f9));
        assert_eq!(core.secondary.tone(80), Argb::from_u32(0xffc5c4dd));
        assert_eq!(core.secondary.tone(70), Argb::from_u32(0xffa9a9c1));
        assert_eq!(core.secondary.tone(60), Argb::from_u32(0xff8f8fa6));
        assert_eq!(core.secondary.tone(50), Argb::from_u32(0xff75758b));
        assert_eq!(core.secondary.tone(40), Argb::from_u32(0xff5c5d72));
        assert_eq!(core.secondary.tone(30), Argb::from_u32(0xff444559));
        assert_eq!(core.secondary.tone(20), Argb::from_u32(0xff2e2f42));
        assert_eq!(core.secondary.tone(10), Argb::from_u32(0xff191a2c));
        assert_eq!(core.secondary.tone(0), Argb::from_u32(0xff000000));
    }

    #[test]
    fn test_content_of_blue() {
        let core = CorePalette::content_of(Argb::from_u32(0xff0000ff));

        assert_eq!(core.primary.tone(100), Argb::from_u32(0xffffffff));
        assert_eq!(core.primary.tone(95), Argb::from_u32(0xfff1efff));
        assert_eq!(core.primary.tone(90), Argb::from_u32(0xffe0e0ff));
        assert_eq!(core.primary.tone(80), Argb::from_u32(0xffbec2ff));
        assert_eq!(core.primary.tone(70), Argb::from_u32(0xff9da3ff));
        assert_eq!(core.primary.tone(60), Argb::from_u32(0xff7c84ff));
        assert_eq!(core.primary.tone(50), Argb::from_u32(0xff5a64ff));
        assert_eq!(core.primary.tone(40), Argb::from_u32(0xff343dff));
        assert_eq!(core.primary.tone(30), Argb::from_u32(0xff0000ef));
        assert_eq!(core.primary.tone(20), Argb::from_u32(0xff0001ac));
        assert_eq!(core.primary.tone(10), Argb::from_u32(0xff00006e));
        assert_eq!(core.primary.tone(0), Argb::from_u32(0xff000000));
        assert_eq!(core.secondary.tone(100), Argb::from_u32(0xffffffff));
        assert_eq!(core.secondary.tone(95), Argb::from_u32(0xfff1efff));
        assert_eq!(core.secondary.tone(90), Argb::from_u32(0xffe0e0ff));
        assert_eq!(core.secondary.tone(80), Argb::from_u32(0xffc1c3f4));
        assert_eq!(core.secondary.tone(70), Argb::from_u32(0xffa5a7d7));
        assert_eq!(core.secondary.tone(60), Argb::from_u32(0xff8b8dbb));
        assert_eq!(core.secondary.tone(50), Argb::from_u32(0xff7173a0));
        assert_eq!(core.secondary.tone(40), Argb::from_u32(0xff585b86));
        assert_eq!(core.secondary.tone(30), Argb::from_u32(0xff40436d));
        assert_eq!(core.secondary.tone(20), Argb::from_u32(0xff2a2d55));
        assert_eq!(core.secondary.tone(10), Argb::from_u32(0xff14173f));
        assert_eq!(core.secondary.tone(0), Argb::from_u32(0xff000000));
    }
}
