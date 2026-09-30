pub mod variant;

use float_cmp::assert_approx_eq;
#[allow(deprecated)]
use material_colors::{color::Rgb, palette::CorePalette};

/// This is similar to `MaterialLightColorSchemeFromPalette` and
/// `MaterialDarkColorSchemeFromPalette` in the C++ implementation of
/// Material Color Utilities.
///
/// We use this to test scheme generation from a core palette.
#[derive(PartialEq, Eq, Debug)]
struct SchemeFromPalette {
    primary: Rgb,
    on_primary: Rgb,
    primary_container: Rgb,
    on_primary_container: Rgb,
    secondary: Rgb,
    on_secondary: Rgb,
    secondary_container: Rgb,
    on_secondary_container: Rgb,
    tertiary: Rgb,
    on_tertiary: Rgb,
    tertiary_container: Rgb,
    on_tertiary_container: Rgb,
    error: Rgb,
    on_error: Rgb,
    error_container: Rgb,
    on_error_container: Rgb,
    surface: Rgb,
    on_surface: Rgb,
    surface_variant: Rgb,
    on_surface_variant: Rgb,
    outline: Rgb,
    outline_variant: Rgb,
    background: Rgb,
    on_background: Rgb,
    shadow: Rgb,
    scrim: Rgb,
    inverse_surface: Rgb,
    inverse_on_surface: Rgb,
    inverse_primary: Rgb,
}

