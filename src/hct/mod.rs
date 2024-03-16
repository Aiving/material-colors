use std::{
    fmt,
    hash::{Hash, Hasher},
};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::color::{lstar_from_y, Argb};

pub use {cam16::Cam16, solver::HctSolver, viewing_conditions::ViewingConditions};

pub mod cam16;
pub mod solver;
pub mod viewing_conditions;

#[derive(Clone, Copy, Debug, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Hct {
    _hue: f64,
    _chroma: f64,
    _tone: f64,
    _argb: Argb,
}

impl Hct {
    /// A number, in degrees, representing ex. red, orange, yellow, etc.
    /// Ranges from 0 <= [hue] < 360
    ///
    /// 0 <= [newHue] < 360; invalid values are corrected.
    /// After setting hue, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_hue(&self) -> f64 {
        self._hue
    }

    /// A number, in degrees, representing ex. red, orange, yellow, etc.
    /// Ranges from 0 <= [hue] < 360
    ///
    /// 0 <= [newHue] < 360; invalid values are corrected.
    /// After setting hue, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_hue(&mut self, value: f64) {
        self._argb = HctSolver::solve_to_int(value, self.get_chroma(), self.get_tone());

        let cam16 = Cam16::from(self._argb);

        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = self._argb.as_lstar();
    }

    /// 0 <= [newChroma] <= ?
    /// After setting chroma, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_chroma(&self) -> f64 {
        self._chroma
    }

    /// 0 <= [newChroma] <= ?
    /// After setting chroma, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_chroma(&mut self, value: f64) {
        self._argb = HctSolver::solve_to_int(self.get_hue(), value, self.get_tone());

        let cam16 = Cam16::from(self._argb);

        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = self._argb.as_lstar();
    }

    /// Lightness. Ranges from 0 to 100.
    ///
    /// 0 <= [newTone] <= 100; invalid values are corrected.
    /// After setting tone, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_tone(&self) -> f64 {
        self._tone
    }

    /// Lightness. Ranges from 0 to 100.
    ///
    /// 0 <= [newTone] <= 100; invalid values are corrected.
    /// After setting tone, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Argb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_tone(&mut self, value: f64) {
        self._argb = HctSolver::solve_to_int(self.get_hue(), self.get_chroma(), value);

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

    /// 0 <= [hue] < 360; invalid values are corrected.
    /// 0 <= [chroma] <= ?; Informally, colorfulness. The color returned may be
    ///    lower than the requested chroma. Chroma has a different maximum for any
    ///    given hue and tone.
    /// 0 <= [tone] <= 100; informally, lightness. Invalid values are corrected.
    pub fn from(hue: f64, chroma: f64, tone: f64) -> Self {
        let argb = HctSolver::solve_to_int(hue, chroma, tone);

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

impl PartialEq for Hct {
    fn eq(&self, other: &Self) -> bool {
        self._argb == other._argb
    }
}

impl Eq for Hct {}

impl Hash for Hct {
    fn hash<H: Hasher>(&self, state: &mut H) {
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
