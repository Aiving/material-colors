use crate::{IndexMap, color::Rgb};

pub trait Quantizer {
    fn quantize(pixels: &[Rgb], max_colors: usize) -> QuantizerResult;
}

pub struct QuantizerResult {
    pub color_to_count: IndexMap<Rgb, u32>,
    pub input_pixel_to_cluster_pixel: IndexMap<Rgb, Rgb>,
}
