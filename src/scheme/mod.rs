#![allow(clippy::too_many_arguments, deprecated)]
use crate::{color::Argb, dynamic_color::DynamicScheme, palette::CorePalette, Map};
#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::{array::IntoIter, fmt};
#[cfg(feature = "serde")]
use serde::Serialize;
#[cfg(feature = "std")]
use std::string::String;

pub mod variant;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Scheme {
    pub primary: Argb,
    pub on_primary: Argb,
    pub primary_container: Argb,
    pub on_primary_container: Argb,
    pub inverse_primary: Argb,
    pub primary_fixed: Argb,
    pub primary_fixed_dim: Argb,
    pub on_primary_fixed: Argb,
    pub on_primary_fixed_variant: Argb,
    pub secondary: Argb,
    pub on_secondary: Argb,
    pub secondary_container: Argb,
    pub on_secondary_container: Argb,
    pub secondary_fixed: Argb,
    pub secondary_fixed_dim: Argb,
    pub on_secondary_fixed: Argb,
    pub on_secondary_fixed_variant: Argb,
    pub tertiary: Argb,
    pub on_tertiary: Argb,
    pub tertiary_container: Argb,
    pub on_tertiary_container: Argb,
    pub tertiary_fixed: Argb,
    pub tertiary_fixed_dim: Argb,
    pub on_tertiary_fixed: Argb,
    pub on_tertiary_fixed_variant: Argb,
    pub error: Argb,
    pub on_error: Argb,
    pub error_container: Argb,
    pub on_error_container: Argb,
    pub surface_dim: Argb,
    pub surface: Argb,
    pub surface_tint: Argb,
    pub surface_bright: Argb,
    pub surface_container_lowest: Argb,
    pub surface_container_low: Argb,
    pub surface_container: Argb,
    pub surface_container_high: Argb,
    pub surface_container_highest: Argb,
    pub on_surface: Argb,
    pub on_surface_variant: Argb,
    pub outline: Argb,
    pub outline_variant: Argb,
    pub inverse_surface: Argb,
    pub inverse_on_surface: Argb,
    pub surface_variant: Argb,
    pub background: Argb,
    pub on_background: Argb,
    pub shadow: Argb,
    pub scrim: Argb,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Scheme {
    pub const fn new(
        primary: Argb,
        on_primary: Argb,
        primary_container: Argb,
        on_primary_container: Argb,
        inverse_primary: Argb,
        primary_fixed: Argb,
        primary_fixed_dim: Argb,
        on_primary_fixed: Argb,
        on_primary_fixed_variant: Argb,
        secondary: Argb,
        on_secondary: Argb,
        secondary_container: Argb,
        on_secondary_container: Argb,
        secondary_fixed: Argb,
        secondary_fixed_dim: Argb,
        on_secondary_fixed: Argb,
        on_secondary_fixed_variant: Argb,
        tertiary: Argb,
        on_tertiary: Argb,
        tertiary_container: Argb,
        on_tertiary_container: Argb,
        tertiary_fixed: Argb,
        tertiary_fixed_dim: Argb,
        on_tertiary_fixed: Argb,
        on_tertiary_fixed_variant: Argb,
        error: Argb,
        on_error: Argb,
        error_container: Argb,
        on_error_container: Argb,
        surface_dim: Argb,
        surface: Argb,
        surface_tint: Argb,
        surface_bright: Argb,
        surface_container_lowest: Argb,
        surface_container_low: Argb,
        surface_container: Argb,
        surface_container_high: Argb,
        surface_container_highest: Argb,
        on_surface: Argb,
        on_surface_variant: Argb,
        outline: Argb,
        outline_variant: Argb,
        inverse_surface: Argb,
        inverse_on_surface: Argb,
        surface_variant: Argb,
        background: Argb,
        on_background: Argb,
        shadow: Argb,
        scrim: Argb,
    ) -> Self {
        Self {
            primary,
            on_primary,
            primary_container,
            on_primary_container,
            inverse_primary,
            primary_fixed,
            primary_fixed_dim,
            on_primary_fixed,
            on_primary_fixed_variant,
            secondary,
            on_secondary,
            secondary_container,
            on_secondary_container,
            secondary_fixed,
            secondary_fixed_dim,
            on_secondary_fixed,
            on_secondary_fixed_variant,
            tertiary,
            on_tertiary,
            tertiary_container,
            on_tertiary_container,
            tertiary_fixed,
            tertiary_fixed_dim,
            on_tertiary_fixed,
            on_tertiary_fixed_variant,
            error,
            on_error,
            error_container,
            on_error_container,
            surface_dim,
            surface,
            surface_tint,
            surface_bright,
            surface_container_lowest,
            surface_container_low,
            surface_container,
            surface_container_high,
            surface_container_highest,
            on_surface,
            on_surface_variant,
            outline,
            outline_variant,
            inverse_surface,
            inverse_on_surface,
            surface_variant,
            background,
            on_background,
            shadow,
            scrim,
        }
    }
}

impl From<DynamicScheme> for Scheme {
    fn from(scheme: DynamicScheme) -> Self {
        Self::new(
            scheme.primary(),
            scheme.on_primary(),
            scheme.primary_container(),
            scheme.on_primary_container(),
            scheme.inverse_primary(),
            scheme.primary_fixed(),
            scheme.primary_fixed_dim(),
            scheme.on_primary_fixed(),
            scheme.on_primary_fixed_variant(),
            scheme.secondary(),
            scheme.on_secondary(),
            scheme.secondary_container(),
            scheme.on_secondary_container(),
            scheme.secondary_fixed(),
            scheme.secondary_fixed_dim(),
            scheme.on_secondary_fixed(),
            scheme.on_secondary_fixed_variant(),
            scheme.tertiary(),
            scheme.on_tertiary(),
            scheme.tertiary_container(),
            scheme.on_tertiary_container(),
            scheme.tertiary_fixed(),
            scheme.tertiary_fixed_dim(),
            scheme.on_tertiary_fixed(),
            scheme.on_tertiary_fixed_variant(),
            scheme.error(),
            scheme.on_error(),
            scheme.error_container(),
            scheme.on_error_container(),
            scheme.surface_dim(),
            scheme.surface(),
            scheme.surface_tint(),
            scheme.surface_bright(),
            scheme.surface_container_lowest(),
            scheme.surface_container_low(),
            scheme.surface_container(),
            scheme.surface_container_high(),
            scheme.surface_container_highest(),
            scheme.on_surface(),
            scheme.on_surface_variant(),
            scheme.outline(),
            scheme.outline_variant(),
            scheme.inverse_surface(),
            scheme.inverse_on_surface(),
            scheme.surface_variant(),
            scheme.background(),
            scheme.on_background(),
            scheme.shadow(),
            scheme.scrim(),
        )
    }
}

impl IntoIterator for Scheme {
    type Item = (String, Argb);

