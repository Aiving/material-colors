use crate::color::{lstar_from_y, y_from_lstar};
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;

/// Returns a contrast ratio, which ranges from 1 to 21.
///
/// - `toneA`: Tone between 0 and 100. Values outside will be clamped.
/// - `toneB`: Tone between 0 and 100. Values outside will be clamped.
pub fn ratio_of_tones(tone_a: f64, tone_b: f64) -> f64 {
    let tone_a = tone_a.clamp(0.0, 100.0);
    let tone_b = tone_b.clamp(0.0, 100.0);

    ratio_of_ys(y_from_lstar(tone_a), y_from_lstar(tone_b))
}

fn ratio_of_ys(y1: f64, y2: f64) -> f64 {
    let lighter = if y1 > y2 { y1 } else { y2 };
    let darker = if (lighter - y2).abs() < f64::EPSILON {
        y1
    } else {
        y2
    };

    (lighter + 5.0) / (darker + 5.0)
}

/// Returns a tone >= `tone` that ensures `ratio`. Return value is between 0 and 100.
/// Returns -1 if `ratio` cannot be achieved with `tone`.
///
/// - `tone`: Tone return value must contrast with. Range is 0 to 100. Invalid values will result in -1 being returned.
/// - `ratio`: Contrast ratio of return value and `tone`. Range is 1 to 21, invalid values have undefined behavior.
pub fn lighter(tone: f64, ratio: f64) -> f64 {
    if !(0.0..=100.0).contains(&tone) {
        return -1.0;
    }

    let dark_y = y_from_lstar(tone);
    let light_y = ratio.mul_add(dark_y + 5.0, -5.0);
    let real_contrast = ratio_of_ys(light_y, dark_y);
    let delta = (real_contrast - ratio).abs();

    if real_contrast < ratio && delta > 0.04 {
        return -1.0;
    }

    // Ensure gamut mapping, which requires a 'range' on tone, will still result
    // the correct ratio by darkening slightly.
    let return_value = lstar_from_y(light_y) + 0.4;

    if !(0.0..=100.0).contains(&return_value) {
        return -1.0;
    }

    return_value
}

/// Returns a tone <= `tone` that ensures `ratio`. Return value is between 0 and 100.
/// Returns -1 if `ratio` cannot be achieved with `tone`.
///
/// - `tone`: Tone return value must contrast with. Range is 0 to 100. Invalid values will result in -1 being returned.
/// - `ratio`: Contrast ratio of return value and `tone`. Range is 1 to 21, invalid values have undefined behavior.
pub fn darker(tone: f64, ratio: f64) -> f64 {
    if !(0.0..=100.0).contains(&tone) {
        return -1.0;
    }

    let light_y = y_from_lstar(tone);
    let dark_y = ((light_y + 5.0) / ratio) - 5.0;
    let real_contrast = ratio_of_ys(light_y, dark_y);

    let delta = (real_contrast - ratio).abs();

    if real_contrast < ratio && delta > 0.04 {
        return -1.0;
    }

    // Ensure gamut mapping, which requires a 'range' on tone, will still result
    // the correct ratio by darkening slightly.
    let return_value = lstar_from_y(dark_y) - 0.4;

    if !(0.0..=100.0).contains(&return_value) {
        return -1.0;
    }

    return_value
}

/// Returns a tone >= `tone` that ensures `ratio`. Return value is between 0 and 100.
/// Returns 100 if `ratio` cannot be achieved with `tone`.
///
/// This method is unsafe because the returned value is guaranteed to be in bounds for tone, i.e. between 0 and 100.
/// However, that value may not reach the `ratio` with `tone`. For example, there is no color lighter than T100.
///
/// - `tone`: Tone return value must contrast with. Range is 0 to 100. Invalid values will result in 100 being returned.
/// - `ratio`: Desired contrast ratio of return value and tone parameter. Range is 1 to 21, invalid values have undefined behavior.
pub fn lighter_unsafe(tone: f64, ratio: f64) -> f64 {
    let lighter_safe = lighter(tone, ratio);

    if lighter_safe < 0.0 {
        100.0
    } else {
        lighter_safe
    }
}

/// Returns a tone <= `tone` that ensures `ratio`. Return value is between 0 and 100.
/// Returns 0 if `ratio` cannot be achieved with `tone`.
///
/// This method is unsafe because the returned value is guaranteed to be in bounds for tone, i.e. between 0 and 100.
/// However, that value may not reach the `ratio` with `tone`. For example, there is no color darker than T0.
///
/// - `tone`: Tone return value must contrast with. Range is 0 to 100. Invalid values will result in 0 being returned.
/// - `ratio`: Desired contrast ratio of return value and tone parameter. Range is 1 to 21, invalid values have undefined behavior.
pub fn darker_unsafe(tone: f64, ratio: f64) -> f64 {
    let darker_safe = darker(tone, ratio);

    if darker_safe < 0.0 {
        0.0
    } else {
        darker_safe
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::contrast::ratio_of_tones;

    use super::{darker, darker_unsafe, lighter, lighter_unsafe};

    #[test]
    fn test_ratio_of_tones_out_of_bounds_input() {
        assert_approx_eq!(f64, 21.0, ratio_of_tones(-10.0, 110.0), epsilon = 0.001);
    }

    #[test]
    fn test_lighter_impossible_ratio_errors() {
        assert_approx_eq!(f64, -1.0, lighter(90.0, 10.0), epsilon = 0.001);
    }

    #[test]
    fn test_lighter_out_of_bounds_input_above_errors() {
        assert_approx_eq!(f64, -1.0, lighter(110.0, 2.0), epsilon = 0.001);
    }

    #[test]
    fn test_lighter_out_of_bounds_input_below_errors() {
        assert_approx_eq!(f64, -1.0, lighter(-10.0, 2.0), epsilon = 0.001);
    }

    #[test]
    fn test_lighter_unsafe_returns_max_tone() {
        assert_approx_eq!(f64, 100.0, lighter_unsafe(100.0, 2.0), epsilon = 0.001);
    }

    #[test]
    fn test_darker_impossible_ratio_errors() {
        assert_approx_eq!(f64, -1.0, darker(10.0, 20.0), epsilon = 0.001);
    }

    #[test]
    fn test_darker_out_of_bounds_input_above_errors() {
        assert_approx_eq!(f64, -1.0, darker(110.0, 2.0), epsilon = 0.001);
    }

    #[test]
    fn test_darker_out_of_bounds_input_below_errors() {
        assert_approx_eq!(f64, -1.0, darker(-10.0, 2.0), epsilon = 0.001);
    }

    #[test]
    fn test_darker_unsafe_returns_min_tone() {
        assert_approx_eq!(f64, 0.0, darker_unsafe(0.0, 2.0), epsilon = 0.001);
    }
}
