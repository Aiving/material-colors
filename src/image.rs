#[cfg(feature = "quantize")]
use crate::{
    color::Rgb,
    quantize::{Quantizer, QuantizerCelebi},
    score::Score,
};

/// Get the source color from an image.
///
/// `image` A struct that implements the [`AsPixels`] trait
///
/// Returns source color - the color most suitable for creating a UI theme
#[cfg(feature = "quantize")]
pub fn extract_color(pixels: &[Rgb]) -> Rgb {
    let result = QuantizerCelebi::quantize(pixels, 128);
    let ranked = Score::score(&result.color_to_count, None, None, None);

    ranked[0]
}
