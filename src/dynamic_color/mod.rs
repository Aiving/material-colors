#![allow(clippy::too_many_arguments)]

#[cfg(not(feature = "std"))] use alloc::{vec, vec::Vec};

pub use contrast_curve::ContrastCurve;
pub use dynamic_scheme::DynamicScheme;
pub use tone_delta_pair::{ToneDeltaPair, TonePolarity};
pub use variant::Variant;

#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::Rgb,
    contrast::{darker_unsafe, lighter_unsafe, ratio_of_tones},
    dynamic_color::{color_spec::SpecVersion, color_spec_2021::ColorSpec2021, color_spec_2025::ColorSpec2025},
    hct::Hct,
    palette::TonalPalette,
};

pub mod color_spec;
pub mod color_spec_2021;
pub mod color_spec_2025;
pub mod color_spec_2026;
pub mod contrast_curve;
pub mod dynamic_scheme;
pub mod material_dynamic_colors;
pub mod tone_delta_pair;
pub mod variant;

type DynamicSchemeFn<T> = fn(Option<ExtendedColorData>, &DynamicScheme) -> T;
type DynamicSchemeFnRef<T> = for<'a> fn(Option<ExtendedColorData>, &'a DynamicScheme) -> &'a T;

#[derive(Clone, Copy)]
pub struct ExtendedColorData {
    pub spec_version: SpecVersion,
    pub it: &'static DynamicColor,
    pub extended_color: &'static DynamicColor,
}

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
#[derive(Clone, Copy)]
pub struct DynamicColor {
    extended_data: Option<ExtendedColorData>,
    pub name: &'static str,
    palette_: DynamicSchemeFnRef<TonalPalette>,
    tone_: DynamicSchemeFn<f64>,
    is_background: bool,
    chroma_multiplier_: DynamicSchemeFn<Option<f64>>,
    background_: DynamicSchemeFn<Option<&'static Self>>,
    second_background_: DynamicSchemeFn<Option<&'static Self>>,
    contrast_curve_: DynamicSchemeFn<Option<ContrastCurve>>,
    tone_delta_pair_: DynamicSchemeFn<Option<ToneDeltaPair>>,
    opacity_: DynamicSchemeFn<Option<f64>>,
}

impl DynamicColor {
    /// The base constructor for `DynamicColor`.
    ///
    /// _Strongly_ prefer using one of the convenience constructors. This class
    /// is arguably too flexible to ensure it can support any scenario.
    /// Functional arguments allow  overriding without risks that come with
    /// subclasses.
    ///
    /// For example, the default behavior of adjust tone at max contrast
    /// to be at a 7.0 ratio with its background is principled and
    /// matches accessibility guidance. That does not mean it's the desired
    /// approach for _every_ design system, and every color pairing,
    /// always, in every case.
    ///
    /// - Parameters:
    ///   - `name`: The name of the dynamic color.
    ///   - `palette`: Function that provides a [`TonalPalette`] given
    ///     [`DynamicScheme`]. A [`TonalPalette`] is defined by a hue and
    ///     chroma, so this replaces the need to specify hue/chroma. By
    ///     providing a tonal palette, when contrast adjustments are made,
    ///     intended chroma can be preserved.
    ///   - `tone`: Function that provides a tone, given a [`DynamicScheme`].
    ///   - `isBackground`: Whether this dynamic color is a background, with
    ///     some other color as the foreground.
    ///   - `background`: The background of the dynamic color (as a function of
    ///     a [`DynamicScheme`]), if it exists.
    ///   - `secondBackground`: A second background of the dynamic color (as a
    ///     function of a [`DynamicScheme`]), if it exists.
    ///   - `contrastCurve`: A [`ContrastCurve`] object specifying how its
    ///     contrast against its background should behave in various contrast
    ///     levels options.
    ///   - `toneDeltaPair`: A [`ToneDeltaPair`] object specifying a tone delta
    ///     constraint between two colors. One of them must be the color being
    ///     constructed.
    ///
    /// Unlikely to be useful unless a design system has some distortions
    /// where colors that don't have a background/foreground relationship
    /// don't want to have a formal relationship or a principled value for their
    /// tone distance based on common contrast / tone delta values, yet, want
    /// tone distance.
    pub const fn foreground_color(name: &'static str, palette: DynamicSchemeFnRef<TonalPalette>, tone: DynamicSchemeFn<f64>) -> Self {
        Self {
            extended_data: None,
            name,
            palette_: palette,
            tone_: tone,
            is_background: false,
            chroma_multiplier_: |_, _| None,
            background_: |_, _| None,
            second_background_: |_, _| None,
            contrast_curve_: |_, _| None,
            tone_delta_pair_: |_, _| None,
            opacity_: |_, _| None,
        }
    }

