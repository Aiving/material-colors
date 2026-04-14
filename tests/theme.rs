use material_colors::{
    Error,
    color::Rgb,
    dynamic_color::{DynamicScheme, Variant, color_spec::SpecVersion, color_spec_2025::ColorSpec2025, dynamic_scheme::Platform},
    hct::Hct,
    scheme::Scheme,
    theme::ThemeBuilder,
};

#[test]
fn test_theme() -> Result<(), Error> {
    let source = Hct::new(Rgb::from_u32(0xFF0000));
    let scheme = DynamicScheme::new(
        source,
        Variant::TonalSpot,
        true,
        None,
        ColorSpec2025::get_primary_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0),
        ColorSpec2025::get_secondary_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0),
        ColorSpec2025::get_tertiary_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0),
        ColorSpec2025::get_neutral_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0),
        ColorSpec2025::get_neutral_variant_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0),
        Some(ColorSpec2025::get_error_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0)),
    )
    .with_spec_version(SpecVersion::Spec2025);

    println!("primary = {}", scheme.primary());
    println!("on_primary = {}", scheme.on_primary());
    println!("primary_container = {}", scheme.primary_container());
    println!("on_primary_container = {}", scheme.on_primary_container());
    println!("inverse_primary = {}", scheme.inverse_primary());
    println!("secondary = {}", scheme.secondary());
    println!("on_secondary = {}", scheme.on_secondary());
    println!("secondary_container = {}", scheme.secondary_container());
    println!("on_secondary_container = {}", scheme.on_secondary_container());
    println!("tertiary = {}", scheme.tertiary());
    println!("on_tertiary = {}", scheme.on_tertiary());
    println!("tertiary_container = {}", scheme.tertiary_container());
    println!("on_tertiary_container = {}", scheme.on_tertiary_container());
    println!("error = {}", scheme.error());
    println!("on_error = {}", scheme.on_error());
    println!("error_container = {}", scheme.error_container());
    println!("on_error_container = {}", scheme.on_error_container());
    println!("primary_fixed = {}", scheme.primary_fixed());
    println!("on_primary_fixed = {}", scheme.on_primary_fixed());
    println!("primary_fixed_dim = {}", scheme.primary_fixed_dim());
    println!("on_primary_fixed_variant = {}", scheme.on_primary_fixed_variant());
    println!("secondary_fixed = {}", scheme.secondary_fixed());
    println!("on_secondary_fixed = {}", scheme.on_secondary_fixed());
    println!("secondary_fixed_dim = {}", scheme.secondary_fixed_dim());
    println!("on_secondary_fixed_variant = {}", scheme.on_secondary_fixed_variant());
    println!("tertiary_fixed = {}", scheme.tertiary_fixed());
    println!("on_tertiary_fixed = {}", scheme.on_tertiary_fixed());
    println!("tertiary_fixed_dim = {}", scheme.tertiary_fixed_dim());
    println!("on_tertiary_fixed_variant = {}", scheme.on_tertiary_fixed_variant());
    println!("surface = {}", scheme.surface());
    println!("on_surface = {}", scheme.on_surface());
    println!("surface_variant = {}", scheme.surface_variant());
    println!("on_surface_variant = {}", scheme.on_surface_variant());
    println!("inverse_surface = {}", scheme.inverse_surface());
    println!("inverse_on_surface = {}", scheme.inverse_on_surface());
    println!("outline = {}", scheme.outline());
    println!("outline_variant = {}", scheme.outline_variant());
    println!("surface_dim = {}", scheme.surface_dim());
    println!("surface_tint = {}", scheme.surface_tint());
    println!("surface_bright = {}", scheme.surface_bright());
    println!("surface_container_lowest = {}", scheme.surface_container_lowest());
    println!("surface_container_low = {}", scheme.surface_container_low());
    println!("surface_container = {}", scheme.surface_container());
    println!("surface_container_high = {}", scheme.surface_container_high());
    println!("surface_container_highest = {}", scheme.surface_container_highest());
    println!("background = {}", scheme.background());
    println!("on_background = {}", scheme.on_background());
    println!("shadow = {}", scheme.shadow());
    println!("scrim = {}", scheme.scrim());

    let theme = ThemeBuilder::with_source(Rgb::from_u32(0xFF0000)).build();

    assert_eq!(theme.schemes.dark, Scheme {
        primary: Rgb::new(255, 180, 168),
        on_primary: Rgb::new(86, 30, 22),
        primary_container: Rgb::new(115, 52, 42),
        on_primary_container: Rgb::new(255, 218, 212),
        inverse_primary: Rgb::new(144, 75, 64),
        secondary: Rgb::new(231, 189, 182),
        on_secondary: Rgb::new(68, 41, 37),
        secondary_container: Rgb::new(93, 63, 59),
        on_secondary_container: Rgb::new(255, 218, 212),
        tertiary: Rgb::new(222, 196, 140),
        on_tertiary: Rgb::new(62, 46, 4),
        tertiary_container: Rgb::new(86, 68, 25),
        on_tertiary_container: Rgb::new(251, 223, 166),
        error: Rgb::new(255, 180, 171),
        on_error: Rgb::new(105, 0, 5),
        error_container: Rgb::new(147, 0, 10),
        on_error_container: Rgb::new(255, 218, 214),
        primary_fixed: Rgb::new(255, 218, 212),
        on_primary_fixed: Rgb::new(58, 9, 5),
        primary_fixed_dim: Rgb::new(255, 180, 168),
        on_primary_fixed_variant: Rgb::new(115, 52, 42),
        secondary_fixed: Rgb::new(255, 218, 212),
        on_secondary_fixed: Rgb::new(44, 21, 18),
        secondary_fixed_dim: Rgb::new(231, 189, 182),
        on_secondary_fixed_variant: Rgb::new(93, 63, 59),
        tertiary_fixed: Rgb::new(251, 223, 166),
        on_tertiary_fixed: Rgb::new(37, 26, 0),
        tertiary_fixed_dim: Rgb::new(222, 196, 140),
        on_tertiary_fixed_variant: Rgb::new(86, 68, 25),
        surface: Rgb::new(26, 17, 16),
        on_surface: Rgb::new(241, 223, 220),
        surface_variant: Rgb::new(83, 67, 65),
        on_surface_variant: Rgb::new(216, 194, 190),
        inverse_surface: Rgb::new(241, 223, 220),
        inverse_on_surface: Rgb::new(57, 46, 44),
        outline: Rgb::new(160, 140, 137),
        outline_variant: Rgb::new(83, 67, 65),
        surface_dim: Rgb::new(26, 17, 16),
        surface_tint: Rgb::new(255, 180, 168),
        surface_bright: Rgb::new(66, 55, 53),
        surface_container_lowest: Rgb::new(20, 12, 11),
        surface_container_low: Rgb::new(35, 25, 24),
        surface_container: Rgb::new(39, 29, 28),
        surface_container_high: Rgb::new(50, 40, 38),
        surface_container_highest: Rgb::new(61, 50, 48),
        background: Rgb::new(26, 17, 16),
        on_background: Rgb::new(241, 223, 220),
        shadow: Rgb::new(0, 0, 0),
        scrim: Rgb::new(0, 0, 0),
    });

    Ok(())
}
