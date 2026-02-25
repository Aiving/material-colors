#[cfg(not(feature = "std"))] use alloc::vec::Vec;
#[cfg(feature = "std")] use std::vec::Vec;

use super::{Quantizer, QuantizerResult, QuantizerWsmeans, QuantizerWu};
use crate::color::Argb;

#[derive(Default)]
pub struct QuantizerCelebi;

impl Quantizer for QuantizerCelebi {
    fn quantize(pixels: &[Argb], max_colors: usize) -> QuantizerResult {
        let wu_result = QuantizerWu::quantize(pixels, max_colors);

        QuantizerWsmeans::quantize(pixels, max_colors, &wu_result.color_to_count.into_keys().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))] use alloc::vec::Vec;
    #[cfg(feature = "std")] use std::vec::Vec;

    use super::QuantizerCelebi;
    use crate::{color::Argb, quantize::Quantizer};

    const RED: Argb = Argb::from_u32(0xFFFF0000);
    const GREEN: Argb = Argb::from_u32(0xFF00FF00);
    const BLUE: Argb = Argb::from_u32(0xFF0000FF);
    // const WHITE: Argb = Argb::from_u32(0xffffffff);
    // const RANDOM: Argb = Argb::from_u32(0xff426088);
    const MAX_COLORS: usize = 256;

    const IMAGE_PIXELS: [Argb; 84] = [
        Argb::from_u32(0xFF050505),
        Argb::from_u32(0xFF000000),
        Argb::from_u32(0xFF000000),
        Argb::from_u32(0xFF000000),
        Argb::from_u32(0xFF000000),
        Argb::from_u32(0xFF090909),
        Argb::from_u32(0xFF060404),
        Argb::from_u32(0xFF030102),
        Argb::from_u32(0xFF080607),
        Argb::from_u32(0xFF070506),
        Argb::from_u32(0xFF010001),
        Argb::from_u32(0xFF070506),
        Argb::from_u32(0xFF364341),
        Argb::from_u32(0xFF223529),
        Argb::from_u32(0xFF14251C),
        Argb::from_u32(0xFF11221A),
        Argb::from_u32(0xFF1F3020),
        Argb::from_u32(0xFF34443A),
        Argb::from_u32(0xFF64817E),
        Argb::from_u32(0xFF638777),
        Argb::from_u32(0xFF486D58),
        Argb::from_u32(0xFF2F5536),
        Argb::from_u32(0xFF467258),
        Argb::from_u32(0xFF7FB7B9),
        Argb::from_u32(0xFF6D8473),
        Argb::from_u32(0xFF859488),
        Argb::from_u32(0xFF7A947E),
        Argb::from_u32(0xFF5F815D),
        Argb::from_u32(0xFF3A5D46),
        Argb::from_u32(0xFF497469),
        Argb::from_u32(0xFF737A73),
        Argb::from_u32(0xFF656453),
        Argb::from_u32(0xFF445938),
        Argb::from_u32(0xFF657C4B),
        Argb::from_u32(0xFF65715B),
        Argb::from_u32(0xFF6A816E),
        Argb::from_u32(0xFF667366),
        Argb::from_u32(0xFF5B5547),
        Argb::from_u32(0xFF3B391E),
        Argb::from_u32(0xFF705E3D),
        Argb::from_u32(0xFF7F6C5E),
        Argb::from_u32(0xFF6D7C6C),
        Argb::from_u32(0xFFA99C9C),
        Argb::from_u32(0xFF8B7671),
        Argb::from_u32(0xFF6A3229),
        Argb::from_u32(0xFF80514B),
        Argb::from_u32(0xFF857970),
        Argb::from_u32(0xFF4F5A4C),
        Argb::from_u32(0xFF897273),
        Argb::from_u32(0xFF745451),
        Argb::from_u32(0xFF512823),
        Argb::from_u32(0xFF78585A),
        Argb::from_u32(0xFF535552),
        Argb::from_u32(0xFF40493F),
        Argb::from_u32(0xFF151616),
        Argb::from_u32(0xFF0A0C0C),
        Argb::from_u32(0xFF050808),
        Argb::from_u32(0xFF010303),
        Argb::from_u32(0xFF000100),
        Argb::from_u32(0xFF010200),
        Argb::from_u32(0xFF191816),
        Argb::from_u32(0xFF181818),
        Argb::from_u32(0xFF0C0C0C),
        Argb::from_u32(0xFF040404),
        Argb::from_u32(0xFF0C0C0C),
        Argb::from_u32(0xFF151514),
        Argb::from_u32(0xFFB1C3B9),
        Argb::from_u32(0xFFBFBFBF),
        Argb::from_u32(0xFFBABABA),
        Argb::from_u32(0xFFB7B7B7),
        Argb::from_u32(0xFFB3B3B3),
        Argb::from_u32(0xFFADADAD),
        Argb::from_u32(0xFF535756),
        Argb::from_u32(0xFF575656),
        Argb::from_u32(0xFF555555),
        Argb::from_u32(0xFF555555),
        Argb::from_u32(0xFF545454),
        Argb::from_u32(0xFF474646),
        Argb::from_u32(0xFF000000),
        Argb::from_u32(0xFF000000),
        Argb::from_u32(0xFF0B0B0B),
        Argb::from_u32(0xFF0B0B0B),
        Argb::from_u32(0xFF000000),
        Argb::from_u32(0xFF000000),
    ];

    #[test]
    fn test_1rando() {
        let result = QuantizerCelebi::quantize(&[Argb::from_u32(0xFF141216)], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &Argb::from_u32(0xFF141216));
    }

    #[test]
    fn test_1r() {
        let result = QuantizerCelebi::quantize(&[RED], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &RED);
    }

    #[test]
    fn test_1g() {
        let result = QuantizerCelebi::quantize(&[GREEN], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &GREEN);
    }

    #[test]
    fn test_1b() {
        let result = QuantizerCelebi::quantize(&[BLUE], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &BLUE);
    }

    #[test]
    fn test_5b() {
        let result = QuantizerCelebi::quantize(&[BLUE, BLUE, BLUE, BLUE, BLUE], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &BLUE);
    }

    #[test]
    fn test_2r_3g() {
        let result = QuantizerCelebi::quantize(&[RED, RED, GREEN, GREEN, GREEN], MAX_COLORS);

        assert_eq!(result.color_to_count.keys().len(), 2);
        assert_eq!(result.color_to_count.get(&GREEN).unwrap(), &3);
        assert_eq!(result.color_to_count.get(&RED).unwrap(), &2);
    }

    #[test]
    fn test_1r_1g_1b() {
        let result = QuantizerCelebi::quantize(&[RED, GREEN, BLUE], MAX_COLORS);

        assert_eq!(result.color_to_count.keys().len(), 3);
        assert_eq!(result.color_to_count.get(&GREEN).unwrap(), &1);
        assert_eq!(result.color_to_count.get(&RED).unwrap(), &1);
        assert_eq!(result.color_to_count.get(&BLUE).unwrap(), &1);
    }

    /// Verifies QuantizerCelebi.quantize returns identical result given same
    /// input.
    #[test]
    fn test_stability() {
        let result1 = QuantizerCelebi::quantize(&IMAGE_PIXELS, 16).color_to_count;
        let result2 = QuantizerCelebi::quantize(&IMAGE_PIXELS, 16).color_to_count;

        assert_eq!(result1, result2);
    }
}
