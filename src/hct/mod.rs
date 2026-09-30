use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

pub use cam16::Cam16;
pub use solver::HctSolver;
pub use viewing_conditions::ViewingConditions;

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::{Rgb, lstar_from_y},
    utils::FromRef,
};

pub mod cam16;
pub mod solver;
pub mod viewing_conditions;

#[derive(Default, Clone, Copy, Debug, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hct {
    hue: f64,
    chroma: f64,
    tone: f64,
    rgb: Rgb,
}

impl Hct {
    pub const fn is_blue(hue: f64) -> bool {
        hue >= 250.0 && hue < 270.0
    }

    pub const fn is_yellow(hue: f64) -> bool {
        hue >= 105.0 && hue < 125.0
    }

    pub const fn is_cyan(hue: f64) -> bool {
        hue >= 170.0 && hue < 207.0
    }

    /// A number, in degrees, representing ex. red, orange, yellow, etc.
    /// Ranges from 0 <= `hue` < 360
    ///
    /// 0 <= `new_hue` < 360; invalid values are corrected.
    /// After setting hue, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Rgb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_hue(&self) -> f64 {
        self.hue
    }

    /// A number, in degrees, representing ex. red, orange, yellow, etc.
    /// Ranges from 0 <= `hue` < 360
    ///
    /// 0 <= `new_hue` < 360; invalid values are corrected.
    /// After setting hue, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Rgb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_hue(&mut self, value: f64) {
        self.rgb = HctSolver::solve_to_rgb(value, self.get_chroma(), self.get_tone());

        let cam16 = Cam16::from(self.rgb);

        self.hue = cam16.hue;
        self.chroma = cam16.chroma;
        self.tone = self.rgb.as_lstar();
    }

    /// 0 <= `new_chroma` <= ?
    /// After setting chroma, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Rgb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_chroma(&self) -> f64 {
        self.chroma
    }

    /// 0 <= `new_chroma` <= ?
    /// After setting chroma, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Rgb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_chroma(&mut self, value: f64) {
        self.rgb = HctSolver::solve_to_rgb(self.get_hue(), value, self.get_tone());

        let cam16 = Cam16::from(self.rgb);

        self.hue = cam16.hue;
        self.chroma = cam16.chroma;
        self.tone = self.rgb.as_lstar();
    }

    /// Lightness. Ranges from 0 to 100.
    ///
    /// 0 <= `new_tone` <= 100; invalid values are corrected.
    /// After setting tone, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Rgb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub const fn get_tone(&self) -> f64 {
        self.tone
    }

    /// Lightness. Ranges from 0 to 100.
    ///
    /// 0 <= `new_tone` <= 100; invalid values are corrected.
    /// After setting tone, the color is mapped from HCT to the more
    /// limited sRgb gamut for display. This will change its Rgb/integer
    /// representation. If the HCT color is outside of the sRgb gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_tone(&mut self, value: f64) {
        self.rgb = HctSolver::solve_to_rgb(self.get_hue(), self.get_chroma(), value);

        let cam16 = Cam16::from(self.rgb);

        self.hue = cam16.hue;
        self.chroma = cam16.chroma;
        self.tone = self.rgb.as_lstar();
    }

    pub fn new(rgb: Rgb) -> Self {
        let cam16 = Cam16::from(rgb);

        Self {
            hue: cam16.hue,
            chroma: cam16.chroma,
            tone: rgb.as_lstar(),
            rgb,
        }
    }

    /// 0 <= `hue` < 360; invalid values are corrected.
    /// 0 <= `chroma` <= ?; Informally, colorfulness. The color returned may be
    ///    lower than the requested chroma. Chroma has a different maximum for
    /// any    given hue and tone.
    /// 0 <= `tone` <= 100; informally, lightness. Invalid values are corrected.
    pub fn from(hue: f64, chroma: f64, tone: f64) -> Self {
        Self::new(HctSolver::solve_to_rgb(hue, chroma, tone))
    }

    /// Translate a color into different [`ViewingConditions`].
    ///
    /// Colors change appearance. They look different with lights on versus off,
    /// the same color, as in hex code, on white looks different when on black.
    /// This is called color relativity, most famously explicated by Josef
    /// Albers in Interaction of Color.
    ///
    /// In color science, color appearance models can account for this and
    /// calculate the appearance of a color in different settings. HCT is based
    /// on CAM16, a color appearance model, and uses it to make these
    /// calculations.
    ///
    /// See [`ViewingConditions`] for parameters affecting color appearance.
    #[must_use]
    pub fn in_viewing_conditions(self, vc: &ViewingConditions) -> Self {
        // 1. Use CAM16 to find Xyz coordinates of color in specified VC.
        let cam16 = Cam16::from(Rgb::from(self));
        let viewed_in_vc = cam16.xyz_in_viewing_conditions(vc);

        // 2. Create CAM16 of those Xyz coordinates in default VC.
        let recast_in_vc = Cam16::from_xyz_in_viewing_conditions(viewed_in_vc, &ViewingConditions::s_rgb());

        // 3. Create HCT from:
        // - CAM16 using default VC with Xyz coordinates in specified VC.
        // - L* converted from Y in Xyz coordinates in specified VC.
        Self::from(recast_in_vc.hue, recast_in_vc.chroma, lstar_from_y(viewed_in_vc.y))
    }
}

impl fmt::Display for Hct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "H{} C{} T{}", self.get_hue().round(), self.get_chroma().round(), self.get_tone().round())
    }
}

impl Ord for Hct {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Hct {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb
    }
}

impl Eq for Hct {}

impl Hash for Hct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hue.to_bits().hash(state);
        self.chroma.to_bits().hash(state);
        self.tone.to_bits().hash(state);
        self.rgb.hash(state);
    }
}

impl From<Rgb> for Hct {
    fn from(value: Rgb) -> Self {
        Self::new(value)
    }
}

impl From<Hct> for Rgb {
    fn from(value: Hct) -> Self {
        value.rgb
    }
}

impl FromRef<Hct> for Rgb {
    fn from_ref(value: &Hct) -> Self {
        value.rgb
    }
}
