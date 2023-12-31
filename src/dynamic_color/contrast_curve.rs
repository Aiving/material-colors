use crate::utils::math::lerp;

/// A class containing a value that changes with the contrast level.
///
/// Usually represents the contrast requirements for a dynamic color on its
/// background. The four values correspond to values for contrast levels
/// -1.0, 0.0, 0.5, and 1.0, respectively.
pub struct ContrastCurve {
    pub low: f64,
    pub normal: f64,
    pub medium: f64,
    pub high: f64,
}

impl ContrastCurve {
    /// Returns the value at a given contrast level.
    ///
    /// - Parameter contrastLevel: The contrast level. 0.0 is the default (normal);
    ///   -1.0 is the lowest; 1.0 is the highest.
    ///
    /// - Returns: The value. For contrast ratios, a number between 1.0 and 21.0.
    pub fn get(&self, contrast_level: f64) -> f64 {
        match contrast_level {
            contrast_level if contrast_level <= -1.0 => self.low,
            contrast_level if contrast_level < 0.0 => {
                lerp(self.low, self.normal, (contrast_level - (-1.0)) / 1.0)
            }
            contrast_level if contrast_level < 0.5 => {
                lerp(self.normal, self.medium, (contrast_level - 0.0) / 0.5)
            }
            contrast_level if contrast_level < 1.0 => {
                lerp(self.medium, self.high, (contrast_level - 0.5) / 0.5)
            }
            _ => self.high,
        }
    }
}
