use float_cmp::assert_approx_eq;
use material_colors::{
    color::Rgb,
    dynamic_color::{Platform, SpecVersion, Variant},
    hct::Hct,
    palette::Palette,
    scheme::variant::SchemeCmf,
};

/// Error hue chosen to stand apart from the primary and tertiary hues.
fn error_hue(primary_hue: f64, tertiary_hue: f64) -> f64 {
    let t = tertiary_hue;

    if primary_hue <= 8.0 {
        if t <= 24.0 {
            28.0
        } else if t <= 32.0 {
            16.0
        } else {
            20.0
        }
    } else if primary_hue <= 16.0 {
        if t <= 24.0 {
            32.0
        } else if t <= 32.0 {
            20.0
        } else {
            24.0
        }
    } else if primary_hue <= 20.0 {
        if t <= 28.0 {
            32.0
        } else if t <= 32.0 {
            24.0
        } else {
            28.0
        }
    } else if primary_hue <= 28.0 {
        if t <= 24.0 { 32.0 } else { 16.0 }
    } else if primary_hue <= 32.0 {
        if t <= 20.0 {
            24.0
        } else if t <= 28.0 {
            16.0
        } else {
            20.0
        }
    } else if primary_hue <= 40.0 {
        if t > 20.0 && t <= 28.0 { 16.0 } else { 24.0 }
    } else if primary_hue <= 152.0 {
        if t > 24.0 && t <= 36.0 { 20.0 } else { 32.0 }
    } else if primary_hue <= 272.0 {
        if t > 20.0 && t <= 28.0 { 16.0 } else { 24.0 }
    } else if t > 12.0 && t <= 28.0 {
        32.0
    } else {
        16.0
    }
}

#[test]
fn test_error_hue_table() {
    // One case per primary-hue band.
    assert_approx_eq!(f64, error_hue(5.0, 10.0), 28.0);
    assert_approx_eq!(f64, error_hue(5.0, 30.0), 16.0);
    assert_approx_eq!(f64, error_hue(5.0, 40.0), 20.0);
    assert_approx_eq!(f64, error_hue(12.0, 30.0), 20.0);
    assert_approx_eq!(f64, error_hue(18.0, 30.0), 24.0);
    assert_approx_eq!(f64, error_hue(25.0, 30.0), 16.0);
    assert_approx_eq!(f64, error_hue(30.0, 25.0), 16.0);
    assert_approx_eq!(f64, error_hue(35.0, 25.0), 16.0);
    assert_approx_eq!(f64, error_hue(100.0, 30.0), 20.0);
    assert_approx_eq!(f64, error_hue(100.0, 200.0), 32.0);
    assert_approx_eq!(f64, error_hue(200.0, 25.0), 16.0);
    assert_approx_eq!(f64, error_hue(300.0, 20.0), 32.0);
    assert_approx_eq!(f64, error_hue(300.0, 200.0), 16.0);
}

#[test]
fn test_single_source() {
    let source: Hct = Rgb::from_u32(0x4285F4).into();
    let scheme = SchemeCmf::new(source, false, None).scheme;

    assert_eq!(scheme.variant, Variant::Cmf);
    assert_eq!(scheme.spec_version, SpecVersion::Spec2026);
    assert_approx_eq!(f64, scheme.tertiary_palette.hue(), source.get_hue());
    assert_approx_eq!(f64, scheme.tertiary_palette.chroma(), source.get_chroma() * 0.75);
    assert_approx_eq!(f64, scheme.secondary_palette.chroma(), source.get_chroma() * 0.5);
    assert_approx_eq!(f64, scheme.neutral_palette.chroma(), source.get_chroma() * 0.2);
    assert!(scheme.error_palette.chroma() >= 50.0);
}

#[test]
fn test_two_sources() {
    let source: Hct = Rgb::from_u32(0x4285F4).into();
    let secondary: Hct = Rgb::from_u32(0xEA4335).into();
    let scheme = SchemeCmf::with_sources(source, Some(secondary), true, Some(0.5), Platform::Phone).scheme;

    assert_eq!(scheme.secondary_source_color_hct, Some(secondary));
    assert_approx_eq!(f64, scheme.tertiary_palette.hue(), secondary.get_hue());
    assert_approx_eq!(f64, scheme.tertiary_palette.chroma(), secondary.get_chroma());

    // `palette` (single source) must agree with the scheme when there is
    // no second source.
    let single = SchemeCmf::new(source, true, Some(0.5)).scheme;

    assert_eq!(SchemeCmf::palette(&source, &Palette::Error), single.error_palette);
}

/// The 2026 `tertiary` tone starts from the second source's tone, before
/// contrast adjustment.
#[test]
fn test_tertiary_uses_second_source_tone() {
    use material_colors::dynamic_color::Role;

    let source: Hct = Rgb::from_u32(0x4285F4).into();
    let secondary: Hct = Rgb::from_u32(0xEA4335).into();
    let scheme = SchemeCmf::with_sources(source, Some(secondary), false, Some(0.0), Platform::Phone).scheme;
    let resolver = scheme.resolver();

    assert_approx_eq!(f64, resolver.context().raw_tone(Role::Tertiary.color()), secondary.get_tone());
}

#[test]
fn test_from_spec_delegates_to_cmf() {
    use material_colors::dynamic_color::DynamicScheme;

    let source: Hct = Rgb::from_u32(0x4285F4).into();
    let a = DynamicScheme::from_spec(source, Variant::Cmf, true, Some(1.0), Platform::Watch, SpecVersion::Spec2021);
    let b = SchemeCmf::with_sources(source, None, true, Some(1.0), Platform::Watch).scheme;

    assert!(a == b);
}
