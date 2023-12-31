use super::math::matrix_multiply;

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

pub type Rgb = [u8; 3];
pub type Argb = [u8; 4];
pub type LinearRgb = [f64; 3];
pub type Xyz = [f64; 3];
pub type Lab = [f64; 3];

/** Converts a color from Rgb components to Argb format. */
pub fn argb_from_rgb([r, g, b]: Rgb) -> Argb {
    [255, r, g, b]
}

/** Converts a color from linear Rgb components to Argb format. */
pub fn argb_from_linrgb([r, g, b]: LinearRgb) -> Argb {
    let r = delinearized(r);
    let g = delinearized(g);
    let b = delinearized(b);

    argb_from_rgb([r, g, b])
}

/** Returns the alpha component of a color in Argb format. */
pub fn alpha_from_argb([alpha, _, _, _]: Argb) -> u8 {
    alpha
}

/** Returns the red component of a color in Argb format. */
pub fn red_from_argb([_, red, _, _]: Argb) -> u8 {
    red
}

/** Returns the green component of a color in Argb format. */
pub fn green_from_argb([_, _, green, _]: Argb) -> u8 {
    green
}

/** Returns the blue component of a color in Argb format. */
pub fn blue_from_argb([_, _, _, blue]: Argb) -> u8 {
    blue
}

/** Returns whether a color in Argb format is opaque. */
//pub fn is_opaque(argb: Argb) -> bool {
//     alpha_from_argb(argb) >= 255
// }

/** Converts a color from Argb to Xyz. */
pub fn argb_from_xyz([x, y, z]: Xyz) -> Argb {
    let matrix = XYZ_TO_SRGB;

    let linear_r = matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z;
    let linear_g = matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z;
    let linear_b = matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z;

    let r = delinearized(linear_r);
    let g = delinearized(linear_g);
    let b = delinearized(linear_b);

    argb_from_rgb([r, g, b])
}

/** Converts a color from Xyz to Argb. */
pub fn xyz_from_argb(argb: Argb) -> Xyz {
    let r = linearized(red_from_argb(argb));
    let g = linearized(green_from_argb(argb));
    let b = linearized(blue_from_argb(argb));

    matrix_multiply([r, g, b], SRGB_TO_XYZ)
}

/** Converts a color represented in Lab color space into an Argb integer. */
pub fn argb_from_lab([l, a, b]: Lab) -> Argb {
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

    argb_from_xyz([x, y, z])
}

/**
 * Converts a color from Argb representation to L*a*b* representation.
 *
 * @param argb the Argb representation of a color
 * @return a Lab object representing the color
 */
pub fn lab_from_argb(argb: Argb) -> Lab {
    let linear_r = linearized(red_from_argb(argb));
    let linear_g = linearized(green_from_argb(argb));
    let linear_b = linearized(blue_from_argb(argb));

    let matrix = SRGB_TO_XYZ;

    let x = matrix[0][0] * linear_r + matrix[0][1] * linear_g + matrix[0][2] * linear_b;
    let y = matrix[1][0] * linear_r + matrix[1][1] * linear_g + matrix[1][2] * linear_b;
    let z = matrix[2][0] * linear_r + matrix[2][1] * linear_g + matrix[2][2] * linear_b;

    let white_point = WHITE_POINT_D65;

    let x_normalized = x / white_point[0];
    let y_normalized = y / white_point[1];
    let z_normalized = z / white_point[2];

    let fx = lab_f(x_normalized);
    let fy = lab_f(y_normalized);
    let fz = lab_f(z_normalized);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);

    [l, a, b]
}

/**
 * Converts an L* value to an Argb representation.
 *
 * @param lstar L* in L*a*b*
 * @return Argb representation of grayscale color with lightness matching L*
 */
pub fn argb_from_lstar(lstar: f64) -> Argb {
    let y = y_from_lstar(lstar);
    let component = delinearized(y);

    argb_from_rgb([component, component, component])
}

/**
 * Computes the L* value of a color in Argb representation.
 *
 * @param argb Argb representation of a color
 * @return L*, from L*a*b*, coordinate of the color
 */
pub fn lstar_from_argb(argb: Argb) -> f64 {
    let y = xyz_from_argb(argb)[1];

    116.0 * lab_f(y / 100.0) - 16.0
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
    lab_f(y / 100.0) * 116.0 - 16.0
}

/**
 * Linearizes an Rgb component.
 *
 * @param rgbComponent 0 <= rgb_component <= 255, represents R/G/B channel
 * @return 0.0 <= output <= 100.0, color channel converted to linear Rgb space
 */
pub fn linearized(rgb_component: u8) -> f64 {
    let normalized = rgb_component as f64 / 255.0;

    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/**
 * Delinearizes an Rgb component.
 *
 * @param rgbComponent 0.0 <= rgb_component <= 100.0, represents linear R/G/B channel
 * @return 0 <= output <= 255, color channel converted to regular Rgb space
 */
pub fn delinearized(rgb_component: f64) -> u8 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };

    (delinearized * 255.0).round().clamp(0.0, 255.0) as u8
}

fn lab_f(t: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;

    if t > e {
        t.powf(1.0 / 3.0)
    } else {
        (kappa * t + 16.0) / 116.0
    }
}

fn lab_invf(ft: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;

    if ft3 > e {
        ft3
    } else {
        (116.0 * ft - 16.0) / kappa
    }
}
