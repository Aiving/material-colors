#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::{lstar_from_y, Argb},
    utils::FromRef,
};
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};
#[cfg(feature = "serde")]
use serde::Serialize;
pub use {cam16::Cam16, solver::HctSolver, viewing_conditions::ViewingConditions};

pub mod cam16;
pub mod solver;
pub mod viewing_conditions;

#[derive(Default, Clone, Copy, Debug, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Hct {
    _hue: f64,
    _chroma: f64,
    _tone: f64,
    _argb: Argb,
}

impl Hct {
    /// A number, in degrees, representing ex. red, orange, yellow, etc.
    /// Ranges from 0 <= `hue` < 360
    ///
    /// 0 <= `new_hue` < 360; invalid values are corrected.
    /// After setting hue, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_hue(&self) -> f64 {
        self._hue
    }

    /// A number, in degrees, representing ex. red, orange, yellow, etc.
    /// Ranges from 0 <= `hue` < 360
    ///
    /// 0 <= `new_hue` < 360; invalid values are corrected.
    /// After setting hue, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_hue(&mut self, value: f64) {
        self._argb = HctSolver::solve_to_argb(value, self.get_chroma(), self.get_tone());

        let cam16 = Cam16::from(self._argb);

        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = self._argb.as_lstar();
    }

    /// 0 <= `new_chroma` <= ?
    /// After setting chroma, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_chroma(&self) -> f64 {
        self._chroma
    }

    /// 0 <= `new_chroma` <= ?
    /// After setting chroma, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_chroma(&mut self, value: f64) {
        self._argb = HctSolver::solve_to_argb(self.get_hue(), value, self.get_tone());

        let cam16 = Cam16::from(self._argb);

        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = self._argb.as_lstar();
    }

    /// Lightness. Ranges from 0 to 100.
    ///
    /// 0 <= `new_tone` <= 100; invalid values are corrected.
    /// After setting tone, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_tone(&self) -> f64 {
        self._tone
    }

    /// Lightness. Ranges from 0 to 100.
    ///
    /// 0 <= `new_tone` <= 100; invalid values are corrected.
    /// After setting tone, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_tone(&mut self, value: f64) {
        self._argb = HctSolver::solve_to_argb(self.get_hue(), self.get_chroma(), value);

        let cam16 = Cam16::from(self._argb);

        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = self._argb.as_lstar();
    }

    pub fn new(argb: Argb) -> Self {
        let cam16 = Cam16::from(argb);

        Self {
            _hue: cam16.hue,
            _chroma: cam16.chroma,
            _tone: argb.as_lstar(),
            _argb: argb,
        }
    }

    /// 0 <= `hue` < 360; invalid values are corrected.
    /// 0 <= `chroma` <= ?; Informally, colorfulness. The color returned may be
    ///    lower than the requested chroma. Chroma has a different maximum for any
    ///    given hue and tone.
    /// 0 <= `tone` <= 100; informally, lightness. Invalid values are corrected.
    pub fn from(hue: f64, chroma: f64, tone: f64) -> Self {
        let argb = HctSolver::solve_to_argb(hue, chroma, tone);

        Self::new(argb)
    }

    /// Translate a color into different [`ViewingConditions`].
    ///
    /// Colors change appearance. They look different with lights on versus off,
    /// the same color, as in hex code, on white looks different when on black.
    /// This is called color relativity, most famously explicated by Josef Albers
    /// in Interaction of Color.
    ///
    /// In color science, color appearance models can account for this and
    /// calculate the appearance of a color in different settings. HCT is based on
    /// CAM16, a color appearance model, and uses it to make these calculations.
    ///
    /// See [`ViewingConditions`] for parameters affecting color appearance.
    #[must_use]
    pub fn in_viewing_conditions(self, vc: &ViewingConditions) -> Self {
        // 1. Use CAM16 to find Xyz coordinates of color in specified VC.
        let cam16 = Cam16::from(Argb::from(self));
        let viewed_in_vc = cam16.xyz_in_viewing_conditions(vc);

        // 2. Create CAM16 of those Xyz coordinates in default VC.
        let recast_in_vc = Cam16::from_xyz_in_viewing_conditions(
            viewed_in_vc.x,
            viewed_in_vc.y,
            viewed_in_vc.z,
            &ViewingConditions::standard(),
        );

        // 3. Create HCT from:
        // - CAM16 using default VC with Xyz coordinates in specified VC.
        // - L* converted from Y in Xyz coordinates in specified VC.
        Self::from(
            recast_in_vc.hue,
            recast_in_vc.chroma,
            lstar_from_y(viewed_in_vc.y),
        )
    }
}

