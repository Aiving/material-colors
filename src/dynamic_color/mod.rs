#![allow(clippy::too_many_arguments)]

use ahash::HashMap;

use crate::{
    contrast::{darker, darker_unsafe, lighter, lighter_unsafe, ratio_of_tones},
    Argb, Hct, TonalPalette,
};

pub use {
    contrast_curve::ContrastCurve, dynamic_scheme::DynamicScheme,
    material_dynamic_colors::MaterialDynamicColors, tone_delta_pair::ToneDeltaPair,
    tone_delta_pair::TonePolarity, variant::Variant,
};

pub mod contrast_curve;
pub mod dynamic_scheme;
pub mod material_dynamic_colors;
pub mod tone_delta_pair;
pub mod variant;

type DynamicSchemeFn<T> = fn(&DynamicScheme) -> T;
type DynamicSchemeFnRef<T> = fn(&DynamicScheme) -> &T;

/// A color that adjusts itself based on UI state provided by `DynamicScheme`.
///
/// This color automatically adjusts to accommodate a desired contrast level, or
/// other adjustments such as differing in light mode versus dark mode, or what
/// the theme is, or what the color that produced the theme is, etc.
///
/// Colors without backgrounds do not change tone when contrast changes. Colors
/// with backgrounds become closer to their background as contrast lowers, and
/// further when contrast increases.
///
/// Prefer the static constructors. They provide a much more simple interface,
/// such as requiring just a hexcode, or just a hexcode and a background.
///
/// Ultimately, each component necessary for calculating a color, adjusting it
/// for a desired contrast level, and ensuring it has a certain lightness/tone
/// difference from another color, is provided by a function that takes a
/// `DynamicScheme` and returns a value. This ensures ultimate flexibility, any
/// desired behavior of a color for any design system, but it usually
/// unnecessary. See the default constructor for more information.
pub struct DynamicColor {
    pub name: String,
    palette: Box<DynamicSchemeFnRef<TonalPalette>>,
    tone: Box<fn(&DynamicScheme) -> f64>,
    is_background: bool,
    background: Option<Box<DynamicSchemeFn<DynamicColor>>>,
    second_background: Option<Box<DynamicSchemeFn<DynamicColor>>>,
    contrast_curve: Option<ContrastCurve>,
    tone_delta_pair: Option<Box<DynamicSchemeFn<ToneDeltaPair>>>,
    _hct_cache: HashMap<DynamicScheme, Hct>,
}

impl DynamicColor {
    /// The base constructor for DynamicColor.
    ///
    /// _Strongly_ prefer using one of the convenience constructors. This class is
    /// arguably too flexible to ensure it can support any scenario. Functional
    /// arguments allow  overriding without risks that come with subclasses.
    ///
    /// For example, the default behavior of adjust tone at max contrast
    /// to be at a 7.0 ratio with its background is principled and
    /// matches accessibility guidance. That does not mean it's the desired
    /// approach for _every_ design system, and every color pairing,
    /// always, in every case.
    ///
    /// - Parameters:
    ///   - name: The name of the dynamic color.
    ///   - palette: Function that provides a TonalPalette given DynamicScheme.
    ///       A TonalPalette is defined by a hue and chroma, so this
    ///       replaces the need to specify hue/chroma. By providing a tonal palette,
    ///       when contrast adjustments are made, intended chroma can be preserved.
    ///   - tone: Function that provides a tone, given a DynamicScheme.
    ///   - isBackground: Whether this dynamic color is a background, with
    ///       some other color as the foreground.
    ///   - background: The background of the dynamic color (as a function of a
    ///       `DynamicScheme`), if it exists.
    ///   - secondBackground: A second background of the dynamic color (as a function
    ///       of a `DynamicScheme`), if it exists.
    ///   - contrastCurve: A `ContrastCurve` object specifying how its contrast
    ///       against its background should behave in various contrast levels options.
    ///   - toneDeltaPair: A `ToneDeltaPair` object specifying a tone delta
    ///       constraint between two colors. One of them must be the color being constructed.
    ///
    /// Unlikely to be useful unless a design system has some distortions
    /// where colors that don't have a background/foreground relationship
    /// don't want to have a formal relationship or a principled value for their
    /// tone distance based on common contrast / tone delta values, yet, want
    /// tone distance.
    pub fn new<T: Into<String>>(
        name: T,
        palette: fn(&DynamicScheme) -> &TonalPalette,
        tone: fn(&DynamicScheme) -> f64,
        is_background: bool,
        background: Option<fn(&DynamicScheme) -> DynamicColor>,
        second_background: Option<fn(&DynamicScheme) -> DynamicColor>,
        contrast_curve: Option<ContrastCurve>,
        tone_delta_pair: Option<fn(&DynamicScheme) -> ToneDeltaPair>,
    ) -> Self {
        Self {
            name: name.into(),
            palette: Box::new(palette),
            tone: Box::new(tone),
            is_background,
            background: background.map(Box::new),
            second_background: second_background.map(Box::new),
            contrast_curve,
            tone_delta_pair: tone_delta_pair.map(Box::new),
            _hct_cache: Default::default(),
        }
    }

