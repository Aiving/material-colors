use ahash::HashMap;

use crate::utils::color::Argb;

pub(crate) trait Quantizer {
    fn quantize(
        &mut self,
        pixels: &[Argb],
        max_colors: i32,
        return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult;
}

pub(crate) struct QuantizerResult {
    pub(crate) color_to_count: HashMap<Argb, u32>,
    pub(crate) input_pixel_to_cluster_pixel: HashMap<Argb, Argb>,
}
