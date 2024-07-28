use crate::color::{Argb, Lab};

pub trait PointProvider {
    fn lab_from_int(argb: &Argb) -> Lab;
    fn lab_to_int(lab: &Lab) -> Argb;
    fn distance(one: &Lab, two: &Lab) -> f64;
}