    type IntoIter = IntoIter<(String, Argb), 49>;

    fn into_iter(self) -> Self::IntoIter {
        [
            ("primary".into(), self.primary),
            ("on_primary".into(), self.on_primary),
            ("primary_container".into(), self.primary_container),
            ("on_primary_container".into(), self.on_primary_container),
            ("inverse_primary".into(), self.inverse_primary),
            ("primary_fixed".into(), self.primary_fixed),
            ("primary_fixed_dim".into(), self.primary_fixed_dim),
            ("on_primary_fixed".into(), self.on_primary_fixed),
            (
                "on_primary_fixed_variant".into(),
                self.on_primary_fixed_variant,
            ),
            ("secondary".into(), self.secondary),
            ("on_secondary".into(), self.on_secondary),
            ("secondary_container".into(), self.secondary_container),
            ("on_secondary_container".into(), self.on_secondary_container),
            ("secondary_fixed".into(), self.secondary_fixed),
            ("secondary_fixed_dim".into(), self.secondary_fixed_dim),
            ("on_secondary_fixed".into(), self.on_secondary_fixed),
            (
                "on_secondary_fixed_variant".into(),
                self.on_secondary_fixed_variant,
            ),
            ("tertiary".into(), self.tertiary),
            ("on_tertiary".into(), self.on_tertiary),
            ("tertiary_container".into(), self.tertiary_container),
            ("on_tertiary_container".into(), self.on_tertiary_container),
            ("tertiary_fixed".into(), self.tertiary_fixed),
            ("tertiary_fixed_dim".into(), self.tertiary_fixed_dim),
            ("on_tertiary_fixed".into(), self.on_tertiary_fixed),
            (
                "on_tertiary_fixed_variant".into(),
                self.on_tertiary_fixed_variant,
            ),
            ("error".into(), self.error),
            ("on_error".into(), self.on_error),
            ("error_container".into(), self.error_container),
            ("on_error_container".into(), self.on_error_container),
            ("surface_dim".into(), self.surface_dim),
            ("surface".into(), self.surface),
            ("surface_tint".into(), self.surface_tint),
            ("surface_bright".into(), self.surface_bright),
            (
                "surface_container_lowest".into(),
                self.surface_container_lowest,
            ),
            ("surface_container_low".into(), self.surface_container_low),
            ("surface_container".into(), self.surface_container),
            ("surface_container_high".into(), self.surface_container_high),
            (
                "surface_container_highest".into(),
                self.surface_container_highest,
            ),
            ("on_surface".into(), self.on_surface),
            ("on_surface_variant".into(), self.on_surface_variant),
            ("outline".into(), self.outline),
            ("outline_variant".into(), self.outline_variant),
            ("inverse_surface".into(), self.inverse_surface),
            ("inverse_on_surface".into(), self.inverse_on_surface),
            ("surface_variant".into(), self.surface_variant),
            ("background".into(), self.background),
            ("on_background".into(), self.on_background),
            ("shadow".into(), self.shadow),
            ("scrim".into(), self.scrim),
        ]
        .into_iter()
    }
}

impl From<Scheme> for Map<String, String> {
    fn from(value: Scheme) -> Self {
        let map: Map<String, Argb> = Map::from_iter(value);

        map.into_iter()
            .map(|(key, value)| (key, value.to_hex_with_pound()))
            .collect()
    }
}

/// This is similar to `MaterialLightColorSchemeFromPalette` and `MaterialDarkColorSchemeFromPalette` in the C++ implementation of Material Color Utilities.
///
/// We use this to test scheme generation from a core palette.
#[derive(PartialEq, Eq, Debug)]
pub struct SchemeFromPalette {
    pub primary: Argb,
    pub on_primary: Argb,
    pub primary_container: Argb,
    pub on_primary_container: Argb,
    pub secondary: Argb,
    pub on_secondary: Argb,
    pub secondary_container: Argb,
    pub on_secondary_container: Argb,
    pub tertiary: Argb,
    pub on_tertiary: Argb,
    pub tertiary_container: Argb,
    pub on_tertiary_container: Argb,
    pub error: Argb,
    pub on_error: Argb,
    pub error_container: Argb,
    pub on_error_container: Argb,
    pub surface: Argb,
    pub on_surface: Argb,
    pub surface_variant: Argb,
    pub on_surface_variant: Argb,
    pub outline: Argb,
    pub outline_variant: Argb,
    pub background: Argb,
    pub on_background: Argb,
    pub shadow: Argb,
    pub scrim: Argb,
    pub inverse_surface: Argb,
    pub inverse_on_surface: Argb,
    pub inverse_primary: Argb,
}

impl SchemeFromPalette {
    /// Generates a light color scheme from a core palette.
    /// This has less fields than [`Scheme`]
    pub fn light_from_palette(palette: &CorePalette) -> Self {
        Self {
            primary: palette.primary.tone(40),
            on_primary: palette.primary.tone(100),
            primary_container: palette.primary.tone(90),
            on_primary_container: palette.primary.tone(10),
            secondary: palette.secondary.tone(40),
            on_secondary: palette.secondary.tone(100),
            secondary_container: palette.secondary.tone(90),
            on_secondary_container: palette.secondary.tone(10),
            tertiary: palette.tertiary.tone(40),
            on_tertiary: palette.tertiary.tone(100),
            tertiary_container: palette.tertiary.tone(90),
            on_tertiary_container: palette.tertiary.tone(10),
            error: palette.error.tone(40),
            on_error: palette.error.tone(100),
            error_container: palette.error.tone(90),
            on_error_container: palette.error.tone(10),
            background: palette.neutral.tone(99),
            on_background: palette.neutral.tone(10),
            surface: palette.neutral.tone(99),
            on_surface: palette.neutral.tone(10),
            surface_variant: palette.neutral_variant.tone(90),
            on_surface_variant: palette.neutral_variant.tone(30),
            outline: palette.neutral_variant.tone(50),
            outline_variant: palette.neutral_variant.tone(80),
            shadow: palette.neutral.tone(0),
            scrim: palette.neutral.tone(0),
            inverse_surface: palette.neutral.tone(20),
            inverse_on_surface: palette.neutral.tone(95),
            inverse_primary: palette.primary.tone(80),
        }
    }