    /// Return a Argb integer (i.e. a hex code).
    ///
    /// - Parameter scheme: Defines the conditions of the user interface, for example,
    ///   whether or not it is dark mode or light mode, and what the desired contrast level is.
    /// - Returns: The color as an integer (Argb).
    pub fn get_argb(&mut self, scheme: &DynamicScheme) -> Argb {
        self.get_hct(scheme).into()
    }

    /// - Parameter scheme: Defines the conditions of the user interface, for example,
    ///   whether or not it is dark mode or light mode, and what the desired
    ///   contrast level is.
    /// - Returns: a color, expressed in the HCT color space, that this
    ///   `DynamicColor` is under the conditions in `scheme`.
    pub fn get_hct(&mut self, scheme: &DynamicScheme) -> Hct {
        if let Some(cached_answer) = self._hct_cache.get(scheme) {
            return *cached_answer;
        }

        let answer = (self.palette)(scheme).get_hct(self.get_tone(scheme));

        if self._hct_cache.len() > 4 {
            self._hct_cache.clear();
        }

        self._hct_cache.insert(scheme.clone(), answer);

        answer
    }

    /// - Parameter scheme: Defines the conditions of the user interface, for example,
    ///   whether or not it is dark mode or light mode, and what the desired
    ///   contrast level is.
    /// - Returns: a tone, T in the HCT color space, that this `DynamicColor` is under
    ///   the conditions in `scheme`.
    pub fn get_tone(&self, scheme: &DynamicScheme) -> f64 {
        let decreasing_contrast = scheme.contrast_level < 0.0;

        // Case 1: dual foreground, pair of colors with delta constraint.
        if let Some(tone_delta_pair) = &self.tone_delta_pair {
            let pair = (tone_delta_pair)(scheme);
            let role_a = pair.subject;
            let role_b = pair.basis;
            let delta = pair.delta;
            let polarity = pair.polarity;
            let stay_together = pair.stay_together;

            let bg = self.background.as_ref().unwrap()(scheme);
            let bg_tone = bg.get_tone(scheme);

            let a_is_nearer = polarity == TonePolarity::Nearer
                || (polarity == TonePolarity::Lighter && !scheme.is_dark)
                || (polarity == TonePolarity::Darker && scheme.is_dark);
            let nearer = if a_is_nearer { &role_a } else { &role_b };
            let farther = if a_is_nearer { &role_b } else { &role_a };
            let am_nearer = self.name == nearer.name;
            let expansion_dir = if scheme.is_dark { 1.0 } else { -1.0 };

            // 1st round: solve to min, each
            let n_contrast = nearer
                .contrast_curve
                .as_ref()
                .unwrap()
                .get(scheme.contrast_level);
            let f_contrast = farther
                .contrast_curve
                .as_ref()
                .unwrap()
                .get(scheme.contrast_level);

            // If a color is good enough, it is not adjusted.
            // Initial and adjusted tones for `nearer`
            let n_initial_tone = (nearer.tone)(scheme);
            let mut n_tone = if ratio_of_tones(bg_tone, n_initial_tone) >= n_contrast {
                n_initial_tone
            } else {
                DynamicColor::foreground_tone(bg_tone, n_contrast)
            };
            // Initial and adjusted tones for `farther`
            let f_initial_tone = (farther.tone)(scheme);
            let mut f_tone = if ratio_of_tones(bg_tone, f_initial_tone) >= f_contrast {
                f_initial_tone
            } else {
                DynamicColor::foreground_tone(bg_tone, f_contrast)
            };

            if decreasing_contrast {
                // If decreasing contrast, adjust color to the "bare minimum"
                // that satisfies contrast.
                n_tone = DynamicColor::foreground_tone(bg_tone, n_contrast);
                f_tone = DynamicColor::foreground_tone(bg_tone, f_contrast);
            }

            if (f_tone - n_tone) * expansion_dir >= delta {
                // Good! Tones satisfy the constraint; no change needed.
            } else {
                // 2nd round: expand farther to match delta.
                f_tone = delta.mul_add(expansion_dir, n_tone).clamp(0.0, 100.0);

                if (f_tone - n_tone) * expansion_dir >= delta {
                    // Good! Tones now satisfy the constraint; no change needed.
                } else {
                    // 3rd round: contract nearer to match delta.
                    n_tone = delta.mul_add(-expansion_dir, f_tone).clamp(0.0, 100.0);
                }
            }

            // Avoids the 50-59 awkward zone.
            if (50.0..60.0).contains(&n_tone) {
                // If `nearer` is in the awkward zone, move it away, together with
                // `farther`.
                if expansion_dir > 0.0 {
                    n_tone = 60.0;
                    f_tone = f_tone.max(delta.mul_add(expansion_dir, n_tone));
                } else {
                    n_tone = 49.0;
                    f_tone = f_tone.min(delta.mul_add(expansion_dir, n_tone));
                }
            } else if (50.0..60.0).contains(&f_tone) {
                if stay_together {
                    // Fixes both, to avoid two colors on opposite sides of the "awkward
                    // zone".
                    if expansion_dir > 0.0 {
                        n_tone = 60.0;
                        f_tone = f_tone.max(delta.mul_add(expansion_dir, n_tone));
                    } else {
                        n_tone = 49.0;
                        f_tone = f_tone.min(delta.mul_add(expansion_dir, n_tone));
                    }
                } else {
                    // Not required to stay together; fixes just one.
                    if expansion_dir > 0.0 {
                        f_tone = 60.0;
                    } else {
                        f_tone = 49.0;
                    }
                }
            }

            // Returns `nTone` if this color is `nearer`, otherwise `fTone`.
            if am_nearer {
                n_tone
            } else {
                f_tone
            }
        } else {
            // Case 2: No contrast pair; just solve for itself.
            let mut answer = (self.tone)(scheme);

            if let Some(background) = &self.background {
                let bg_tone = background(scheme).get_tone(scheme);

                let desired_ratio = self
                    .contrast_curve
                    .as_ref()
                    .unwrap()
                    .get(scheme.contrast_level);

                if ratio_of_tones(bg_tone, answer) >= desired_ratio {
                    // Don't "improve" what's good enough.
                } else {
                    // Rough improvement.
                    answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
                }

                if decreasing_contrast {
                    answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
                }

                if self.is_background && (50.0..60.0).contains(&answer) {
                    // Must adjust
                    if ratio_of_tones(49.0, bg_tone) >= desired_ratio {
                        answer = 49.0;
                    } else {
                        answer = 60.0;
                    }
                }

                if let Some(second_background) = &self.second_background {
                    // Case 3: Adjust for dual backgrounds.

                    let bg_tone1 = self.background.as_ref().unwrap()(scheme).get_tone(scheme);
                    let bg_tone2 = second_background(scheme).get_tone(scheme);

                    let upper = bg_tone1.max(bg_tone2);
                    let lower = bg_tone1.min(bg_tone2);

                    if ratio_of_tones(upper, answer) >= desired_ratio
                        && ratio_of_tones(lower, answer) >= desired_ratio
                    {
                        return answer;
                    }

                    // The darkest light tone that satisfies the desired ratio,
                    // or -1 if such ratio cannot be reached.
                    let light_option = lighter(upper, desired_ratio);

                    // The lightest dark tone that satisfies the desired ratio,
                    // or -1 if such ratio cannot be reached.
                    let dark_option = darker(lower, desired_ratio);

                    // Tones suitable for the foreground.
                    let mut availables: Vec<f64> = vec![];

                    if light_option != -1.0 {
                        availables.push(light_option);
                    }
                    if dark_option != -1.0 {
                        availables.push(dark_option);
                    }

                    let prefers_light = DynamicColor::tone_prefers_light_foreground(bg_tone1)
                        || DynamicColor::tone_prefers_light_foreground(bg_tone2);

                    if prefers_light {
                        return if light_option < 0.0 {
                            100.0
                        } else {
                            light_option
                        };
                    }

                    if availables.len() == 1 {
                        return availables[0];
                    }

                    return if dark_option < 0.0 { 0.0 } else { dark_option };
                }
            }

            answer
        }
    }

