use crate::color::{Lab, Rgb};

pub trait PointProvider {
    fn lab_from_int(rgb: &Rgb) -> Lab;
    fn lab_to_int(lab: &Lab) -> Rgb;
    fn distance(one: &Lab, two: &Lab) -> f64;
}
