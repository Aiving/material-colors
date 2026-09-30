use material_colors::{blend::hct_hue, color::Rgb};

#[test]
fn test_red_to_blue() {
    let blended = hct_hue(Rgb::from_u32(0xFF0000), Rgb::from_u32(0x0000FF), 0.8);

    assert_eq!(blended.as_u32(), 0x905EFF);
}
