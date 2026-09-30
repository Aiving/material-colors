use float_cmp::assert_approx_eq;
use material_colors::color::{Lab, Rgb, Xyz, delinearized, linearized, lstar_from_y, y_from_lstar};

fn range<const N: usize>(start: f64, stop: f64) -> [f64; N] {
    let step_size = (stop - start) / (N as f64 - 1.0);

    core::array::from_fn(|index| step_size.mul_add(index as f64, start))
}

fn rgb_range() -> [u8; 8] {
    range::<8>(0.0, 255.0).map(|element| element.round() as u8)
}

fn full_rgb_range() -> [u8; 256] {
    core::array::from_fn(|i| i as u8)
}

#[test]
fn test_range_integrity() {
    let range = range::<1234>(3.0, 9999.0);

    for (i, value) in range.into_iter().enumerate().take(1234) {
        assert_approx_eq!(f64, value, 8.1070559611f64.mul_add(i as f64, 3.0), epsilon = 1e-5);
    }
}

#[test]
fn test_y_to_lstar_to_y() {
    for y in range::<1001>(0.0, 100.0) {
        let result = y_from_lstar(lstar_from_y(y));

        assert_approx_eq!(f64, result, y, epsilon = 1e-5);
    }
}

#[test]
fn test_lstar_to_y_to_lstar() {
    for lstar in range::<1001>(0.0, 100.0) {
        let result = lstar_from_y(y_from_lstar(lstar));

        assert_approx_eq!(f64, result, lstar, epsilon = 1e-5);
    }
}

#[test]
fn test_yfrom_lstar() {
    assert_approx_eq!(f64, y_from_lstar(0.0), 0.0, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(0.1), 0.0110705, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(0.2), 0.0221411, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(0.3), 0.0332116, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(0.4), 0.0442822, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(0.5), 0.0553528, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(1.0), 0.1107056, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(2.0), 0.2214112, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(3.0), 0.3321169, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(4.0), 0.4428225, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(5.0), 0.5535282, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(8.0), 0.8856451, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(10.0), 1.1260199, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(15.0), 1.9085832, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(20.0), 2.9890524, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(25.0), 4.4154767, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(30.0), 6.2359055, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(40.0), 11.2509737, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(50.0), 18.4186518, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(60.0), 28.1233342, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(70.0), 40.7494157, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(80.0), 56.6812907, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(90.0), 76.3033539, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(95.0), 87.6183294, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(99.0), 97.4360239, epsilon = 1e-5);
    assert_approx_eq!(f64, y_from_lstar(100.0), 100.0, epsilon = 1e-5);
}

#[test]
fn test_lstar_from_y() {
    assert_approx_eq!(f64, lstar_from_y(0.0), 0.0, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(0.1), 0.9032962, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(0.2), 1.8065925, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(0.3), 2.7098888, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(0.4), 3.6131851, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(0.5), 4.5164814, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(0.8856451), 8.0, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(1.0), 8.9914424, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(2.0), 15.4872443, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(3.0), 20.0438970, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(4.0), 23.6714419, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(5.0), 26.7347653, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(10.0), 37.8424304, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(15.0), 45.6341970, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(20.0), 51.8372115, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(25.0), 57.0754208, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(30.0), 61.6542222, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(40.0), 69.4695307, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(50.0), 76.0692610, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(60.0), 81.8381891, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(70.0), 86.9968642, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(80.0), 91.6848609, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(90.0), 95.9967686, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(95.0), 98.0335184, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(99.0), 99.6120372, epsilon = 1e-5);
    assert_approx_eq!(f64, lstar_from_y(100.0), 100.0, epsilon = 1e-5);
}

#[test]
fn test_ycontinuity() {
    let epsilon = 1e-6;
    let delta = 1e-8;
    let left = 8.0 - delta;
    let mid = 8.0;
    let right = 8.0 + delta;

    assert_approx_eq!(f64, y_from_lstar(left), y_from_lstar(mid), epsilon = epsilon);
    assert_approx_eq!(f64, y_from_lstar(right), y_from_lstar(mid), epsilon = epsilon);
}

#[test]
fn test_rgb_to_xyz_to_rgb() {
    for r in rgb_range() {
        for g in rgb_range() {
            for b in rgb_range() {
                let rgb = Rgb::new(r, g, b);
                let xyz = Xyz::from(rgb);
                let converted = Rgb::from(xyz);

                assert_approx_eq!(f64, f64::from(converted.red), f64::from(r), epsilon = 1.5);
                assert_approx_eq!(f64, f64::from(converted.green), f64::from(g), epsilon = 1.5);
                assert_approx_eq!(f64, f64::from(converted.blue), f64::from(b), epsilon = 1.5);
            }
        }
    }
}

#[test]
fn test_rgb_to_lab_to_rgb() {
    for r in rgb_range() {
        for g in rgb_range() {
            for b in rgb_range() {
                let rgb = Rgb::new(r, g, b);
                let lab = Lab::from(rgb);
                let converted = Rgb::from(lab);

                assert_approx_eq!(f64, f64::from(converted.red), f64::from(r), epsilon = 1.5);
                assert_approx_eq!(f64, f64::from(converted.green), f64::from(g), epsilon = 1.5);
                assert_approx_eq!(f64, f64::from(converted.blue), f64::from(b), epsilon = 1.5);
            }
        }
    }
}

#[test]
fn test_rgb_to_lstar_to_rgb() {
    let full_rgb_range = full_rgb_range();

    for component in full_rgb_range {
        let rgb = Rgb::new(component, component, component);
        let lstar = rgb.as_lstar();
        let converted = Rgb::from_lstar(lstar);

        assert_eq!(converted, rgb);
    }
}

#[test]
fn test_rgb_to_lstar_to_ycommutes() {
    for r in rgb_range() {
        for g in rgb_range() {
            for b in rgb_range() {
                let rgb = Rgb::new(r, g, b);
                let lstar = rgb.as_lstar();
                let y = y_from_lstar(lstar);
                let y2 = Xyz::from(rgb).y;

                assert_approx_eq!(f64, y, y2, epsilon = 1e-5);
            }
        }
    }
}

#[test]
fn test_lstar_to_rgb_to_ycommutes() {
    for lstar in range::<1001>(0.0, 100.0) {
        let rgb = Rgb::from_lstar(lstar);
        let y = Xyz::from(rgb).y;
        let y2 = y_from_lstar(lstar);

        assert_approx_eq!(f64, y, y2, epsilon = 1.0);
    }
}

#[test]
fn test_linearize_delinearize() {
    let full_rgb_range = full_rgb_range();

    for rgb_component in full_rgb_range {
        let converted = delinearized(linearized(rgb_component));

        assert_eq!(converted, rgb_component);
    }
}
