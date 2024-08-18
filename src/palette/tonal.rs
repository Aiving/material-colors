use super::Palette;
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::Argb,
    dynamic_color::Variant,
    hct::Hct,
    scheme::variant::{
        SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
        SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
    },
    Map,
};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};
#[cfg(feature = "serde")]
use serde::Serialize;

/// A convenience class for retrieving colors that are constant in hue and
/// chroma, but vary in tone.
#[derive(Clone, Copy, Debug, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TonalPalette {
    _hue: f64,
    _chroma: f64,
    _key_color: Hct,
}

impl TonalPalette {
    /// Commonly-used tone values.
    const COMMON_TONES: [i32; 13] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];

    pub const fn common_size() -> usize {
        Self::COMMON_TONES.len()
    }

    pub const fn hue(&self) -> f64 {
        self._hue
    }

    pub const fn chroma(&self) -> f64 {
        self._chroma
    }

    pub const fn key_color(&self) -> Hct {
        self._key_color
    }

    const fn new(_hue: f64, _chroma: f64, _key_color: Hct) -> Self {
        Self {
            _hue,
            _chroma,
            _key_color,
        }
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
        }
    }

    /// Create a Tonal Palette from `hue` and `chroma`, which generates a key color.
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
        Self::new(hue, chroma, KeyColor::new(hue, chroma).create())
    }

    /// Create colors using `hue` and `chroma`.
    pub fn of(hue: f64, chroma: f64) -> Self {
        Self::from_hue_and_chroma(hue, chroma)
    }

    /// Returns the Argb representation of an HCT color.
    ///
    /// If the class was instantiated from `_hue` and `_chroma`, will return the
    /// color with corresponding `tone`.
    /// If the class was instantiated from a fixed-size list of color ints, `tone`
    /// must be in `common_mones`.
    pub fn tone(&self, tone: i32) -> Argb {
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
        self._hue == other._hue && self._chroma == other._chroma
    }
}

impl Eq for TonalPalette {}

impl Hash for TonalPalette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._hue.to_bits().hash(state);
        self._chroma.to_bits().hash(state);
        self._key_color.hash(state);
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
    chroma_cache: Map<i32, f64>,
}

impl KeyColor {
    const MAX_CHROMA_VALUE: f64 = 200.0;

    pub fn new(hue: f64, requested_chroma: f64) -> Self {
        Self {
            hue,
            requested_chroma,
            chroma_cache: Map::default(),
        }
    }

    /// Creates a key color from a [`hue`] and a [`chroma`].
    /// The key color is the first tone, starting from T50, matching the given hue
    /// and chroma.
    ///
    /// Returns key color in [`Hct`].
    pub fn create(&mut self) -> Hct {
        // Pivot around T50 because T50 has the most chroma available, on average. Thus it is most
        // likely to have a direct answer.
        let pivot_tone = 50;
        let tone_step_size = 1;
        // Epsilon to accept values slightly higher than the requested chroma.
        let epsilon = 0.01;

        // Binary search to find the tone that can provide a chroma that is closest
        // to the requested chroma.
        let mut lower_tone = 0;
        let mut upper_tone = 100;

        while lower_tone < upper_tone {
            let mid_tone = (lower_tone + upper_tone) / 2;
            let is_ascending =
                self.max_chroma(mid_tone) < self.max_chroma(mid_tone + tone_step_size);
            let sufficient_chroma = self.max_chroma(mid_tone) >= self.requested_chroma - epsilon;

            if sufficient_chroma {
                // Either range [lowerTone, midTone] or [midTone, upperTone] has answer, so search in the
                // range that is closer the pivot tone.
                if (lower_tone - pivot_tone).abs() < (upper_tone - pivot_tone).abs() {
                    upper_tone = mid_tone;
                } else if lower_tone == mid_tone {
                    return Hct::from(self.hue, self.requested_chroma, f64::from(lower_tone));
                } else {
                    lower_tone = mid_tone;
                }
            } else if is_ascending {
                // As there is no sufficient chroma in the midTone, follow the direction to the chroma
                // peak.
                lower_tone = mid_tone + tone_step_size;
            } else {
                // Keep midTone for potential chroma peak.
                upper_tone = mid_tone;
            }
        }

        Hct::from(self.hue, self.requested_chroma, f64::from(lower_tone))
    }

