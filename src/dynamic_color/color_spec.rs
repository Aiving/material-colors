//! `ColorSpec` trait, color definition tables and helpers shared by specs.

use super::{
    Context, ContrastCurve, DynamicColor, DynamicScheme, Platform, Role, RoleMap, SchemePalette, ToneDeltaPair, Variant, color_spec_2021, color_spec_2025,
    color_spec_2026,
};
use crate::{hct::Hct, palette::TonalPalette};

/// All available spec versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SpecVersion {
    Spec2021,
    Spec2025,
    Spec2026,
}

pub type ToneFn = for<'a> fn(Context<'a>) -> f64;
pub type ChromaFn = for<'a> fn(Context<'a>) -> f64;
pub type ColorFn = for<'a> fn(Context<'a>) -> Option<DynamicColor<'static>>;
pub type CurveFn = for<'a> fn(Context<'a>) -> Option<ContrastCurve>;
pub type PairFn = for<'a> fn(Context<'a>) -> Option<ToneDeltaPair<'static>>;

/// Definition of a built-in color for one spec version.
#[derive(Clone, Copy)]
pub struct ColorDefinition {
    pub palette: SchemePalette,
    pub is_background: bool,
    pub tone: Option<ToneFn>,
    pub chroma_multiplier: Option<ChromaFn>,
    pub background: Option<ColorFn>,
    pub second_background: Option<ColorFn>,
    pub contrast_curve: Option<CurveFn>,
    pub tone_delta_pair: Option<PairFn>,
}

impl ColorDefinition {
    pub const fn foreground(palette: SchemePalette) -> Self {
        Self {
            palette,
            is_background: false,
            tone: None,
            chroma_multiplier: None,
            background: None,
            second_background: None,
            contrast_curve: None,
            tone_delta_pair: None,
        }
    }

    pub const fn background(palette: SchemePalette) -> Self {
        Self {
            is_background: true,
            ..Self::foreground(palette)
        }
    }
}

/// What a spec declares for a role.
#[derive(Clone, Copy)]
pub enum Entry {
    Definition(ColorDefinition),
    /// Use `Role`'s definition (as resolved for the scheme's spec version)
    /// under this role's name.
    Alias(Role),
    /// Like `Alias`, but with its own tone function.
    AliasWithTone(Role, ToneFn),
}

pub type SpecTable = RoleMap<Option<Entry>>;

impl SpecTable {
    pub const EMPTY: Self = Self([None; Role::COUNT]);

    #[inline]
    pub const fn define(&mut self, role: Role, definition: ColorDefinition) {
        self.0[role as usize] = Some(Entry::Definition(definition));
    }

    /// `role` uses `target`'s definition.
    #[inline]
    pub const fn alias(&mut self, role: Role, target: Role) {
        self.0[role as usize] = Some(Entry::Alias(target));
    }

    /// `role` uses `target`'s definition with its own tone.
    #[inline]
    pub const fn alias_with_tone(&mut self, role: Role, target: Role, tone: ToneFn) {
        self.0[role as usize] = Some(Entry::AliasWithTone(target, tone));
    }
}

macro_rules! spec_table {
    (@entries $t:ident;) => {};
    (@entries $t:ident; $role:ident: alias($target:ident) { tone: $tone:expr $(,)? } $(, $($rest:tt)*)?) => {
        $t.alias_with_tone($crate::dynamic_color::Role::$role, $crate::dynamic_color::Role::$target, $tone);
        $crate::dynamic_color::color_spec::spec_table!(@entries $t; $($($rest)*)?);
    };
    (@entries $t:ident; $role:ident: alias($target:ident) $(, $($rest:tt)*)?) => {
        $t.alias($crate::dynamic_color::Role::$role, $crate::dynamic_color::Role::$target);
        $crate::dynamic_color::color_spec::spec_table!(@entries $t; $($($rest)*)?);
    };
    (@entries $t:ident; $role:ident: $kind:ident($palette:ident) { $($field:ident: $value:expr),* $(,)? } $(, $($rest:tt)*)?) => {
        $t.define(
            $crate::dynamic_color::Role::$role,
            $crate::dynamic_color::ColorDefinition {
                $($field: Some($value),)*
                ..$crate::dynamic_color::ColorDefinition::$kind($crate::dynamic_color::SchemePalette::$palette)
            },
        );
        $crate::dynamic_color::color_spec::spec_table!(@entries $t; $($($rest)*)?);
    };
    ($($entries:tt)*) => {{
        let mut table = $crate::dynamic_color::SpecTable::EMPTY;

        $crate::dynamic_color::color_spec::spec_table!(@entries table; $($entries)*);

        table
    }};
}