#[allow(deprecated)]
impl SchemeFromPalette {
    /// Generates a light color scheme from a core palette.
    /// This has less fields than [`Scheme`]
    fn light_from_palette(palette: &CorePalette) -> Self {
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
    fn dark_from_palette(palette: &CorePalette) -> Self {
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

    fn light(rgb: Rgb) -> Self {
        Self::light_from_palette(&CorePalette::of(rgb))
    }

    fn light_content(rgb: Rgb) -> Self {
        Self::light_from_palette(&CorePalette::content_of(rgb))
    }

    fn dark(rgb: Rgb) -> Self {
        Self::dark_from_palette(&CorePalette::of(rgb))
    }

    fn dark_content(rgb: Rgb) -> Self {
        Self::dark_from_palette(&CorePalette::content_of(rgb))
    }
}

#[test]
fn test_surface_tones() {
    let c = Rgb::from_u32(0xFF0000);

    let light = SchemeFromPalette::light(c);
    let dark = SchemeFromPalette::dark(c);

    assert_approx_eq!(f64, light.surface.as_lstar(), 99.0, epsilon = 0.1); // 99.015;
    assert_approx_eq!(f64, dark.surface.as_lstar(), 10.0, epsilon = 0.1); // 9.923
}

#[test]
fn test_blue_scheme() {
    let c = Rgb::from_u32(0x0000FF);

    let light = SchemeFromPalette::light(c);
    let dark = SchemeFromPalette::dark(c);

    assert_eq!(light.primary.as_u32(), 0x343DFF);
    assert_eq!(dark.primary.as_u32(), 0xBEC2FF);
}

#[test]
fn test_light_scheme_from_high_chroma_color() {
    let c = Rgb::from_u32(0xFA2BEC);

    let scheme = SchemeFromPalette::light(c);

    let expected = SchemeFromPalette {
        primary: Rgb::from_u32(0xAB00A2),
        on_primary: Rgb::from_u32(0xFFFFFF),
        primary_container: Rgb::from_u32(0xFFD7F3),
        on_primary_container: Rgb::from_u32(0x390035),
        secondary: Rgb::from_u32(0x6E5868),
        on_secondary: Rgb::from_u32(0xFFFFFF),
        secondary_container: Rgb::from_u32(0xF8DAEE),
        on_secondary_container: Rgb::from_u32(0x271624),
        tertiary: Rgb::from_u32(0x815343),
        on_tertiary: Rgb::from_u32(0xFFFFFF),
        tertiary_container: Rgb::from_u32(0xFFDBD0),
        on_tertiary_container: Rgb::from_u32(0x321207),
        error: Rgb::from_u32(0xBA1A1A),
        on_error: Rgb::from_u32(0xFFFFFF),
        error_container: Rgb::from_u32(0xFFDAD6),
        on_error_container: Rgb::from_u32(0x410002),
        background: Rgb::from_u32(0xFFFBFF),
        on_background: Rgb::from_u32(0x1F1A1D),
        surface: Rgb::from_u32(0xFFFBFF),
        on_surface: Rgb::from_u32(0x1F1A1D),
        surface_variant: Rgb::from_u32(0xEEDEE7),
        on_surface_variant: Rgb::from_u32(0x4E444B),
        outline: Rgb::from_u32(0x80747B),
        outline_variant: Rgb::from_u32(0xD2C2CB),
        shadow: Rgb::from_u32(0x000000),
        scrim: Rgb::from_u32(0x000000),
        inverse_surface: Rgb::from_u32(0x342F32),
        inverse_on_surface: Rgb::from_u32(0xF8EEF2),
        inverse_primary: Rgb::from_u32(0xFFABEE),
    };

    assert_eq!(scheme, expected);
}

#[test]
fn test_dark_scheme_from_high_chroma_color() {
    let c = Rgb::from_u32(0xFA2BEC);

    let scheme = SchemeFromPalette::dark(c);

    let expected = SchemeFromPalette {
        primary: Rgb::from_u32(0xFFABEE),
        on_primary: Rgb::from_u32(0x5C0057),
        primary_container: Rgb::from_u32(0x83007B),
        on_primary_container: Rgb::from_u32(0xFFD7F3),
        secondary: Rgb::from_u32(0xDBBED1),
        on_secondary: Rgb::from_u32(0x3E2A39),
        secondary_container: Rgb::from_u32(0x554050),
        on_secondary_container: Rgb::from_u32(0xF8DAEE),
        tertiary: Rgb::from_u32(0xF5B9A5),
        on_tertiary: Rgb::from_u32(0x4C2619),
        tertiary_container: Rgb::from_u32(0x663C2D),
        on_tertiary_container: Rgb::from_u32(0xFFDBD0),
        error: Rgb::from_u32(0xFFB4AB),
        on_error: Rgb::from_u32(0x690005),
        error_container: Rgb::from_u32(0x93000A),
        on_error_container: Rgb::from_u32(0xFFB4AB),
        background: Rgb::from_u32(0x1F1A1D),
        on_background: Rgb::from_u32(0xEAE0E4),
        surface: Rgb::from_u32(0x1F1A1D),
        on_surface: Rgb::from_u32(0xEAE0E4),
        surface_variant: Rgb::from_u32(0x4E444B),
        on_surface_variant: Rgb::from_u32(0xD2C2CB),
        outline: Rgb::from_u32(0x9A8D95),
        outline_variant: Rgb::from_u32(0x4E444B),
        shadow: Rgb::from_u32(0x000000),
        scrim: Rgb::from_u32(0x000000),
        inverse_surface: Rgb::from_u32(0xEAE0E4),
        inverse_on_surface: Rgb::from_u32(0x342F32),
        inverse_primary: Rgb::from_u32(0xAB00A2),
    };

    assert_eq!(scheme, expected);
}

#[test]
fn test_light_content_scheme_from_high_chroma_color() {
    let c = Rgb::from_u32(0xFA2BEC);

    let scheme = SchemeFromPalette::light_content(c);

    let expected = SchemeFromPalette {
        primary: Rgb::from_u32(0xAB00A2),
        on_primary: Rgb::from_u32(0xFFFFFF),
        primary_container: Rgb::from_u32(0xFFD7F3),
        on_primary_container: Rgb::from_u32(0x390035),
        secondary: Rgb::from_u32(0x7F4E75),
        on_secondary: Rgb::from_u32(0xFFFFFF),
        secondary_container: Rgb::from_u32(0xFFD7F3),
        on_secondary_container: Rgb::from_u32(0x330B2F),
        tertiary: Rgb::from_u32(0x9C4323),
        on_tertiary: Rgb::from_u32(0xFFFFFF),
        tertiary_container: Rgb::from_u32(0xFFDBD0),
        on_tertiary_container: Rgb::from_u32(0x390C00),
        error: Rgb::from_u32(0xBA1A1A),
        on_error: Rgb::from_u32(0xFFFFFF),
        error_container: Rgb::from_u32(0xFFDAD6),
        on_error_container: Rgb::from_u32(0x410002),
        background: Rgb::from_u32(0xFFFBFF),
        on_background: Rgb::from_u32(0x1F1A1D),
        surface: Rgb::from_u32(0xFFFBFF),
        on_surface: Rgb::from_u32(0x1F1A1D),
        surface_variant: Rgb::from_u32(0xEEDEE7),
        on_surface_variant: Rgb::from_u32(0x4E444B),
        outline: Rgb::from_u32(0x80747B),
        outline_variant: Rgb::from_u32(0xD2C2CB),
        shadow: Rgb::from_u32(0x000000),
        scrim: Rgb::from_u32(0x000000),
        inverse_surface: Rgb::from_u32(0x342F32),
        inverse_on_surface: Rgb::from_u32(0xF8EEF2),
        inverse_primary: Rgb::from_u32(0xFFABEE),
    };

    assert_eq!(scheme, expected);
}

#[test]
fn test_dark_content_scheme_from_high_chroma_color() {
    let c = Rgb::from_u32(0xFA2BEC);

    let scheme = SchemeFromPalette::dark_content(c);

    let expected = SchemeFromPalette {
        primary: Rgb::from_u32(0xFFABEE),
        on_primary: Rgb::from_u32(0x5C0057),
        primary_container: Rgb::from_u32(0x83007B),
        on_primary_container: Rgb::from_u32(0xFFD7F3),
        secondary: Rgb::from_u32(0xF0B4E1),
        on_secondary: Rgb::from_u32(0x4B2145),
        secondary_container: Rgb::from_u32(0x64375C),
        on_secondary_container: Rgb::from_u32(0xFFD7F3),
        tertiary: Rgb::from_u32(0xFFB59C),
        on_tertiary: Rgb::from_u32(0x5C1900),
        tertiary_container: Rgb::from_u32(0x7D2C0D),
        on_tertiary_container: Rgb::from_u32(0xFFDBD0),
        error: Rgb::from_u32(0xFFB4AB),
        on_error: Rgb::from_u32(0x690005),
        error_container: Rgb::from_u32(0x93000A),
        on_error_container: Rgb::from_u32(0xFFB4AB),
        background: Rgb::from_u32(0x1F1A1D),
        on_background: Rgb::from_u32(0xEAE0E4),
        surface: Rgb::from_u32(0x1F1A1D),
        on_surface: Rgb::from_u32(0xEAE0E4),
        surface_variant: Rgb::from_u32(0x4E444B),
        on_surface_variant: Rgb::from_u32(0xD2C2CB),
        outline: Rgb::from_u32(0x9A8D95),
        outline_variant: Rgb::from_u32(0x4E444B),
        shadow: Rgb::from_u32(0x000000),
        scrim: Rgb::from_u32(0x000000),
        inverse_surface: Rgb::from_u32(0xEAE0E4),
        inverse_on_surface: Rgb::from_u32(0x342F32),
        inverse_primary: Rgb::from_u32(0xAB00A2),
    };

    assert_eq!(scheme, expected);
}