    /// Generates a dark color scheme from a core palette.
    /// This has less fields than [`Scheme`]
    pub fn dark_from_palette(palette: &CorePalette) -> Self {
        Self {
            primary: palette.primary.tone(80),
            on_primary: palette.primary.tone(20),
            primary_container: palette.primary.tone(30),
            on_primary_container: palette.primary.tone(90),
            secondary: palette.secondary.tone(80),
            on_secondary: palette.secondary.tone(20),
            secondary_container: palette.secondary.tone(30),
            on_secondary_container: palette.secondary.tone(90),
            tertiary: palette.tertiary.tone(80),
            on_tertiary: palette.tertiary.tone(20),
            tertiary_container: palette.tertiary.tone(30),
            on_tertiary_container: palette.tertiary.tone(90),
            error: palette.error.tone(80),
            on_error: palette.error.tone(20),
            error_container: palette.error.tone(30),
            on_error_container: palette.error.tone(80),
            background: palette.neutral.tone(10),
            on_background: palette.neutral.tone(90),
            surface: palette.neutral.tone(10),
            on_surface: palette.neutral.tone(90),
            surface_variant: palette.neutral_variant.tone(30),
            on_surface_variant: palette.neutral_variant.tone(80),
            outline: palette.neutral_variant.tone(60),
            outline_variant: palette.neutral_variant.tone(30),
            shadow: palette.neutral.tone(0),
            scrim: palette.neutral.tone(0),
            inverse_surface: palette.neutral.tone(90),
            inverse_on_surface: palette.neutral.tone(20),
            inverse_primary: palette.primary.tone(40),
        }
    }

