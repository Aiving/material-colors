use core::f64::consts::PI;

use crate::{color::Xyz, Argb};

use super::ViewingConditions;

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
    pub j: f64,

    /// Brightness; ratio of lightness to white point's lightness
    pub q: f64,

    /// Colorfulness
    pub m: f64,

    /// Saturation; ratio of chroma to white point's chroma
    pub s: f64,

    /// CAM16-UCS J coordinate
    pub jstar: f64,

    /// CAM16-UCS a coordinate
    pub astar: f64,

    /// CAM16-UCS b coordinate
    pub bstar: f64,
}

impl Cam16 {
    /// CAM16 instances also have coordinates in the CAM16-UCS space, called J*,
    /// a*, b*, or jstar, astar, bstar in code. CAM16-UCS is included in the CAM16
    /// specification, and should be used when measuring distances between colors.
    pub fn distance(&self, other: Cam16) -> f64 {
        let d_j = self.jstar - other.jstar;
        let d_a = self.astar - other.astar;
        let d_b = self.bstar - other.bstar;
        let d_eprime = (d_j * d_j + d_a * d_a + d_b * d_b).sqrt();

        1.41 * d_eprime.powf(0.63)
    }

    /// Given [viewing_conditions], convert [argb] to
    pub fn fromi32_in_viewing_conditions(
        argb: Argb,
        viewing_conditions: ViewingConditions,
    ) -> Cam16 {
        // Transform Argb int to Xyz
        let Xyz { x, y, z } = Xyz::from(argb);

        Cam16::from_xyz_in_viewing_conditions(x, y, z, viewing_conditions)
    }

