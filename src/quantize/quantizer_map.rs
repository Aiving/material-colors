use indexmap::IndexMap;

use crate::utils::color::Argb;

use super::quantizer::Quantizer;
use super::quantizer::QuantizerResult;

#[derive(Default)]
pub struct QuantizerMap;

impl Quantizer for QuantizerMap {
    fn quantize(
        &mut self,
        pixels: &[Argb],
        _max_colors: i32,
        _return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult {
        let mut color_to_count: IndexMap<Argb, u32> = Default::default();

        for pixel in pixels {
            let current_pixel_count = color_to_count.get_mut(pixel);

            if let Some(current_pixel_count) = current_pixel_count {
                *current_pixel_count += 1;
            } else {
                color_to_count.insert(*pixel, 1);
            }
        }

        QuantizerResult {
            color_to_count,
            input_pixel_to_cluster_pixel: Default::default(),
        }
    }
}
