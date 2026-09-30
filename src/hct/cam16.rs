use core::f64::consts::PI;

use super::ViewingConditions;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::{Rgb, Xyz},
    utils::math::signum,
};

/// CAM16, a color appearance model. Colors are not just defined by their hex
/// code, but rather, a hex code and viewing conditions.
///
/// CAM16 instances also have coordinates in the CAM16-UCS space, called J*, a*,
/// b*, or jstar, astar, bstar in code. CAM16-UCS is included in the CAM16
/// specification, and should be used when measuring distances between colors.
///
/// In traditional color spaces, a color can be identified solely by the
/// observer's measurement of the color. Color appearance models such as CAM16
/// also use information about the environment where the color was
/// observed, known as the viewing conditions.
///
/// For example, white under the traditional assumption of a midday sun white
/// point is accurately measured as a slightly chromatic blue by
/// (roughly, hue 203, chroma 3, lightness 100)
/// CAM16, a color appearance model. Colors are not just defined by their hex
/// code, but rather, a hex code and viewing conditions.
///
/// CAM16 instances also have coordinates in the CAM16-UCS space, called J*, a*,
/// b*, or jstar, astar, bstar in code. CAM16-UCS is included in the CAM16
/// specification, and should be used when measuring distances between colors.
///
/// In traditional color spaces, a color can be identified solely by the
/// observer's measurement of the color. Color appearance models such as CAM16
/// also use information about the environment where the color was
/// observed, known as the viewing conditions.
///
/// For example, white under the traditional assumption of a midday sun white
/// point is accurately measured as a slightly chromatic blue by
/// (roughly, hue 203, chroma 3, lightness 100)
pub struct Cam16 {
    /// Like red, orange, yellow, green, etc.
    pub hue: f64,

    /// Informally, colorfulness / color intensity. Like saturation in HSL,
    /// except perceptually accurate.
    pub chroma: f64,

    /// Lightness
    pub lightness: f64,

    /// Brightness; ratio of lightness to white point's lightness
    pub brightness: f64,

    /// Colorfulness
    pub colorfulness: f64,

    /// Saturation; ratio of chroma to white point's chroma
    pub saturation: f64,

    /// CAM16-UCS J coordinate
    pub jstar: f64,

    /// CAM16-UCS a coordinate
    pub astar: f64,

    /// CAM16-UCS b coordinate
    pub bstar: f64,
}

impl Cam16 {
    /// CAM16 instances also have coordinates in the CAM16-UCS space, called J*,
    /// a*, b*, or jstar, astar, bstar in code. CAM16-UCS is included in the
    /// CAM16 specification, and should be used when measuring distances
    /// between colors.
    pub fn distance(&self, other: &Self) -> f64 {
        let d_j = self.jstar - other.jstar;
        let d_a = self.astar - other.astar;
        let d_b = self.bstar - other.bstar;
        let d_eprime = d_b.mul_add(d_b, d_j.mul_add(d_j, d_a * d_a)).sqrt();

        1.41 * d_eprime.powf(0.63)
    }

    /// Given color expressed in [`Rgb`] form and viewed in
    /// `viewing_conditions`, convert to [`Cam16`].
    ///
    /// # Panics
    ///
    /// Will panic if the hue is between 0 and 360
    pub fn from_rgb_in_viewing_conditions(rgb: Rgb, viewing_conditions: &ViewingConditions) -> Self {
        Self::from_xyz_in_viewing_conditions(rgb.into(), viewing_conditions)
    }

