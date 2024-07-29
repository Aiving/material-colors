use super::{Quantizer, QuantizerResult, QuantizerWsmeans, QuantizerWu};
use crate::color::Argb;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

#[derive(Default)]
pub struct QuantizerCelebi;

impl Quantizer for QuantizerCelebi {
    fn quantize(pixels: &[Argb], max_colors: usize) -> QuantizerResult {
        let wu_result = QuantizerWu::quantize(pixels, max_colors);

        QuantizerWsmeans::quantize(
            pixels,
            max_colors,
            &wu_result.color_to_count.into_keys().collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::QuantizerCelebi;
    use crate::{color::Argb, quantize::Quantizer};
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    #[cfg(feature = "std")]
    use std::vec::Vec;

    const RED: Argb = Argb::from_u32(0xffff0000);
    const GREEN: Argb = Argb::from_u32(0xff00ff00);
    const BLUE: Argb = Argb::from_u32(0xff0000ff);
    // const WHITE: Argb = Argb::from_u32(0xffffffff);
    // const RANDOM: Argb = Argb::from_u32(0xff426088);
    const MAX_COLORS: usize = 256;

    const IMAGE_PIXELS: [Argb; 84] = [
        Argb::from_u32(0xff050505),
        Argb::from_u32(0xff000000),
        Argb::from_u32(0xff000000),
        Argb::from_u32(0xff000000),
        Argb::from_u32(0xff000000),
        Argb::from_u32(0xff090909),
        Argb::from_u32(0xff060404),
        Argb::from_u32(0xff030102),
        Argb::from_u32(0xff080607),
        Argb::from_u32(0xff070506),
        Argb::from_u32(0xff010001),
        Argb::from_u32(0xff070506),
        Argb::from_u32(0xff364341),
        Argb::from_u32(0xff223529),
        Argb::from_u32(0xff14251c),
        Argb::from_u32(0xff11221a),
        Argb::from_u32(0xff1f3020),
        Argb::from_u32(0xff34443a),
        Argb::from_u32(0xff64817e),
        Argb::from_u32(0xff638777),
        Argb::from_u32(0xff486d58),
        Argb::from_u32(0xff2f5536),
        Argb::from_u32(0xff467258),
        Argb::from_u32(0xff7fb7b9),
        Argb::from_u32(0xff6d8473),
        Argb::from_u32(0xff859488),
        Argb::from_u32(0xff7a947e),
        Argb::from_u32(0xff5f815d),
        Argb::from_u32(0xff3a5d46),
        Argb::from_u32(0xff497469),
        Argb::from_u32(0xff737a73),
        Argb::from_u32(0xff656453),
        Argb::from_u32(0xff445938),
        Argb::from_u32(0xff657c4b),
        Argb::from_u32(0xff65715b),
        Argb::from_u32(0xff6a816e),
        Argb::from_u32(0xff667366),
        Argb::from_u32(0xff5b5547),
        Argb::from_u32(0xff3b391e),
        Argb::from_u32(0xff705e3d),
        Argb::from_u32(0xff7f6c5e),
        Argb::from_u32(0xff6d7c6c),
        Argb::from_u32(0xffa99c9c),
        Argb::from_u32(0xff8b7671),
        Argb::from_u32(0xff6a3229),
        Argb::from_u32(0xff80514b),
        Argb::from_u32(0xff857970),
        Argb::from_u32(0xff4f5a4c),
        Argb::from_u32(0xff897273),
        Argb::from_u32(0xff745451),
        Argb::from_u32(0xff512823),
        Argb::from_u32(0xff78585a),
        Argb::from_u32(0xff535552),
        Argb::from_u32(0xff40493f),
        Argb::from_u32(0xff151616),
        Argb::from_u32(0xff0a0c0c),
        Argb::from_u32(0xff050808),
        Argb::from_u32(0xff010303),
        Argb::from_u32(0xff000100),
        Argb::from_u32(0xff010200),
        Argb::from_u32(0xff191816),
        Argb::from_u32(0xff181818),
        Argb::from_u32(0xff0c0c0c),
        Argb::from_u32(0xff040404),
        Argb::from_u32(0xff0c0c0c),
        Argb::from_u32(0xff151514),
        Argb::from_u32(0xffb1c3b9),
        Argb::from_u32(0xffbfbfbf),
        Argb::from_u32(0xffbababa),
        Argb::from_u32(0xffb7b7b7),
        Argb::from_u32(0xffb3b3b3),
        Argb::from_u32(0xffadadad),
        Argb::from_u32(0xff535756),
        Argb::from_u32(0xff575656),
        Argb::from_u32(0xff555555),
        Argb::from_u32(0xff555555),
        Argb::from_u32(0xff545454),
        Argb::from_u32(0xff474646),
        Argb::from_u32(0xff000000),
        Argb::from_u32(0xff000000),
        Argb::from_u32(0xff0b0b0b),
        Argb::from_u32(0xff0b0b0b),
        Argb::from_u32(0xff000000),
        Argb::from_u32(0xff000000),
    ];

    #[test]
    fn test_1rando() {
        let result = QuantizerCelebi::quantize(&[Argb::from_u32(0xff141216)], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &Argb::from_u32(0xff141216));
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

    /// Verifies QuantizerCelebi.quantize returns identical result given same input.
    #[test]
    fn test_stability() {
        let result1 = QuantizerCelebi::quantize(&IMAGE_PIXELS, 16).color_to_count;
        let result2 = QuantizerCelebi::quantize(&IMAGE_PIXELS, 16).color_to_count;

        assert_eq!(result1, result2);
    }
}
