//! `DynamicColor`: a color that adjusts itself based on UI state, represented
//! by a [`DynamicScheme`].
#![allow(clippy::float_cmp, clippy::too_many_arguments)]

pub mod color_spec;
pub mod color_spec_2021;
pub mod color_spec_2025;
pub mod color_spec_2026;
pub mod contrast_curve;
pub mod dynamic_scheme;
pub mod material_dynamic_colors;
pub mod tone_delta_pair;
pub mod variant;

use core::{
    cell::Cell,
    fmt,
    ops::{Deref, Index, IndexMut},
};

pub use color_spec::{ColorDefinition, ColorSpec, Entry, SpecTable, SpecVersion, color_spec};
pub use color_spec_2021::ColorSpec2021;
pub use color_spec_2025::ColorSpec2025;
pub use color_spec_2026::ColorSpec2026;
pub use contrast_curve::ContrastCurve;
pub use dynamic_scheme::{DynamicScheme, Platform, SchemePalette};
pub use material_dynamic_colors::MaterialDynamicColors;
pub use tone_delta_pair::{DeltaConstraint, ToneDeltaPair, TonePolarity};
pub use variant::Variant;

use crate::{
    color::Rgb,
    contrast::{darker_unsafe, lighter_unsafe, ratio_of_tones},
    dynamic_color::color_spec::{coerce_in, find_best_tone_for_chroma, resolved},
    hct::Hct,
    palette::TonalPalette,
};

/// A user-defined color. Default methods describe a foreground color with no
/// background, contrast curve, tone delta pair or opacity.
pub trait CustomColor {
    fn name(&self) -> &str;

    fn palette<'s>(&'s self, scheme: &'s DynamicScheme) -> &'s TonalPalette;

    /// `None` = the tone of `background`, or 50 without a background.
    fn tone(&self, _scheme: &DynamicScheme) -> Option<f64> {
        None
    }

    fn is_background(&self) -> bool {
        false
    }

    fn chroma_multiplier(&self, _scheme: &DynamicScheme) -> Option<f64> {
        None
    }

    fn background(&self, _scheme: &DynamicScheme) -> Option<DynamicColor<'_>> {
        None
    }

    fn second_background(&self, _scheme: &DynamicScheme) -> Option<DynamicColor<'_>> {
        None
    }

    fn contrast_curve(&self, _scheme: &DynamicScheme) -> Option<ContrastCurve> {
        None
    }

    fn tone_delta_pair(&self, _scheme: &DynamicScheme) -> Option<ToneDeltaPair<'_>> {
        None
    }

    fn opacity(&self, _scheme: &DynamicScheme) -> Option<f64> {
        None
    }
}

/// A color with a fixed hue/chroma/tone and no background, so it is never
/// adjusted for contrast.
#[derive(Clone)]
pub struct ArgbColor {
    pub name: &'static str,
    pub palette: TonalPalette,
    pub tone: f64,
}

impl ArgbColor {
    pub const fn new(name: &'static str, hct: Hct) -> Self {
        Self {
            name,
            palette: TonalPalette::from_hct(hct),
            tone: hct.get_tone(),
        }
    }
}

impl CustomColor for ArgbColor {
    fn name(&self) -> &str {
        self.name
    }

    fn palette<'s>(&'s self, _: &'s DynamicScheme) -> &'s TonalPalette {
        &self.palette
    }

    fn tone(&self, _: &DynamicScheme) -> Option<f64> {
        Some(self.tone)
    }
}

/// A dynamic color: either a Material role (resolved through the scheme's
/// spec version) or a user-defined color.
#[derive(Clone, Copy)]
pub enum DynamicColor<'a> {
    Material(Role),
    Custom(&'a dyn CustomColor),
}

impl From<Role> for DynamicColor<'_> {
    fn from(role: Role) -> Self {
        Self::Material(role)
    }
}

impl fmt::Debug for DynamicColor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynamicColor").field(&self.name()).finish()
    }
}

impl PartialEq for DynamicColor<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.same_as(other)
    }
}

