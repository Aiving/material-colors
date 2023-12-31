use crate::quantize::quantizer::Quantizer;
use crate::quantize::quantizer_celebi::QuantizerCelebi;
use crate::score::Score;
use crate::utils::color::argb_from_rgb;

use super::color::Argb;

/// Get the source color from an image.
///
/// @param image The image element
///
/// @return Source color - the color most suitable for creating a UI theme
pub fn source_color_from_image(image: &[u8]) -> Argb {
    // Convert Image data to Pixel Array
    let mut pixels = vec![];

    for i in (0..=image.len()).step_by(4) {
        let [r, g, b, a] = [image[i], image[i + 1], image[i + 2], image[i + 3]];

        if a < 255 {
            continue;
        }

        pixels.push(argb_from_rgb([r, g, b]));
    }

    // Convert Pixels to Material Colors
    let result = QuantizerCelebi.quantize(&pixels, 128, None);
    let ranked = Score::score(&result.color_to_count, None, None, None);

    ranked[0]
}