    pub fn light(argb: Argb) -> Self {
        Self::light_from_palette(&CorePalette::of(argb))
    }

    pub fn light_content(argb: Argb) -> Self {
        Self::light_from_palette(&CorePalette::content_of(argb))
    }

    pub fn dark(argb: Argb) -> Self {
        Self::dark_from_palette(&CorePalette::of(argb))
    }

    pub fn dark_content(argb: Argb) -> Self {
        Self::dark_from_palette(&CorePalette::content_of(argb))
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Argb, scheme::SchemeFromPalette};
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_surface_tones() {
        let c = Argb::from_u32(0xffff0000);

        let light = SchemeFromPalette::light(c);
        let dark = SchemeFromPalette::dark(c);

        assert_approx_eq!(f64, light.surface.as_lstar(), 99.0, epsilon = 0.1); // 99.015;
        assert_approx_eq!(f64, dark.surface.as_lstar(), 10.0, epsilon = 0.1); // 9.923
    }

    #[test]
    fn test_blue_scheme() {
        let c = Argb::from_u32(0xff0000ff);

        let light = SchemeFromPalette::light(c);
        let dark = SchemeFromPalette::dark(c);

        assert_eq!(light.primary.to_hex(), "343dff");
        assert_eq!(dark.primary.to_hex(), "bec2ff");
    }