impl<'a> DynamicColor<'a> {
    pub fn name(&self) -> &str {
        match self {
            Self::Material(role) => role.name(),
            Self::Custom(color) => color.name(),
        }
    }

    #[inline]
    pub fn same_as(&self, other: &DynamicColor<'_>) -> bool {
        match (self, other) {
            (Self::Material(a), DynamicColor::Material(b)) => a == b,
            _ => self.name() == other.name(),
        }
    }

    #[inline]
    pub(super) fn is_fixed_dim(&self) -> bool {
        match self {
            Self::Material(role) => role.is_fixed_dim(),
            Self::Custom(color) => color.name().ends_with("_fixed_dim"),
        }
    }

    #[inline]
    pub(super) fn view(self, version: SpecVersion) -> View<'a> {
        match self {
            Self::Material(role) => View::Def(resolved(version, role)),
            Self::Custom(color) => View::Custom(color),
        }
    }

    /// Returns the tone in HCT, ranging from 0 to 100, of the resolved color.
    pub fn get_tone(&self, scheme: &DynamicScheme) -> f64 {
        let cache = ToneCache::new();

        Context::new(scheme, &cache).tone(*self)
    }

    /// Returns the resolved color in HCT.
    pub fn get_hct(&self, scheme: &DynamicScheme) -> Hct {
        let cache = ToneCache::new();

        Context::new(scheme, &cache).hct(*self)
    }

    pub fn get_rgb(&self, scheme: &DynamicScheme) -> Rgb {
        self.get_hct(scheme).into()
    }

    /// Alpha channel from `opacity`, 255 when unset.
    pub fn get_alpha(&self, scheme: &DynamicScheme) -> u8 {
        let cache = ToneCache::new();

        alpha(self.view(scheme.spec_version).opacity(Context::new(scheme, &cache)))
    }

    pub fn palette<'s>(&self, scheme: &'s DynamicScheme) -> &'s TonalPalette
    where
        'a: 's,
    {
        match *self {
            Self::Material(role) => scheme.palette(resolved(scheme.spec_version, role).palette),
            Self::Custom(color) => color.palette(scheme),
        }
    }

    pub fn is_background(&self, scheme: &DynamicScheme) -> bool {
        self.view(scheme.spec_version).is_background()
    }

    pub fn background(&self, scheme: &DynamicScheme) -> Option<Self> {
        let cache = ToneCache::new();

        self.view(scheme.spec_version).background(Context::new(scheme, &cache))
    }

    pub fn second_background(&self, scheme: &DynamicScheme) -> Option<Self> {
        let cache = ToneCache::new();

        self.view(scheme.spec_version).second_background(Context::new(scheme, &cache))
    }

    pub fn contrast_curve(&self, scheme: &DynamicScheme) -> Option<ContrastCurve> {
        let cache = ToneCache::new();

        self.view(scheme.spec_version).contrast_curve(Context::new(scheme, &cache))
    }

    pub fn tone_delta_pair(&self, scheme: &DynamicScheme) -> Option<ToneDeltaPair<'a>> {
        let cache = ToneCache::new();

        self.view(scheme.spec_version).tone_delta_pair(Context::new(scheme, &cache))
    }

    pub fn chroma_multiplier(&self, scheme: &DynamicScheme) -> Option<f64> {
        let cache = ToneCache::new();

        self.view(scheme.spec_version).chroma_multiplier(Context::new(scheme, &cache))
    }

    /// Given a background tone, find a foreground tone, while ensuring they
    /// reach a contrast ratio that is as close to `ratio` as possible.
    pub fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
        let lighter_tone = lighter_unsafe(bg_tone, ratio);
        let darker_tone = darker_unsafe(bg_tone, ratio);
        let lighter_ratio = ratio_of_tones(lighter_tone, bg_tone);
        let darker_ratio = ratio_of_tones(darker_tone, bg_tone);

        if Self::tone_prefers_light_foreground(bg_tone) {
            // "Negligible difference" handles an edge case where the initial
            // contrast ratio is high (ex. 13.0), and the ratio passed to the
            // function is that high ratio, and both the lighter and darker
            // ratio fails to pass that ratio.
            let negligible_difference = abs(lighter_ratio - darker_ratio) < 0.1 && lighter_ratio < ratio && darker_ratio < ratio;

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

    /// Adjust a tone down such that white has 4.5 contrast, if the tone is
    /// reasonably close to supporting it.
    pub const fn enable_light_foreground(tone: f64) -> f64 {
        if Self::tone_prefers_light_foreground(tone) && !Self::tone_allows_light_foreground(tone) {
            49.0
        } else {
            tone
        }
    }

    /// People prefer white foregrounds on ~T60-70. T60 itself is excluded
    /// since dark monochrome `tertiary_container` requires a tone of 60.
    #[inline]
    pub const fn tone_prefers_light_foreground(tone: f64) -> bool {
        round_to_int(tone) < 60
    }

    /// Tones less than ~T50 always permit white at 4.5 contrast.
    #[inline]
    pub const fn tone_allows_light_foreground(tone: f64) -> bool {
        round_to_int(tone) <= 49
    }
}

