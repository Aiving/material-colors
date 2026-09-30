//! `ColorSpec` implementation for the 2021 spec.

use super::{
    Context, ContrastCurve, DeltaConstraint, DynamicColor, DynamicScheme, Platform, Role, SpecVersion, ToneCache, ToneDeltaPair, TonePolarity, Variant, abs,
    color_spec::{ColorSpec, Entry, SpecTable, coerce_in, dual_background_tone, highest_surface_fn, spec_table},
    dynamic_scheme::sanitize_degrees,
};
use crate::{contrast::ratio_of_tones, dislike::fix_if_disliked, hct::Hct, palette::TonalPalette, temperature::TemperatureCache};

#[derive(Debug, Clone, Copy, Default)]
pub struct ColorSpec2021;

#[inline]
const fn is_fidelity(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Fidelity | Variant::Content)
}

#[inline]
const fn is_monochrome(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Monochrome)
}

const fn container_pair(a: Role, b: Role) -> ToneDeltaPair<'static> {
    ToneDeltaPair::new(DynamicColor::Material(a), DynamicColor::Material(b), 10.0, TonePolarity::RelativeLighter)
        .with_stay_together(false)
        .with_constraint(DeltaConstraint::Nearer)
}

const fn fixed_pair(a: Role, b: Role) -> ToneDeltaPair<'static> {
    ToneDeltaPair::new(DynamicColor::Material(a), DynamicColor::Material(b), 10.0, TonePolarity::Lighter)
}

static DECLARED: SpecTable = TABLE;

