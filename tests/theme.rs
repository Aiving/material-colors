use material_colors::argb_from_hex;
use material_colors::theme_from_source_color;
use material_colors::utils::string::ParseError;
use material_colors::Scheme;

#[test]
fn test_theme() -> Result<(), ParseError> {
    let source = argb_from_hex("#AAE5A4")?;
    let theme = theme_from_source_color(source, vec![]);

    assert_eq!(
        theme.schemes.dark,
        Scheme {
            primary: [255, 160, 211, 154],
            on_primary: [255, 9, 57, 16],
            primary_container: [255, 35, 80, 36],
            on_primary_container: [255, 188, 240, 181],
            inverse_primary: [255, 59, 105, 58],
            primary_fixed: [255, 188, 240, 181],
            primary_fixed_dim: [255, 160, 211, 154],
            on_primary_fixed: [255, 0, 34, 4],
            on_primary_fixed_variant: [255, 35, 80, 36],
            secondary: [255, 185, 204, 180],
            on_secondary: [255, 37, 52, 35],
            secondary_container: [255, 185, 204, 180],
            on_secondary_container: [255, 213, 232, 207],
            secondary_fixed: [255, 213, 232, 207],
            secondary_fixed_dim: [255, 185, 204, 180],
            on_secondary_fixed: [255, 16, 31, 16],
            on_secondary_fixed_variant: [255, 59, 75, 56],
            tertiary: [255, 161, 207, 212],
            on_tertiary: [255, 0, 54, 59],
            tertiary_container: [255, 31, 77, 82],
            on_tertiary_container: [255, 188, 235, 241],
            tertiary_fixed: [255, 188, 235, 241],
            tertiary_fixed_dim: [255, 161, 207, 212],
            on_tertiary_fixed: [255, 0, 31, 35],
            on_tertiary_fixed_variant: [255, 31, 77, 82],
            error: [255, 255, 180, 171],
            on_error: [255, 105, 0, 5],
            error_container: [255, 147, 0, 10],
            on_error_container: [255, 255, 218, 214],
            surface_dim: [255, 16, 20, 15],
            surface: [255, 16, 20, 15],
            surface_bright: [255, 54, 58, 52],
            surface_container_lowest: [255, 11, 15, 10],
            surface_container_low: [255, 24, 29, 23],
            surface_container: [255, 28, 33, 27],
            surface_container_high: [255, 39, 43, 37],
            surface_container_highest: [255, 50, 54, 48],
            on_surface: [255, 224, 228, 219],
            on_surface_variant: [255, 194, 201, 189],
            outline: [255, 140, 147, 136],
            outline_variant: [255, 66, 73, 64],
            inverse_surface: [255, 224, 228, 219],
            inverse_on_surface: [255, 45, 50, 44],
            surface_variant: [255, 66, 73, 64],
            background: [255, 16, 20, 15],
            on_background: [255, 224, 228, 219],
            shadow: [255, 0, 0, 0],
            scrim: [255, 0, 0, 0],
        },
    );

    Ok(())
}