impl fmt::Display for Hct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "H{} C{} T{}",
            self.get_hue().round(),
            self.get_chroma().round(),
            self.get_tone().round()
        )
    }
}

impl Ord for Hct {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Hct {
    fn eq(&self, other: &Self) -> bool {
        self._argb == other._argb
    }
}

impl Eq for Hct {}

impl Hash for Hct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._hue.to_bits().hash(state);
        self._chroma.to_bits().hash(state);
        self._tone.to_bits().hash(state);
        self._argb.hash(state);
    }
}

impl From<Argb> for Hct {
    fn from(value: Argb) -> Self {
        Self::new(value)
    }
}

impl From<Hct> for Argb {
    fn from(value: Hct) -> Self {
        value._argb
    }
}

impl FromRef<Hct> for Argb {
    fn from_ref(value: &Hct) -> Self {
        value._argb
    }
}

#[cfg(test)]
mod tests {
    use super::{Cam16, Hct, ViewingConditions};
    use crate::color::{y_from_lstar, Argb};
    use ahash::AHasher;
    #[cfg(not(feature = "std"))]
    use alloc::format;
    use core::hash::{Hash, Hasher};
    use float_cmp::{approx_eq, assert_approx_eq};
    #[cfg(feature = "std")]
    use std::format;

    const BLACK: Argb = Argb::from_u32(0xFF000000);
    const WHITE: Argb = Argb::from_u32(0xFFFFFFFF);
    const RED: Argb = Argb::from_u32(0xFFFF0000);
    const GREEN: Argb = Argb::from_u32(0xFF00FF00);
    const BLUE: Argb = Argb::from_u32(0xFF0000FF);
    const MIDGRAY: Argb = Argb::from_u32(0xFF777777);

    fn hash_value<T: Hash>(value: T) -> u64 {
        let mut hasher = AHasher::default();

        value.hash(&mut hasher);

        hasher.finish()
    }

    const fn color_is_on_boundary(argb: Argb) -> bool {
        argb.red == 0
            || argb.red == 255
            || argb.green == 0
            || argb.green == 255
            || argb.blue == 0
            || argb.blue == 255
    }

    #[test]
    fn test_hash_code() {
        let a: Hct = Argb::from_u32(123).into();
        let b: Hct = Argb::from_u32(123).into();

        assert_eq!(a, b);
        assert_eq!(hash_value(a), hash_value(b));
    }

    #[test]
    fn test_conversions_are_reflexive() {
        let cam = Cam16::from(RED);
        let color = cam.viewed(&ViewingConditions::standard());

        assert_eq!(color, RED);
    }

    #[test]
    fn test_ymidgray() {
        assert_approx_eq!(f64, 18.418, y_from_lstar(50.0), epsilon = 0.001);
    }

    #[test]
    fn test_yblack() {
        assert_approx_eq!(f64, 0.0, y_from_lstar(0.0), epsilon = 0.001);
    }

    #[test]
    fn test_ywhite() {
        assert_approx_eq!(f64, 100.0, y_from_lstar(100.0), epsilon = 0.001);
    }

    #[test]
    fn test_cam_red() {
        let cam = Cam16::from(RED);

        assert_approx_eq!(f64, 46.445, cam.j, epsilon = 0.001);
        assert_approx_eq!(f64, 113.357, cam.chroma, epsilon = 0.001);
        assert_approx_eq!(f64, 27.408, cam.hue, epsilon = 0.001);
        assert_approx_eq!(f64, 89.494, cam.m, epsilon = 0.001);
        assert_approx_eq!(f64, 91.889, cam.s, epsilon = 0.001);
        assert_approx_eq!(f64, 105.988, cam.q, epsilon = 0.001);
    }

