use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

use super::{DynamicColor, Role, SchemeResolver, SpecVersion, Variant, color_spec};
use crate::{
    color::Rgb,
    hct::Hct,
    palette::{Palette, TonalPalette},
    scheme::variant::{
        SchemeCmf, SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome, SchemeNeutral, SchemeRainbow, SchemeTonalSpot,
        SchemeVibrant,
    },
};

/// The platform on which a scheme is intended to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Platform {
    #[default]
    Phone,
    Watch,
}

pub const DEFAULT_PLATFORM: Platform = Platform::Phone;
pub const DEFAULT_SPEC_VERSION: SpecVersion = SpecVersion::Spec2021;

/// Identifies one of the six palettes owned by a [`DynamicScheme`].
///
/// Built-in color definitions refer to palettes by this tag instead of by a
/// closure, which keeps the definition tables plain data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SchemePalette {
    Primary,
    Secondary,
    Tertiary,
    Neutral,
    NeutralVariant,
    Error,
}

impl SchemePalette {
    pub const COUNT: usize = 6;
}

/// Constructed by a set of values representing the current UI state and
/// provides a set of [`TonalPalette`]s that can create colors that fit in with
/// the theme style.
///
/// Used by [`DynamicColor`] to resolve into a color.
///
/// At most two source colors are used, so they are stored as
/// `source_color_hct` + `secondary_source_color_hct` to stay allocation-free.
#[derive(Clone, PartialOrd)]
pub struct DynamicScheme {
    /// The source color of the scheme in HCT format.
    pub source_color_hct: Hct,

    /// Optional second source color, used by the
    /// 2026 spec for tertiary roles.
    pub secondary_source_color_hct: Option<Hct>,

    /// The variant of the scheme.
    pub variant: Variant,

    /// Whether or not the scheme is dark mode.
    pub is_dark: bool,

    /// Value from -1 to 1. -1 represents minimum contrast, 0 represents
    /// standard (i.e. the design as spec'd), and 1 represents maximum contrast.
    pub contrast_level: f64,

    pub platform: Platform,

    /// Always the *effective* version (after [`Self::fallback_spec_version`]);
    /// set it through [`Self::with_spec_version`].
    pub spec_version: SpecVersion,

    pub primary_palette: TonalPalette,
    pub secondary_palette: TonalPalette,
    pub tertiary_palette: TonalPalette,
    pub neutral_palette: TonalPalette,
    pub neutral_variant_palette: TonalPalette,
    pub error_palette: TonalPalette,
}

impl DynamicScheme {
    /// Same signature as before the port: spec 2021, phone.
    pub fn new(
        source_color_hct: Hct,
        variant: Variant,
        is_dark: bool,
        contrast_level: Option<f64>,
        primary_palette: TonalPalette,
        secondary_palette: TonalPalette,
        tertiary_palette: TonalPalette,
        neutral_palette: TonalPalette,
        neutral_variant_palette: TonalPalette,
        error_palette: Option<TonalPalette>,
    ) -> Self {
        Self {
            source_color_hct,
            secondary_source_color_hct: None,
            variant,
            is_dark,
            contrast_level: contrast_level.unwrap_or(0.0),
            platform: DEFAULT_PLATFORM,
            spec_version: Self::fallback_spec_version(DEFAULT_SPEC_VERSION, variant),
            primary_palette,
            secondary_palette,
            tertiary_palette,
            neutral_palette,
            neutral_variant_palette,
            error_palette: error_palette.unwrap_or_else(|| TonalPalette::of(25.0, 84.0)),
        }
    }

