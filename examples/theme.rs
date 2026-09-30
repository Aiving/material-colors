use material_colors::{
    Error,
    color::Rgb,
    dynamic_color::{ColorSpec, DynamicScheme, Variant, color_spec::SpecVersion, color_spec_2025::ColorSpec2025, dynamic_scheme::Platform},
    hct::Hct,
};

fn main() -> Result<(), Error> {
    let source = Hct::new(Rgb::from_u32(0x2FAB34));
    let scheme: DynamicScheme = DynamicScheme::new(
        source,
        Variant::TonalSpot,
        true,
        None,
        ColorSpec2025
            .get_primary_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0)
            .unwrap(),
        ColorSpec2025
            .get_secondary_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0)
            .unwrap(),
        ColorSpec2025
            .get_tertiary_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0)
            .unwrap(),
        ColorSpec2025
            .get_neutral_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0)
            .unwrap(),
        ColorSpec2025
            .get_neutral_variant_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0)
            .unwrap(),
        ColorSpec2025.get_error_palette(Variant::TonalSpot, source, true, Platform::Phone, 1.0),
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

    Ok(())
}
