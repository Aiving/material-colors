use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

use super::Palette;
use crate::{
    color::Rgb,
    dynamic_color::Variant,
    hct::Hct,
    scheme::variant::{
        SchemeCmf, SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome, SchemeNeutral, SchemeRainbow, SchemeTonalSpot,
        SchemeVibrant,
    },
};

/// A convenience type for retrieving colors that are constant in hue and
/// chroma, but vary in tone.
#[derive(Clone, Copy, Debug, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TonalPalette {
    hue: f64,
    chroma: f64,
    key_color: Hct,
}

impl TonalPalette {
    /// Commonly-used tone values.
    const COMMON_TONES: [i32; 13] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];

    pub const fn common_size() -> usize {
        Self::COMMON_TONES.len()
    }

    pub const fn hue(&self) -> f64 {
        self.hue
    }

    pub const fn chroma(&self) -> f64 {
        self.chroma
    }

    pub const fn key_color(&self) -> Hct {
        self.key_color
    }

    const fn new(hue: f64, chroma: f64, key_color: Hct) -> Self {
        Self { hue, chroma, key_color }
    }

    /// Create a Tonal Palette from hue and chroma of `hct`.
    pub const fn from_hct(hct: Hct) -> Self {
        Self::new(hct.get_hue(), hct.get_chroma(), hct)
    }

    pub fn by_variant(source_hct: &Hct, scheme: &Variant, variant: &Palette) -> Self {
        match scheme {
            Variant::Monochrome => SchemeMonochrome::palette(source_hct, variant),
            Variant::Neutral => SchemeNeutral::palette(source_hct, variant),
            Variant::TonalSpot => SchemeTonalSpot::palette(source_hct, variant),
            Variant::Vibrant => SchemeVibrant::palette(source_hct, variant),
            Variant::Expressive => SchemeExpressive::palette(source_hct, variant),
            Variant::Fidelity => SchemeFidelity::palette(source_hct, variant),
            Variant::Content => SchemeContent::palette(source_hct, variant),
            Variant::Rainbow => SchemeRainbow::palette(source_hct, variant),
            Variant::FruitSalad => SchemeFruitSalad::palette(source_hct, variant),
            Variant::Cmf => SchemeCmf::palette(source_hct, variant),
        }
    }

    /// Create a Tonal Palette from `hue` and `chroma`, which generates a key
    /// color.
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
        Self::new(hue, chroma, KeyColor::new(hue, chroma).create())
    }

    /// Create colors using `hue` and `chroma`.
    pub fn of(hue: f64, chroma: f64) -> Self {
        Self::from_hue_and_chroma(hue, chroma)
    }

    /// The color with this palette's hue and chroma at `tone`, as RGB.
    pub fn tone(&self, tone: i32) -> Rgb {
        Hct::from(self.hue(), self.chroma(), f64::from(tone)).into()
    }

    pub fn get_hct(&self, tone: f64) -> Hct {
        Hct::from(self.hue(), self.chroma(), tone)
    }
}

impl Ord for TonalPalette {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for TonalPalette {
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue && self.chroma == other.chroma
    }
}

impl Eq for TonalPalette {}

impl Hash for TonalPalette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hue.to_bits().hash(state);
        self.chroma.to_bits().hash(state);
        self.key_color.hash(state);
    }
}

impl fmt::Display for TonalPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TonalPalette.of({}, {})", self.hue(), self.chroma())
    }
}

/// Key color is a color that represents the hue and chroma of a tonal palette
pub struct KeyColor {
    hue: f64,
    requested_chroma: f64,
    /// Cache that maps tone to max chroma to avoid duplicated HCT calculation.
    chroma_cache: [f64; 100],
}

impl KeyColor {
    const MAX_CHROMA_VALUE: f64 = 200.0;

    pub const fn new(hue: f64, requested_chroma: f64) -> Self {
        Self {
            hue,
            requested_chroma,
            chroma_cache: [-1.0; 100],
        }
    }

    /// Creates a key color from a [`hue`] and a [`chroma`].
    /// The key color is the first tone, starting from T50, matching the given
    /// hue and chroma.
    ///
    /// Returns key color in [`Hct`].
    pub fn create(&mut self) -> Hct {
        // Pivot around T50 because T50 has the most chroma available, on average. Thus
        // it is most likely to have a direct answer.
        let pivot_tone = 50;
        let tone_step_size = 1;
        // Epsilon to accept values slightly higher than the requested chroma.
        let epsilon = 0.01;

        // Binary search to find the tone that can provide a chroma that is closest
        // to the requested chroma.
        let mut lower_tone = 0;
        let mut upper_tone = 100;

        while lower_tone < upper_tone {
            let mid_tone = usize::midpoint(lower_tone, upper_tone);
            let mid_tone_max_chroma = self.max_chroma(mid_tone);
            let is_ascending = mid_tone_max_chroma < self.max_chroma(mid_tone + tone_step_size);
            let sufficient_chroma = mid_tone_max_chroma >= self.requested_chroma - epsilon;

            if sufficient_chroma {
                // Either range [`lower_tone`, `mid_tone`] or [`mid_tone`, `upper_tone`] has
                // answer, so search in the range that is closer the pivot tone.
                if (lower_tone as isize - pivot_tone).abs() < (upper_tone as isize - pivot_tone).abs() {
                    upper_tone = mid_tone;
                } else if lower_tone == mid_tone {
                    return Hct::from(self.hue, self.requested_chroma, lower_tone as f64);
                } else {
                    lower_tone = mid_tone;
                }
            } else if is_ascending {
                // As there is no sufficient chroma in the `mid_tone`, follow the direction to
                // the chroma peak.
                lower_tone = mid_tone + tone_step_size;
            } else {
                // Keep `mid_tone` for potential chroma peak.
                upper_tone = mid_tone;
            }
        }

        Hct::from(self.hue, self.requested_chroma, lower_tone as f64)
    }

    fn max_chroma(&mut self, tone: usize) -> f64 {
        let chroma = self.chroma_cache[tone];

        if chroma < 0.0 {
            let chroma = Hct::from(self.hue, Self::MAX_CHROMA_VALUE, tone as f64).get_chroma();

            self.chroma_cache[tone] = chroma;

            chroma
        } else {
            chroma
        }
    }
}