pub const TABLE: SpecTable = spec_table! {
    // Main palettes.
    PrimaryPaletteKeyColor: foreground(Primary) { tone: |scheme| scheme.primary_palette.key_color().get_tone() },
    SecondaryPaletteKeyColor: foreground(Secondary) { tone: |scheme| scheme.secondary_palette.key_color().get_tone() },
    TertiaryPaletteKeyColor: foreground(Tertiary) { tone: |scheme| scheme.tertiary_palette.key_color().get_tone() },
    NeutralPaletteKeyColor: foreground(Neutral) { tone: |scheme| scheme.neutral_palette.key_color().get_tone() },
    NeutralVariantPaletteKeyColor: foreground(NeutralVariant) { tone: |scheme| scheme.neutral_variant_palette.key_color().get_tone() },
    ErrorPaletteKeyColor: foreground(Error) { tone: |scheme| scheme.error_palette.key_color().get_tone() },

    // Surfaces.
    Background: background(Neutral) { tone: |scheme| if scheme.is_dark { 6.0 } else { 98.0 } },
    OnBackground: foreground(Neutral) {
        tone: |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        background: |_| Some(DynamicColor::Material(Role::Background)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 3.0, 4.5, 7.0)),
    },

    Surface: background(Neutral) { tone: |scheme| if scheme.is_dark { 6.0 } else { 98.0 } },
    SurfaceDim: background(Neutral) {
        tone: |scheme| if scheme.is_dark { 6.0 } else { ContrastCurve::new(87.0, 87.0, 80.0, 75.0).get(scheme.contrast_level) },
    },

    SurfaceBright: background(Neutral) {
        tone: |scheme| if scheme.is_dark { ContrastCurve::new(24.0, 24.0, 29.0, 34.0).get(scheme.contrast_level) } else { 98.0 },
    },

    SurfaceContainerLowest: background(Neutral) {
        tone: |scheme| if scheme.is_dark { ContrastCurve::new(4.0, 4.0, 2.0, 0.0).get(scheme.contrast_level) } else { 100.0 },
    },

    SurfaceContainerLow: background(Neutral) {
        tone: |scheme| {
            let curve = if scheme.is_dark {
                ContrastCurve::new(10.0, 10.0, 11.0, 12.0)
            } else {
                ContrastCurve::new(96.0, 96.0, 96.0, 95.0)
            };

            curve.get(scheme.contrast_level)
        },
    },

    SurfaceContainer: background(Neutral) {
        tone: |scheme| {
            let curve = if scheme.is_dark {
                ContrastCurve::new(12.0, 12.0, 16.0, 20.0)
            } else {
                ContrastCurve::new(94.0, 94.0, 92.0, 90.0)
            };

            curve.get(scheme.contrast_level)
        },
    },

    SurfaceContainerHigh: background(Neutral) {
        tone: |scheme| {
            let curve = if scheme.is_dark {
                ContrastCurve::new(17.0, 17.0, 21.0, 25.0)
            } else {
                ContrastCurve::new(92.0, 92.0, 88.0, 85.0)
            };

            curve.get(scheme.contrast_level)
        },
    },

    SurfaceContainerHighest: background(Neutral) {
        tone: |scheme| {
            let curve = if scheme.is_dark {
                ContrastCurve::new(22.0, 22.0, 26.0, 30.0)
            } else {
                ContrastCurve::new(90.0, 90.0, 84.0, 80.0)
            };

            curve.get(scheme.contrast_level)
        },
    },

    OnSurface: foreground(Neutral) {
        tone: |scheme| if scheme.is_dark { 90.0 } else { 10.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    SurfaceVariant: background(NeutralVariant) { tone: |scheme| if scheme.is_dark { 30.0 } else { 90.0 } },
    OnSurfaceVariant: foreground(NeutralVariant) {
        tone: |scheme| if scheme.is_dark { 80.0 } else { 30.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    InverseSurface: background(Neutral) { tone: |scheme| if scheme.is_dark { 90.0 } else { 20.0 } },
    InverseOnSurface: foreground(Neutral) {
        tone: |scheme| if scheme.is_dark { 20.0 } else { 95.0 },
        background: |_| Some(DynamicColor::Material(Role::InverseSurface)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    Outline: foreground(NeutralVariant) {
        tone: |scheme| if scheme.is_dark { 60.0 } else { 50.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.5, 3.0, 4.5, 7.0)),
    },

    OutlineVariant: foreground(NeutralVariant) {
        tone: |scheme| if scheme.is_dark { 30.0 } else { 80.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
    },

    Shadow: foreground(Neutral) { tone: |_| 0.0 },
    Scrim: foreground(Neutral) { tone: |_| 0.0 },
    SurfaceTint: background(Primary) { tone: |scheme| if scheme.is_dark { 80.0 } else { 40.0 } },

    // Primaries.
    Primary: background(Primary) {
        tone: |scheme| match (is_monochrome(&scheme), scheme.is_dark) {
            (true, true) => 100.0,
            (true, false) => 0.0,
            (false, true) => 80.0,
            (false, false) => 40.0,
        },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
        tone_delta_pair: |_| Some(container_pair(Role::PrimaryContainer, Role::Primary)),
    },

    OnPrimary: foreground(Primary) {
        tone: |scheme| match (is_monochrome(&scheme), scheme.is_dark) {
            (true, true) => 10.0,
            (true, false) => 90.0,
            (false, true) => 20.0,
            (false, false) => 100.0,
        },
        background: |_| Some(DynamicColor::Material(Role::Primary)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    PrimaryContainer: background(Primary) {
        tone: |scheme| {
            if is_fidelity(&scheme) {
                scheme.source_color_hct.get_tone()
            } else if is_monochrome(&scheme) {
                if scheme.is_dark { 85.0 } else { 25.0 }
            } else if scheme.is_dark {
                30.0
            } else {
                90.0
            }
        },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(container_pair(Role::PrimaryContainer, Role::Primary)),
    },

    OnPrimaryContainer: foreground(Primary) {
        tone: |scheme| {
            if is_fidelity(&scheme) {
                DynamicColor::foreground_tone(scheme.raw_tone(DynamicColor::Material(Role::PrimaryContainer)), 4.5)
            } else if is_monochrome(&scheme) {
                if scheme.is_dark { 0.0 } else { 100.0 }
            } else if scheme.is_dark {
                90.0
            } else {
                30.0
            }
        },
        background: |_| Some(DynamicColor::Material(Role::PrimaryContainer)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    InversePrimary: foreground(Primary) {
        tone: |scheme| if scheme.is_dark { 40.0 } else { 80.0 },
        background: |_| Some(DynamicColor::Material(Role::InverseSurface)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
    },

    // Secondaries.
    Secondary: background(Secondary) {
        tone: |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
        tone_delta_pair: |_| Some(container_pair(Role::SecondaryContainer, Role::Secondary)),
    },

    OnSecondary: foreground(Secondary) {
        tone: |scheme| match (is_monochrome(&scheme), scheme.is_dark) {
            (true, true) => 10.0,
            (false, true) => 20.0,
            (_, false) => 100.0,
        },
        background: |_| Some(DynamicColor::Material(Role::Secondary)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    SecondaryContainer: background(Secondary) {
        tone: |scheme| {
            let initial_tone = if scheme.is_dark { 30.0 } else { 90.0 };

            if is_monochrome(&scheme) {
                if scheme.is_dark { 30.0 } else { 85.0 }
            } else if !is_fidelity(&scheme) {
                initial_tone
            } else {
                find_desired_chroma_by_tone(scheme.secondary_palette.hue(), scheme.secondary_palette.chroma(), initial_tone, !scheme.is_dark)
            }
        },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(container_pair(Role::SecondaryContainer, Role::Secondary)),
    },

    OnSecondaryContainer: foreground(Secondary) {
        tone: |scheme| {
            if is_monochrome(&scheme) {
                if scheme.is_dark { 90.0 } else { 10.0 }
            } else if !is_fidelity(&scheme) {
                if scheme.is_dark { 90.0 } else { 30.0 }
            } else {
                DynamicColor::foreground_tone(scheme.raw_tone(DynamicColor::Material(Role::SecondaryContainer)), 4.5)
            }
        },
        background: |_| Some(DynamicColor::Material(Role::SecondaryContainer)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    // Tertiaries.
    Tertiary: background(Tertiary) {
        tone: |scheme| match (is_monochrome(&scheme), scheme.is_dark) {
            (true, true) => 90.0,
            (true, false) => 25.0,
            (false, true) => 80.0,
            (false, false) => 40.0,
        },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
        tone_delta_pair: |_| Some(container_pair(Role::TertiaryContainer, Role::Tertiary)),
    },

    OnTertiary: foreground(Tertiary) {
        tone: |scheme| match (is_monochrome(&scheme), scheme.is_dark) {
            (true, true) => 10.0,
            (true, false) => 90.0,
            (false, true) => 20.0,
            (false, false) => 100.0,
        },
        background: |_| Some(DynamicColor::Material(Role::Tertiary)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    TertiaryContainer: background(Tertiary) {
        tone: |scheme| {
            if is_monochrome(&scheme) {
                if scheme.is_dark { 60.0 } else { 49.0 }
            } else if !is_fidelity(&scheme) {
                if scheme.is_dark { 30.0 } else { 90.0 }
            } else {
                let proposed_hct = scheme.tertiary_palette.get_hct(scheme.source_color_hct.get_tone());

                fix_if_disliked(proposed_hct).get_tone()
            }
        },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(container_pair(Role::TertiaryContainer, Role::Tertiary)),
    },

    OnTertiaryContainer: foreground(Tertiary) {
        tone: |scheme| {
            if is_monochrome(&scheme) {
                if scheme.is_dark { 0.0 } else { 100.0 }
            } else if !is_fidelity(&scheme) {
                if scheme.is_dark { 90.0 } else { 30.0 }
            } else {
                DynamicColor::foreground_tone(scheme.raw_tone(DynamicColor::Material(Role::TertiaryContainer)), 4.5)
            }
        },
        background: |_| Some(DynamicColor::Material(Role::TertiaryContainer)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    // Errors.
    Error: background(Error) {
        tone: |scheme| if scheme.is_dark { 80.0 } else { 40.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
        tone_delta_pair: |_| Some(container_pair(Role::ErrorContainer, Role::Error)),
    },

    OnError: foreground(Error) {
        tone: |scheme| if scheme.is_dark { 20.0 } else { 100.0 },
        background: |_| Some(DynamicColor::Material(Role::Error)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    ErrorContainer: background(Error) {
        tone: |scheme| if scheme.is_dark { 30.0 } else { 90.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(container_pair(Role::ErrorContainer, Role::Error)),
    },

    OnErrorContainer: foreground(Error) {
        tone: |scheme| match (is_monochrome(&scheme), scheme.is_dark) {
            (_, true) => 90.0,
            (true, false) => 10.0,
            (false, false) => 30.0,
        },
        background: |_| Some(DynamicColor::Material(Role::ErrorContainer)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    // Primary fixed colors.
    PrimaryFixed: background(Primary) {
        tone: |scheme| if is_monochrome(&scheme) { 40.0 } else { 90.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(fixed_pair(Role::PrimaryFixed, Role::PrimaryFixedDim)),
    },

    PrimaryFixedDim: background(Primary) {
        tone: |scheme| if is_monochrome(&scheme) { 30.0 } else { 80.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(fixed_pair(Role::PrimaryFixed, Role::PrimaryFixedDim)),
    },

    OnPrimaryFixed: foreground(Primary) {
        tone: |scheme| if is_monochrome(&scheme) { 100.0 } else { 10.0 },
        background: |_| Some(DynamicColor::Material(Role::PrimaryFixedDim)),
        second_background: |_| Some(DynamicColor::Material(Role::PrimaryFixed)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    OnPrimaryFixedVariant: foreground(Primary) {
        tone: |scheme| if is_monochrome(&scheme) { 90.0 } else { 30.0 },
        background: |_| Some(DynamicColor::Material(Role::PrimaryFixedDim)),
        second_background: |_| Some(DynamicColor::Material(Role::PrimaryFixed)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    // Secondary fixed colors.
    SecondaryFixed: background(Secondary) {
        tone: |scheme| if is_monochrome(&scheme) { 80.0 } else { 90.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(fixed_pair(Role::SecondaryFixed, Role::SecondaryFixedDim)),
    },

    SecondaryFixedDim: background(Secondary) {
        tone: |scheme| if is_monochrome(&scheme) { 70.0 } else { 80.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(fixed_pair(Role::SecondaryFixed, Role::SecondaryFixedDim)),
    },

    OnSecondaryFixed: foreground(Secondary) {
        tone: |_| 10.0,
        background: |_| Some(DynamicColor::Material(Role::SecondaryFixedDim)),
        second_background: |_| Some(DynamicColor::Material(Role::SecondaryFixed)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    OnSecondaryFixedVariant: foreground(Secondary) {
        tone: |scheme| if is_monochrome(&scheme) { 25.0 } else { 30.0 },
        background: |_| Some(DynamicColor::Material(Role::SecondaryFixedDim)),
        second_background: |_| Some(DynamicColor::Material(Role::SecondaryFixed)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    // Tertiary fixed colors.
    TertiaryFixed: background(Tertiary) {
        tone: |scheme| if is_monochrome(&scheme) { 40.0 } else { 90.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(fixed_pair(Role::TertiaryFixed, Role::TertiaryFixedDim)),
    },

    TertiaryFixedDim: background(Tertiary) {
        tone: |scheme| if is_monochrome(&scheme) { 30.0 } else { 80.0 },
        background: highest_surface_fn,
        contrast_curve: |_| Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
        tone_delta_pair: |_| Some(fixed_pair(Role::TertiaryFixed, Role::TertiaryFixedDim)),
    },

    OnTertiaryFixed: foreground(Tertiary) {
        tone: |scheme| if is_monochrome(&scheme) { 100.0 } else { 10.0 },
        background: |_| Some(DynamicColor::Material(Role::TertiaryFixedDim)),
        second_background: |_| Some(DynamicColor::Material(Role::TertiaryFixed)),
        contrast_curve: |_| Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
    },

    OnTertiaryFixedVariant: foreground(Tertiary) {
        tone: |scheme| if is_monochrome(&scheme) { 90.0 } else { 30.0 },
        background: |_| Some(DynamicColor::Material(Role::TertiaryFixedDim)),
        second_background: |_| Some(DynamicColor::Material(Role::TertiaryFixed)),
        contrast_curve: |_| Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
    },

    // `*_dim` roles have no 2021 definition; resolution falls back to 2025.
};

fn find_desired_chroma_by_tone(hue: f64, chroma: f64, tone: f64, by_decreasing_tone: bool) -> f64 {
    let mut answer = tone;
    let mut closest_to_chroma = Hct::from(hue, chroma, tone);

    if closest_to_chroma.get_chroma() < chroma {
        let mut chroma_peak = closest_to_chroma.get_chroma();

        while closest_to_chroma.get_chroma() < chroma {
            answer += if by_decreasing_tone { -1.0 } else { 1.0 };

            let potential_solution = Hct::from(hue, chroma, answer);

            if chroma_peak > potential_solution.get_chroma() {
                break;
            }

            if abs(potential_solution.get_chroma() - chroma) < 0.4 {
                break;
            }

            let potential_delta = abs(potential_solution.get_chroma() - chroma);
            let current_delta = abs(closest_to_chroma.get_chroma() - chroma);

            if potential_delta < current_delta {
                closest_to_chroma = potential_solution;
            }

            chroma_peak = chroma_peak.max(potential_solution.get_chroma());
        }
    }

    answer
}

#[inline]
fn in_awkward_zone(tone: f64) -> bool {
    (50.0..60.0).contains(&tone)
}

/// 2021 HCT of `color`, given its already-resolved tone.
pub(super) fn hct(context: Context<'_>, color: DynamicColor<'_>, tone: f64) -> Hct {
    // We find the tone for contrast, then use the palette's chroma: this lets
    // e.g. a T90 color "recover" intended chroma as contrast increases.
    color.view(context.spec_version).palette(context).get_hct(tone)
}

/// 2021 tone algorithm.
pub(super) fn tone(context: Context<'_>, color: DynamicColor<'_>) -> f64 {
    let version = context.spec_version;
    let contrast_level = context.contrast_level;
    let decreasing_contrast = contrast_level < 0.0;
    let view = color.view(version);

    // Case 1: dual foreground, pair of colors with delta constraint.
    if let Some(pair) = view.tone_delta_pair(context) {
        let ToneDeltaPair {
            role_a,
            role_b,
            delta,
            polarity,
            stay_together,
            constraint,
        } = pair;

        // NOTE: DARKER also tests light mode here, like LIGHTER. The previous
        // implementation tested dark mode for DARKER.
        let a_is_nearer = constraint == DeltaConstraint::Nearer
            || (polarity == TonePolarity::Lighter && !context.is_dark)
            || (polarity == TonePolarity::Darker && !context.is_dark);

        let (nearer, farther) = if a_is_nearer { (role_a, role_b) } else { (role_b, role_a) };
        let am_nearer = color.same_as(&nearer);
        let expansion_dir = if context.is_dark { 1.0 } else { -1.0 };
        let nearer_view = nearer.view(version);
        let farther_view = farther.view(version);
        let mut n_tone = nearer_view.raw_tone(context);
        let mut f_tone = farther_view.raw_tone(context);

        // 1st round: solve to min, each.
        if let (Some(n_curve), Some(f_curve)) = (nearer_view.contrast_curve(context), farther_view.contrast_curve(context))
            && let Some(bg) = view.background(context)
        {
            let n_contrast = n_curve.get(contrast_level);
            let f_contrast = f_curve.get(contrast_level);
            let bg_tone = context.tone(bg);

            // A color that is good enough is not adjusted, unless decreasing
            // contrast, where it goes to the "bare minimum".
            if decreasing_contrast || ratio_of_tones(bg_tone, n_tone) < n_contrast {
                n_tone = DynamicColor::foreground_tone(bg_tone, n_contrast);
            }

            if decreasing_contrast || ratio_of_tones(bg_tone, f_tone) < f_contrast {
                f_tone = DynamicColor::foreground_tone(bg_tone, f_contrast);
            }
        }

        if (f_tone - n_tone) * expansion_dir < delta {
            // 2nd round: expand farther to match delta.
            f_tone = coerce_in(n_tone + delta * expansion_dir, 0.0, 100.0);

            if (f_tone - n_tone) * expansion_dir < delta {
                // 3rd round: contract nearer to match delta.
                n_tone = coerce_in(f_tone - delta * expansion_dir, 0.0, 100.0);
            }
        }

        // Avoids the 50-59 awkward zone.
        let move_both = in_awkward_zone(n_tone) || (in_awkward_zone(f_tone) && stay_together);

        if move_both {
            if expansion_dir > 0.0 {
                n_tone = 60.0;
                f_tone = f_tone.max(n_tone + delta * expansion_dir);
            } else {
                n_tone = 49.0;
                f_tone = f_tone.min(n_tone + delta * expansion_dir);
            }
        } else if in_awkward_zone(f_tone) {
            // Not required to stay together; fixes just one.
            f_tone = if expansion_dir > 0.0 { 60.0 } else { 49.0 };
        }

        return if am_nearer { n_tone } else { f_tone };
    }

    // Case 2: No contrast pair; just solve for itself.
    let mut answer = view.raw_tone(context);
    let Some(curve) = view.contrast_curve(context) else {
        return answer;
    };

    let Some(background) = view.background(context) else {
        return answer; // No adjustment for colors with no background.
    };

    let bg_tone = context.tone(background);
    let desired_ratio = curve.get(contrast_level);

    if decreasing_contrast || ratio_of_tones(bg_tone, answer) < desired_ratio {
        answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
    }

    if view.is_background() && in_awkward_zone(answer) {
        answer = if ratio_of_tones(49.0, bg_tone) >= desired_ratio { 49.0 } else { 60.0 };
    }

    // Case 3: Adjust for dual backgrounds.
    view.second_background(context)
        .map_or(answer, |second| dual_background_tone(answer, bg_tone, context.tone(second), desired_ratio))
}

#[inline]
fn hue_chroma(hue: f64, chroma: f64) -> TonalPalette {
    TonalPalette::from_hue_and_chroma(hue, chroma)
}

pub(super) fn primary_palette(variant: Variant, source: Hct) -> Option<TonalPalette> {
    let (hue, chroma) = (source.get_hue(), source.get_chroma());

    Some(match variant {
        Variant::Content | Variant::Fidelity => hue_chroma(hue, chroma),
        Variant::FruitSalad => hue_chroma(sanitize_degrees(hue - 50.0), 48.0),
        Variant::Monochrome => hue_chroma(hue, 0.0),
        Variant::Neutral => hue_chroma(hue, 12.0),
        Variant::Rainbow => hue_chroma(hue, 48.0),
        Variant::TonalSpot => hue_chroma(hue, 36.0),
        Variant::Expressive => hue_chroma(sanitize_degrees(hue + 240.0), 40.0),
        Variant::Vibrant => hue_chroma(hue, 200.0),
        Variant::Cmf => return None,
    })
}

pub(super) fn secondary_palette(variant: Variant, source: Hct) -> Option<TonalPalette> {
    let (hue, chroma) = (source.get_hue(), source.get_chroma());

    Some(match variant {
        Variant::Content | Variant::Fidelity => hue_chroma(hue, (chroma - 32.0).max(chroma * 0.5)),
        Variant::FruitSalad => hue_chroma(sanitize_degrees(hue - 50.0), 36.0),
        Variant::Monochrome => hue_chroma(hue, 0.0),
        Variant::Neutral => hue_chroma(hue, 8.0),
        Variant::Rainbow | Variant::TonalSpot => hue_chroma(hue, 16.0),
        Variant::Expressive => hue_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0], &[
                45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0,
            ]),
            24.0,
        ),
        Variant::Vibrant => hue_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0], &[
                18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0,
            ]),
            24.0,
        ),
        Variant::Cmf => return None,
    })
}

pub(super) fn tertiary_palette(variant: Variant, source: Hct) -> Option<TonalPalette> {
    let hue = source.get_hue();

    Some(match variant {
        Variant::Content => TonalPalette::from_hct(fix_if_disliked(content_tertiary_source(source))),
        Variant::Fidelity => TonalPalette::from_hct(fix_if_disliked(fidelity_tertiary_source(source))),
        Variant::FruitSalad => hue_chroma(hue, 36.0),
        Variant::Monochrome => hue_chroma(hue, 0.0),
        Variant::Neutral => hue_chroma(hue, 16.0),
        Variant::Rainbow | Variant::TonalSpot => hue_chroma(sanitize_degrees(hue + 60.0), 24.0),
        Variant::Expressive => hue_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0], &[
                120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0,
            ]),
            32.0,
        ),
        Variant::Vibrant => hue_chroma(
            DynamicScheme::get_rotated_hue(hue, &[0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0], &[
                35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0,
            ]),
            32.0,
        ),
        Variant::Cmf => return None,
    })
}

// `TemperatureCache` is array-based (no `alloc`), but it holds 362 HCTs, so
// these two calls put ~15 KB on the stack.
fn content_tertiary_source(source: Hct) -> Hct {
    // Third of 3 analogous colors on a wheel of 6 divisions.
    TemperatureCache::new(source).analogous_generic::<3, 6>()[2]
}

fn fidelity_tertiary_source(source: Hct) -> Hct {
    TemperatureCache::new(source).complement()
}

pub(super) fn neutral_palette(variant: Variant, source: Hct) -> Option<TonalPalette> {
    let (hue, chroma) = (source.get_hue(), source.get_chroma());

    Some(match variant {
        Variant::Content | Variant::Fidelity => hue_chroma(hue, chroma / 8.0),
        Variant::FruitSalad | Variant::Vibrant => hue_chroma(hue, 10.0),
        Variant::Monochrome | Variant::Rainbow => hue_chroma(hue, 0.0),
        Variant::Neutral => hue_chroma(hue, 2.0),
        Variant::TonalSpot => hue_chroma(hue, 6.0),
        Variant::Expressive => hue_chroma(sanitize_degrees(hue + 15.0), 8.0),
        Variant::Cmf => return None,
    })
}

pub(super) fn neutral_variant_palette(variant: Variant, source: Hct) -> Option<TonalPalette> {
    let (hue, chroma) = (source.get_hue(), source.get_chroma());

    Some(match variant {
        Variant::Content | Variant::Fidelity => hue_chroma(hue, chroma / 8.0 + 4.0),
        Variant::FruitSalad => hue_chroma(hue, 16.0),
        Variant::Monochrome | Variant::Rainbow => hue_chroma(hue, 0.0),
        Variant::Neutral => hue_chroma(hue, 2.0),
        Variant::TonalSpot => hue_chroma(hue, 8.0),
        Variant::Expressive => hue_chroma(sanitize_degrees(hue + 15.0), 12.0),
        Variant::Vibrant => hue_chroma(hue, 12.0),
        Variant::Cmf => return None,
    })
}

/// Same for every variant (CMF included).
pub(super) fn error_palette() -> TonalPalette {
    hue_chroma(25.0, 84.0)
}

impl ColorSpec for ColorSpec2021 {
    fn spec_version(&self) -> SpecVersion {
        SpecVersion::Spec2021
    }

    fn declared(&self, role: Role) -> Option<Entry> {
        DECLARED[role]
    }

    fn get_hct(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> Hct {
        let cache = ToneCache::new();
        let context = Context::new(scheme, &cache);

        hct(context, color, tone(context, color))
    }

    fn get_tone(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> f64 {
        let cache = ToneCache::new();

        tone(Context::new(scheme, &cache), color)
    }

    fn get_primary_palette(&self, variant: Variant, source: Hct, _: bool, _: Platform, _: f64) -> Option<TonalPalette> {
        primary_palette(variant, source)
    }

    fn get_secondary_palette(&self, variant: Variant, source: Hct, _: bool, _: Platform, _: f64) -> Option<TonalPalette> {
        secondary_palette(variant, source)
    }

    fn get_tertiary_palette(&self, variant: Variant, source: Hct, _: bool, _: Platform, _: f64) -> Option<TonalPalette> {
        tertiary_palette(variant, source)
    }

    fn get_neutral_palette(&self, variant: Variant, source: Hct, _: bool, _: Platform, _: f64) -> Option<TonalPalette> {
        neutral_palette(variant, source)
    }

    fn get_neutral_variant_palette(&self, variant: Variant, source: Hct, _: bool, _: Platform, _: f64) -> Option<TonalPalette> {
        neutral_variant_palette(variant, source)
    }

    fn get_error_palette(&self, _: Variant, _: Hct, _: bool, _: Platform, _: f64) -> Option<TonalPalette> {
        Some(error_palette())
    }
}
