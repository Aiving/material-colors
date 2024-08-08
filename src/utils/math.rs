#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;

pub fn signum(value: f64) -> f64 {
    if value < 0.0 {
        -1.0
    } else if value == 0.0 {
        0.0
    } else {
        1.0
    }
}

pub fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount).mul_add(start, amount * stop)
}

pub const fn sanitize_degrees_int(degrees: i32) -> u32 {
    match degrees {
        value if value < 0 => (value + 360) as u32,
        value => value as u32 % 360,
    }
}

pub fn sanitize_degrees_double(degrees: f64) -> f64 {
    match degrees {
        value if value < 0.0 => value + 360.0,
        value => value % 360.0,
    }
}

pub fn rotate_direction(from: f64, to: f64) -> f64 {
    let increasing_difference = sanitize_degrees_double(to - from);

    if increasing_difference <= 180.0 {
        1.0
    } else {
        -1.0
    }
}

pub fn difference_degrees(a: f64, b: f64) -> f64 {
    180.0 - ((a - b).abs() - 180.0).abs()
}

pub fn matrix_multiply(row: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    [
        row[2].mul_add(
            matrix[0][2],
            row[0].mul_add(matrix[0][0], row[1] * matrix[0][1]),
        ),
        row[2].mul_add(
            matrix[1][2],
            row[0].mul_add(matrix[1][0], row[1] * matrix[1][1]),
        ),
        row[2].mul_add(
            matrix[2][2],
            row[0].mul_add(matrix[2][0], row[1] * matrix[2][1]),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::utils::math::{lerp, matrix_multiply, signum};

    use super::{
        difference_degrees, rotate_direction, sanitize_degrees_double, sanitize_degrees_int,
    };

    #[test]
    fn test_signum() {
        let result1 = signum(-2.0);
        let result2 = signum(0.0);
        let result3 = signum(2.0);

        assert_approx_eq!(f64, result1, -1.0);
        assert_approx_eq!(f64, result2, 0.0);
        assert_approx_eq!(f64, result3, 1.0);
    }

    #[test]
    fn test_lerp() {
        let result1 = lerp(0.0, 1.0, 0.5);
        let result2 = lerp(0.0, 1.0, 0.2);
        let result3 = lerp(0.0, 1.0, 0.8);
        let result4 = lerp(0.0, 100.0, 0.5);

        assert_approx_eq!(f64, result1, 0.5);
        assert_approx_eq!(f64, result2, 0.2);
        assert_approx_eq!(f64, result3, 0.8);
        assert_approx_eq!(f64, result4, 50.0);
    }

    #[test]
    fn test_sanitize_degrees_int() {
        let result1 = sanitize_degrees_int(20);
        let result2 = sanitize_degrees_int(360);
        let result3 = sanitize_degrees_int(420);

        assert_eq!(result1, 20);
        assert_eq!(result2, 0);
        assert_eq!(result3, 60);
    }

    #[test]
    fn test_sanitize_degrees_double() {
        let result1 = sanitize_degrees_double(20.0);
        let result2 = sanitize_degrees_double(360.0);
        let result3 = sanitize_degrees_double(420.0);

        assert_approx_eq!(f64, result1, 20.0);
        assert_approx_eq!(f64, result2, 0.0);
        assert_approx_eq!(f64, result3, 60.0);
    }

    #[test]
    fn test_rotation_direction() {
        let mut from = 0.0;

        while from < 360.0 {
            let mut to = 7.5;

            while to < 360.0 {
                let expected_answer = _rotate_direction(from, to);
                let actual_answer = rotate_direction(from, to);

                assert_approx_eq!(f64, actual_answer, expected_answer);
                assert_approx_eq!(f64, actual_answer.abs(), 1.0);

                to += 15.0;
            }

            from += 15.0;
        }
    }

    #[test]
    fn test_difference_degrees() {
        let result0 = difference_degrees(274.6219605304299, 77.72547409861208);
        let result1 = difference_degrees(34.300891775453685, 345.71661212068005);
        let result2 = difference_degrees(133.40137028102632, 242.91571888782715);
        let result3 = difference_degrees(190.0917641906071, 212.50303144714914);
        let result4 = difference_degrees(74.32650812140363, 8.216795031646305);
        let result5 = difference_degrees(297.86357401167163, 40.68646508824719);
        let result6 = difference_degrees(267.81615364209966, 278.2059089562928);
        let result7 = difference_degrees(174.5904980548994, 23.563620558167703);
        let result8 = difference_degrees(157.88312470562747, 136.9620280065816);
        let result9 = difference_degrees(125.57678704737181, 232.55663167025088);
        let result10 = difference_degrees(29.516420072682976, 282.618422595264);

        assert_approx_eq!(f64, result0, 163.10351356818222);
        assert_approx_eq!(f64, result1, 48.58427965477364);
        assert_approx_eq!(f64, result2, 109.51434860680084);
        assert_approx_eq!(f64, result3, 22.411267256542033);
        assert_approx_eq!(f64, result4, 66.10971308975734);
        assert_approx_eq!(f64, result5, 102.82289107657556);
        assert_approx_eq!(f64, result6, 10.38975531419311);
        assert_approx_eq!(f64, result7, 151.0268774967317);
        assert_approx_eq!(f64, result8, 20.921096699045876);
        assert_approx_eq!(f64, result9, 106.97984462287907);
        assert_approx_eq!(f64, result10, 106.89799747741898);
    }

    #[test]
    fn test_matrix_multiply() {
        let result1 = matrix_multiply(
            [0.0, 1.0, 2.0],
            [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [4.0, 5.0, 6.0]],
        );
        let result2 = matrix_multiply(
            [3.0, 4.0, 5.0],
            [[3.0, 7.0, 1.0], [5.0, 8.0, 2.0], [6.0, 9.0, 3.0]],
        );

        assert_approx_eq!(&[f64], &result1, &[8.0, 11.0, 17.0]);
        assert_approx_eq!(&[f64], &result2, &[42.0, 57.0, 69.0]);
    }

    // Original implementation for MathUtils.rotateDirection.
    // Included here to test equivalence with new implementation.
    fn _rotate_direction(from: f64, to: f64) -> f64 {
        let a = to - from;
        let b = to - from + 360.0;
        let c = to - from - 360.0;

        let (a_abs, b_abs, c_abs) = (a.abs(), b.abs(), c.abs());

        if a_abs <= b_abs && a_abs <= c_abs {
            if a >= 0.0 {
                1.0
            } else {
                -1.0
            }
        } else if b_abs <= a_abs && b_abs <= c_abs {
            if b >= 0.0 {
                1.0
            } else {
                -1.0
            }
        } else if c >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }
}
