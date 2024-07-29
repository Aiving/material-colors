use crate::{color::Argb, IndexMap};

pub trait Quantizer {
    fn quantize(pixels: &[Argb], max_colors: usize) -> QuantizerResult;
}

pub struct QuantizerResult {
    pub color_to_count: IndexMap<Argb, u32>,
    pub input_pixel_to_cluster_pixel: IndexMap<Argb, Argb>,
}
