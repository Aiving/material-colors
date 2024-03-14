use crate::{color::Lab, Argb};

use super::PointProvider;

pub struct PointProviderLab;

impl PointProvider for PointProviderLab {
    fn lab_from_int(&self, argb: &Argb) -> Lab {
        (*argb).into()
    }

    fn lab_to_int(&self, lab: &Lab) -> Argb {
        (*lab).into()
    }

    fn distance(&self, one: &Lab, two: &Lab) -> f64 {
        // Standard CIE 1976 delta E formula also takes the square root, unneeded
        // here. This method is used by quantization algorithms to compare distance,
        // and the relative ordering is the same, with or without a square root.

        // This relatively minor optimization is helpful because this method is
        // called at least once for each pixel in an image.
        (one.l - two.l).powi(2) + (one.a - two.a).powi(2) + (one.b - two.b).powi(2)
    }

    fn new() -> Self {
        Self
    }
}
