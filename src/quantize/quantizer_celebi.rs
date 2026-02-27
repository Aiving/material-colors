#[cfg(not(feature = "std"))] use alloc::vec::Vec;
#[cfg(feature = "std")] use std::vec::Vec;

use super::{Quantizer, QuantizerResult, QuantizerWsmeans, QuantizerWu};
use crate::color::Rgb;

#[derive(Default)]
pub struct QuantizerCelebi;

impl Quantizer for QuantizerCelebi {
    fn quantize(pixels: &[Rgb], max_colors: usize) -> QuantizerResult {
        let wu_result = QuantizerWu::quantize(pixels, max_colors);

        QuantizerWsmeans::quantize(pixels, max_colors, &wu_result.color_to_count.into_keys().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))] use alloc::vec::Vec;
    #[cfg(feature = "std")] use std::vec::Vec;

    use super::QuantizerCelebi;
    use crate::{color::Rgb, quantize::Quantizer};

    const RED: Rgb = Rgb::from_u32(0xFF0000);
    const GREEN: Rgb = Rgb::from_u32(0x00FF00);
    const BLUE: Rgb = Rgb::from_u32(0x0000FF);
    // const WHITE: Rgb = Rgb::from_u32(0xffffffff);
    // const RANDOM: Rgb = Rgb::from_u32(0xff426088);
    const MAX_COLORS: usize = 256;

    const IMAGE_PIXELS: [Rgb; 84] = [
        Rgb::from_u32(0x050505),
        Rgb::from_u32(0x000000),
        Rgb::from_u32(0x000000),
        Rgb::from_u32(0x000000),
        Rgb::from_u32(0x000000),
        Rgb::from_u32(0x090909),
        Rgb::from_u32(0x060404),
        Rgb::from_u32(0x030102),
        Rgb::from_u32(0x080607),
        Rgb::from_u32(0x070506),
        Rgb::from_u32(0x010001),
        Rgb::from_u32(0x070506),
        Rgb::from_u32(0x364341),
        Rgb::from_u32(0x223529),
        Rgb::from_u32(0x14251C),
        Rgb::from_u32(0x11221A),
        Rgb::from_u32(0x1F3020),
        Rgb::from_u32(0x34443A),
        Rgb::from_u32(0x64817E),
        Rgb::from_u32(0x638777),
        Rgb::from_u32(0x486D58),
        Rgb::from_u32(0x2F5536),
        Rgb::from_u32(0x467258),
        Rgb::from_u32(0x7FB7B9),
        Rgb::from_u32(0x6D8473),
        Rgb::from_u32(0x859488),
        Rgb::from_u32(0x7A947E),
        Rgb::from_u32(0x5F815D),
        Rgb::from_u32(0x3A5D46),
        Rgb::from_u32(0x497469),
        Rgb::from_u32(0x737A73),
        Rgb::from_u32(0x656453),
        Rgb::from_u32(0x445938),
        Rgb::from_u32(0x657C4B),
        Rgb::from_u32(0x65715B),
        Rgb::from_u32(0x6A816E),
        Rgb::from_u32(0x667366),
        Rgb::from_u32(0x5B5547),
        Rgb::from_u32(0x3B391E),
        Rgb::from_u32(0x705E3D),
        Rgb::from_u32(0x7F6C5E),
        Rgb::from_u32(0x6D7C6C),
        Rgb::from_u32(0xA99C9C),
        Rgb::from_u32(0x8B7671),
        Rgb::from_u32(0x6A3229),
        Rgb::from_u32(0x80514B),
        Rgb::from_u32(0x857970),
        Rgb::from_u32(0x4F5A4C),
        Rgb::from_u32(0x897273),
        Rgb::from_u32(0x745451),
        Rgb::from_u32(0x512823),
        Rgb::from_u32(0x78585A),
        Rgb::from_u32(0x535552),
        Rgb::from_u32(0x40493F),
        Rgb::from_u32(0x151616),
        Rgb::from_u32(0x0A0C0C),
        Rgb::from_u32(0x050808),
        Rgb::from_u32(0x010303),
        Rgb::from_u32(0x000100),
        Rgb::from_u32(0x010200),
        Rgb::from_u32(0x191816),
        Rgb::from_u32(0x181818),
        Rgb::from_u32(0x0C0C0C),
        Rgb::from_u32(0x040404),
        Rgb::from_u32(0x0C0C0C),
        Rgb::from_u32(0x151514),
        Rgb::from_u32(0xB1C3B9),
        Rgb::from_u32(0xBFBFBF),
        Rgb::from_u32(0xBABABA),
        Rgb::from_u32(0xB7B7B7),
        Rgb::from_u32(0xB3B3B3),
        Rgb::from_u32(0xADADAD),
        Rgb::from_u32(0x535756),
        Rgb::from_u32(0x575656),
        Rgb::from_u32(0x555555),
        Rgb::from_u32(0x555555),
        Rgb::from_u32(0x545454),
        Rgb::from_u32(0x474646),
        Rgb::from_u32(0x000000),
        Rgb::from_u32(0x000000),
        Rgb::from_u32(0x0B0B0B),
        Rgb::from_u32(0x0B0B0B),
        Rgb::from_u32(0x000000),
        Rgb::from_u32(0x000000),
    ];

    #[test]
    fn test_1rando() {
        let result = QuantizerCelebi::quantize(&[Rgb::from_u32(0x141216)], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &Rgb::from_u32(0x141216));
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
