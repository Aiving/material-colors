use std::str::FromStr;

use material_colors::{
    color::{Argb, Rgb},
    scheme::Scheme,
    theme::ThemeBuilder,
    Error,
};

#[test]
fn test_theme() -> Result<(), Error> {
    let theme = ThemeBuilder::default()
        .source(Argb::from_str("aae5a4")?)
        .build();

    assert_eq!(
        theme.schemes.dark,
        Scheme {
            primary: Rgb::new(160, 211, 154).into(),
            on_primary: Rgb::new(9, 57, 16).into(),
            primary_container: Rgb::new(35, 80, 36).into(),
            on_primary_container: Rgb::new(188, 240, 181).into(),
            inverse_primary: Rgb::new(59, 105, 58).into(),
            primary_fixed: Rgb::new(188, 240, 181).into(),
            primary_fixed_dim: Rgb::new(160, 211, 154).into(),
            on_primary_fixed: Rgb::new(0, 34, 4).into(),
            on_primary_fixed_variant: Rgb::new(35, 80, 36).into(),
            secondary: Rgb::new(185, 204, 180).into(),
            on_secondary: Rgb::new(37, 52, 35).into(),
            secondary_container: Rgb::new(59, 75, 56).into(),
            on_secondary_container: Rgb::new(213, 232, 207).into(),
            secondary_fixed: Rgb::new(213, 232, 207).into(),
            secondary_fixed_dim: Rgb::new(185, 204, 180).into(),
            on_secondary_fixed: Rgb::new(16, 31, 16).into(),
            on_secondary_fixed_variant: Rgb::new(59, 75, 56).into(),
            tertiary: Rgb::new(161, 207, 212).into(),
            on_tertiary: Rgb::new(0, 54, 59).into(),
            tertiary_container: Rgb::new(31, 77, 82).into(),
            on_tertiary_container: Rgb::new(188, 235, 241).into(),
            tertiary_fixed: Rgb::new(188, 235, 241).into(),
            tertiary_fixed_dim: Rgb::new(161, 207, 212).into(),
            on_tertiary_fixed: Rgb::new(0, 31, 35).into(),
            on_tertiary_fixed_variant: Rgb::new(31, 77, 82).into(),
            error: Rgb::new(255, 180, 171).into(),
            on_error: Rgb::new(105, 0, 5).into(),
            error_container: Rgb::new(147, 0, 10).into(),
            on_error_container: Rgb::new(255, 218, 214).into(),
            surface_dim: Rgb::new(16, 20, 15).into(),
            surface: Rgb::new(16, 20, 15).into(),
            surface_bright: Rgb::new(54, 58, 52).into(),
            surface_container_lowest: Rgb::new(11, 15, 10).into(),
            surface_container_low: Rgb::new(24, 29, 23).into(),
            surface_container: Rgb::new(28, 33, 27).into(),
            surface_container_high: Rgb::new(39, 43, 37).into(),
            surface_container_highest: Rgb::new(50, 54, 48).into(),
            on_surface: Rgb::new(224, 228, 219).into(),
            on_surface_variant: Rgb::new(194, 201, 189).into(),
            outline: Rgb::new(140, 147, 136).into(),
            outline_variant: Rgb::new(66, 73, 64).into(),
            inverse_surface: Rgb::new(224, 228, 219).into(),
            inverse_on_surface: Rgb::new(45, 50, 44).into(),
            surface_variant: Rgb::new(66, 73, 64).into(),
            background: Rgb::new(16, 20, 15).into(),
            on_background: Rgb::new(224, 228, 219).into(),
            shadow: Rgb::new(0, 0, 0).into(),
            scrim: Rgb::new(0, 0, 0).into(),
        },
    );

    Ok(())
}