    fn max_chroma(&mut self, tone: i32) -> f64 {
        if let Some(chroma) = self.chroma_cache.get(&tone) {
            *chroma
        } else {
            let chroma = Hct::from(self.hue, Self::MAX_CHROMA_VALUE, f64::from(tone)).get_chroma();

            self.chroma_cache.insert(tone, chroma);

            chroma
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::{color::Argb, hct::Hct, palette::TonalPalette};

    #[test]
    fn test_exact_chroma_available() {
        let palette = TonalPalette::of(50.0, 60.0);
        let result = palette.key_color();

        assert_approx_eq!(f64, result.get_hue(), 50.0, epsilon = 10.0);
        assert_approx_eq!(f64, result.get_chroma(), 60.0, epsilon = 0.5);

        assert!(result.get_tone() > 0.0);
        assert!(result.get_tone() < 100.0);
    }

    #[test]
    fn test_unusually_high_chroma() {
        let palette = TonalPalette::of(149.0, 200.0);
        let result = palette.key_color();

        assert_approx_eq!(f64, result.get_hue(), 149.0, epsilon = 10.0);

        assert!(result.get_chroma() > 89.0);
        assert!(result.get_tone() > 0.0);
        assert!(result.get_tone() < 100.0);
    }

    #[test]
    fn test_unusually_low_chroma() {
        let palette = TonalPalette::of(50.0, 3.0);
        let result = palette.key_color();

        assert_approx_eq!(f64, result.get_hue(), 50.0, epsilon = 10.0);
        assert_approx_eq!(f64, result.get_chroma(), 3.0, epsilon = 0.5);
        assert_approx_eq!(f64, result.get_tone(), 50.0, epsilon = 0.5);
    }

    #[test]
    fn test_of_tones_of_blue() {
        let hct: Hct = Argb::from_u32(0xff0000ff).into();
        let tones = TonalPalette::of(hct.get_hue(), hct.get_chroma());

        assert_eq!(tones.tone(0), Argb::from_u32(0xff000000));
        assert_eq!(tones.tone(10), Argb::from_u32(0xff00006e));
        assert_eq!(tones.tone(20), Argb::from_u32(0xff0001ac));
        assert_eq!(tones.tone(30), Argb::from_u32(0xff0000ef));
        assert_eq!(tones.tone(40), Argb::from_u32(0xff343dff));
        assert_eq!(tones.tone(50), Argb::from_u32(0xff5a64ff));
        assert_eq!(tones.tone(60), Argb::from_u32(0xff7c84ff));
        assert_eq!(tones.tone(70), Argb::from_u32(0xff9da3ff));
        assert_eq!(tones.tone(80), Argb::from_u32(0xffbec2ff));
        assert_eq!(tones.tone(90), Argb::from_u32(0xffe0e0ff));
        assert_eq!(tones.tone(95), Argb::from_u32(0xfff1efff));
        assert_eq!(tones.tone(99), Argb::from_u32(0xfffffbff));
        assert_eq!(tones.tone(100), Argb::from_u32(0xffffffff));

        // Tone not in [TonalPalette.commonTones]
        assert_eq!(tones.tone(3), Argb::from_u32(0xff00003c));
    }

    #[test]
    fn test_of_operator_and_hash() {
        let hct_ab: Hct = Argb::from_u32(0xff0000ff).into();
        let tones_a = TonalPalette::of(hct_ab.get_hue(), hct_ab.get_chroma());
        let tones_b = TonalPalette::of(hct_ab.get_hue(), hct_ab.get_chroma());
        let hct_c: Hct = Argb::from_u32(0xff123456).into();
        let tones_c = TonalPalette::of(hct_c.get_hue(), hct_c.get_chroma());

        assert_eq!(tones_a, tones_b);
        assert!(tones_b != tones_c);
    }
}