    #[test]
    fn test_cam_green() {
        let cam = Cam16::from(GREEN);

        assert_approx_eq!(f64, 79.331, cam.j, epsilon = 0.001);
        assert_approx_eq!(f64, 108.410, cam.chroma, epsilon = 0.001);
        assert_approx_eq!(f64, 142.139, cam.hue, epsilon = 0.001);
        assert_approx_eq!(f64, 85.587, cam.m, epsilon = 0.001);
        assert_approx_eq!(f64, 78.604, cam.s, epsilon = 0.001);
        assert_approx_eq!(f64, 138.520, cam.q, epsilon = 0.001);
    }

    #[test]
    fn test_cam_blue() {
        let cam = Cam16::from(BLUE);

        assert_approx_eq!(f64, 25.465, cam.j, epsilon = 0.001);
        assert_approx_eq!(f64, 87.230, cam.chroma, epsilon = 0.001);
        assert_approx_eq!(f64, 282.788, cam.hue, epsilon = 0.001);
        assert_approx_eq!(f64, 68.867, cam.m, epsilon = 0.001);
        assert_approx_eq!(f64, 93.674, cam.s, epsilon = 0.001);
        assert_approx_eq!(f64, 78.481, cam.q, epsilon = 0.001);
    }

    #[test]
    fn test_cam_black() {
        let cam = Cam16::from(BLACK);

        assert_approx_eq!(f64, 0.0, cam.j, epsilon = 0.001);
        assert_approx_eq!(f64, 0.0, cam.chroma, epsilon = 0.001);
        assert_approx_eq!(f64, 0.0, cam.hue, epsilon = 0.001);
        assert_approx_eq!(f64, 0.0, cam.m, epsilon = 0.001);
        assert_approx_eq!(f64, 0.0, cam.s, epsilon = 0.001);
        assert_approx_eq!(f64, 0.0, cam.q, epsilon = 0.001);
    }

    #[test]
    fn test_cam_white() {
        let cam = Cam16::from(WHITE);

        assert_approx_eq!(f64, 100.0, cam.j, epsilon = 0.001);
        assert_approx_eq!(f64, 2.869, cam.chroma, epsilon = 0.001);
        assert_approx_eq!(f64, 209.492, cam.hue, epsilon = 0.001);
        assert_approx_eq!(f64, 2.265, cam.m, epsilon = 0.001);
        assert_approx_eq!(f64, 12.068, cam.s, epsilon = 0.001);
        assert_approx_eq!(f64, 155.521, cam.q, epsilon = 0.001);
    }

    #[test]
    fn test_camut_map_red() {
        let color_to_test = RED;
        let cam = Cam16::from(color_to_test);
        let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

        assert_eq!(color_to_test, color);
    }

    #[test]
    fn test_camut_map_green() {
        let color_to_test = GREEN;
        let cam = Cam16::from(color_to_test);
        let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

        assert_eq!(color_to_test, color);
    }

    #[test]
    fn test_camut_map_blue() {
        let color_to_test = BLUE;
        let cam = Cam16::from(color_to_test);
        let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

        assert_eq!(color_to_test, color);
    }

    #[test]
    fn test_camut_map_white() {
        let color_to_test = WHITE;
        let cam = Cam16::from(color_to_test);
        let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

        assert_eq!(color_to_test, color);
    }

    #[test]
    fn test_camut_map_midgray() {
        let color_to_test = MIDGRAY;
        let cam = Cam16::from(color_to_test);
        let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

        assert_eq!(color_to_test, color);
    }

    #[test]
    fn test_camut_map_black() {
        let color_to_test = BLACK;
        let cam = Cam16::from(color_to_test);
        let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

        assert_eq!(color_to_test, color);
    }