    #[test]
    fn test_light_scheme_from_high_chroma_color() {
        let c = Argb::from_u32(0xfffa2bec);

        let scheme = SchemeFromPalette::light(c);

        let expected = SchemeFromPalette {
            primary: Argb::from_u32(0xffab00a2),
            on_primary: Argb::from_u32(0xffffffff),
            primary_container: Argb::from_u32(0xffffd7f3),
            on_primary_container: Argb::from_u32(0xff390035),
            secondary: Argb::from_u32(0xff6e5868),
            on_secondary: Argb::from_u32(0xffffffff),
            secondary_container: Argb::from_u32(0xfff8daee),
            on_secondary_container: Argb::from_u32(0xff271624),
            tertiary: Argb::from_u32(0xff815343),
            on_tertiary: Argb::from_u32(0xffffffff),
            tertiary_container: Argb::from_u32(0xffffdbd0),
            on_tertiary_container: Argb::from_u32(0xff321207),
            error: Argb::from_u32(0xffba1a1a),
            on_error: Argb::from_u32(0xffffffff),
            error_container: Argb::from_u32(0xffffdad6),
            on_error_container: Argb::from_u32(0xff410002),
            background: Argb::from_u32(0xfffffbff),
            on_background: Argb::from_u32(0xff1f1a1d),
            surface: Argb::from_u32(0xfffffbff),
            on_surface: Argb::from_u32(0xff1f1a1d),
            surface_variant: Argb::from_u32(0xffeedee7),
            on_surface_variant: Argb::from_u32(0xff4e444b),
            outline: Argb::from_u32(0xff80747b),
            outline_variant: Argb::from_u32(0xffd2c2cb),
            shadow: Argb::from_u32(0xff000000),
            scrim: Argb::from_u32(0xff000000),
            inverse_surface: Argb::from_u32(0xff342f32),
            inverse_on_surface: Argb::from_u32(0xfff8eef2),
            inverse_primary: Argb::from_u32(0xffffabee),
        };

        assert_eq!(scheme, expected);
    }

    #[test]
    fn test_dark_scheme_from_high_chroma_color() {
        let c = Argb::from_u32(0xfffa2bec);

        let scheme = SchemeFromPalette::dark(c);

        let expected = SchemeFromPalette {
            primary: Argb::from_u32(0xffffabee),
            on_primary: Argb::from_u32(0xff5c0057),
            primary_container: Argb::from_u32(0xff83007b),
            on_primary_container: Argb::from_u32(0xffffd7f3),
            secondary: Argb::from_u32(0xffdbbed1),
            on_secondary: Argb::from_u32(0xff3e2a39),
            secondary_container: Argb::from_u32(0xff554050),
            on_secondary_container: Argb::from_u32(0xfff8daee),
            tertiary: Argb::from_u32(0xfff5b9a5),
            on_tertiary: Argb::from_u32(0xff4c2619),
            tertiary_container: Argb::from_u32(0xff663c2d),
            on_tertiary_container: Argb::from_u32(0xffffdbd0),
            error: Argb::from_u32(0xffffb4ab),
            on_error: Argb::from_u32(0xff690005),
            error_container: Argb::from_u32(0xff93000a),
            on_error_container: Argb::from_u32(0xffffb4ab),
            background: Argb::from_u32(0xff1f1a1d),
            on_background: Argb::from_u32(0xffeae0e4),
            surface: Argb::from_u32(0xff1f1a1d),
            on_surface: Argb::from_u32(0xffeae0e4),
            surface_variant: Argb::from_u32(0xff4e444b),
            on_surface_variant: Argb::from_u32(0xffd2c2cb),
            outline: Argb::from_u32(0xff9a8d95),
            outline_variant: Argb::from_u32(0xff4e444b),
            shadow: Argb::from_u32(0xff000000),
            scrim: Argb::from_u32(0xff000000),
            inverse_surface: Argb::from_u32(0xffeae0e4),
            inverse_on_surface: Argb::from_u32(0xff342f32),
            inverse_primary: Argb::from_u32(0xffab00a2),
        };

        assert_eq!(scheme, expected);
    }