/// A color's definition as seen by one scheme.
#[derive(Clone, Copy)]
pub(super) enum View<'a> {
    Def(&'static ColorDefinition),
    Custom(&'a dyn CustomColor),
}

impl<'a> View<'a> {
    #[inline]
    pub(super) fn palette<'s>(self, context: Context<'s>) -> &'s TonalPalette
    where
        'a: 's,
    {
        match self {
            Self::Def(def) => context.scheme.palette(def.palette),
            Self::Custom(color) => color.palette(context.scheme),
        }
    }

    /// The unadjusted tone, before any contrast or delta constraint.
    pub(super) fn raw_tone(self, context: Context<'_>) -> f64 {
        let explicit = match self {
            Self::Def(def) => def.tone.map(|tone| tone(context)),
            Self::Custom(color) => color.tone(context.scheme),
        };

        // No explicit tone: start from the background's tone.
        explicit.unwrap_or_else(|| self.background(context).map_or(50.0, |bg| context.tone(bg)))
    }

    #[inline]
    pub(super) fn is_background(self) -> bool {
        match self {
            Self::Def(def) => def.is_background,
            Self::Custom(color) => color.is_background(),
        }
    }

    #[inline]
    pub(super) fn chroma_multiplier(self, context: Context<'_>) -> Option<f64> {
        match self {
            Self::Def(def) => def.chroma_multiplier.map(|f| f(context)),
            Self::Custom(color) => color.chroma_multiplier(context.scheme),
        }
    }

    #[inline]
    pub(super) fn background(self, context: Context<'_>) -> Option<DynamicColor<'a>> {
        match self {
            Self::Def(def) => def.background.and_then(|f| f(context)),
            Self::Custom(color) => color.background(context.scheme),
        }
    }

    #[inline]
    pub(super) fn second_background(self, context: Context<'_>) -> Option<DynamicColor<'a>> {
        match self {
            Self::Def(def) => def.second_background.and_then(|f| f(context)),
            Self::Custom(color) => color.second_background(context.scheme),
        }
    }

    #[inline]
    pub(super) fn contrast_curve(self, context: Context<'_>) -> Option<ContrastCurve> {
        match self {
            Self::Def(def) => def.contrast_curve.and_then(|f| f(context)),
            Self::Custom(color) => color.contrast_curve(context.scheme),
        }
    }

    #[inline]
    pub(super) fn tone_delta_pair(self, context: Context<'_>) -> Option<ToneDeltaPair<'a>> {
        match self {
            Self::Def(def) => def.tone_delta_pair.and_then(|f| f(context)),
            Self::Custom(color) => color.tone_delta_pair(context.scheme),
        }
    }

    #[inline]
    pub(super) fn opacity(self, context: Context<'_>) -> Option<f64> {
        match self {
            Self::Def(_) => None,
            Self::Custom(color) => color.opacity(context.scheme),
        }
    }
}

/// `(palette, max|min)` slots for [`Context::t_max_c`]/[`Context::t_min_c`].
const PEAK_SLOTS: usize = SchemePalette::COUNT * 2;

type RoleTones = RoleMap<Cell<f64>>;
type PeakTones = [Cell<f64>; PEAK_SLOTS];

/// Per-scheme memo of resolved role tones and of palette chroma peaks.
/// `NaN` marks an empty slot. Plain stack memory, no allocation.
///
/// Only valid for one scheme;
/// [`SchemeResolver`] enforces that by owning both.
pub struct ToneCache {
    roles: RoleTones,
    peaks: PeakTones,
}

impl ToneCache {
    pub const fn new() -> Self {
        Self {
            roles: RoleMap([const { Cell::new(f64::NAN) }; Role::COUNT]),
            peaks: [const { Cell::new(f64::NAN) }; PEAK_SLOTS],
        }
    }
}

impl Default for ToneCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluation context passed to every definition function: the scheme plus
/// its memo tables.
#[derive(Clone, Copy)]
pub struct Context<'a> {
    scheme: &'a DynamicScheme,
    roles: &'a RoleTones,
    peaks: &'a PeakTones,
}

impl Deref for Context<'_> {
    type Target = DynamicScheme;

    #[inline]
    fn deref(&self) -> &DynamicScheme {
        self.scheme
    }
}

impl<'a> Context<'a> {
    #[inline]
    pub const fn new(scheme: &'a DynamicScheme, cache: &'a ToneCache) -> Self {
        Self {
            scheme,
            roles: &cache.roles,
            peaks: &cache.peaks,
        }
    }

    #[inline]
    pub const fn scheme(self) -> &'a DynamicScheme {
        self.scheme
    }

    /// Resolved tone of `color`: dispatches on the scheme's spec version,
    /// memoized for roles.
    pub fn tone(self, color: DynamicColor<'_>) -> f64 {
        let DynamicColor::Material(role) = color else {
            return self.compute_tone(color);
        };

        let slot = &self.roles[role];
        let cached = slot.get();

        if !cached.is_nan() {
            return cached;
        }

        let tone = self.compute_tone(color);

        slot.set(tone);

        tone
    }

    #[inline]
    fn compute_tone(self, color: DynamicColor<'_>) -> f64 {
        match self.scheme.spec_version {
            SpecVersion::Spec2021 => color_spec_2021::tone(self, color),
            // The 2026 spec uses the 2025 tone and HCT algorithm.
            SpecVersion::Spec2025 | SpecVersion::Spec2026 => color_spec_2025::tone(self, color),
        }
    }

    /// Resolved HCT of `color`.
    pub fn hct(self, color: DynamicColor<'_>) -> Hct {
        let tone = self.tone(color);

        match self.scheme.spec_version {
            SpecVersion::Spec2021 => color_spec_2021::hct(self, color, tone),
            SpecVersion::Spec2025 | SpecVersion::Spec2026 => color_spec_2025::hct(self, color, tone),
        }
    }

    /// Unadjusted tone of another color.
    #[inline]
    pub fn raw_tone(self, color: DynamicColor<'_>) -> f64 {
        color.view(self.scheme.spec_version).raw_tone(self)
    }

    /// Runs `f` against a copy of the scheme in light mode at standard
    /// contrast. Palettes are identical, so the palette-peak memo
    /// is shared; role tones get a fresh memo. No copy when the scheme already
    /// is light at standard contrast.
    pub fn in_light_standard<R>(self, f: impl FnOnce(Context<'_>) -> R) -> R {
        if !self.scheme.is_dark && self.scheme.contrast_level == 0.0 {
            return f(self);
        }

        let temp = self.scheme.with_mode(false, 0.0);
        let roles: RoleTones = RoleMap([const { Cell::new(f64::NAN) }; Role::COUNT]);

        f(Context {
            scheme: &temp,
            roles: &roles,
            peaks: self.peaks,
        })
    }

    /// Tone with the highest chroma in `palette`, searched from tone 100 down,
    /// clamped to `[lower_bound, upper_bound]`.
    #[inline]
    pub fn t_max_c(self, palette: SchemePalette, lower_bound: f64, upper_bound: f64) -> f64 {
        coerce_in(self.peak(palette, true), lower_bound, upper_bound)
    }

    /// Tone with the highest chroma in `palette`, searched from tone 0 up,
    /// clamped to `[lower_bound, upper_bound]`.
    #[inline]
    pub fn t_min_c(self, palette: SchemePalette, lower_bound: f64, upper_bound: f64) -> f64 {
        coerce_in(self.peak(palette, false), lower_bound, upper_bound)
    }

    /// [`Self::t_max_c`] with a chroma multiplier other than 1 (uncached; used
    /// once).
    pub fn t_max_c_scaled(self, palette: SchemePalette, lower_bound: f64, upper_bound: f64, chroma_multiplier: f64) -> f64 {
        let palette = self.scheme.palette(palette);
        let answer = find_best_tone_for_chroma(palette.hue(), palette.chroma() * chroma_multiplier, 100.0, true);

        coerce_in(answer, lower_bound, upper_bound)
    }

    fn peak(self, palette: SchemePalette, max: bool) -> f64 {
        let slot = &self.peaks[palette as usize * 2 + usize::from(max)];
        let cached = slot.get();

        if !cached.is_nan() {
            return cached;
        }

        let pal = self.scheme.palette(palette);
        let answer = find_best_tone_for_chroma(pal.hue(), pal.chroma(), if max { 100.0 } else { 0.0 }, max);

        slot.set(answer);

        answer
    }
}

/// Resolves many colors of one scheme while sharing the memo, e.g. to build a
/// full theme (~60 roles).
pub struct SchemeResolver<'a> {
    scheme: &'a DynamicScheme,
    cache: ToneCache,
}

