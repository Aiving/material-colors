#[cfg(feature = "serde")]
use serde::Serialize;
use std::{fmt, str::FromStr};

use crate::{utils::math::matrix_multiply, Error};

pub const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.41233895, 0.35762064, 0.18051042],
    [0.2126, 0.7152, 0.0722],
    [0.01932141, 0.11916382, 0.95034478],
];
pub const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [
        3.2413774792388685,
        -1.5376652402851851,
        -0.49885366846268053,
    ],
    [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
    [
        0.05562093689691305,
        -0.20395524564742123,
        1.0571799111220335,
    ],
];
pub const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// ARGB representation of color. Can be created using [`Argb::new`] or
/// [`from_str`].
///
/// ## Examples:
/// ```rust
/// use std::str::FromStr;
/// use material_colors::color::Argb;
///
/// // from_str can accept any valid HEX color
/// let color = Argb::from_str("abc").unwrap();
/// let color = Argb::from_str("abcabc").unwrap();
/// let color = Argb::from_str("#abc").unwrap();
/// let color = Argb::from_str("#abcabc").unwrap();
/// ```
///
/// [`from_str`]: std::str::FromStr::from_str
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Argb {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LinearRgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

/** Converts a color from Rgb components to Argb format. */
impl From<Rgb> for Argb {
    fn from(Rgb { red, green, blue }: Rgb) -> Self {
        Self {
            alpha: 255,
            red,
            green,
            blue,
        }
    }
}

/** Converts a color from linear Rgb components to Argb format. */
impl From<LinearRgb> for Argb {
    fn from(linear: LinearRgb) -> Self {
        let r = delinearized(linear.red);
        let g = delinearized(linear.green);
        let b = delinearized(linear.blue);

        Rgb::new(r, g, b).into()
    }
}

/** Converts a color from Argb to Xyz. */
impl From<Xyz> for Argb {
    fn from(Xyz { x, y, z }: Xyz) -> Self {
        let matrix = XYZ_TO_SRGB;

        let linear_r = matrix[0][2].mul_add(z, matrix[0][0].mul_add(x, matrix[0][1] * y));
        let linear_g = matrix[1][2].mul_add(z, matrix[1][0].mul_add(x, matrix[1][1] * y));
        let linear_b = matrix[2][2].mul_add(z, matrix[2][0].mul_add(x, matrix[2][1] * y));

        let r = delinearized(linear_r);
        let g = delinearized(linear_g);
        let b = delinearized(linear_b);

        Rgb::new(r, g, b).into()
    }
}

/** Converts a color from Xyz to Argb. */
impl From<Argb> for Xyz {
    fn from(
        Argb {
            alpha: _,
            red,
            green,
            blue,
        }: Argb,
    ) -> Self {
        let r = linearized(red);
        let g = linearized(green);
        let b = linearized(blue);

        let [x, y, z] = matrix_multiply([r, g, b], SRGB_TO_XYZ);

        Self { x, y, z }
    }
}

/** Converts a color represented in Lab color space into an Argb integer. */
impl From<Lab> for Argb {
    fn from(Lab { l, a, b }: Lab) -> Self {
        let white_point = WHITE_POINT_D65;

        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;

        let x_normalized = lab_invf(fx);
        let y_normalized = lab_invf(fy);
        let z_normalized = lab_invf(fz);

        let x = x_normalized * white_point[0];
        let y = y_normalized * white_point[1];
        let z = z_normalized * white_point[2];

        Rgb::new(x as u8, y as u8, z as u8).into()
    }
}

impl From<Argb> for Lab {
    fn from(
        Argb {
            alpha: _,
            red,
            green,
            blue,
        }: Argb,
    ) -> Self {
        let linear_r = linearized(red);
        let linear_g = linearized(green);
        let linear_b = linearized(blue);

        let matrix = SRGB_TO_XYZ;

        let x = matrix[0][2].mul_add(
            linear_b,
            matrix[0][0].mul_add(linear_r, matrix[0][1] * linear_g),
        );
        let y = matrix[1][2].mul_add(
            linear_b,
            matrix[1][0].mul_add(linear_r, matrix[1][1] * linear_g),
        );
        let z = matrix[2][2].mul_add(
            linear_b,
            matrix[2][0].mul_add(linear_r, matrix[2][1] * linear_g),
        );

        let white_point = WHITE_POINT_D65;

        let x_normalized = x / white_point[0];
        let y_normalized = y / white_point[1];
        let z_normalized = z / white_point[2];

        let fx = lab_f(x_normalized);
        let fy = lab_f(y_normalized);
        let fz = lab_f(z_normalized);

        let l = 116.0_f64.mul_add(fy, -16.0);
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        Self { l, a, b }
    }
}

const fn hex_digit_to_rgb(number: u32) -> Rgb {
    let r = number >> 16;
    let g = (number >> 8) & 0x00FF;
    let b = number & 0x0000_00FF;

    Rgb::new(r as u8, g as u8, b as u8)
}

const HASH: u8 = b'#';

impl FromStr for Argb {
    type Err = Error;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let s = hex.as_bytes();
        let mut buff: [u8; 6] = [0; 6];
        let mut buff_len = 0;

        for b in s {
            if !b.is_ascii() || buff_len == 6 {
                return Err(Error::ParseRGB);
            }

            let bl = b.to_ascii_lowercase();

            if bl == HASH {
                continue;
            }

            if bl.is_ascii_hexdigit() {
                buff[buff_len] = bl;
                buff_len += 1;
            } else {
                return Err(Error::ParseRGB);
            }
        }

        if buff_len == 3 {
            buff = [buff[0], buff[0], buff[1], buff[1], buff[2], buff[2]];
        }

        let hex_str = core::str::from_utf8(&buff).map_err(|_| Error::ParseRGB)?;
        let hex_digit = u32::from_str_radix(hex_str, 16).map_err(|_| Error::ParseRGB)?;

        Ok(hex_digit_to_rgb(hex_digit).into())
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
}

impl Argb {
    pub const fn new(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha,
            red,
            green,
            blue,
        }
    }

    /// Converts an L* value to an Argb representation.
    ///
    /// - `lstar`: L* in L*a*b*
    ///
    /// Returns ARGB representation of grayscale color with lightness matching L*
    pub fn from_lstar(lstar: f64) -> Self {
        let y = y_from_lstar(lstar);
        let component = delinearized(y);

        Rgb::new(component, component, component).into()
    }

    /// Computes the L* value of a color in Argb representation.
    ///
    /// - `argb`: ARGB representation of a color
    ///
    /// returns L*, from L*a*b*, coordinate of the color
    pub fn as_lstar(&self) -> f64 {
        116.0f64.mul_add(lab_f(Xyz::from(*self).y / 100.0), -16.0)
    }

    fn hex(number: u8) -> String {
        let string = format!("{number:x}");

        if string.len() == 1 {
            String::from("0") + &string
        } else {
            string
        }
    }

    pub fn to_hex(&self) -> String {
        format!(
            "{}{}{}",
            Self::hex(self.red),
            Self::hex(self.green),
            Self::hex(self.blue)
        )
    }

    pub fn to_hex_with_pound(&self) -> String {
        format!(
            "#{}{}{}",
            Self::hex(self.red),
            Self::hex(self.green),
            Self::hex(self.blue)
        )
    }
}