pub(super) use spec_table;

/// Merges spec tables in priority order and resolves aliases. Runs in const
/// context only; a missing definition or a nested alias is a compile error.
const fn resolve(order: &[&SpecTable]) -> RoleMap<ColorDefinition> {
    let mut picked = SpecTable::EMPTY;
    let mut i = 0;

    while i < Role::COUNT {
        let role = Role::ALL[i];
        let mut j = 0;

        while j < order.len() {
            if let Some(entry) = *order[j].get(role) {
                picked.0[i] = Some(entry);

                break;
            }

            j += 1;
        }

        assert!(picked.get(role).is_some(), "a role has no definition in any spec");

        i += 1;
    }

    let mut out = RoleMap([ColorDefinition::foreground(SchemePalette::Neutral); Role::COUNT]);
    let mut i = 0;

    while i < Role::COUNT {
        out.0[i] = match *picked.get(Role::ALL[i]) {
            Some(Entry::Definition(definition)) => definition,
            Some(Entry::Alias(target)) => match *picked.get(target) {
                Some(Entry::Definition(definition)) => definition,
                _ => panic!("alias must point at a concrete definition"),
            },
            Some(Entry::AliasWithTone(target, tone)) => match *picked.get(target) {
                Some(Entry::Definition(definition)) => ColorDefinition {
                    tone: Some(tone),
                    ..definition
                },
                _ => panic!("alias must point at a concrete definition"),
            },
            None => panic!("unreachable"),
        };

        i += 1;
    }

    out
}

static RESOLVED: [RoleMap<ColorDefinition>; 3] = [
    resolve(&[&color_spec_2021::TABLE, &color_spec_2025::TABLE]),
    resolve(&[&color_spec_2025::TABLE, &color_spec_2021::TABLE]),
    resolve(&[&color_spec_2026::TABLE, &color_spec_2025::TABLE, &color_spec_2021::TABLE]),
];

#[inline]
pub(super) fn resolved(version: SpecVersion, role: Role) -> &'static ColorDefinition {
    &RESOLVED[version as usize][role]
}

/// Everything that can differ between specs.
///
/// Roles are not per-spec methods: [`ColorSpec::declared`] /
/// [`ColorSpec::definition`] expose the tables, and a role's `DynamicColor` is
/// just `DynamicColor::Material(role)`, resolved against the scheme when
/// evaluated.
pub trait ColorSpec {
    fn spec_version(&self) -> SpecVersion;

    /// The entry this spec itself declares for `role` (`None` = inherited).
    fn declared(&self, role: Role) -> Option<Entry>;

    /// The effective definition of `role` for schemes of this spec version.
    fn definition(&self, role: Role) -> &'static ColorDefinition {
        resolved(self.spec_version(), role)
    }

    fn highest_surface(&self, scheme: &DynamicScheme) -> DynamicColor<'static> {
        highest_surface(scheme)
    }

    /// Resolves `color` using this spec's algorithm (nested colors are
    /// resolved by the scheme's own spec version).
    fn get_hct(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> Hct;

    fn get_tone(&self, scheme: &DynamicScheme, color: DynamicColor<'_>) -> f64;

    // Scheme palettes. `None` when the variant is unsupported by this spec.
    fn get_primary_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> Option<TonalPalette>;
    fn get_secondary_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> Option<TonalPalette>;
    fn get_tertiary_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> Option<TonalPalette>;
    fn get_neutral_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> Option<TonalPalette>;
    fn get_neutral_variant_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> Option<TonalPalette>;
    fn get_error_palette(&self, variant: Variant, source: Hct, is_dark: bool, platform: Platform, contrast_level: f64) -> Option<TonalPalette>;
}

/// The spec implementation for `version`.
pub fn color_spec(version: SpecVersion) -> &'static dyn ColorSpec {
    match version {
        SpecVersion::Spec2021 => &color_spec_2021::ColorSpec2021,
        SpecVersion::Spec2025 => &color_spec_2025::ColorSpec2025,
        SpecVersion::Spec2026 => &color_spec_2026::ColorSpec2026,
    }
}