    pub const fn background_color(name: &'static str, palette: DynamicSchemeFnRef<TonalPalette>, tone: DynamicSchemeFn<f64>) -> Self {
        Self {
            extended_data: None,
            name,
            palette_: palette,
            tone_: tone,
            is_background: true,
            chroma_multiplier_: |_, _| None,
            background_: |_, _| None,
            second_background_: |_, _| None,
            contrast_curve_: |_, _| None,
            tone_delta_pair_: |_, _| None,
            opacity_: |_, _| None,
        }
    }

    pub fn tone(&self, scheme: &DynamicScheme) -> f64 {
        (self.tone_)(self.extended_data, scheme)
    }

    pub fn palette<'a>(&self, scheme: &'a DynamicScheme) -> &'a TonalPalette {
        (self.palette_)(self.extended_data, scheme)
    }

    pub fn background(&self, scheme: &DynamicScheme) -> Option<&'static Self> {
        (self.background_)(self.extended_data, scheme)
    }

    pub fn second_background(&self, scheme: &DynamicScheme) -> Option<&'static Self> {
        (self.second_background_)(self.extended_data, scheme)
    }

    pub fn contrast_curve(&self, scheme: &DynamicScheme) -> Option<ContrastCurve> {
        (self.contrast_curve_)(self.extended_data, scheme)
    }

    pub fn tone_delta_pair(&self, scheme: &DynamicScheme) -> Option<ToneDeltaPair> {
        (self.tone_delta_pair_)(self.extended_data, scheme)
    }

    pub fn chroma_multiplier(&self, scheme: &DynamicScheme) -> Option<f64> {
        (self.chroma_multiplier_)(self.extended_data, scheme)
    }

    pub fn opacity(&self, scheme: &DynamicScheme) -> Option<f64> {
        (self.opacity_)(self.extended_data, scheme)
    }

    #[must_use]
    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;

        self
    }

    #[must_use]
    pub const fn with_tone(mut self, func: DynamicSchemeFn<f64>) -> Self {
        self.tone_ = func;

        self
    }

    #[must_use]
    pub const fn with_palette(mut self, func: DynamicSchemeFnRef<TonalPalette>) -> Self {
        self.palette_ = func;

        self
    }

    #[must_use]
    pub const fn with_background(mut self, func: DynamicSchemeFn<Option<&'static Self>>) -> Self {
        self.background_ = func;

        self
    }

    #[must_use]
    pub const fn with_second_background(mut self, func: DynamicSchemeFn<Option<&'static Self>>) -> Self {
        self.second_background_ = func;

        self
    }

    #[must_use]
    pub const fn with_contrast_curve(mut self, curve: DynamicSchemeFn<Option<ContrastCurve>>) -> Self {
        self.contrast_curve_ = curve;

        self
    }

    #[must_use]
    pub const fn with_tone_delta_pair(mut self, func: DynamicSchemeFn<Option<ToneDeltaPair>>) -> Self {
        self.tone_delta_pair_ = func;

        self
    }

    #[must_use]
    pub const fn with_chroma_multiplier(mut self, func: DynamicSchemeFn<Option<f64>>) -> Self {
        self.chroma_multiplier_ = func;

        self
    }

    #[must_use]
    pub const fn with_opacity(mut self, func: DynamicSchemeFn<Option<f64>>) -> Self {
        self.opacity_ = func;

        self
    }

    /// Given a background tone, find a foreground tone, while ensuring they
    /// reach a contrast ratio that is as close to `ratio` as possible.
    ///
    /// - Parameters:
    ///   - bgTone: Tone in HCT. Range is 0 to 100, undefined behavior when it
    ///     falls outside that range.
    ///   - ratio: The contrast ratio desired between `bgTone` and the return
    ///     value.
    ///
    /// - Returns: The desired foreground tone.
    pub fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
        let lighter_tone = lighter_unsafe(bg_tone, ratio);
        let darker_tone = darker_unsafe(bg_tone, ratio);
        let lighter_ratio = ratio_of_tones(lighter_tone, bg_tone);
        let darker_ratio = ratio_of_tones(darker_tone, bg_tone);
        let prefer_lighter = Self::tone_prefers_light_foreground(bg_tone);

        if prefer_lighter {
            // This handles an edge case where the initial contrast ratio is high
            // (ex. 13.0), and the ratio passed to the function is that high ratio,
            // and both the lighter and darker ratio fails to pass that ratio.
            //
            // This was observed with Tonal Spot's On Primary Container turning black
            // momentarily between high and max contrast in light mode.
            // PC's standard tone was T90, OPC's was T10, it was light mode, and the
            // contrast value was 0.6568521221032331.
            let negligible_difference = (lighter_ratio - darker_ratio).abs() < 0.1 && lighter_ratio < ratio && darker_ratio < ratio;

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

    pub fn get_rgb(&self, scheme: &DynamicScheme) -> Rgb {
        match scheme.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021.get_hct(scheme, self).into(),
            SpecVersion::Spec2025 => ColorSpec2025.get_hct(scheme, self).into(),
            SpecVersion::Spec2026 => todo!(),
        }
    }

    pub fn get_hct(&self, scheme: &DynamicScheme) -> Hct {
        match scheme.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021.get_hct(scheme, self),
            SpecVersion::Spec2025 => ColorSpec2025.get_hct(scheme, self),
            SpecVersion::Spec2026 => todo!(),
        }
    }

    pub fn get_tone(&self, scheme: &DynamicScheme) -> f64 {
        match scheme.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021.get_tone(scheme, self),
            SpecVersion::Spec2025 => ColorSpec2025.get_tone(scheme, self),
            SpecVersion::Spec2026 => todo!(),
        }
    }

    const fn validate_extended_color(&self, _: SpecVersion, extended_color: &'static Self) {
        const fn eq_str(left: &str, right: &str) -> bool {
            let left = left.as_bytes();
            let right = right.as_bytes();

            if left.len() != right.len() {
                return false;
            }

            let mut i = 0;

            while i != left.len() {
                if left[i] != right[i] {
                    return false;
                }

                i += 1;
            }

            true
        }

        debug_assert!(eq_str(self.name, extended_color.name));
        debug_assert!(self.is_background == extended_color.is_background);
    }

    #[must_use]
    pub const fn extend_spec_version(&'static self, spec_version: SpecVersion, extended_color: &'static Self) -> Self {
        self.validate_extended_color(spec_version, extended_color);

        Self {
            extended_data: Some(ExtendedColorData {
                spec_version,
                it: self,
                extended_color,
            }),
            name: self.name,
            palette_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.palette(scheme)
                } else {
                    data.it.palette(scheme)
                }
            },
            tone_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.tone(scheme)
                } else {
                    data.it.tone(scheme)
                }
            },
            is_background: self.is_background,
            chroma_multiplier_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.chroma_multiplier(scheme)
                } else {
                    data.it.chroma_multiplier(scheme)
                }
                .or(Some(1.0))
            },
            background_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.background(scheme)
                } else {
                    data.it.background(scheme)
                }
            },
            second_background_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.second_background(scheme)
                } else {
                    data.it.second_background(scheme)
                }
            },
            contrast_curve_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.contrast_curve(scheme)
                } else {
                    data.it.contrast_curve(scheme)
                }
            },
            tone_delta_pair_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.tone_delta_pair(scheme)
                } else {
                    data.it.tone_delta_pair(scheme)
                }
            },
            opacity_: |data, scheme| {
                let data = unsafe { data.unwrap_unchecked() };

                if scheme.spec_version >= data.spec_version {
                    data.extended_color.opacity(scheme)
                } else {
                    data.it.opacity(scheme)
                }
            },
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

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::{DynamicColor, material_dynamic_colors::MaterialDynamicColors};
    use crate::{
        Map,
        color::Rgb,
        contrast::ratio_of_tones,
        hct::Hct,
        scheme::variant::{SchemeContent, SchemeFidelity, SchemeMonochrome, SchemeTonalSpot},
    };

    #[test]
    fn test_contrast_pairs() {
        let seed_colors: [Hct; 4] = [
            Rgb::from_u32(0xFF0000).into(),
            Rgb::from_u32(0xFFFF00).into(),
            Rgb::from_u32(0x00FF00).into(),
            Rgb::from_u32(0x0000FF).into(),
        ];

        let contrast_levels = [-1.0, -0.5, 0.0, 0.5, 1.0];

        let mut _colors: Map<&str, &'static DynamicColor> = Map::from_iter([
            ("background", MaterialDynamicColors::background()),
            ("onBackground", MaterialDynamicColors::on_background()),
            ("surface", MaterialDynamicColors::surface()),
            ("surfaceDim", MaterialDynamicColors::surface_dim()),
            ("surfaceBright", MaterialDynamicColors::surface_bright()),
            ("surfaceContainerLowest", MaterialDynamicColors::surface_container_lowest()),
            ("surfaceContainerLow", MaterialDynamicColors::surface_container_low()),
            ("surfaceContainer", MaterialDynamicColors::surface_container()),
            ("surfaceContainerHigh", MaterialDynamicColors::surface_container_high()),
            ("surfaceContainerHighest", MaterialDynamicColors::surface_container_highest()),
            ("onSurface", MaterialDynamicColors::on_surface()),
            ("surfaceVariant", MaterialDynamicColors::surface_variant()),
            ("onSurfaceVariant", MaterialDynamicColors::on_surface_variant()),
            ("inverseSurface", MaterialDynamicColors::inverse_surface()),
            ("inverseOnSurface", MaterialDynamicColors::inverse_on_surface()),
            ("outline", MaterialDynamicColors::outline()),
            ("outlineVariant", MaterialDynamicColors::outline_variant()),
            ("shadow", MaterialDynamicColors::shadow()),
            ("scrim", MaterialDynamicColors::scrim()),
            ("surfaceTint", MaterialDynamicColors::surface_tint()),
            ("primary", MaterialDynamicColors::primary()),
            ("onPrimary", MaterialDynamicColors::on_primary()),
            ("primaryContainer", MaterialDynamicColors::primary_container()),
            ("onPrimaryContainer", MaterialDynamicColors::on_primary_container()),
            ("inversePrimary", MaterialDynamicColors::inverse_primary()),
            ("secondary", MaterialDynamicColors::secondary()),
            ("onSecondary", MaterialDynamicColors::on_secondary()),
            ("secondaryContainer", MaterialDynamicColors::secondary_container()),
            ("onSecondaryContainer", MaterialDynamicColors::on_secondary_container()),
            ("tertiary", MaterialDynamicColors::tertiary()),
            ("onTertiary", MaterialDynamicColors::on_tertiary()),
            ("tertiaryContainer", MaterialDynamicColors::tertiary_container()),
            ("onTertiaryContainer", MaterialDynamicColors::on_tertiary_container()),
            ("error", MaterialDynamicColors::error()),
            ("onError", MaterialDynamicColors::on_error()),
            ("errorContainer", MaterialDynamicColors::error_container()),
            ("onErrorContainer", MaterialDynamicColors::on_error_container()),
        ]);

        for color in seed_colors {
            for contrast_level in contrast_levels {
                for is_dark in [false, true] {
                    #[allow(unused_variables)]
                    for (scheme_name, scheme) in [
                        ("SchemeContent", SchemeContent::new(color, is_dark, Some(contrast_level)).scheme),
                        ("SchemeMonochrome", SchemeMonochrome::new(color, is_dark, Some(contrast_level)).scheme),
                        ("SchemeTonalSpot", SchemeTonalSpot::new(color, is_dark, Some(contrast_level)).scheme),
                        ("SchemeFidelity", SchemeFidelity::new(color, is_dark, Some(contrast_level)).scheme),
                    ] {
                        #[cfg(feature = "std")]
                        std::println!("Scheme: {scheme_name}; Seed color: {color}; Contrast level: {contrast_level}; Dark: {is_dark}");

                        for (fg_name, bg_name) in [
                            ("onPrimary", "primary"),
                            ("onPrimaryContainer", "primaryContainer"),
                            ("onSecondary", "secondary"),
                            ("onSecondaryContainer", "secondaryContainer"),
                            ("onTertiary", "tertiary"),
                            ("onTertiaryContainer", "tertiaryContainer"),
                            ("onError", "error"),
                            ("onErrorContainer", "errorContainer"),
                            ("onBackground", "background"),
                            ("onSurfaceVariant", "surfaceBright"),
                            ("onSurfaceVariant", "surfaceDim"),
                        ] {
                            let foreground_tone = _colors.get_mut(fg_name).unwrap().get_hct(&scheme).get_tone();
                            let background_tone = _colors.get_mut(bg_name).unwrap().get_hct(&scheme).get_tone();
                            let contrast = ratio_of_tones(foreground_tone, background_tone);

                            let minimum_requirement = if contrast_level >= 0.0 { 4.5 } else { 3.0 };

                            assert!(
                                contrast >= minimum_requirement,
                                "Contrast {contrast} is too low between foreground ({fg_name}; {foreground_tone}) and ({bg_name}; {background_tone})"
                            );
                        }
                    }
                }
            }
        }
    }

    // Tests for fixed colors.
    #[test]
    fn test_fixed_colors_in_non_monochrome_schemes() {
        let scheme = SchemeTonalSpot::new(Rgb::from_u32(0xFF0000).into(), true, Some(0.0)).scheme;

        assert_approx_eq!(f64, MaterialDynamicColors::primary_fixed().get_hct(&scheme).get_tone(), 90.0, epsilon = 1.0);
        assert_approx_eq!(f64, MaterialDynamicColors::primary_fixed_dim().get_hct(&scheme).get_tone(), 80.0, epsilon = 1.0);
        assert_approx_eq!(f64, MaterialDynamicColors::on_primary_fixed().get_hct(&scheme).get_tone(), 10.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_primary_fixed_variant().get_hct(&scheme).get_tone(),
            30.0,
            epsilon = 1.0
        );
        assert_approx_eq!(f64, MaterialDynamicColors::secondary_fixed().get_hct(&scheme).get_tone(), 90.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::secondary_fixed_dim().get_hct(&scheme).get_tone(),
            80.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_fixed().get_hct(&scheme).get_tone(),
            10.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_fixed_variant().get_hct(&scheme).get_tone(),
            30.0,
            epsilon = 1.0
        );
        assert_approx_eq!(f64, MaterialDynamicColors::tertiary_fixed().get_hct(&scheme).get_tone(), 90.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::tertiary_fixed_dim().get_hct(&scheme).get_tone(),
            80.0,
            epsilon = 1.0
        );
        assert_approx_eq!(f64, MaterialDynamicColors::on_tertiary_fixed().get_hct(&scheme).get_tone(), 10.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary_fixed_variant().get_hct(&scheme).get_tone(),
            30.0,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_fixed_colors_in_light_monochrome_schemes() {
        let scheme = SchemeMonochrome::new(Rgb::from_u32(0xFF0000).into(), false, Some(0.0)).scheme;

        assert_approx_eq!(f64, MaterialDynamicColors::primary_fixed().get_hct(&scheme).get_tone(), 40.0, epsilon = 1.0);
        assert_approx_eq!(f64, MaterialDynamicColors::primary_fixed_dim().get_hct(&scheme).get_tone(), 30.0, epsilon = 1.0);
        assert_approx_eq!(f64, MaterialDynamicColors::on_primary_fixed().get_hct(&scheme).get_tone(), 100.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_primary_fixed_variant().get_hct(&scheme).get_tone(),
            90.0,
            epsilon = 1.0
        );
        assert_approx_eq!(f64, MaterialDynamicColors::secondary_fixed().get_hct(&scheme).get_tone(), 80.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::secondary_fixed_dim().get_hct(&scheme).get_tone(),
            70.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_fixed().get_hct(&scheme).get_tone(),
            10.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_fixed_variant().get_hct(&scheme).get_tone(),
            25.0,
            epsilon = 1.0
        );
        assert_approx_eq!(f64, MaterialDynamicColors::tertiary_fixed().get_hct(&scheme).get_tone(), 40.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::tertiary_fixed_dim().get_hct(&scheme).get_tone(),
            30.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary_fixed().get_hct(&scheme).get_tone(),
            100.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary_fixed_variant().get_hct(&scheme).get_tone(),
            90.0,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_fixed_colors_in_dark_monochrome_schemes() {
        let scheme = SchemeMonochrome::new(Rgb::from_u32(0xFF0000).into(), true, Some(0.0)).scheme;

        assert_approx_eq!(f64, MaterialDynamicColors::primary_fixed().get_hct(&scheme).get_tone(), 40.0, epsilon = 1.0);
        assert_approx_eq!(f64, MaterialDynamicColors::primary_fixed_dim().get_hct(&scheme).get_tone(), 30.0, epsilon = 1.0);
        assert_approx_eq!(f64, MaterialDynamicColors::on_primary_fixed().get_hct(&scheme).get_tone(), 100.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_primary_fixed_variant().get_hct(&scheme).get_tone(),
            90.0,
            epsilon = 1.0
        );
        assert_approx_eq!(f64, MaterialDynamicColors::secondary_fixed().get_hct(&scheme).get_tone(), 80.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::secondary_fixed_dim().get_hct(&scheme).get_tone(),
            70.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_fixed().get_hct(&scheme).get_tone(),
            10.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_secondary_fixed_variant().get_hct(&scheme).get_tone(),
            25.0,
            epsilon = 1.0
        );
        assert_approx_eq!(f64, MaterialDynamicColors::tertiary_fixed().get_hct(&scheme).get_tone(), 40.0, epsilon = 1.0);
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::tertiary_fixed_dim().get_hct(&scheme).get_tone(),
            30.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary_fixed().get_hct(&scheme).get_tone(),
            100.0,
            epsilon = 1.0
        );
        assert_approx_eq!(
            f64,
            MaterialDynamicColors::on_tertiary_fixed_variant().get_hct(&scheme).get_tone(),
            90.0,
            epsilon = 1.0
        );
    }
}
