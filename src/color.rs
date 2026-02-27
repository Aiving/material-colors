#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use core::{fmt, str::FromStr};
#[cfg(feature = "std")] use std::{format, string::String};

#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{Error, utils::math::matrix_multiply};

pub const SRGB_TO_XYZ: [[f64; 3]; 3] = [[0.41233895, 0.35762064, 0.18051042], [0.2126, 0.7152, 0.0722], [
    0.01932141, 0.11916382, 0.95034478,
]];
pub const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [3.2413774792388685, -1.5376652402851851, -0.49885366846268053],
    [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
    [0.05562093689691305, -0.20395524564742123, 1.0571799111220335],
];
pub const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

/// RGB representation of color. Can be created using [`Rgb::new`],
/// [`Rgb::from_u32`] or [`Rgb::from_str`].
///
/// ## Examples:
/// ```rust
/// use std::str::FromStr;
///
/// use material_colors::color::Rgb;
///
/// // from_str can accept any valid HEX color
/// let color = Rgb::from_str("abc").unwrap();
/// let color = Rgb::from_str("aabbcc").unwrap();
/// let color = Rgb::from_str("#abc").unwrap();
/// let color = Rgb::from_str("#aabbcc").unwrap();
/// ```
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearRgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

/// Converts a color from linear Rgb components to Rgb format.
impl From<LinearRgb> for Rgb {
    fn from(linear: LinearRgb) -> Self {
        let r = delinearized(linear.red);
        let g = delinearized(linear.green);
        let b = delinearized(linear.blue);

        Self::new(r, g, b)
    }
}

/// Converts a color from Rgb to Xyz.
impl From<Xyz> for Rgb {
    fn from(value: Xyz) -> Self {
        let matrix = XYZ_TO_SRGB;

        let (linear_r, linear_g, linear_b) = (
            matrix[0][2].mul_add(value.z, matrix[0][0].mul_add(value.x, matrix[0][1] * value.y)),
            matrix[1][2].mul_add(value.z, matrix[1][0].mul_add(value.x, matrix[1][1] * value.y)),
            matrix[2][2].mul_add(value.z, matrix[2][0].mul_add(value.x, matrix[2][1] * value.y)),
        );

        let r = delinearized(linear_r);
        let g = delinearized(linear_g);
        let b = delinearized(linear_b);

        Self::new(r, g, b)
    }
}

/// Converts a color from Xyz to Rgb.
impl From<Rgb> for Xyz {
    fn from(value: Rgb) -> Self {
        let r = linearized(value.red);
        let g = linearized(value.green);
        let b = linearized(value.blue);

        let [x, y, z] = matrix_multiply([r, g, b], SRGB_TO_XYZ);

        Self { x, y, z }
    }
}

/// Converts a color represented in Lab color space into an Rgb integer.
impl From<Lab> for Rgb {
    fn from(value: Lab) -> Self {
        let white_point = WHITE_POINT_D65;

        let fy = (value.l + 16.0) / 116.0;
        let fx = value.a / 500.0 + fy;
        let fz = fy - value.b / 200.0;

        let x_normalized = lab_invf(fx);
        let y_normalized = lab_invf(fy);
        let z_normalized = lab_invf(fz);

        let x = x_normalized * white_point[0];
        let y = y_normalized * white_point[1];
        let z = z_normalized * white_point[2];

        Xyz::new(x, y, z).into()
    }
}

impl From<Rgb> for Lab {
    fn from(value: Rgb) -> Self {
        let linear_r = linearized(value.red);
        let linear_g = linearized(value.green);
        let linear_b = linearized(value.blue);

        let matrix = SRGB_TO_XYZ;

        let (x, y, z) = (
            matrix[0][2].mul_add(linear_b, matrix[0][0].mul_add(linear_r, matrix[0][1] * linear_g)),
            matrix[1][2].mul_add(linear_b, matrix[1][0].mul_add(linear_r, matrix[1][1] * linear_g)),
            matrix[2][2].mul_add(linear_b, matrix[2][0].mul_add(linear_r, matrix[2][1] * linear_g)),
        );

        let white_point = WHITE_POINT_D65;

        let x_normalized = x / white_point[0];
        let y_normalized = y / white_point[1];
        let z_normalized = z / white_point[2];

        let fx = lab_f(x_normalized);
        let fy = lab_f(y_normalized);
        let fz = lab_f(z_normalized);

        let l = 116.0f64.mul_add(fy, -16.0);
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        Self { l, a, b }
    }
}

const HASH: char = '#';

impl FromStr for Rgb {
    type Err = Error;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let hex = hex.strip_prefix(HASH).unwrap_or(hex);

        if hex.chars().all(|c| c.is_ascii_hexdigit()) {
            let value = u32::from_str_radix(hex, 16).map_err(|_| Error::ParseRGB)?;

            match hex.len() {
                3 => {
                    let red = ((value >> 8) & 0xF) as u8;
                    let green = ((value >> 4) & 0xF) as u8;
                    let blue = (value & 0xF) as u8;

                    Ok(Self::new(red, green, blue))
                }
                6 => Ok(Self::from_u32(value)),
                _ => Err(Error::ParseRGB),
            }
        } else {
            Err(Error::ParseRGB)
        }
    }
}