/// Surface a color should contrast with: bright in dark mode, dim in light
/// mode.
#[inline]
pub(super) const fn highest_surface(scheme: &DynamicScheme) -> DynamicColor<'static> {
    DynamicColor::Material(if scheme.is_dark { Role::SurfaceBright } else { Role::SurfaceDim })
}

pub(super) const fn highest_surface_fn(scheme: Context<'_>) -> Option<DynamicColor<'static>> {
    Some(highest_surface(scheme.scheme()))
}

pub(super) const fn contrast_curve(default_contrast: f64) -> ContrastCurve {
    if default_contrast == 1.5 {
        ContrastCurve::new(1.5, 1.5, 3.0, 5.5)
    } else if default_contrast == 3.0 {
        ContrastCurve::new(3.0, 3.0, 4.5, 7.0)
    } else if default_contrast == 4.5 {
        ContrastCurve::new(4.5, 4.5, 7.0, 11.0)
    } else if default_contrast == 6.0 {
        ContrastCurve::new(6.0, 6.0, 7.0, 11.0)
    } else if default_contrast == 7.0 {
        ContrastCurve::new(7.0, 7.0, 11.0, 21.0)
    } else if default_contrast == 9.0 {
        ContrastCurve::new(9.0, 9.0, 11.0, 21.0)
    } else if default_contrast == 11.0 {
        ContrastCurve::new(11.0, 11.0, 21.0, 21.0)
    } else if default_contrast == 21.0 {
        ContrastCurve::new(21.0, 21.0, 21.0, 21.0)
    } else {
        ContrastCurve::new(default_contrast, default_contrast, 7.0, 21.0)
    }
}

#[inline]
pub(super) const fn coerce_in(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub(super) fn find_best_tone_for_chroma(hue: f64, chroma: f64, tone: f64, by_decreasing_tone: bool) -> f64 {
    let mut tone = tone;
    let mut answer = tone;
    let mut best_candidate = Hct::from(hue, chroma, answer);

    while best_candidate.get_chroma() < chroma {
        if !(0.0..=100.0).contains(&tone) {
            break;
        }

        tone += if by_decreasing_tone { -1.0 } else { 1.0 };

        let new_candidate = Hct::from(hue, chroma, tone);

        if best_candidate.get_chroma() < new_candidate.get_chroma() {
            best_candidate = new_candidate;
            answer = tone;
        }
    }

    answer
}

pub(super) fn dual_background_tone(answer: f64, bg_tone1: f64, bg_tone2: f64, desired_ratio: f64) -> f64 {
    use crate::contrast::{darker, lighter, ratio_of_tones};

    let upper = bg_tone1.max(bg_tone2);
    let lower = bg_tone1.min(bg_tone2);

    if ratio_of_tones(upper, answer) >= desired_ratio && ratio_of_tones(lower, answer) >= desired_ratio {
        return answer;
    }

    let light_option = lighter(upper, desired_ratio);
    let dark_option = darker(lower, desired_ratio);

    if DynamicColor::tone_prefers_light_foreground(bg_tone1) || DynamicColor::tone_prefers_light_foreground(bg_tone2) {
        return light_option.unwrap_or(100.0);
    }

    dark_option.or(light_option).unwrap_or(0.0)
}
