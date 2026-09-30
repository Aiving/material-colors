use float_cmp::assert_approx_eq;
use material_colors::{dynamic_color::DynamicScheme, hct::Hct};

#[test]
fn test_0_length_input() {
    let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[], &[]);

    assert_approx_eq!(f64, hue, 43.0, epsilon = 1.0);
}

#[test]
fn test_1_length_input_no_rotation() {
    let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[0.0], &[0.0]);

    assert_approx_eq!(f64, hue, 43.0, epsilon = 1.0);
}

#[test]
fn test_on_boundary_rotation_correct() {
    let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[0.0, 42.0, 360.0], &[0.0, 15.0, 0.0]);

    assert_approx_eq!(f64, hue, 43.0 + 15.0, epsilon = 1.0);
}

#[test]
fn test_rotation_result_larger_than_360_degrees_wraps() {
    let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[0.0, 42.0, 360.0], &[0.0, 480.0, 0.0]);

    assert_approx_eq!(f64, hue, 163.0, epsilon = 1.0);
}

/// Regression for the old `min(breakpoints, values - 1)` bound: a hue in
/// the last segment of a 9-breakpoint / 8-value table must be rotated.
#[test]
fn test_last_segment_is_rotated() {
    let breakpoints = [0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0];
    let rotations = [-160.0, 155.0, -100.0, 96.0, -96.0, -156.0, -165.0, -160.0];

    assert_approx_eq!(f64, DynamicScheme::get_rotated_hue(340.0, &breakpoints, &rotations), 180.0, epsilon = 1e-9);
}
