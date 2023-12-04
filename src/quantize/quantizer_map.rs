use ahash::HashMap;

use crate::utils::color::alpha_from_argb;
use crate::utils::color::Argb;

use super::quantizer::Quantizer;
use super::quantizer::QuantizerResult;

#[derive(Default)]
pub(crate) struct QuantizerMap;

impl Quantizer for QuantizerMap {
    fn quantize(
        &mut self,
        pixels: &[Argb],
        _max_colors: i32,
        _return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult {
        let mut count_by_color: HashMap<Argb, u32> = Default::default();

        for pixel in pixels {
            let alpha = alpha_from_argb(*pixel);

            if alpha < 255 {
                continue;
            }

            count_by_color
                .entry(*pixel)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        QuantizerResult {
            color_to_count: count_by_color,
            input_pixel_to_cluster_pixel: Default::default(),
        }
    }
}
