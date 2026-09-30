use float_cmp::assert_approx_eq;
use material_colors::hct::ViewingConditions;

#[test]
fn test_viewing_conditions() {
    let result1 = ViewingConditions::make(None, None, None, None, None);

    assert_approx_eq!(f64, result1.adapting_luminance, 11.725677948856951);
    assert_approx_eq!(f64, result1.background_lstar, 50.0);
    assert_approx_eq!(f64, result1.surround, 2.0);

    assert!(!result1.discounting_illuminant);
}
