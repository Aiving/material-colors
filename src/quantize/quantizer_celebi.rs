use crate::utils::color::Argb;

use super::point_provider::PointProvider;
use super::point_provider_lab::PointProviderLab;
use super::quantizer::Quantizer;
use super::quantizer::QuantizerResult;
use super::quantizer_wsmeans::QuantizerWsmeans;
use super::quantizer_wu::QuantizerWu;

#[derive(Default)]
pub struct QuantizerCelebi;

impl Quantizer for QuantizerCelebi {
    fn quantize(
        &mut self,
        pixels: &[Argb],
        max_colors: i32,
        return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult {
        let wu_result = QuantizerWu::default().quantize(pixels, max_colors, None);

        QuantizerWsmeans::quantize(
            pixels,
            max_colors,
            Some(wu_result.color_to_count.into_keys().collect()),
            Some(PointProviderLab::new()),
            None,
            return_input_pixel_to_cluster_pixel,
        )
    }
}