    /// Given a background tone, find a foreground tone, while ensuring they reach
    /// a contrast ratio that is as close to `ratio` as possible.
    ///
    /// - Parameters:
    ///   - bgTone: Tone in HCT. Range is 0 to 100, undefined behavior when it falls
    ///     outside that range.
    ///   - ratio: The contrast ratio desired between `bgTone` and the return value.
    ///
    /// - Returns: The desired foreground tone.
    pub fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
        let lighter_tone = lighter_unsafe(bg_tone, ratio);
        let darker_tone = darker_unsafe(bg_tone, ratio);
        let lighter_ratio = ratio_of_tones(lighter_tone, bg_tone);
        let darker_ratio = ratio_of_tones(darker_tone, bg_tone);
        let prefer_ligher = Self::tone_prefers_light_foreground(bg_tone);

        if prefer_ligher {
            // This handles an edge case where the initial contrast ratio is high
            // (ex. 13.0), and the ratio passed to the function is that high ratio,
            // and both the lighter and darker ratio fails to pass that ratio.
            //
            // This was observed with Tonal Spot's On Primary Container turning black
            // momentarily between high and max contrast in light mode.
            // PC's standard tone was T90, OPC's was T10, it was light mode, and the
            // contrast value was 0.6568521221032331.
            let negligible_difference = (lighter_ratio - darker_ratio).abs() < 0.1
                && lighter_ratio < ratio
                && darker_ratio < ratio;

            if lighter_ratio >= ratio || lighter_ratio >= darker_ratio || negligible_difference {
                lighter_tone
            } else {
                darker_tone
            }
        } else if darker_ratio >= ratio || darker_ratio >= lighter_ratio {
            darker_tone
        } else {
            lighter_tone
        }
    }

    /// Adjusts a tone such that white has 4.5 contrast, if the tone is
    /// reasonably close to supporting it.
    /// - Parameter tone: The tone to be adjusted.
    /// - Returns: The (possibly updated) tone.
    pub fn enable_light_foreground(tone: f64) -> f64 {
        if Self::tone_prefers_light_foreground(tone) && !Self::tone_allows_light_foreground(tone) {
            return 49.0;
        }

        tone
    }

    /// Returns whether `tone` prefers a light foreground.
    ///
    /// People prefer white foregrounds on ~T60-70. Observed over time, and also
    /// by Andrew Somers during research for APCA.
    ///
    /// T60 used as to create the smallest discontinuity possible when skipping
    /// down to T49 in order to ensure light foregrounds.
    ///
    /// Since `tertiaryContainer` in dark monochrome scheme requires a tone of
    /// 60, it should not be adjusted. Therefore, 60 is excluded here.
    ///
    /// - Parameter tone: The tone to be judged.
    /// - Returns: whether `tone` prefers a light foreground.
    pub fn tone_prefers_light_foreground(tone: f64) -> bool {
        tone.round() < 60.0
    }

    /// Returns whether `tone` can reach a contrast ratio of 4.5 with a lighter
    /// color.
    ///
    /// - Parameter tone: The tone to be judged.
    /// - Returns: whether `tone` allows a light foreground.
    pub fn tone_allows_light_foreground(tone: f64) -> bool {
        tone.round() <= 49.0
    }
}