impl fmt::Display for Argb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_with_pound())
    }
}

/**
 * Converts an L* value to a Y value.
 *
 * <p>L* in L*a*b* and Y in Xyz measure the same quantity, luminance.
 *
 * <p>L* measures perceptual luminance, a linear scale. Y in Xyz measures relative luminance, a
 * logarithmic scale.
 *
 * @param lstar L* in L*a*b*
 * @return Y in Xyz
 */
pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

/**
 * Converts a Y value to an L* value.
 *
 * <p>L* in L*a*b* and Y in Xyz measure the same quantity, luminance.
 *
 * <p>L* measures perceptual luminance, a linear scale. Y in Xyz measures relative luminance, a
 * logarithmic scale.
 *
 * @param y Y in Xyz
 * @return L* in L*a*b*
 */
pub fn lstar_from_y(y: f64) -> f64 {
    lab_f(y / 100.0).mul_add(116.0, -16.0)
}

/**
 * Linearizes an Rgb component.
 *
 * @param rgbComponent 0 <= `rgb_component` <= 255, represents R/G/B channel
 * @return 0.0 <= output <= 100.0, color channel converted to linear Rgb space
 */
pub fn linearized(rgb_component: u8) -> f64 {
    let normalized = f64::from(rgb_component) / 255.0;

    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/**
 * Delinearizes an Rgb component.
 *
 * @param rgbComponent 0.0 <= `rgb_component` <= 100.0, represents linear R/G/B channel
 * @return 0 <= output <= 255, color channel converted to regular Rgb space
 */
pub fn delinearized(rgb_component: f64) -> u8 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055f64.mul_add(normalized.powf(1.0 / 2.4), -0.055)
    };

    (delinearized * 255.0).round().clamp(0.0, 255.0) as u8
}

fn lab_f(t: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0_f64 / 27.0;

    if t > e {
        t.cbrt()
    } else {
        kappa.mul_add(t, 16.0) / 116.0
    }
}

fn lab_invf(ft: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;

    if ft3 > e {
        ft3
    } else {
        116.0f64.mul_add(ft, -16.0) / kappa
    }
}
