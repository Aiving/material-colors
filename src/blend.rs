use crate::{
    color::Argb,
    hct::Hct,
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
