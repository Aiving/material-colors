use float_cmp::assert_approx_eq;
use material_colors::contrast::{darker, darker_unsafe, lighter, lighter_unsafe, ratio_of_tones};

#[test]
fn test_ratio_of_tones_out_of_bounds_input() {
    assert_approx_eq!(f64, 21.0, ratio_of_tones(-10.0, 110.0), epsilon = 0.001);
}

#[test]
fn test_lighter_impossible_ratio_errors() {
    assert!(lighter(90.0, 10.0).is_none());
}

#[test]
fn test_lighter_out_of_bounds_input_above_errors() {
    assert!(lighter(110.0, 2.0).is_none());
}

#[test]
fn test_lighter_out_of_bounds_input_below_errors() {
    assert!(lighter(-10.0, 2.0).is_none());
}

#[test]
fn test_lighter_unsafe_returns_max_tone() {
    assert_approx_eq!(f64, 100.0, lighter_unsafe(100.0, 2.0), epsilon = 0.001);
}

#[test]
fn test_darker_impossible_ratio_errors() {
    assert!(darker(10.0, 20.0).is_none());
}

#[test]
fn test_darker_out_of_bounds_input_above_errors() {
    assert!(darker(110.0, 2.0).is_none());
}

#[test]
fn test_darker_out_of_bounds_input_below_errors() {
    assert!(darker(-10.0, 2.0).is_none());
}

#[test]
fn test_darker_unsafe_returns_min_tone() {
    assert_approx_eq!(f64, 0.0, darker_unsafe(0.0, 2.0), epsilon = 0.001);
}