    /// Builds a scheme whose palettes come from the given spec.
    ///
    /// The effective spec version follows [`Self::fallback_spec_version`]
    /// (e.g. `Content` is always 2021). `Variant::Cmf` is delegated to
    /// [`SchemeCmf`], which only exists for spec 2026.
    pub fn from_spec(
        source_color_hct: Hct,
        variant: Variant,
        is_dark: bool,
        contrast_level: Option<f64>,
        platform: Platform,
        spec_version: SpecVersion,
    ) -> Self {
        if variant == Variant::Cmf {
            return SchemeCmf::with_sources(source_color_hct, None, is_dark, contrast_level, platform).scheme;
        }

        let contrast = contrast_level.unwrap_or(0.0);
        let palette = |palette: Palette| Self::spec_palette(variant, source_color_hct, &palette, is_dark, platform, contrast, spec_version);

        Self {
            source_color_hct,
            secondary_source_color_hct: None,
            variant,
            is_dark,
            contrast_level: contrast,
            platform,
            spec_version: Self::fallback_spec_version(spec_version, variant),
            primary_palette: palette(Palette::Primary),
            secondary_palette: palette(Palette::Secondary),
            tertiary_palette: palette(Palette::Tertiary),
            neutral_palette: palette(Palette::Neutral),
            neutral_variant_palette: palette(Palette::NeutralVariant),
            error_palette: palette(Palette::Error),
        }
    }

    /// One palette of a scheme, from `color_spec(spec_version)`,
    /// with `SchemeCmf`'s palettes for `Variant::Cmf`.
    pub fn spec_palette(
        variant: Variant,
        source_color_hct: Hct,
        palette: &Palette,
        is_dark: bool,
        platform: Platform,
        contrast_level: f64,
        spec_version: SpecVersion,
    ) -> TonalPalette {
        if variant == Variant::Cmf {
            return SchemeCmf::palette(&source_color_hct, palette);
        }

        let spec = color_spec(Self::fallback_spec_version(spec_version, variant));
        let palette = match palette {
            Palette::Primary => spec.get_primary_palette(variant, source_color_hct, is_dark, platform, contrast_level),
            Palette::Secondary => spec.get_secondary_palette(variant, source_color_hct, is_dark, platform, contrast_level),
            Palette::Tertiary => spec.get_tertiary_palette(variant, source_color_hct, is_dark, platform, contrast_level),
            Palette::Neutral => spec.get_neutral_palette(variant, source_color_hct, is_dark, platform, contrast_level),
            Palette::NeutralVariant => spec.get_neutral_variant_palette(variant, source_color_hct, is_dark, platform, contrast_level),
            Palette::Error => spec.get_error_palette(variant, source_color_hct, is_dark, platform, contrast_level),
        };

        // Every spec returns `None` only for CMF, handled above.
        palette.unwrap_or_else(|| unreachable!("fallback spec has no palettes for {variant:?}"))
    }

    /// Sets the spec version, applying [`Self::fallback_spec_version`] (e.g.
    /// `Content` always resolves to 2021).
    #[must_use]
    pub const fn with_spec_version(mut self, version: SpecVersion) -> Self {
        self.spec_version = Self::fallback_spec_version(version, self.variant);

        self
    }

    #[must_use]
    pub const fn with_platform(mut self, platform: Platform) -> Self {
        self.platform = platform;

        self
    }

    #[must_use]
    pub const fn with_secondary_source_color_hct(mut self, hct: Option<Hct>) -> Self {
        self.secondary_source_color_hct = hct;

        self
    }

    pub fn by_variant(source: Rgb, variant: &Variant, is_dark: bool, contrast_level: Option<f64>) -> Self {
        let source_hct = source.into();

        match variant {
            Variant::Monochrome => SchemeMonochrome::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Neutral => SchemeNeutral::new(source_hct, is_dark, contrast_level).scheme,
            Variant::TonalSpot => SchemeTonalSpot::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Vibrant => SchemeVibrant::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Expressive => SchemeExpressive::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Fidelity => SchemeFidelity::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Content => SchemeContent::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Rainbow => SchemeRainbow::new(source_hct, is_dark, contrast_level).scheme,
            Variant::FruitSalad => SchemeFruitSalad::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Cmf => SchemeCmf::new(source_hct, is_dark, contrast_level).scheme,
        }
    }

    #[inline]
    pub const fn palette(&self, palette: SchemePalette) -> &TonalPalette {
        match palette {
            SchemePalette::Primary => &self.primary_palette,
            SchemePalette::Secondary => &self.secondary_palette,
            SchemePalette::Tertiary => &self.tertiary_palette,
            SchemePalette::Neutral => &self.neutral_palette,
            SchemePalette::NeutralVariant => &self.neutral_variant_palette,
            SchemePalette::Error => &self.error_palette,
        }
    }