impl Xyz {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Lab {
    pub const fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub const fn from_u32(value: u32) -> Self {
        Self {
            red: ((value >> 16) & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            blue: ((value) & 0xFF) as u8,
        }
    }

    /// Converts an L* value to an Rgb representation.
    ///
    /// - `lstar`: L* in L*a*b*
    ///
    /// Returns RGB representation of grayscale color with lightness matching L*
    pub fn from_lstar(lstar: f64) -> Self {
        let y = y_from_lstar(lstar);
        let component = delinearized(y);

        Self::new(component, component, component)
    }

    /// Computes the L* value of a color in Rgb representation.
    ///
    /// - `self`: RGB representation of a color
    ///
    /// returns L*, from L*a*b*, coordinate of the color
    pub fn as_lstar(&self) -> f64 {
        116.0f64.mul_add(lab_f(Xyz::from(*self).y / 100.0), -16.0)
    }

    fn hex(number: u8) -> String {
        let string = format!("{number:x}");

        if string.len() == 1 { String::from("0") + &string } else { string }
    }

    pub fn to_hex(&self) -> String {
        format!("{}{}{}", Self::hex(self.red), Self::hex(self.green), Self::hex(self.blue))
    }

    pub fn to_hex_with_pound(&self) -> String {
        format!("#{}{}{}", Self::hex(self.red), Self::hex(self.green), Self::hex(self.blue))
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

/// Converts an L* value to a Y value.
///
/// L* in L*a*b* and Y in Xyz measure the same quantity, luminance.
///
/// L* measures perceptual luminance, a linear scale. Y in Xyz measures relative
/// luminance, a logarithmic scale.
///
/// - `lstar`: L* in L*a*b*
///
/// Returns Y in Xyz
pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

/// Converts a Y value to an L* value.
///
/// L* in L*a*b* and Y in Xyz measure the same quantity, luminance.
///
/// L* measures perceptual luminance, a linear scale. Y in Xyz measures relative
/// luminance, a logarithmic scale.
///
/// - `y`: Y in Xyz
///
/// Returns L* in L*a*b*
pub fn lstar_from_y(y: f64) -> f64 {
    lab_f(y / 100.0).mul_add(116.0, -16.0)
}

/// Linearizes an Rgb component.
///
/// - `rgb_component`: 0 <= `rgb_component` <= 255, represents R/G/B channel
///
/// Returns 0.0 <= output <= 100.0, color channel converted to linear Rgb space
pub fn linearized(rgb_component: u8) -> f64 {
    let normalized = f64::from(rgb_component) / 255.0;

    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/// Delinearizes an Rgb component.
///
/// - `rgb_component`: 0.0 <= `rgb_component` <= 100.0, represents linear R/G/B
///   channel
///
/// Returns 0 <= output <= 255, color channel converted to regular Rgb space
pub fn delinearized(rgb_component: f64) -> u8 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055f64.mul_add(normalized.powf(1.0 / 2.4), -0.055)
    };

    ((delinearized * 255.0).round() as u8).clamp(0, 255)
}

fn lab_f(t: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa: f64 = 24389.0 / 27.0;

    if t > e { t.cbrt() } else { kappa.mul_add(t, 16.0) / 116.0 }
}

const fn lab_invf(ft: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;

    if ft3 > e { ft3 } else { ((116.0f64 * ft) - 16.0) / kappa }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))] use alloc::vec::Vec;
    #[cfg(feature = "std")] use std::vec::Vec;

    use float_cmp::assert_approx_eq;

    use super::Lab;
    use crate::color::{Rgb, Xyz, delinearized, linearized, lstar_from_y, y_from_lstar};

    fn range(start: f64, stop: f64, case_count: i64) -> Vec<f64> {
        let step_size = (stop - start) / (case_count as f64 - 1.0);

        (0..case_count).map(|index| step_size.mul_add(index as f64, start)).collect()
    }

    fn rgb_range() -> Vec<u8> {
        range(0.0, 255.0, 8).into_iter().map(|element| element.round() as u8).collect()
    }

    fn full_rgb_range() -> Vec<u8> {
        (0..=255).collect()
    }

    #[test]
    fn test_range_integrity() {
        let range = range(3.0, 9999.0, 1234);

        for (i, value) in range.into_iter().enumerate().take(1234) {
            assert_approx_eq!(f64, value, 8.1070559611f64.mul_add(i as f64, 3.0), epsilon = 1e-5);
        }
    }

    #[test]
    fn test_y_to_lstar_to_y() {
        for y in range(0.0, 100.0, 1001) {
            let result = y_from_lstar(lstar_from_y(y));

            assert_approx_eq!(f64, result, y, epsilon = 1e-5);
        }
    }

    #[test]
    fn test_lstar_to_y_to_lstar() {
        for lstar in range(0.0, 100.0, 1001) {
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
        for lstar in range(0.0, 100.0, 1001) {
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
}
