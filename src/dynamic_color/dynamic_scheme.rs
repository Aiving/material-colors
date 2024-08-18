use super::{MaterialDynamicColors, Variant};
use crate::{
    color::Argb,
    hct::Hct,
    palette::TonalPalette,
    scheme::variant::{
        SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
        SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
    },
    utils::math::sanitize_degrees_double,
};
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

/// Constructed by a set of values representing the current UI state (such as
/// whether or not its dark theme, what the theme style is, etc.), and
/// provides a set of [`TonalPalette`]s that can create colors that fit in
/// with the theme style. Used by [`DynamicColor`] to resolve into a color.
///
/// [`DynamicColor`]: super::DynamicColor
#[derive(Clone, PartialOrd)]
pub struct DynamicScheme {
    /// The source color of the theme in HCT.
    pub source_color_hct: Hct,

    /// The variant, or style, of the theme.
    pub variant: Variant,

    /// Whether or not the scheme is in 'dark mode' or 'light mode'.
    pub is_dark: bool,

    /// Value from -1 to 1. -1 represents minimum contrast, 0 represents
    /// standard (i.e. the design as spec'd), and 1 represents maximum contrast.
    pub contrast_level: f64,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually colorful.
    pub primary_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually less colorful.
    pub secondary_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually a different hue from
    /// primary and colorful.
    pub tertiary_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually not colorful at all,
    /// intended for background & surface colors.
    pub neutral_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually not colorful, but
    /// slightly more colorful than Neutral. Intended for backgrounds & surfaces.
    pub neutral_variant_palette: TonalPalette,

