use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

use super::Variant;
use crate::{
    color::Rgb,
    dynamic_color::{color_spec::SpecVersion, color_spec_2021::ColorSpec2021, color_spec_2025::ColorSpec2025},
    hct::Hct,
    palette::TonalPalette,
    scheme::variant::{
        SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome, SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
    },
    utils::math::sanitize_degrees_double,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Platform {
    Phone,
    Watch,
}

pub const DEFAULT_PLATFORM: Platform = Platform::Phone;
pub const DEFAULT_SPEC_VERSION: SpecVersion = SpecVersion::Spec2021;

/// Constructed by a set of values representing the current UI state and
/// provides a set of [`TonalPalette`]s that can create colors that fit in with
/// the theme style.
///
/// Used by [`DynamicColor`] to resolve into a color.
///
/// [`DynamicColor`]: super::DynamicColor
#[derive(Clone, PartialOrd)]
pub struct DynamicScheme {
    /// The source color of the scheme in HCT format.
    pub source_color_hct: Hct,

    /// The variant of the scheme.
    pub variant: Variant,

    /// Whether or not the scheme is dark mode.
    pub is_dark: bool,

    /// Value from -1 to 1. -1 represents minimum contrast, 0 represents
    /// standard (i.e. the design as spec'd), and 1 represents maximum contrast.
    pub contrast_level: f64,

    pub platform: Platform,
    pub spec_version: SpecVersion,

    pub primary_palette: TonalPalette,
    pub secondary_palette: TonalPalette,
    pub tertiary_palette: TonalPalette,
    pub neutral_palette: TonalPalette,
    pub neutral_variant_palette: TonalPalette,
    pub error_palette: TonalPalette,
}

impl DynamicScheme {
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
            variant,
            is_dark,
            contrast_level: contrast_level.unwrap_or(0.0),
            platform: DEFAULT_PLATFORM,
            spec_version: DEFAULT_SPEC_VERSION,
            primary_palette,
            secondary_palette,
            tertiary_palette,
            neutral_palette,
            neutral_variant_palette,
            error_palette: error_palette.unwrap_or_else(|| TonalPalette::of(25.0, 84.0)),
        }
    }

    #[must_use]
    pub const fn with_spec_version(mut self, version: SpecVersion) -> Self {
        self.spec_version = version;

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
        }
    }

    fn get_piecewise_value(source_hue: f64, hue_breakpoints: &[f64], hues: &[f64]) -> f64 {
        let size = hue_breakpoints.len().cast_signed().min(hues.len().cast_signed() - 1);

        for i in 0..size {
            let i = i.cast_unsigned();

            if source_hue >= hue_breakpoints[i] && source_hue < hue_breakpoints[i + 1] {
                return sanitize_degrees_double(hues[i]);
            }
        }

        // No condition matched, return the source value.
        source_hue
    }

    /// # Panics
    ///
    /// Will panic if the count of hues does not equal the count of rotations
    pub fn get_rotated_hue(source_hue: f64, hue_breakpoints: &[f64], rotations: &[f64]) -> f64 {
        sanitize_degrees_double(
            source_hue
                + if rotations.len().cast_signed().min(hue_breakpoints.len().cast_signed() - 1) <= 0 {
                    // No condition matched, return the source hue.
                    0.0
                } else {
                    Self::get_piecewise_value(source_hue, hue_breakpoints, rotations)
                },
        )
    }

    pub fn primary_palette_key_color(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::primary_palette_key_color(),
            SpecVersion::Spec2025 => ColorSpec2025::primary_palette_key_color(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn secondary_palette_key_color(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::secondary_palette_key_color(),
            SpecVersion::Spec2025 => ColorSpec2025::secondary_palette_key_color(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn tertiary_palette_key_color(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::tertiary_palette_key_color(),
            SpecVersion::Spec2025 => ColorSpec2025::tertiary_palette_key_color(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn neutral_palette_key_color(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::neutral_palette_key_color(),
            SpecVersion::Spec2025 => ColorSpec2025::neutral_palette_key_color(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn neutral_variant_palette_key_color(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::neutral_palette_key_color(),
            SpecVersion::Spec2025 => ColorSpec2025::neutral_palette_key_color(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn background(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::background(),
            SpecVersion::Spec2025 => ColorSpec2025::background(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_background(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_background(),
            SpecVersion::Spec2025 => ColorSpec2025::on_background(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface(),
            SpecVersion::Spec2025 => ColorSpec2025::surface(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_dim(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_dim(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_dim(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_bright(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_bright(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_bright(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_container_lowest(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_container_lowest(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_container_lowest(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_container_low(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_container_low(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_container_low(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_container(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_container_high(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_container_high(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_container_high(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_container_highest(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_container_highest(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_container_highest(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_surface(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_surface(),
            SpecVersion::Spec2025 => ColorSpec2025::on_surface(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_variant(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_variant(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_variant(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_surface_variant(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_surface_variant(),
            SpecVersion::Spec2025 => ColorSpec2025::on_surface_variant(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn inverse_surface(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::inverse_surface(),
            SpecVersion::Spec2025 => ColorSpec2025::inverse_surface(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn inverse_on_surface(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::inverse_on_surface(),
            SpecVersion::Spec2025 => ColorSpec2025::inverse_on_surface(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn outline(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::outline(),
            SpecVersion::Spec2025 => ColorSpec2025::outline(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn outline_variant(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::outline_variant(),
            SpecVersion::Spec2025 => ColorSpec2025::outline_variant(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn shadow(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::shadow(),
            SpecVersion::Spec2025 => ColorSpec2025::shadow(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn scrim(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::scrim(),
            SpecVersion::Spec2025 => ColorSpec2025::scrim(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn surface_tint(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::surface_tint(),
            SpecVersion::Spec2025 => ColorSpec2025::surface_tint(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn primary(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::primary(),
            SpecVersion::Spec2025 => ColorSpec2025::primary(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_primary(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_primary(),
            SpecVersion::Spec2025 => ColorSpec2025::on_primary(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn primary_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::primary_container(),
            SpecVersion::Spec2025 => ColorSpec2025::primary_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_primary_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_primary_container(),
            SpecVersion::Spec2025 => ColorSpec2025::on_primary_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn inverse_primary(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::inverse_primary(),
            SpecVersion::Spec2025 => ColorSpec2025::inverse_primary(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn secondary(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::secondary(),
            SpecVersion::Spec2025 => ColorSpec2025::secondary(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_secondary(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_secondary(),
            SpecVersion::Spec2025 => ColorSpec2025::on_secondary(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn secondary_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::secondary_container(),
            SpecVersion::Spec2025 => ColorSpec2025::secondary_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_secondary_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_secondary_container(),
            SpecVersion::Spec2025 => ColorSpec2025::on_secondary_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn tertiary(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::tertiary(),
            SpecVersion::Spec2025 => ColorSpec2025::tertiary(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_tertiary(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_tertiary(),
            SpecVersion::Spec2025 => ColorSpec2025::on_tertiary(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn tertiary_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::tertiary_container(),
            SpecVersion::Spec2025 => ColorSpec2025::tertiary_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_tertiary_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_tertiary_container(),
            SpecVersion::Spec2025 => ColorSpec2025::on_tertiary_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn error(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::error(),
            SpecVersion::Spec2025 => ColorSpec2025::error(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_error(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_error(),
            SpecVersion::Spec2025 => ColorSpec2025::on_error(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn error_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::error_container(),
            SpecVersion::Spec2025 => ColorSpec2025::error_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_error_container(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_error_container(),
            SpecVersion::Spec2025 => ColorSpec2025::on_error_container(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn primary_fixed(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::primary_fixed(),
            SpecVersion::Spec2025 => ColorSpec2025::primary_fixed(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn primary_fixed_dim(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::primary_fixed_dim(),
            SpecVersion::Spec2025 => ColorSpec2025::primary_fixed_dim(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_primary_fixed(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_primary_fixed(),
            SpecVersion::Spec2025 => ColorSpec2025::on_primary_fixed(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_primary_fixed_variant(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_primary_fixed_variant(),
            SpecVersion::Spec2025 => ColorSpec2025::on_primary_fixed_variant(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn secondary_fixed(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::secondary_fixed(),
            SpecVersion::Spec2025 => ColorSpec2025::secondary_fixed(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn secondary_fixed_dim(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::secondary_fixed_dim(),
            SpecVersion::Spec2025 => ColorSpec2025::secondary_fixed_dim(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_secondary_fixed(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_secondary_fixed(),
            SpecVersion::Spec2025 => ColorSpec2025::on_secondary_fixed(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_secondary_fixed_variant(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_secondary_fixed_variant(),
            SpecVersion::Spec2025 => ColorSpec2025::on_secondary_fixed_variant(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn tertiary_fixed(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::tertiary_fixed(),
            SpecVersion::Spec2025 => ColorSpec2025::tertiary_fixed(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn tertiary_fixed_dim(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::tertiary_fixed_dim(),
            SpecVersion::Spec2025 => ColorSpec2025::tertiary_fixed_dim(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_tertiary_fixed(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_tertiary_fixed(),
            SpecVersion::Spec2025 => ColorSpec2025::on_tertiary_fixed(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }

    pub fn on_tertiary_fixed_variant(&self) -> Rgb {
        match self.spec_version {
            SpecVersion::Spec2021 => ColorSpec2021::on_tertiary_fixed_variant(),
            SpecVersion::Spec2025 => ColorSpec2025::on_tertiary_fixed_variant(),
            SpecVersion::Spec2026 => todo!(),
        }
        .get_rgb(self)
    }
}

impl Ord for DynamicScheme {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for DynamicScheme {
    fn eq(&self, other: &Self) -> bool {
        self.source_color_hct == other.source_color_hct
            && self.variant == other.variant
            && self.is_dark == other.is_dark
            && self.contrast_level == other.contrast_level
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
        self.variant.hash(state);
        self.is_dark.hash(state);
        self.contrast_level.to_bits().hash(state);
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
        writeln!(f, "Scheme {{")?;
        writeln!(f, "  primary = {}", self.primary())?;
        writeln!(f, "  on_primary = {}", self.on_primary())?;
        writeln!(f, "  primary_container = {}", self.primary_container())?;
        writeln!(f, "  on_primary_container = {}", self.on_primary_container())?;
        writeln!(f, "  secondary = {}", self.secondary())?;
        writeln!(f, "  on_secondary = {}", self.on_secondary())?;
        writeln!(f, "  secondary_container = {}", self.secondary_container())?;
        writeln!(f, "  on_secondary_container = {}", self.on_secondary_container())?;
        writeln!(f, "  tertiary = {}", self.tertiary())?;
        writeln!(f, "  on_tertiary = {}", self.on_tertiary())?;
        writeln!(f, "  tertiary_container = {}", self.tertiary_container())?;
        writeln!(f, "  on_tertiary_container = {}", self.on_tertiary_container())?;
        writeln!(f, "  error = {}", self.error())?;
        writeln!(f, "  on_error = {}", self.on_error())?;
        writeln!(f, "  error_container = {}", self.error_container())?;
        writeln!(f, "  on_error_container = {}", self.on_error_container())?;
        writeln!(f, "  background = {}", self.background())?;
        writeln!(f, "  on_background = {}", self.on_background())?;
        writeln!(f, "  surface = {}", self.surface())?;
        writeln!(f, "  on_surface = {}", self.on_surface())?;
        writeln!(f, "  surface_variant = {}", self.surface_variant())?;
        writeln!(f, "  on_surface_variant = {}", self.on_surface_variant())?;
        writeln!(f, "  outline = {}", self.outline())?;
        writeln!(f, "  outline_variant = {}", self.outline_variant())?;
        writeln!(f, "  shadow = {}", self.shadow())?;
        writeln!(f, "  scrim = {}", self.scrim())?;
        writeln!(f, "  inverse_surface = {}", self.inverse_surface())?;
        writeln!(f, "  inverse_on_surface = {}", self.inverse_on_surface())?;
        writeln!(f, "  inverse_primary = {}", self.inverse_primary())?;
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::{dynamic_color::DynamicScheme, hct::Hct};

    #[test]
    fn test_0_length_input() {
        let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[], &[]);

        assert_approx_eq!(f64, hue, 43.0, epsilon = 1.0);
    }

    #[test]
    fn test_1_length_input_no_rotation() {
        let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[0.0], &[0.0]);

        assert_approx_eq!(f64, hue, 43.0, epsilon = 1.0);
    }

    #[test]
    fn test_on_boundary_rotation_correct() {
        let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[0.0, 42.0, 360.0], &[0.0, 15.0, 0.0]);

        assert_approx_eq!(f64, hue, 43.0 + 15.0, epsilon = 1.0);
    }

    #[test]
    fn test_rotation_result_larger_than_360_degrees_wraps() {
        let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[0.0, 42.0, 360.0], &[0.0, 480.0, 0.0]);

        assert_approx_eq!(f64, hue, 163.0, epsilon = 1.0);
    }
}
