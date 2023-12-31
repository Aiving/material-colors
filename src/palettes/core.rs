use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::hct::cam16::Cam16;
use crate::utils::color::Argb;

use super::tonal::TonalPalette;

/// An intermediate concept between the key color for a UI theme, and a full
/// color scheme. 5 tonal palettes are generated, all except one use the same
/// hue as the key color, and all vary in chroma.
pub struct CorePalette {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
    pub error: TonalPalette,
}

impl CorePalette {
    /// The number of generated tonal palettes.
    pub const SIZE: usize = 5;

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
            error: error.unwrap_or(TonalPalette::of(25.0, 84.0)),
        }
    }

    /// Create a [CorePalette] from a source Argb color.
    pub fn of(argb: Argb) -> CorePalette {
        let cam = Cam16::from(argb);

        CorePalette::_of(cam.hue, cam.chroma)
    }

    fn _of(hue: f64, chroma: f64) -> CorePalette {
        CorePalette::new(
            TonalPalette::of(hue, 48.0_f64.max(chroma)),
            TonalPalette::of(hue, 16.0),
            TonalPalette::of(hue + 60.0, 24.0),
            TonalPalette::of(hue, 4.0),
            TonalPalette::of(hue, 8.0),
            None,
        )
    }

    /// Create a content [CorePalette] from a source Argb color.
    pub fn content_of(argb: Argb) -> CorePalette {
        let cam = Cam16::from(argb);

        CorePalette::_content_of(cam.hue, cam.chroma)
    }

    fn _content_of(hue: f64, chroma: f64) -> CorePalette {
        CorePalette::new(
            TonalPalette::of(hue, chroma),
            TonalPalette::of(hue, chroma / 3.0),
            TonalPalette::of(hue + 60.0, chroma / 2.0),
            TonalPalette::of(hue, (chroma / 12.0).min(4.0)),
            TonalPalette::of(hue, (chroma / 6.0).min(8.0)),
            None,
        )
    }
}

impl Display for CorePalette {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "primary{} secondary{} tertiary{} neutral{} neutralVariant{}",
            self.primary, self.secondary, self.tertiary, self.neutral, self.neutral_variant
        )
    }
}

impl PartialEq for CorePalette {
    fn eq(&self, other: &Self) -> bool {
        self.primary == other.primary
            && self.secondary == other.secondary
            && self.tertiary == other.tertiary
            && self.neutral == other.neutral
            && self.neutral_variant == other.neutral_variant
    }
}

// Returns a partition from a list.
//
// For example, given a list with 2 partitions of size 3.
// range = [1, 2, 3, 4, 5, 6];
//
// range.getPartition(0, 3) // [1, 2, 3]
// range.getPartition(1, 3) // [4, 5, 6]
fn _get_partition(list: &[i32], partition_number: usize, partition_size: usize) -> &[i32] {
    &list[(partition_number * partition_size)..((partition_number + 1) * partition_size)]
}
