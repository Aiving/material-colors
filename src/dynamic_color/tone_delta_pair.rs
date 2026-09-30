use super::DynamicColor;

/// Describes how to fulfill a tone delta pair constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeltaConstraint {
    /// The tone of `role_a` must be an exact delta away from the tone of
    /// `role_b`.
    Exact,
    /// The tonal distance of `role_a` and `role_b` must be at most delta.
    Nearer,
    /// The tonal distance of `role_a` and `role_b` must be at least delta.
    Farther,
}

/// Describes the relationship in lightness between two colors.
///
/// `RelativeDarker`/`RelativeLighter` are relative to the surface trend (white
/// in light mode, black in dark mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TonePolarity {
    Darker,
    Lighter,
    RelativeDarker,
    RelativeLighter,
}

/// Documents a constraint between two `DynamicColor`s, in which their tones
/// must have a certain distance from each other.
///
/// `polarity` describes `role_a` compared to `role_b`: `ToneDeltaPair(A, B, 15,
/// Darker)` states that A's tone should be at least 15 darker than B's.
///
/// Prefer a `DynamicColor` with a background, this is for special cases when
/// designers want tonal distance, literally contrast, between two colors that
/// don't have a background / foreground relationship or a contrast guarantee.
#[derive(Debug, Clone, Copy)]
pub struct ToneDeltaPair<'a> {
    pub role_a: DynamicColor<'a>,
    pub role_b: DynamicColor<'a>,
    /// Required difference between tones. Absolute value, negative values
    /// have undefined behavior.
    pub delta: f64,
    pub polarity: TonePolarity,
    /// Whether these two roles should stay on the same side of the "awkward
    /// zone" (T50-59). This is necessary for certain cases where one role has
    /// two backgrounds.
    pub stay_together: bool,
    pub constraint: DeltaConstraint,
}

impl<'a> ToneDeltaPair<'a> {
    /// Defaults: `stay_together = true`, `constraint = Exact`.
    pub const fn new(role_a: DynamicColor<'a>, role_b: DynamicColor<'a>, delta: f64, polarity: TonePolarity) -> Self {
        Self {
            role_a,
            role_b,
            delta,
            polarity,
            stay_together: true,
            constraint: DeltaConstraint::Exact,
        }
    }

    #[must_use]
    pub const fn with_stay_together(mut self, stay_together: bool) -> Self {
        self.stay_together = stay_together;
        self
    }

    #[must_use]
    pub const fn with_constraint(mut self, constraint: DeltaConstraint) -> Self {
        self.constraint = constraint;
        self
    }
}