    #[test]
    fn test_hct_returns_sufficiently_close_color() {
        for hue in (15..361).step_by(30) {
            for chroma in (0..100).step_by(10) {
                for tone in (20..80).step_by(10) {
                    let hct_request_description = format!("H{hue} C{chroma} T{tone}");
                    let hct_color = Hct::from(f64::from(hue), f64::from(chroma), f64::from(tone));

                    if chroma > 0 {
                        assert!(
                            approx_eq!(f64, hct_color.get_hue(), f64::from(hue), epsilon = 4.0),
                            "Hue should be close for {hct_request_description}"
                        );
                    }

                    assert!(
                        (0.0..(f64::from(chroma) + 2.5)).contains(&hct_color.get_chroma()),
                        "Chroma should be close or less for {hct_request_description}"
                    );

                    if hct_color.get_chroma() < f64::from(chroma) - 2.5 {
                        assert!(
                            color_is_on_boundary(hct_color.into()),
                            "HCT request for non-sRGB color should return a color on the boundary of the sRGB cube for {hct_request_description}, but got {} instead",
                            Argb::from(hct_color).to_hex_with_pound()
                        );
                    }

                    assert!(
                        approx_eq!(f64, hct_color.get_tone(), f64::from(tone), epsilon = 0.5),
                        "Tone should be close for {hct_request_description}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_cam16_to_xyz_without_array() {
        let color_to_test = RED;
        let cam = Cam16::from(color_to_test);
        let xyz = cam.xyz_in_viewing_conditions(&ViewingConditions::s_rgb());

        assert_approx_eq!(f64, xyz.x, 41.23, epsilon = 0.01);
        assert_approx_eq!(f64, xyz.y, 21.26, epsilon = 0.01);
        assert_approx_eq!(f64, xyz.z, 1.93, epsilon = 0.01);
    }

    #[test]
    fn test_color_relativity_red_in_black() {
        let color_to_test = RED;
        let hct: Hct = color_to_test.into();

        let result =
            hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF9F5C51));
    }

    #[test]
    fn test_color_relativity_red_in_white() {
        let color_to_test = RED;
        let hct: Hct = color_to_test.into();

        let result = hct.in_viewing_conditions(&ViewingConditions::make(
            None,
            None,
            Some(100.0),
            None,
            None,
        ));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFFFF5D48));
    }

    #[test]
    fn test_color_relativity_green_in_black() {
        let color_to_test = GREEN;
        let hct: Hct = color_to_test.into();

        let result =
            hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFFACD69D));
    }

    #[test]
    fn test_color_relativity_green_in_white() {
        let color_to_test = GREEN;
        let hct: Hct = color_to_test.into();

        let result = hct.in_viewing_conditions(&ViewingConditions::make(
            None,
            None,
            Some(100.0),
            None,
            None,
        ));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF8EFF77));
    }

    #[test]
    fn test_color_relativity_blue_in_black() {
        let color_to_test = BLUE;
        let hct: Hct = color_to_test.into();

        let result =
            hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF343654));
    }

    #[test]
    fn test_color_relativity_blue_in_white() {
        let color_to_test = BLUE;
        let hct: Hct = color_to_test.into();

        let result = hct.in_viewing_conditions(&ViewingConditions::make(
            None,
            None,
            Some(100.0),
            None,
            None,
        ));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF3F49FF));
    }

    #[test]
    fn test_color_relativity_white_in_black() {
        let color_to_test = WHITE;
        let hct: Hct = color_to_test.into();

        let result =
            hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFFFFFFFF));
    }

    #[test]
    fn test_color_relativity_white_in_white() {
        let color_to_test = WHITE;
        let hct: Hct = color_to_test.into();

        let result = hct.in_viewing_conditions(&ViewingConditions::make(
            None,
            None,
            Some(100.0),
            None,
            None,
        ));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFFFFFFFF));
    }

    #[test]
    fn test_color_relativity_midgray_in_black() {
        let color_to_test = MIDGRAY;
        let hct: Hct = color_to_test.into();

        let result =
            hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF605F5F));
    }

    #[test]
    fn test_color_relativity_midgray_in_white() {
        let color_to_test = MIDGRAY;
        let hct: Hct = color_to_test.into();

        let result = hct.in_viewing_conditions(&ViewingConditions::make(
            None,
            None,
            Some(100.0),
            None,
            None,
        ));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF8E8E8E));
    }

    #[test]
    fn test_color_relativity_black_in_black() {
        let color_to_test = BLACK;
        let hct: Hct = color_to_test.into();

        let result =
            hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF000000));
    }

    #[test]
    fn test_color_relativity_black_in_white() {
        let color_to_test = BLACK;
        let hct: Hct = color_to_test.into();

        let result = hct.in_viewing_conditions(&ViewingConditions::make(
            None,
            None,
            Some(100.0),
            None,
            None,
        ));

        assert_eq!(Argb::from(result), Argb::from_u32(0xFF000000));
    }
}