    /// The second source color, or the first if there is none.
    #[inline]
    pub fn secondary_source_or_primary(&self) -> Hct {
        self.secondary_source_color_hct.unwrap_or(self.source_color_hct)
    }

    /// Copy of the scheme with another mode and contrast level.
    #[must_use]
    pub fn with_mode(&self, is_dark: bool, contrast_level: f64) -> Self {
        Self {
            is_dark,
            contrast_level,
            ..self.clone()
        }
    }

    /// Returns the spec version to use for `variant`, falling back when the
    /// variant is not supported by `spec_version`.
    pub const fn fallback_spec_version(spec_version: SpecVersion, variant: Variant) -> SpecVersion {
        match variant {
            Variant::Cmf => spec_version,
            Variant::Expressive | Variant::Vibrant | Variant::TonalSpot | Variant::Neutral => match spec_version {
                SpecVersion::Spec2026 => SpecVersion::Spec2025,
                v => v,
            },
            _ => SpecVersion::Spec2021,
        }
    }

    pub fn get_piecewise_value(source_hue: f64, hue_breakpoints: &[f64], hues: &[f64]) -> f64 {
        let size = hue_breakpoints.len().saturating_sub(1).min(hues.len());

        for i in 0..size {
            if source_hue >= hue_breakpoints[i] && source_hue < hue_breakpoints[i + 1] {
                return sanitize_degrees(hues[i]);
            }
        }

        // No condition matched, return the source value.
        source_hue
    }

    /// `source_hue` rotated by the value of its breakpoint segment.
    pub fn get_rotated_hue(source_hue: f64, hue_breakpoints: &[f64], rotations: &[f64]) -> f64 {
        let rotation = if hue_breakpoints.len().saturating_sub(1).min(rotations.len()) == 0 {
            // No condition matched, return the source hue.
            0.0
        } else {
            Self::get_piecewise_value(source_hue, hue_breakpoints, rotations)
        };

        sanitize_degrees(source_hue + rotation)
    }

    /// Resolver sharing one tone memo; use it when reading many colors.
    pub const fn resolver(&self) -> SchemeResolver<'_> {
        SchemeResolver::new(self)
    }

    pub fn get_hct(&self, color: DynamicColor<'_>) -> Hct {
        color.get_hct(self)
    }

    pub fn get_rgb(&self, color: DynamicColor<'_>) -> Rgb {
        color.get_rgb(self)
    }
}

#[inline]
pub(crate) fn sanitize_degrees(degrees: f64) -> f64 {
    let degrees = degrees % 360.0;

    if degrees < 0.0 { degrees + 360.0 } else { degrees }
}

macro_rules! rgb_getters {
    ($($method:ident => $role:ident,)*) => {
        impl DynamicScheme {
            $(
                pub fn $method(&self) -> Rgb {
                    Role::$role.color().get_rgb(self)
                }
            )*
        }
    };
}

