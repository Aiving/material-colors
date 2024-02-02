use crate::utils::color::argb_from_lab;
use crate::utils::color::lab_from_argb;
use crate::utils::color::Argb;
use crate::utils::color::Lab;

use super::point_provider::PointProvider;

pub struct PointProviderLab;

impl PointProvider for PointProviderLab {
    fn lab_from_int(&self, argb: &Argb) -> Lab {
        lab_from_argb(argb)
    }

    fn lab_to_int(&self, lab: &Lab) -> Argb {
        argb_from_lab(lab)
    }

    fn distance(&self, one: &Lab, two: &Lab) -> f64 {
        let d_l = one[0] - two[0];
        let d_a = one[1] - two[1];
        let d_b = one[2] - two[2];

        // Standard CIE 1976 delta E formula also takes the square root, unneeded
        // here. This method is used by quantization algorithms to compare distance,
        // and the relative ordering is the same, with or without a square root.

        // This relatively minor optimization is helpful because this method is
        // called at least once for each pixel in an image.
        d_l.powi(2) + d_a.powi(2) + d_b.powi(2)
    }

    fn new() -> Self {
        Self
    }
}
