pub mod dynamic_scheme;

use float_cmp::assert_approx_eq;
use material_colors::{
    color::Rgb,
    contrast::ratio_of_tones,
    dynamic_color::{DynamicScheme, MaterialDynamicColors, Platform, Role, SpecVersion, Variant},
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
    let colors = [
        ("background", MaterialDynamicColors::background()),
        ("onBackground", MaterialDynamicColors::on_background()),
        ("surfaceDim", MaterialDynamicColors::surface_dim()),
        ("surfaceBright", MaterialDynamicColors::surface_bright()),
        ("onSurface", MaterialDynamicColors::on_surface()),
        ("surfaceVariant", MaterialDynamicColors::surface_variant()),
        ("onSurfaceVariant", MaterialDynamicColors::on_surface_variant()),
        ("primary", MaterialDynamicColors::primary()),
        ("onPrimary", MaterialDynamicColors::on_primary()),
        ("primaryContainer", MaterialDynamicColors::primary_container()),
        ("onPrimaryContainer", MaterialDynamicColors::on_primary_container()),
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
    ];

    for color in seed_colors {
        for contrast_level in contrast_levels {
            for is_dark in [false, true] {
                for scheme in [
                    SchemeContent::new(color, is_dark, Some(contrast_level)).scheme,
                    SchemeMonochrome::new(color, is_dark, Some(contrast_level)).scheme,
                    SchemeTonalSpot::new(color, is_dark, Some(contrast_level)).scheme,
                    SchemeFidelity::new(color, is_dark, Some(contrast_level)).scheme,
                ] {
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
                        let foreground_tone = colors.iter().find(|(color, _)| *color == fg_name).unwrap().1.get_hct(&scheme).get_tone();
                        let background_tone = colors.iter().find(|(color, _)| *color == bg_name).unwrap().1.get_hct(&scheme).get_tone();
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

fn all_schemes() -> impl Iterator<Item = DynamicScheme> {
    let seeds = [0xFF0000, 0x00FF00, 0x7B6F63];
    let variants = [
        Variant::Monochrome,
        Variant::Neutral,
        Variant::TonalSpot,
        Variant::Vibrant,
        Variant::Expressive,
        Variant::Fidelity,
        Variant::Content,
        Variant::Rainbow,
        Variant::FruitSalad,
        Variant::Cmf,
    ];

    seeds.into_iter().flat_map(move |seed| {
        let hct: Hct = Rgb::from_u32(seed).into();

        variants.into_iter().flat_map(move |variant| {
            [SpecVersion::Spec2021, SpecVersion::Spec2025, SpecVersion::Spec2026]
                .into_iter()
                // Skip combinations that fall back to an already-covered spec.
                .filter(move |&spec| DynamicScheme::fallback_spec_version(spec, variant) == spec && (variant != Variant::Cmf || spec == SpecVersion::Spec2026))
                .flat_map(move |spec| {
                    [Platform::Phone, Platform::Watch].into_iter().flat_map(move |platform| {
                        [false, true].into_iter().flat_map(move |is_dark| {
                            [-1.0, 0.0, 1.0]
                                .into_iter()
                                .map(move |contrast| DynamicScheme::from_spec(hct, variant, is_dark, Some(contrast), platform, spec))
                        })
                    })
                })
        })
    })
}

/// Memoization must not change any result, and every role must resolve
/// (no panics, tone in range) for every spec, variant and platform.
#[test]
fn test_resolver_matches_uncached_for_every_role() {
    for scheme in all_schemes() {
        let resolver = scheme.resolver();

        for role in Role::ALL {
            let cached = resolver.tone(role);
            let uncached = role.color().get_tone(&scheme);

            assert!(cached.to_bits() == uncached.to_bits(), "{} differs: {cached} vs {uncached}", role.name());
            assert!((0.0..=100.0).contains(&cached), "{} out of range: {cached}", role.name());
            assert_eq!(resolver.rgb(role), role.color().get_rgb(&scheme));
        }
    }
}

/// Alias entries resolve to the aliased role.
#[test]
fn test_aliases() {
    for scheme in all_schemes() {
        let tone = |role: Role| role.color().get_tone(&scheme);

        if scheme.spec_version >= SpecVersion::Spec2025 {
            assert_eq!(tone(Role::Background).to_bits(), tone(Role::Surface).to_bits());
            assert_eq!(tone(Role::SurfaceVariant).to_bits(), tone(Role::SurfaceContainerHighest).to_bits());
            assert_eq!(tone(Role::SurfaceTint).to_bits(), tone(Role::Primary).to_bits());
        }

        if scheme.spec_version == SpecVersion::Spec2026 {
            assert_eq!(tone(Role::PrimaryDim).to_bits(), tone(Role::Primary).to_bits());
            assert_eq!(tone(Role::SecondaryDim).to_bits(), tone(Role::Secondary).to_bits());
            assert_eq!(tone(Role::TertiaryDim).to_bits(), tone(Role::Tertiary).to_bits());
            assert_eq!(tone(Role::ErrorDim).to_bits(), tone(Role::Error).to_bits());
        }
    }
}

/// 2025 error hue is an absolute lookup (`get_piecewise_value`), not a
/// rotation of the source hue.
#[test]
fn test_2025_error_palette_hue() {
    for (source_hue, expected) in [(100.0, 32.0), (200.0, 22.0), (340.0, 12.0)] {
        let source = Hct::from(source_hue, 40.0, 50.0);
        let scheme = DynamicScheme::from_spec(source, Variant::TonalSpot, false, None, Platform::Phone, SpecVersion::Spec2025);

        assert_approx_eq!(f64, scheme.error_palette.hue(), expected, epsilon = 1e-9);
    }
}

#[test]
fn test_spec_fallback() {
    let source: Hct = Rgb::from_u32(0x4285F4).into();
    let content = DynamicScheme::from_spec(source, Variant::Content, false, None, Platform::Phone, SpecVersion::Spec2025);
    let tonal = DynamicScheme::from_spec(source, Variant::TonalSpot, false, None, Platform::Phone, SpecVersion::Spec2026);
    let cmf = DynamicScheme::from_spec(source, Variant::Cmf, false, None, Platform::Phone, SpecVersion::Spec2021);

    assert_eq!(content.spec_version, SpecVersion::Spec2021);
    assert_eq!(tonal.spec_version, SpecVersion::Spec2025);
    assert_eq!(cmf.spec_version, SpecVersion::Spec2026);
}