    /// Given color expressed in [`Xyz`] form and viewed in
    /// `viewing_conditions`, convert to [`Cam16`].
    ///
    /// # Panics
    ///
    /// Will panic if the hue is between 0 and 360
    pub fn from_xyz_in_viewing_conditions(value: Xyz, viewing_conditions: &ViewingConditions) -> Self {
        let (r_c, g_c, b_c) = (
            0.051461f64.mul_add(-value.z, 0.401288f64.mul_add(value.x, 0.650173 * value.y)),
            0.045854f64.mul_add(value.z, (-0.250268f64).mul_add(value.x, 1.204414 * value.y)),
            0.953127f64.mul_add(value.z, (-0.002079f64).mul_add(value.x, 0.048952 * value.y)),
        );

        // Discount illuminant
        let r_d = viewing_conditions.rgb_d[0] * r_c;
        let g_d = viewing_conditions.rgb_d[1] * g_c;
        let b_d = viewing_conditions.rgb_d[2] * b_c;

        // chromatic adaptation
        let (r_af, g_af, b_af) = (
            (viewing_conditions.fl * r_d.abs() / 100.0).powf(0.42),
            (viewing_conditions.fl * g_d.abs() / 100.0).powf(0.42),
            (viewing_conditions.fl * b_d.abs() / 100.0).powf(0.42),
        );
        let r_a = signum(r_d) * 400.0 * r_af / (r_af + 27.13);
        let g_a = signum(g_d) * 400.0 * g_af / (g_af + 27.13);
        let b_a = signum(b_d) * 400.0 * b_af / (b_af + 27.13);

        let (a, b, u, p2) = (
            (11.0f64.mul_add(r_a, -12.0 * g_a) + b_a) / 11.0,
            2.0f64.mul_add(-b_a, r_a + g_a) / 9.0,
            21.0f64.mul_add(b_a, 20.0f64.mul_add(r_a, 20.0 * g_a)) / 20.0,
            (40.0f64.mul_add(r_a, 20.0 * g_a) + b_a) / 20.0,
        );

        // hue
        let atan2 = b.atan2(a);
        let atan_degrees = atan2.to_degrees();
        let hue = if atan_degrees < 0.0 {
            atan_degrees + 360.0
        } else if atan_degrees >= 360.0 {
            atan_degrees - 360.0
        } else {
            atan_degrees
        };
        let hue_radians = hue.to_radians();

        assert!((0.0..360.0).contains(&hue), "hue was really {hue}");

        // achromatic response to color
        let ac = p2 * viewing_conditions.nbb;

        // CAM16 lightness and brightness
        let lightness = 100.0 * (ac / viewing_conditions.aw).powf(viewing_conditions.c * viewing_conditions.z);

        let brightness = (4.0 / viewing_conditions.c) * (lightness / 100.0).sqrt() * (viewing_conditions.aw + 4.0) * (viewing_conditions.f_lroot);

        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = (1.0 / 4.0) * ((hue_prime.to_radians() + 2.0).cos() + 3.8);
        let p1 = 50000.0 / 13.0 * e_hue * viewing_conditions.n_c * viewing_conditions.ncb;
        let t = p1 * a.hypot(b) / (u + 0.305);

        let alpha = t.powf(0.9) * (1.64 - 0.29f64.powf(viewing_conditions.background_ytowhite_point_y)).powf(0.73);

        // CAM16 chroma, colorfulness, chroma
        let chroma = alpha * (lightness / 100.0).sqrt();
        let colorfulness = chroma * viewing_conditions.f_lroot;
        let saturation = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        // CAM16-UCS components
        let (jstar, mstar) = (
            100.0f64.mul_add(0.007, 1.0) * lightness / 0.007f64.mul_add(lightness, 1.0),
            (0.0228 * colorfulness).ln_1p() / 0.0228,
        );

        let (astar, bstar) = (mstar * hue_radians.cos(), mstar * hue_radians.sin());

        Self {
            hue,
            chroma,
            lightness,
            brightness,
            colorfulness,
            saturation,
            jstar,
            astar,
            bstar,
        }
    }

    /// Create a CAM16 color from lightness `j`, chroma `c`, and hue `h`,
    /// assuming the color was viewed in default viewing conditions.
    pub fn from_jch(j: f64, c: f64, h: f64) -> Self {
        Self::from_jch_in_viewing_conditions(j, c, h, &ViewingConditions::s_rgb())
    }

    /// Create a CAM16 color from lightness `j`, chroma `c`, and hue `h`,
    /// in `viewing_conditions`.
    pub fn from_jch_in_viewing_conditions(lightness: f64, chroma: f64, hue: f64, viewing_conditions: &ViewingConditions) -> Self {
        let brightness = (4.0 / viewing_conditions.c) * (lightness / 100.0).sqrt() * (viewing_conditions.aw + 4.0) * (viewing_conditions.f_lroot);
        let colorfulness = chroma * viewing_conditions.f_lroot;
        let alpha = chroma / (lightness / 100.0).sqrt();
        let saturation = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        let hue_radians = hue.to_radians();
        let (jstar, mstar) = (
            100.0_f64.mul_add(0.007, 1.0) * lightness / 0.007_f64.mul_add(lightness, 1.0),
            1.0 / 0.0228 * 0.0228_f64.mul_add(colorfulness, 1.0).ln(),
        );

        let (astar, bstar) = (mstar * hue_radians.cos(), mstar * hue_radians.sin());

        Self {
            hue,
            chroma,
            lightness,
            brightness,
            colorfulness,
            saturation,
            jstar,
            astar,
            bstar,
        }
    }

    /// Create a CAM16 color from CAM16-UCS coordinates `jstar`, `astar`,
    /// `bstar`. assuming the color was viewed in default viewing
    /// conditions.
    pub fn from_ucs(jstar: f64, astar: f64, bstar: f64) -> Self {
        Self::from_ucs_in_viewing_conditions(jstar, astar, bstar, &ViewingConditions::s_rgb())
    }