    /// Given color expressed in Xyz and viewed in [viewing_conditions], convert to
    ///
    pub fn from_xyz_in_viewing_conditions(
        x: f64,
        y: f64,
        z: f64,
        viewing_conditions: ViewingConditions,
    ) -> Cam16 {
        // Transform Xyz to 'cone'/'rgb' responses
        let r_c = 0.401288 * x + 0.650173 * y - 0.051461 * z;
        let g_c = -0.250268 * x + 1.204414 * y + 0.045854 * z;
        let b_c = -0.002079 * x + 0.048952 * y + 0.953127 * z;

        // Discount illuminant
        let r_d = viewing_conditions.rgb_d[0] * r_c;
        let g_d = viewing_conditions.rgb_d[1] * g_c;
        let b_d = viewing_conditions.rgb_d[2] * b_c;

        // chromatic adaptation
        let r_af = (viewing_conditions.fl * r_d.abs() / 100.0).powf(0.42);
        let g_af = (viewing_conditions.fl * g_d.abs() / 100.0).powf(0.42);
        let b_af = (viewing_conditions.fl * b_d.abs() / 100.0).powf(0.42);
        let r_a = r_d.signum() * 400.0 * r_af / (r_af + 27.13);
        let g_a = g_d.signum() * 400.0 * g_af / (g_af + 27.13);
        let b_a = b_d.signum() * 400.0 * b_af / (b_af + 27.13);

        // redness-greenness
        let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
        // yellowness-blueness
        let b = 2.0_f64.mul_add(-b_a, r_a + g_a) / 9.0;
        // auxiliary components
        let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
        let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;

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

        assert!((0.0..360.0).contains(&hue));

        // achromatic response to color
        let ac = p2 * viewing_conditions.nbb;

        // CAM16 lightness and brightness
        let j =
            100.0 * (ac / viewing_conditions.aw).powf(viewing_conditions.c * viewing_conditions.z);
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * (viewing_conditions.f_lroot);

        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = (1.0 / 4.0) * ((hue_prime.to_radians() + 2.0).cos() + 3.8);
        let p1 = 50000.0 / 13.0 * e_hue * viewing_conditions.n_c * viewing_conditions.ncb;
        let t = p1 * (a * a + b * b).sqrt() / (u + 0.305);
        let alpha = t.powf(0.9)
            * (1.64 - 0.29_f64.powf(viewing_conditions.background_ytowhite_point_y)).powf(0.73);

        // CAM16 chroma, colorfulness, chroma
        let c = alpha * (j / 100.0).sqrt();
        let m = c * viewing_conditions.f_lroot;
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        // CAM16-UCS components
        let jstar = 100.0_f64.mul_add(0.007, 1.0) * j / 0.007f64.mul_add(j, 1.0);
        let mstar = 0.0228_f64.mul_add(m, 1.0) / 0.0228;
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();

        Self {
            hue,
            chroma: c,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }

    /// Create a CAM16 color from lightness [j], chroma [c], and hue [h],
    /// assuming the color was viewed in default viewing conditions.
    pub fn from_jch(j: f64, c: f64, h: f64) -> Self {
        Cam16::from_jch_in_viewing_conditions(j, c, h, ViewingConditions::s_rgb())
    }

    /// Create a CAM16 color from lightness [j], chroma [c], and hue [h],
    /// in [viewing_conditions].
    pub fn from_jch_in_viewing_conditions(
        j: f64,
        c: f64,
        h: f64,
        viewing_conditions: ViewingConditions,
    ) -> Cam16 {
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * (viewing_conditions.f_lroot);
        let m = c * viewing_conditions.f_lroot;
        let alpha = c / (j / 100.0).sqrt();
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        let hue_radians = h.to_radians();
        let jstar = 100.0_f64.mul_add(0.007, 1.0) * j / 0.007_f64.mul_add(j, 1.0);
        let mstar = 1.0 / 0.0228 * 0.0228_f64.mul_add(m, 1.0).ln();
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();

        Cam16 {
            hue: h,
            chroma: c,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }

    /// Create a CAM16 color from CAM16-UCS coordinates [jstar], [astar], [bstar].
    /// assuming the color was viewed in default viewing conditions.
    pub fn from_ucs(jstar: f64, astar: f64, bstar: f64) -> Cam16 {
        Cam16::from_ucs_in_viewing_conditions(jstar, astar, bstar, ViewingConditions::standard())
    }

    /// Create a CAM16 color from CAM16-UCS coordinates [jstar], [astar], [bstar].
    /// in [viewing_conditions].
    pub fn from_ucs_in_viewing_conditions(
        jstar: f64,
        astar: f64,
        bstar: f64,
        viewing_conditions: ViewingConditions,
    ) -> Cam16 {
        let a = astar;
        let b = bstar;
        let m = (a * a + b * b).sqrt();
        let m = ((m * 0.0228).exp() - 1.0) / 0.0228;
        let c = m / viewing_conditions.f_lroot;
        let h = b.atan2(a) * (180.0 / PI);
        let h = if h < 0.0 { h + 360.0 } else { h };
        let j = jstar / (jstar - 100.0).mul_add(-0.007, 1.0);

        Cam16::from_jch_in_viewing_conditions(j, c, h, viewing_conditions)
    }

    // Avoid allocations during conversion by pre-allocating an array.
    // private var viewedArray: [f64] = [0, 0, 0]

    /// Argb representation of a color, given the color was viewed in
    /// [viewing_conditions]
    pub fn viewed(&self, viewing_conditions: ViewingConditions) -> Argb {
        let xyz = self.xyz_in_viewing_conditions(viewing_conditions);

        xyz.into()
    }

    /// Xyz representation of CAM16 seen in [viewing_conditions].
    pub fn xyz_in_viewing_conditions(&self, viewing_conditions: ViewingConditions) -> Xyz {
        let alpha = if self.chroma == 0.0 || self.j == 0.0 {
            0.0
        } else {
            self.chroma / (self.j / 100.0).sqrt()
        };
        let t = (alpha
            / (1.64 - 0.29_f64.powf(viewing_conditions.background_ytowhite_point_y)).powf(0.73))
        .powf(1.0 / 0.9);
        let h_rad = self.hue.to_radians();

        let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let ac = viewing_conditions.aw
            * (self.j / 100.0).powf(1.0 / viewing_conditions.c / viewing_conditions.z);
        let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.n_c * viewing_conditions.ncb;

        let p2 = ac / viewing_conditions.nbb;

        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();

        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

        let r_cbase = 0.0_f64.max((27.13 * r_a.abs()) / (400.0 - (r_a.abs())));
        let r_c = r_a.signum() * (100.0 / viewing_conditions.fl) * r_cbase.powf(1.0 / 0.42);
        let g_cbase = 0.0_f64.max((27.13 * (g_a.abs())) / (400.0 - (g_a.abs())));
        let g_c = g_a.signum() * (100.0 / viewing_conditions.fl) * g_cbase.powf(1.0 / 0.42);
        let b_cbase = 0.0_f64.max((27.13 * (b_a.abs())) / (400.0 - (b_a.abs())));
        let b_c = b_a.signum() * (100.0 / viewing_conditions.fl) * b_cbase.powf(1.0 / 0.42);
        let r_f = r_c / viewing_conditions.rgb_d[0];
        let g_f = g_c / viewing_conditions.rgb_d[1];
        let b_f = b_c / viewing_conditions.rgb_d[2];

        let x = 1.86206786 * r_f - 1.01125463 * g_f + 0.14918677 * b_f;
        let y = 0.38752654 * r_f + 0.62144744 * g_f - 0.00897398 * b_f;
        let z = -0.01584150 * r_f - 0.03412294 * g_f + 1.04996444 * b_f;

        Xyz::new(x, y, z)
    }
}

impl From<Argb> for Cam16 {
    fn from(argb: Argb) -> Self {
        Cam16::fromi32_in_viewing_conditions(argb, ViewingConditions::s_rgb())
    }
}

impl From<Cam16> for Argb {
    fn from(val: Cam16) -> Self {
        val.viewed(ViewingConditions::s_rgb())
    }
}