rgb_getters! {
    primary_palette_key_color => PrimaryPaletteKeyColor,
    secondary_palette_key_color => SecondaryPaletteKeyColor,
    tertiary_palette_key_color => TertiaryPaletteKeyColor,
    neutral_palette_key_color => NeutralPaletteKeyColor,
    neutral_variant_palette_key_color => NeutralVariantPaletteKeyColor,
    error_palette_key_color => ErrorPaletteKeyColor,
    background => Background,
    on_background => OnBackground,
    surface => Surface,
    surface_dim => SurfaceDim,
    surface_bright => SurfaceBright,
    surface_container_lowest => SurfaceContainerLowest,
    surface_container_low => SurfaceContainerLow,
    surface_container => SurfaceContainer,
    surface_container_high => SurfaceContainerHigh,
    surface_container_highest => SurfaceContainerHighest,
    on_surface => OnSurface,
    surface_variant => SurfaceVariant,
    on_surface_variant => OnSurfaceVariant,
    inverse_surface => InverseSurface,
    inverse_on_surface => InverseOnSurface,
    outline => Outline,
    outline_variant => OutlineVariant,
    shadow => Shadow,
    scrim => Scrim,
    surface_tint => SurfaceTint,
    primary => Primary,
    primary_dim => PrimaryDim,
    on_primary => OnPrimary,
    primary_container => PrimaryContainer,
    on_primary_container => OnPrimaryContainer,
    inverse_primary => InversePrimary,
    secondary => Secondary,
    secondary_dim => SecondaryDim,
    on_secondary => OnSecondary,
    secondary_container => SecondaryContainer,
    on_secondary_container => OnSecondaryContainer,
    tertiary => Tertiary,
    tertiary_dim => TertiaryDim,
    on_tertiary => OnTertiary,
    tertiary_container => TertiaryContainer,
    on_tertiary_container => OnTertiaryContainer,
    error => Error,
    error_dim => ErrorDim,
    on_error => OnError,
    error_container => ErrorContainer,
    on_error_container => OnErrorContainer,
    primary_fixed => PrimaryFixed,
    primary_fixed_dim => PrimaryFixedDim,
    on_primary_fixed => OnPrimaryFixed,
    on_primary_fixed_variant => OnPrimaryFixedVariant,
    secondary_fixed => SecondaryFixed,
    secondary_fixed_dim => SecondaryFixedDim,
    on_secondary_fixed => OnSecondaryFixed,
    on_secondary_fixed_variant => OnSecondaryFixedVariant,
    tertiary_fixed => TertiaryFixed,
    tertiary_fixed_dim => TertiaryFixedDim,
    on_tertiary_fixed => OnTertiaryFixed,
    on_tertiary_fixed_variant => OnTertiaryFixedVariant,
}

impl Ord for DynamicScheme {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialEq for DynamicScheme {
    fn eq(&self, other: &Self) -> bool {
        self.source_color_hct == other.source_color_hct
            && self.secondary_source_color_hct == other.secondary_source_color_hct
            && self.variant == other.variant
            && self.is_dark == other.is_dark
            && self.contrast_level == other.contrast_level
            && self.platform == other.platform
            && self.spec_version == other.spec_version
            && self.primary_palette == other.primary_palette
            && self.secondary_palette == other.secondary_palette
            && self.tertiary_palette == other.tertiary_palette
            && self.neutral_palette == other.neutral_palette
            && self.neutral_variant_palette == other.neutral_variant_palette
            && self.error_palette == other.error_palette
    }
}

impl Eq for DynamicScheme {}

impl Hash for DynamicScheme {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source_color_hct.hash(state);
        self.secondary_source_color_hct.hash(state);
        self.variant.hash(state);
        self.is_dark.hash(state);
        self.contrast_level.to_bits().hash(state);
        self.platform.hash(state);
        self.spec_version.hash(state);
        self.primary_palette.hash(state);
        self.secondary_palette.hash(state);
        self.tertiary_palette.hash(state);
        self.neutral_palette.hash(state);
        self.neutral_variant_palette.hash(state);
        self.error_palette.hash(state);
    }
}

impl fmt::Display for DynamicScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Same roles as before; one shared memo instead of 29 cold resolves.
        let r = self.resolver();

        writeln!(f, "Scheme {{")?;

        for role in [
            Role::Primary,
            Role::OnPrimary,
            Role::PrimaryContainer,
            Role::OnPrimaryContainer,
            Role::Secondary,
            Role::OnSecondary,
            Role::SecondaryContainer,
            Role::OnSecondaryContainer,
            Role::Tertiary,
            Role::OnTertiary,
            Role::TertiaryContainer,
            Role::OnTertiaryContainer,
            Role::Error,
            Role::OnError,
            Role::ErrorContainer,
            Role::OnErrorContainer,
            Role::Background,
            Role::OnBackground,
            Role::Surface,
            Role::OnSurface,
            Role::SurfaceVariant,
            Role::OnSurfaceVariant,
            Role::Outline,
            Role::OutlineVariant,
            Role::Shadow,
            Role::Scrim,
            Role::InverseSurface,
            Role::InverseOnSurface,
            Role::InversePrimary,
        ] {
            writeln!(f, "  {} = {}", role.name(), r.rgb(role))?;
        }

        writeln!(f, "}}")
    }
}
