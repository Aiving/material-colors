use super::DynamicColor;

/// Describes the different in tone between colors. If there is no preference,
/// the tones at standard contrast are examined and the polarity of those is
/// attempted to be maintained.
#[derive(PartialEq, Eq)]
pub enum TonePolarity {
    Darker,
    Lighter,
    Nearer,
    Farther,
}

/// Documents a constraint between two `DynamicColor`s, in which their tones must
/// have a certain distance from each other. Prefer a `DynamicColor` with a
/// background, this is for special cases when designers want tonal distance,
/// literally contrast, between two colors that don't have a background /
/// foreground relationship or a contrast guarantee.
pub struct ToneDeltaPair {
    pub subject: DynamicColor,
    pub basis: DynamicColor,
    pub delta: f64,
    pub polarity: TonePolarity,
    pub stay_together: bool,
}

impl ToneDeltaPair {
    /// Documents a constraint in tone distance between two `DynamicColor`s.
    ///
    /// The polarity is an adjective that describes "A", compared to "B".
    ///
    /// For instance, ToneDeltaPair(A, B, 15, 'darker', stayTogether) states that
    /// A's tone should be at least 15 darker than B's.
    ///
    /// 'nearer' and 'farther' describes closeness to the surface roles. For
    /// instance, ToneDeltaPair(A, B, 10, 'nearer', stayTogether) states that A
    /// should be 10 lighter than B in light mode, and 10 darker than B in dark
    /// mode.
    ///
    /// # Arguments
    ///
    /// * `subject`: The color role to be judged.
    /// * `basis`: The role used as a basis of comparison.
    /// * `delta`: Required difference between tones. Absolute value, negative
    ///   values have undefined behavior.
    /// * `polarity`: The relative relation between tones of subject and basis,
    ///   as described above.
    /// * `stayTogether`: Whether these two roles should stay on the same side of
    ///   the "awkward zone" (T50-59). This is necessary for certain cases where
    ///   one role has two backgrounds.
    pub const fn new(
        subject: DynamicColor,
        basis: DynamicColor,
        delta: f64,
        polarity: TonePolarity,
        stay_together: bool,
    ) -> Self {
        Self {
            subject,
            basis,
            delta,
            polarity,
            stay_together,
        }
    }
}