impl<'a> SchemeResolver<'a> {
    pub const fn new(scheme: &'a DynamicScheme) -> Self {
        Self {
            scheme,
            cache: ToneCache::new(),
        }
    }

    #[inline]
    pub const fn context(&self) -> Context<'_> {
        Context::new(self.scheme, &self.cache)
    }

    pub fn tone(&self, color: impl Into<DynamicColor<'a>>) -> f64 {
        self.context().tone(color.into())
    }

    pub fn hct(&self, color: impl Into<DynamicColor<'a>>) -> Hct {
        self.context().hct(color.into())
    }

    pub fn rgb(&self, color: impl Into<DynamicColor<'a>>) -> Rgb {
        self.hct(color).into()
    }
}

#[inline]
pub(super) const fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

/// Round half up for the tone range. Casts saturate
/// and truncate toward zero; the correction makes it a floor.
#[inline]
const fn round_to_int(x: f64) -> i64 {
    let y = x + 0.5;
    let t = y as i64;

    if (t as f64) > y { t - 1 } else { t }
}

const fn alpha(opacity: Option<f64>) -> u8 {
    match opacity {
        Some(opacity) => {
            let opacity = round_to_int(opacity * 255.0);

            if opacity < 0 {
                0
            } else if opacity > 255 {
                255
            } else {
                opacity as u8
            }
        }
        None => 255,
    }
}

