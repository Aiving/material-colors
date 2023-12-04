use crate::utils::color::Argb;
use crate::utils::color::Lab;

pub(crate) trait PointProvider {
    fn new() -> Self
    where
        Self: Sized;
    fn lab_from_int(&self, argb: Argb) -> Lab;
    fn lab_to_int(&self, lab: Lab) -> Argb;
    fn distance(&self, one: Lab, two: Lab) -> f64;
}