    #[test]
    fn test_light_content_scheme_from_high_chroma_color() {
        let c = Argb::from_u32(0xfffa2bec);

        let scheme = SchemeFromPalette::light_content(c);

        let expected = SchemeFromPalette {
            primary: Argb::from_u32(0xffab00a2),
            on_primary: Argb::from_u32(0xffffffff),
            primary_container: Argb::from_u32(0xffffd7f3),
            on_primary_container: Argb::from_u32(0xff390035),
            secondary: Argb::from_u32(0xff7f4e75),
            on_secondary: Argb::from_u32(0xffffffff),
            secondary_container: Argb::from_u32(0xffffd7f3),
            on_secondary_container: Argb::from_u32(0xff330b2f),
            tertiary: Argb::from_u32(0xff9c4323),
            on_tertiary: Argb::from_u32(0xffffffff),
            tertiary_container: Argb::from_u32(0xffffdbd0),
            on_tertiary_container: Argb::from_u32(0xff390c00),
            error: Argb::from_u32(0xffba1a1a),
            on_error: Argb::from_u32(0xffffffff),
            error_container: Argb::from_u32(0xffffdad6),
            on_error_container: Argb::from_u32(0xff410002),
            background: Argb::from_u32(0xfffffbff),
            on_background: Argb::from_u32(0xff1f1a1d),
            surface: Argb::from_u32(0xfffffbff),
            on_surface: Argb::from_u32(0xff1f1a1d),
            surface_variant: Argb::from_u32(0xffeedee7),
            on_surface_variant: Argb::from_u32(0xff4e444b),
            outline: Argb::from_u32(0xff80747b),
            outline_variant: Argb::from_u32(0xffd2c2cb),
            shadow: Argb::from_u32(0xff000000),
            scrim: Argb::from_u32(0xff000000),
            inverse_surface: Argb::from_u32(0xff342f32),
            inverse_on_surface: Argb::from_u32(0xfff8eef2),
            inverse_primary: Argb::from_u32(0xffffabee),
        };

        assert_eq!(scheme, expected);
    }

    #[test]
    fn test_dark_content_scheme_from_high_chroma_color() {
        let c = Argb::from_u32(0xfffa2bec);

        let scheme = SchemeFromPalette::dark_content(c);

        let expected = SchemeFromPalette {
            primary: Argb::from_u32(0xffffabee),
            on_primary: Argb::from_u32(0xff5c0057),
            primary_container: Argb::from_u32(0xff83007b),
            on_primary_container: Argb::from_u32(0xffffd7f3),
            secondary: Argb::from_u32(0xfff0b4e1),
            on_secondary: Argb::from_u32(0xff4b2145),
            secondary_container: Argb::from_u32(0xff64375c),
            on_secondary_container: Argb::from_u32(0xffffd7f3),
            tertiary: Argb::from_u32(0xffffb59c),
            on_tertiary: Argb::from_u32(0xff5c1900),
            tertiary_container: Argb::from_u32(0xff7d2c0d),
            on_tertiary_container: Argb::from_u32(0xffffdbd0),
            error: Argb::from_u32(0xffffb4ab),
            on_error: Argb::from_u32(0xff690005),
            error_container: Argb::from_u32(0xff93000a),
            on_error_container: Argb::from_u32(0xffffb4ab),
            background: Argb::from_u32(0xff1f1a1d),
            on_background: Argb::from_u32(0xffeae0e4),
            surface: Argb::from_u32(0xff1f1a1d),
            on_surface: Argb::from_u32(0xffeae0e4),
            surface_variant: Argb::from_u32(0xff4e444b),
            on_surface_variant: Argb::from_u32(0xffd2c2cb),
            outline: Argb::from_u32(0xff9a8d95),
            outline_variant: Argb::from_u32(0xff4e444b),
            shadow: Argb::from_u32(0xff000000),
            scrim: Argb::from_u32(0xff000000),
            inverse_surface: Argb::from_u32(0xffeae0e4),
            inverse_on_surface: Argb::from_u32(0xff342f32),
            inverse_primary: Argb::from_u32(0xffab00a2),
        };

        assert_eq!(scheme, expected);
    }
}