macro_rules! roles {
    ($($variant:ident => $name:literal,)*) => {
        /// Every color role defined by `ColorSpec`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum Role {
            $($variant,)*
        }

        impl Role {
            pub const COUNT: usize = [$($name,)*].len();
            pub const ALL: [Self; Self::COUNT] = [$(Self::$variant,)*];

            /// The token name, e.g. `"on_primary_container"`.
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }
        }
    };
}

roles! {
    PrimaryPaletteKeyColor => "primary_palette_key_color",
    SecondaryPaletteKeyColor => "secondary_palette_key_color",
    TertiaryPaletteKeyColor => "tertiary_palette_key_color",
    NeutralPaletteKeyColor => "neutral_palette_key_color",
    NeutralVariantPaletteKeyColor => "neutral_variant_palette_key_color",
    ErrorPaletteKeyColor => "error_palette_key_color",

    Background => "background",
    OnBackground => "on_background",
    Surface => "surface",
    SurfaceDim => "surface_dim",
    SurfaceBright => "surface_bright",
    SurfaceContainerLowest => "surface_container_lowest",
    SurfaceContainerLow => "surface_container_low",
    SurfaceContainer => "surface_container",
    SurfaceContainerHigh => "surface_container_high",
    SurfaceContainerHighest => "surface_container_highest",
    OnSurface => "on_surface",
    SurfaceVariant => "surface_variant",
    OnSurfaceVariant => "on_surface_variant",
    InverseSurface => "inverse_surface",
    InverseOnSurface => "inverse_on_surface",
    Outline => "outline",
    OutlineVariant => "outline_variant",
    Shadow => "shadow",
    Scrim => "scrim",
    SurfaceTint => "surface_tint",

    Primary => "primary",
    PrimaryDim => "primary_dim",
    OnPrimary => "on_primary",
    PrimaryContainer => "primary_container",
    OnPrimaryContainer => "on_primary_container",
    InversePrimary => "inverse_primary",

    Secondary => "secondary",
    SecondaryDim => "secondary_dim",
    OnSecondary => "on_secondary",
    SecondaryContainer => "secondary_container",
    OnSecondaryContainer => "on_secondary_container",

    Tertiary => "tertiary",
    TertiaryDim => "tertiary_dim",
    OnTertiary => "on_tertiary",
    TertiaryContainer => "tertiary_container",
    OnTertiaryContainer => "on_tertiary_container",

    Error => "error",
    ErrorDim => "error_dim",
    OnError => "on_error",
    ErrorContainer => "error_container",
    OnErrorContainer => "on_error_container",

    PrimaryFixed => "primary_fixed",
    PrimaryFixedDim => "primary_fixed_dim",
    OnPrimaryFixed => "on_primary_fixed",
    OnPrimaryFixedVariant => "on_primary_fixed_variant",

    SecondaryFixed => "secondary_fixed",
    SecondaryFixedDim => "secondary_fixed_dim",
    OnSecondaryFixed => "on_secondary_fixed",
    OnSecondaryFixedVariant => "on_secondary_fixed_variant",

    TertiaryFixed => "tertiary_fixed",
    TertiaryFixedDim => "tertiary_fixed_dim",
    OnTertiaryFixed => "on_tertiary_fixed",
    OnTertiaryFixedVariant => "on_tertiary_fixed_variant",
}

