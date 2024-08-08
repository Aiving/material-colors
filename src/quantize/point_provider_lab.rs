use super::PointProvider;
use crate::color::{Argb, Lab};
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;

pub struct PointProviderLab;

impl PointProvider for PointProviderLab {
    fn lab_from_int(argb: &Argb) -> Lab {
        (*argb).into()
    }

    fn lab_to_int(lab: &Lab) -> Argb {
        (*lab).into()
    }

    fn distance(one: &Lab, two: &Lab) -> f64 {
        // Standard CIE 1976 delta E formula also takes the square root, unneeded
        // here. This method is used by quantization algorithms to compare distance,
        // and the relative ordering is the same, with or without a square root.

        // This relatively minor optimization is helpful because this method is
        // called at least once for each pixel in an image.
        (one.b - two.b).mul_add(
            one.b - two.b,
            (one.l - two.l).mul_add(one.l - two.l, (one.a - two.a).powi(2)),
        )
    }
}
