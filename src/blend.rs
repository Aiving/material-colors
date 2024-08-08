#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::Argb,
    hct::{Cam16, Hct},
    utils::math::{difference_degrees, rotate_direction, sanitize_degrees_double},
};

pub fn harmonize(design_color: Argb, source_color: Argb) -> Argb {
    let from_hct: Hct = design_color.into();
    let to_hct: Hct = source_color.into();

    let difference_degrees = difference_degrees(from_hct.get_hue(), to_hct.get_hue());
    let rotation_degrees = (difference_degrees * 0.5).min(15.0);

    let output_hue = sanitize_degrees_double(rotation_degrees.mul_add(
        rotate_direction(from_hct.get_hue(), to_hct.get_hue()),
        from_hct.get_hue(),
    ));

    Hct::from(output_hue, from_hct.get_chroma(), from_hct.get_tone()).into()
}

pub fn hct_hue(from: Argb, to: Argb, amount: f64) -> Argb {
    let ucs = cam16_ucs(from, to, amount);

    let ucs_cam = Cam16::from(ucs);
    let from_cam = Cam16::from(from);

    let blended = Hct::from(ucs_cam.hue, from_cam.chroma, from.as_lstar());

    blended.into()
}

pub fn cam16_ucs(from: Argb, to: Argb, amount: f64) -> Argb {
    let from_cam = Cam16::from(from);
    let to_cam = Cam16::from(to);

    let from_j = from_cam.jstar;
    let from_a = from_cam.astar;
    let from_b = from_cam.bstar;

    let to_j = to_cam.jstar;
    let to_a = to_cam.astar;
    let to_b = to_cam.bstar;

    let (jstar, astar, bstar) = (
        (to_j - from_j).mul_add(amount, from_j),
        (to_a - from_a).mul_add(amount, from_a),
        (to_b - from_b).mul_add(amount, from_b),
    );

    Cam16::from_ucs(jstar, astar, bstar).into()
}

#[cfg(test)]
mod tests {
    use super::hct_hue;
    use crate::color::Argb;
    use core::str::FromStr;

    #[test]
    fn test_red_to_blue() {
        let blended = hct_hue(
            Argb::from_str("ff0000").unwrap(),
            Argb::from_str("0000ff").unwrap(),
            0.8,
        );

        assert_eq!(blended.to_hex(), "905eff");
    }
}
