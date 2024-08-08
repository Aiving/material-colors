use super::ViewingConditions;
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::{Argb, Xyz},
    utils::math::signum,
};
use core::f64::consts::PI;

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
    pub fn distance(&self, other: &Self) -> f64 {
        let d_j = self.jstar - other.jstar;
        let d_a = self.astar - other.astar;
        let d_b = self.bstar - other.bstar;
        let d_eprime = d_b.mul_add(d_b, d_j.mul_add(d_j, d_a * d_a)).sqrt();

        1.41 * d_eprime.powf(0.63)
    }

    /// Given `viewing_conditions`, convert `argb` to
    pub fn fromi32_in_viewing_conditions(
        argb: Argb,
        viewing_conditions: &ViewingConditions,
    ) -> Self {
        // Transform Argb int to Xyz
        let Xyz { x, y, z } = Xyz::from(argb);

        Self::from_xyz_in_viewing_conditions(x, y, z, viewing_conditions)
    }

    /// Given color expressed in Xyz and viewed in `viewing_conditions`, convert to
    /// Cam16
    ///
    /// # Panics
    ///
    /// Will panic if the hue is between 0 and 360
    pub fn from_xyz_in_viewing_conditions(
        x: f64,
        y: f64,
        z: f64,
        viewing_conditions: &ViewingConditions,
    ) -> Self {
        let (r_c, g_c, b_c) = (
            0.051461f64.mul_add(-z, 0.401288f64.mul_add(x, 0.650173 * y)),
            0.045854f64.mul_add(z, (-0.250268f64).mul_add(x, 1.204414 * y)),
            0.953127f64.mul_add(z, (-0.002079f64).mul_add(x, 0.048952 * y)),
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
        let j =
            100.0 * (ac / viewing_conditions.aw).powf(viewing_conditions.c * viewing_conditions.z);

        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * (viewing_conditions.f_lroot);

        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = (1.0 / 4.0) * ((hue_prime.to_radians() + 2.0).cos() + 3.8);
        let p1 = 50000.0 / 13.0 * e_hue * viewing_conditions.n_c * viewing_conditions.ncb;
        let t = p1 * a.hypot(b) / (u + 0.305);

        let alpha = t.powf(0.9)
            * (1.64 - 0.29f64.powf(viewing_conditions.background_ytowhite_point_y)).powf(0.73);

        // CAM16 chroma, colorfulness, chroma
        let c = alpha * (j / 100.0).sqrt();
        let m = c * viewing_conditions.f_lroot;
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        // CAM16-UCS components
        let (jstar, mstar) = (
            100.0f64.mul_add(0.007, 1.0) * j / 0.007f64.mul_add(j, 1.0),
            (0.0228 * m).ln_1p() / 0.0228,
        );

        let (astar, bstar) = (mstar * hue_radians.cos(), mstar * hue_radians.sin());

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

    /// Create a CAM16 color from lightness `j`, chroma `c`, and hue `h`,
    /// assuming the color was viewed in default viewing conditions.
    pub fn from_jch(j: f64, c: f64, h: f64) -> Self {
        Self::from_jch_in_viewing_conditions(j, c, h, &ViewingConditions::s_rgb())
    }

    /// Create a CAM16 color from lightness `j`, chroma `c`, and hue `h`,
    /// in `viewing_conditions`.
    pub fn from_jch_in_viewing_conditions(
        j: f64,
        c: f64,
        h: f64,
        viewing_conditions: &ViewingConditions,
    ) -> Self {
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * (viewing_conditions.f_lroot);
        let m = c * viewing_conditions.f_lroot;
        let alpha = c / (j / 100.0).sqrt();
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        let hue_radians = h.to_radians();
        let (jstar, mstar) = (
            100.0_f64.mul_add(0.007, 1.0) * j / 0.007_f64.mul_add(j, 1.0),
            1.0 / 0.0228 * 0.0228_f64.mul_add(m, 1.0).ln(),
        );

        let (astar, bstar) = (mstar * hue_radians.cos(), mstar * hue_radians.sin());

        Self {
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

    /// Create a CAM16 color from CAM16-UCS coordinates `jstar`, `astar`, `bstar`.
    /// assuming the color was viewed in default viewing conditions.
    pub fn from_ucs(jstar: f64, astar: f64, bstar: f64) -> Self {
        Self::from_ucs_in_viewing_conditions(jstar, astar, bstar, &ViewingConditions::standard())
    }

    /// Create a CAM16 color from CAM16-UCS coordinates `jstar`, `astar`, `bstar`.
    /// in `viewing_conditions`.
    pub fn from_ucs_in_viewing_conditions(
        jstar: f64,
        astar: f64,
        bstar: f64,
        viewing_conditions: &ViewingConditions,
    ) -> Self {
        let a = astar;
        let b = bstar;
        let m = a.hypot(b);
        let m = (m * 0.0228).exp_m1() / 0.0228;
        let c = m / viewing_conditions.f_lroot;
        let h = b.atan2(a) * (180.0 / PI);
        let h = if h < 0.0 { h + 360.0 } else { h };
        let j = jstar / (jstar - 100.0).mul_add(-0.007, 1.0);

        Self::from_jch_in_viewing_conditions(j, c, h, viewing_conditions)
    }

    /// Argb representation of a color, given the color was viewed in
    /// `viewing_conditions`
    pub fn viewed(&self, viewing_conditions: &ViewingConditions) -> Argb {
        let xyz = self.xyz_in_viewing_conditions(viewing_conditions);

        xyz.into()
    }

    /// Xyz representation of CAM16 seen in `viewing_conditions`.
    pub fn xyz_in_viewing_conditions(&self, viewing_conditions: &ViewingConditions) -> Xyz {
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

        let (h_sin, h_cos) = (h_rad.sin(), h_rad.cos());

        let gamma = 23.0 * (p2 + 0.305) * t
            / (108.0 * t).mul_add(h_sin, 23.0f64.mul_add(p1, 11.0 * t * h_cos));
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

impl From<Argb> for Cam16 {
    fn from(argb: Argb) -> Self {
        Self::fromi32_in_viewing_conditions(argb, &ViewingConditions::s_rgb())
    }
}

impl From<Cam16> for Argb {
    fn from(val: Cam16) -> Self {
        val.viewed(&ViewingConditions::s_rgb())
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Argb, hct::Cam16};
    use core::str::FromStr;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_cam16() {
        let result0 = Cam16::from(Argb::from_str("449B3BEE").unwrap());
        let result1 = Cam16::from(Argb::from_str("9AF54BA2").unwrap());
        let result2 = Cam16::from(Argb::from_str("0C56B056").unwrap());
        let result3 = Cam16::from(Argb::from_str("81D2AE51").unwrap());
        let result4 = Cam16::from(Argb::from_str("88B0E2B9").unwrap());
        let result5 = Cam16::from(Argb::from_str("7ECCD39F").unwrap());
        let result6 = Cam16::from(Argb::from_str("A07D168E").unwrap());
        let result7 = Cam16::from(Argb::from_str("1CB60B70").unwrap());
        let result8 = Cam16::from(Argb::from_str("400279E4").unwrap());
        let result9 = Cam16::from(Argb::from_str("DE9DA476").unwrap());

        assert_approx_eq!(f64, result0.hue, 311.42806917590127, epsilon = 1e-7);
        assert_approx_eq!(f64, result0.j, 39.80957637025326, epsilon = 1e-7);
        assert_approx_eq!(f64, result0.q, 98.12583617460575, epsilon = 1e-7);
        assert_approx_eq!(f64, result0.m, 64.10143150621671, epsilon = 1e-7);
        assert_approx_eq!(f64, result0.s, 80.82434221770161, epsilon = 1e-7);
        assert_approx_eq!(f64, result0.jstar, 52.927210914635715, epsilon = 1e-7);
        assert_approx_eq!(f64, result0.astar, 26.14144025259719, epsilon = 1e-7);
        assert_approx_eq!(f64, result0.bstar, -29.622376253821233, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.hue, 355.0503461678604, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.j, 52.56866623390567, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.q, 112.75948188554017, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.m, 64.2339418261725, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.s, 75.4754569748874, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.jstar, 65.32748230521139, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.astar, 39.413992608446186, epsilon = 1e-7);
        assert_approx_eq!(f64, result1.bstar, -3.413381791164169, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.hue, 145.62456894249067, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.j, 53.54270205682524, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.q, 113.79933774011006, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.m, 45.67944977111023, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.s, 63.35641059229854, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.jstar, 66.20793233348957, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.astar, -25.83510432830831, epsilon = 1e-7);
        assert_approx_eq!(f64, result2.bstar, 17.67339768662175, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.hue, 89.18218954198817, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.j, 64.64864806089051, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.q, 125.04585955071941, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.m, 31.023158944993195, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.s, 49.809060584658496, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.jstar, 75.66239905009027, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.astar, 0.3348706268561027, epsilon = 1e-7);
        assert_approx_eq!(f64, result3.bstar, 23.45943416825876, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.hue, 154.90292039856698, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.j, 79.40954826675019, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.q, 138.58810463022758, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.m, 24.01419462632291, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.s, 41.62660916534058, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.jstar, 86.76592929927428, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.astar, -17.343486416766375, epsilon = 1e-7);
        assert_approx_eq!(f64, result4.bstar, 8.123204738848699, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.hue, 119.29861501791848, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.j, 76.65379834326399, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.q, 136.16216008227642, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.m, 18.68775872501647, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.s, 37.04677374071979, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.jstar, 84.80635340083987, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.astar, -7.617941092812117, epsilon = 1e-7);
        assert_approx_eq!(f64, result5.bstar, 13.575780288737059, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.hue, 327.9022451708669, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.j, 25.207401197509327, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.q, 78.0824855218106, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.m, 53.16273184281286, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.s, 82.51384599304502, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.jstar, 36.425276182524954, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.astar, 29.499403055932383, epsilon = 1e-7);
        assert_approx_eq!(f64, result6.bstar, -18.50332986780255, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.hue, 355.279570048603, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.j, 33.2614419664756, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.q, 89.69332605634818, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.m, 64.28874467824023, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.s, 84.6617825549819, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.jstar, 45.865567063105644, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.astar, 39.449488663086846, epsilon = 1e-7);
        assert_approx_eq!(f64, result7.bstar, -3.257500355999049, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.hue, 261.1968416808902, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.j, 40.7183615122085, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.q, 99.23953929867855, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.m, 49.66881860103603, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.s, 70.74561810312906, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.jstar, 53.86745346363419, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.astar, -5.083026592209834, epsilon = 1e-7);
        assert_approx_eq!(f64, result8.bstar, -32.82238686945024, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.hue, 119.84832142132542, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.j, 56.17844931089786, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.q, 116.56669043770763, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.m, 17.906925043592874, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.s, 39.19433269789186, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.jstar, 68.547225856322, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.astar, -7.47360894560527, epsilon = 1e-7);
        assert_approx_eq!(f64, result9.bstar, 13.024174399350978, epsilon = 1e-7);
    }
}
