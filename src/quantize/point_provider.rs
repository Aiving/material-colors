use crate::color::{Rgb, Lab};

pub trait PointProvider {
    fn lab_from_int(argb: &Rgb) -> Lab;
    fn lab_to_int(lab: &Lab) -> Rgb;
    fn distance(one: &Lab, two: &Lab) -> f64;
}
