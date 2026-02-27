use super::{Quantizer, QuantizerResult};
use crate::{IndexMap, color::Rgb};

#[derive(Default)]
pub struct QuantizerMap;

impl Quantizer for QuantizerMap {
    fn quantize(pixels: &[Rgb], _max_colors: usize) -> QuantizerResult {
        let mut color_to_count = IndexMap::<Rgb, u32>::default();

        for pixel in pixels {
            color_to_count
                .entry(*pixel)
                .and_modify(|current_pixel_count| *current_pixel_count += 1)
                .or_insert(1);
        }

        QuantizerResult {
            color_to_count,
            input_pixel_to_cluster_pixel: IndexMap::default(),
        }
    }
}