impl Role {
    #[inline]
    pub const fn color(self) -> DynamicColor<'static> {
        DynamicColor::Material(self)
    }

    /// Whether this is one of the `*_fixed_dim` roles.
    #[inline]
    pub const fn is_fixed_dim(self) -> bool {
        matches!(self, Self::PrimaryFixedDim | Self::SecondaryFixedDim | Self::TertiaryFixedDim)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RoleMap<T>(pub(crate) [T; Role::COUNT]);

impl<T> RoleMap<T> {
    #[inline]
    pub const fn from_array(values: [T; Role::COUNT]) -> Self {
        Self(values)
    }

    #[inline]
    pub const fn get(&self, role: Role) -> &T {
        &self.0[role as usize]
    }

    #[inline]
    pub const fn as_array(&self) -> &[T; Role::COUNT] {
        &self.0
    }

    /// `(role, value)` pairs in `Role` order.
    pub fn iter(&self) -> impl Iterator<Item = (Role, &T)> {
        Role::ALL.into_iter().zip(self.0.iter())
    }
}

impl<T> Index<Role> for RoleMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, role: Role) -> &T {
        &self.0[role as usize]
    }
}

impl<T> IndexMut<Role> for RoleMap<T> {
    #[inline]
    fn index_mut(&mut self, role: Role) -> &mut T {
        &mut self.0[role as usize]
    }
}