    /// Given a tone, produces a reddish, colorful, color.
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
            primary_palette,
            secondary_palette,
            tertiary_palette,
            neutral_palette,
            neutral_variant_palette,
            error_palette: error_palette.unwrap_or_else(|| TonalPalette::of(25.0, 84.0)),
        }
    }

    pub fn by_variant(
        source: Argb,
        variant: &Variant,
        is_dark: bool,
        contrast_level: Option<f64>,
    ) -> Self {
        let source_hct = source.into();

        match variant {
            Variant::Monochrome => {
                SchemeMonochrome::new(source_hct, is_dark, contrast_level).scheme
            }
            Variant::Neutral => SchemeNeutral::new(source_hct, is_dark, contrast_level).scheme,
            Variant::TonalSpot => SchemeTonalSpot::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Vibrant => SchemeVibrant::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Expressive => {
                SchemeExpressive::new(source_hct, is_dark, contrast_level).scheme
            }
            Variant::Fidelity => SchemeFidelity::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Content => SchemeContent::new(source_hct, is_dark, contrast_level).scheme,
            Variant::Rainbow => SchemeRainbow::new(source_hct, is_dark, contrast_level).scheme,
            Variant::FruitSalad => {
                SchemeFruitSalad::new(source_hct, is_dark, contrast_level).scheme
            }
        }
    }

    /// # Panics
    ///
    /// Will panic if the count of hues does not equal the count of rotations
    pub fn get_rotated_hue(source_hue: f64, hues: &[f64], rotations: &[f64]) -> f64 {
        assert!(hues.len() == rotations.len());

        if rotations.len() == 1 {
            return sanitize_degrees_double(source_hue + rotations[0]);
        }

        if hues.is_empty() || rotations.is_empty() {
            return source_hue;
        }

        let size = hues.len();
        let mut i = 0;

        while i <= (size - 2) {
            let this_hue = hues[i];
            let next_hue = hues[i + 1];

            if this_hue < source_hue && source_hue < next_hue {
                return sanitize_degrees_double(source_hue + rotations[i]);
            }

            i += 1;
        }

        // If this statement executes, something is wrong, there should have been a rotation
        // found using the arrays.
        source_hue
    }

    pub fn primary_palette_key_color(&self) -> Argb {
        MaterialDynamicColors::primary_palette_key_color().get_argb(self)
    }

    pub fn secondary_palette_key_color(&self) -> Argb {
        MaterialDynamicColors::secondary_palette_key_color().get_argb(self)
    }

    pub fn tertiary_palette_key_color(&self) -> Argb {
        MaterialDynamicColors::tertiary_palette_key_color().get_argb(self)
    }

    pub fn neutral_palette_key_color(&self) -> Argb {
        MaterialDynamicColors::neutral_palette_key_color().get_argb(self)
    }

    pub fn neutral_variant_palette_key_color(&self) -> Argb {
        MaterialDynamicColors::neutral_palette_key_color().get_argb(self)
    }

    pub fn background(&self) -> Argb {
        MaterialDynamicColors::background().get_argb(self)
    }

    pub fn on_background(&self) -> Argb {
        MaterialDynamicColors::on_background().get_argb(self)
    }

    pub fn surface(&self) -> Argb {
        MaterialDynamicColors::surface().get_argb(self)
    }

    pub fn surface_dim(&self) -> Argb {
        MaterialDynamicColors::surface_dim().get_argb(self)
    }

    pub fn surface_bright(&self) -> Argb {
        MaterialDynamicColors::surface_bright().get_argb(self)
    }

    pub fn surface_container_lowest(&self) -> Argb {
        MaterialDynamicColors::surface_container_lowest().get_argb(self)
    }

    pub fn surface_container_low(&self) -> Argb {
        MaterialDynamicColors::surface_container_low().get_argb(self)
    }

    pub fn surface_container(&self) -> Argb {
        MaterialDynamicColors::surface_container().get_argb(self)
    }

    pub fn surface_container_high(&self) -> Argb {
        MaterialDynamicColors::surface_container_high().get_argb(self)
    }

    pub fn surface_container_highest(&self) -> Argb {
        MaterialDynamicColors::surface_container_highest().get_argb(self)
    }

    pub fn on_surface(&self) -> Argb {
        MaterialDynamicColors::on_surface().get_argb(self)
    }

    pub fn surface_variant(&self) -> Argb {
        MaterialDynamicColors::surface_variant().get_argb(self)
    }

    pub fn on_surface_variant(&self) -> Argb {
        MaterialDynamicColors::on_surface_variant().get_argb(self)
    }

    pub fn inverse_surface(&self) -> Argb {
        MaterialDynamicColors::inverse_surface().get_argb(self)
    }

    pub fn inverse_on_surface(&self) -> Argb {
        MaterialDynamicColors::inverse_on_surface().get_argb(self)
    }

    pub fn outline(&self) -> Argb {
        MaterialDynamicColors::outline().get_argb(self)
    }

    pub fn outline_variant(&self) -> Argb {
        MaterialDynamicColors::outline_variant().get_argb(self)
    }

    pub fn shadow(&self) -> Argb {
        MaterialDynamicColors::shadow().get_argb(self)
    }

    pub fn scrim(&self) -> Argb {
        MaterialDynamicColors::scrim().get_argb(self)
    }

    pub fn surface_tint(&self) -> Argb {
        MaterialDynamicColors::surface_tint().get_argb(self)
    }

    pub fn primary(&self) -> Argb {
        MaterialDynamicColors::primary().get_argb(self)
    }

    pub fn on_primary(&self) -> Argb {
        MaterialDynamicColors::on_primary().get_argb(self)
    }

    pub fn primary_container(&self) -> Argb {
        MaterialDynamicColors::primary_container().get_argb(self)
    }

    pub fn on_primary_container(&self) -> Argb {
        MaterialDynamicColors::on_primary_container().get_argb(self)
    }

    pub fn inverse_primary(&self) -> Argb {
        MaterialDynamicColors::inverse_primary().get_argb(self)
    }

    pub fn secondary(&self) -> Argb {
        MaterialDynamicColors::secondary().get_argb(self)
    }

    pub fn on_secondary(&self) -> Argb {
        MaterialDynamicColors::on_secondary().get_argb(self)
    }

    pub fn secondary_container(&self) -> Argb {
        MaterialDynamicColors::secondary_container().get_argb(self)
    }

    pub fn on_secondary_container(&self) -> Argb {
        MaterialDynamicColors::on_secondary_container().get_argb(self)
    }

    pub fn tertiary(&self) -> Argb {
        MaterialDynamicColors::tertiary().get_argb(self)
    }

    pub fn on_tertiary(&self) -> Argb {
        MaterialDynamicColors::on_tertiary().get_argb(self)
    }

    pub fn tertiary_container(&self) -> Argb {
        MaterialDynamicColors::tertiary_container().get_argb(self)
    }

    pub fn on_tertiary_container(&self) -> Argb {
        MaterialDynamicColors::on_tertiary_container().get_argb(self)
    }

    pub fn error(&self) -> Argb {
        MaterialDynamicColors::error().get_argb(self)
    }

    pub fn on_error(&self) -> Argb {
        MaterialDynamicColors::on_error().get_argb(self)
    }

    pub fn error_container(&self) -> Argb {
        MaterialDynamicColors::error_container().get_argb(self)
    }

    pub fn on_error_container(&self) -> Argb {
        MaterialDynamicColors::on_error_container().get_argb(self)
    }

    pub fn primary_fixed(&self) -> Argb {
        MaterialDynamicColors::primary_fixed().get_argb(self)
    }

    pub fn primary_fixed_dim(&self) -> Argb {
        MaterialDynamicColors::primary_fixed_dim().get_argb(self)
    }

    pub fn on_primary_fixed(&self) -> Argb {
        MaterialDynamicColors::on_primary_fixed().get_argb(self)
    }

    pub fn on_primary_fixed_variant(&self) -> Argb {
        MaterialDynamicColors::on_primary_fixed_variant().get_argb(self)
    }

    pub fn secondary_fixed(&self) -> Argb {
        MaterialDynamicColors::secondary_fixed().get_argb(self)
    }

    pub fn secondary_fixed_dim(&self) -> Argb {
        MaterialDynamicColors::secondary_fixed_dim().get_argb(self)
    }

    pub fn on_secondary_fixed(&self) -> Argb {
        MaterialDynamicColors::on_secondary_fixed().get_argb(self)
    }

    pub fn on_secondary_fixed_variant(&self) -> Argb {
        MaterialDynamicColors::on_secondary_fixed_variant().get_argb(self)
    }

    pub fn tertiary_fixed(&self) -> Argb {
        MaterialDynamicColors::tertiary_fixed().get_argb(self)
    }

    pub fn tertiary_fixed_dim(&self) -> Argb {
        MaterialDynamicColors::tertiary_fixed_dim().get_argb(self)
    }

    pub fn on_tertiary_fixed(&self) -> Argb {
        MaterialDynamicColors::on_tertiary_fixed().get_argb(self)
    }

    pub fn on_tertiary_fixed_variant(&self) -> Argb {
        MaterialDynamicColors::on_tertiary_fixed_variant().get_argb(self)
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
        writeln!(
            f,
            "  on_primary_container = {}",
            self.on_primary_container()
        )?;
        writeln!(f, "  secondary = {}", self.secondary())?;
        writeln!(f, "  on_secondary = {}", self.on_secondary())?;
        writeln!(f, "  secondary_container = {}", self.secondary_container())?;
        writeln!(
            f,
            "  on_secondary_container = {}",
            self.on_secondary_container()
        )?;
        writeln!(f, "  tertiary = {}", self.tertiary())?;
        writeln!(f, "  on_tertiary = {}", self.on_tertiary())?;
        writeln!(f, "  tertiary_container = {}", self.tertiary_container())?;
        writeln!(
            f,
            "  on_tertiary_container = {}",
            self.on_tertiary_container()
        )?;
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
    use crate::{dynamic_color::DynamicScheme, hct::Hct};
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_0_length_input() {
        let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[], &[]);

        assert_approx_eq!(f64, hue, 43.0, epsilon = 1.0);
    }

    #[test]
    fn test_1_length_input_no_rotation() {
        let hue =
            DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0).get_hue(), &[0.0], &[0.0]);

        assert_approx_eq!(f64, hue, 43.0, epsilon = 1.0);
    }

    #[test]
    fn test_on_boundary_rotation_correct() {
        let hue = DynamicScheme::get_rotated_hue(
            Hct::from(43.0, 16.0, 16.0).get_hue(),
            &[0.0, 42.0, 360.0],
            &[0.0, 15.0, 0.0],
        );

        assert_approx_eq!(f64, hue, 43.0 + 15.0, epsilon = 1.0);
    }

    #[test]
    fn test_rotation_result_larger_than_360_degrees_wraps() {
        let hue = DynamicScheme::get_rotated_hue(
            Hct::from(43.0, 16.0, 16.0).get_hue(),
            &[0.0, 42.0, 360.0],
            &[0.0, 480.0, 0.0],
        );

        assert_approx_eq!(f64, hue, 163.0, epsilon = 1.0);
    }
}
