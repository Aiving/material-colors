use std::fmt;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
    color::Argb,
    dynamic_color::Variant,
    hct::Hct,
    scheme::variant::{
        SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
        SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
    },
};

use super::Palette;

/// A convenience class for retrieving colors that are constant in hue and
/// chroma, but vary in tone.
///
/// This class can be instantiated in two ways:
/// 1. [of] From hue and chroma. (preferred)
/// 2. [fromList] From a fixed-size ([TonalPalette.commonSize]) list of ints
/// representing ARBG colors. Correctness (constant hue and chroma) of the input
/// is not enforced. [get] will only return the input colors, corresponding to
/// [commonTones].
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

    /// Create a Tonal Palette from hue and chroma of [hct].
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

    /// Create a Tonal Palette from hue and chroma, which generates a key color.
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
        Self::new(hue, chroma, Self::create_key_color(hue, chroma))
    }

    /// Create colors using [hue] and [chroma].
    pub fn of(hue: f64, chroma: f64) -> Self {
        Self::from_hue_and_chroma(hue, chroma)
    }

    /// Creates a key color from a [hue] and a [chroma].
    /// The key color is the first tone, starting from T50, matching the given hue and chroma.
    /// Key color [Hct]
    pub fn create_key_color(hue: f64, chroma: f64) -> Hct {
        let start_tone = 50.0;
        let mut smallest_delta_hct = Hct::from(hue, chroma, start_tone);
        let mut smallest_delta = (smallest_delta_hct.get_chroma() - chroma).abs();

        // Starting from T50, check T+/-delta to see if they match the requested
        // chroma.
        //
        // Starts from T50 because T50 has the most chroma available, on
        // average. Thus it is most likely to have a direct answer and minimize
        // iteration.
        for delta in 1..=49 {
            // Termination condition rounding instead of minimizing delta to avoid
            // case where requested chroma is 16.51, and the closest chroma is 16.49.
            // Error is minimized, but when rounded and displayed, requested chroma
            // is 17, key color's chroma is 16.
            if (chroma.round() - smallest_delta_hct.get_chroma().round()).abs() < f64::EPSILON {
                return smallest_delta_hct;
            }

            let hct_add = Hct::from(hue, chroma, start_tone + f64::from(delta));
            let hct_add_delta = (hct_add.get_chroma() - chroma).abs();

            if hct_add_delta < smallest_delta {
                smallest_delta = hct_add_delta;
                smallest_delta_hct = hct_add;
            }

            let hct_subtract = Hct::from(hue, chroma, start_tone - f64::from(delta));
            let hct_subtract_delta = (hct_subtract.get_chroma() - chroma).abs();

            if hct_subtract_delta < smallest_delta {
                smallest_delta = hct_subtract_delta;
                smallest_delta_hct = hct_subtract;
            }
        }

        smallest_delta_hct
    }

    /// Returns the Argb representation of an HCT color.
    ///
    /// If the class was instantiated from [_hue] and [_chroma], will return the
    /// color with corresponding [tone].
    /// If the class was instantiated from a fixed-size list of color ints, [tone]
    /// must be in [commonTones].
    pub fn tone(&self, tone: i32) -> Argb {
        Hct::from(self.hue(), self.chroma(), f64::from(tone)).into()
    }

    pub fn get_hct(&self, tone: f64) -> Hct {
        Hct::from(self.hue(), self.chroma(), tone)
    }
}

impl PartialEq for TonalPalette {
    fn eq(&self, other: &Self) -> bool {
        self._hue == other._hue && self._chroma == other._chroma
    }
}

impl fmt::Display for TonalPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TonalPalette.of({}, {})", self.hue(), self.chroma())
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Argb;
    use crate::hct::Hct;
    use crate::palette::TonalPalette;

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
