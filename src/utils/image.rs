use crate::quantize::quantizer::Quantizer;
use crate::quantize::quantizer_celebi::QuantizerCelebi;
use crate::score::Score;

use super::color::Argb;

/// Get the source color from an image.
///
/// @param image The image element
///
/// @return Source color - the color most suitable for creating a UI theme
pub fn source_color_from_image(image: &[Argb]) -> Argb {
    let result = QuantizerCelebi.quantize(image, 128, None);
    let ranked = Score::score(&result.color_to_count, None, None, None);

    ranked[0]
}
