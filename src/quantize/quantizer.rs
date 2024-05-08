use indexmap::IndexMap;

use crate::color::Argb;

pub trait Quantizer {
    fn quantize(
        &mut self,
        pixels: &[Argb],
        max_colors: usize,
        return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult;
}

pub struct QuantizerResult {
    pub color_to_count: IndexMap<Argb, u32>,
    pub input_pixel_to_cluster_pixel: IndexMap<Argb, Argb>,
}