    /// Create a CAM16 color from CAM16-UCS coordinates `jstar`, `astar`,
    /// `bstar`. in `viewing_conditions`.
    pub fn from_ucs_in_viewing_conditions(jstar: f64, astar: f64, bstar: f64, viewing_conditions: &ViewingConditions) -> Self {
        let a = astar;
        let b = bstar;
        let chroma = a.hypot(b);
        let chroma = (chroma * 0.0228).exp_m1() / 0.0228;
        let chroma = chroma / viewing_conditions.f_lroot;
        let hue = b.atan2(a) * (180.0 / PI);
        let hue = if hue < 0.0 { hue + 360.0 } else { hue };
        let lightness = jstar / (jstar - 100.0).mul_add(-0.007, 1.0);

        Self::from_jch_in_viewing_conditions(lightness, chroma, hue, viewing_conditions)
    }

    /// Rgb representation of a color, given the color was viewed in
    /// `viewing_conditions`
    pub fn viewed(&self, viewing_conditions: &ViewingConditions) -> Rgb {
        let xyz = self.xyz_in_viewing_conditions(viewing_conditions);

        xyz.into()
    }

    /// Xyz representation of CAM16 seen in `viewing_conditions`.
    pub fn xyz_in_viewing_conditions(&self, viewing_conditions: &ViewingConditions) -> Xyz {
        let alpha = if self.chroma == 0.0 || self.lightness == 0.0 {
            0.0
        } else {
            self.chroma / (self.lightness / 100.0).sqrt()
        };

        let t = (alpha / (1.64 - 0.29_f64.powf(viewing_conditions.background_ytowhite_point_y)).powf(0.73)).powf(1.0 / 0.9);
        let h_rad = self.hue.to_radians();

        let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let ac = viewing_conditions.aw * (self.lightness / 100.0).powf(1.0 / viewing_conditions.c / viewing_conditions.z);
        let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.n_c * viewing_conditions.ncb;

        let p2 = ac / viewing_conditions.nbb;

        let (h_sin, h_cos) = (h_rad.sin(), h_rad.cos());

        let gamma = 23.0 * (p2 + 0.305) * t / (108.0 * t).mul_add(h_sin, 23.0f64.mul_add(p1, 11.0 * t * h_cos));
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let (r_a, g_a, b_a) = (
            288.0f64.mul_add(b, 460.0f64.mul_add(p2, 451.0 * a)) / 1403.0,
            261.0f64.mul_add(-b, 460.0f64.mul_add(p2, -891.0 * a)) / 1403.0,
            6300.0f64.mul_add(-b, 460.0f64.mul_add(p2, -220.0 * a)) / 1403.0,
        );

        let (r_cbase, g_cbase, b_cbase) = (
            0.0f64.max((27.13 * r_a.abs()) / (400.0 - r_a.abs())),
            0.0f64.max((27.13 * g_a.abs()) / (400.0 - g_a.abs())),
            0.0f64.max((27.13 * b_a.abs()) / (400.0 - b_a.abs())),
        );

        let (r_c, g_c, b_c) = (
            signum(r_a) * (100.0 / viewing_conditions.fl) * r_cbase.powf(1.0 / 0.42),
            signum(g_a) * (100.0 / viewing_conditions.fl) * g_cbase.powf(1.0 / 0.42),
            signum(b_a) * (100.0 / viewing_conditions.fl) * b_cbase.powf(1.0 / 0.42),
        );

        let r_f = r_c / viewing_conditions.rgb_d[0];
        let g_f = g_c / viewing_conditions.rgb_d[1];
        let b_f = b_c / viewing_conditions.rgb_d[2];

        let (x, y, z) = (
            0.14918677f64.mul_add(b_f, 1.86206786f64.mul_add(r_f, -1.01125463 * g_f)),
            0.00897398f64.mul_add(-b_f, 0.38752654f64.mul_add(r_f, 0.62144744 * g_f)),
            1.04996444f64.mul_add(b_f, (-0.01584150f64).mul_add(r_f, -0.03412294 * g_f)),
        );

        Xyz::new(x, y, z)
    }
}

impl From<Rgb> for Cam16 {
    fn from(rgb: Rgb) -> Self {
        Self::from_rgb_in_viewing_conditions(rgb, &ViewingConditions::s_rgb())
    }
}

impl From<Cam16> for Rgb {
    fn from(val: Cam16) -> Self {
        val.viewed(&ViewingConditions::s_rgb())
    }
}
