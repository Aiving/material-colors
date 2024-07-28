use material_colors::{
    color::{Argb, Rgb},
    scheme::Scheme,
    theme::ThemeBuilder,
    Error,
};

#[test]
fn test_theme() -> Result<(), Error> {
    let theme = ThemeBuilder::with_source(Argb::from_u32(0xffff0000)).build();

    assert_eq!(
        theme.schemes.dark,
        Scheme {
            primary: Rgb::new(255, 180, 168).into(),
            on_primary: Rgb::new(86, 30, 22).into(),
            primary_container: Rgb::new(115, 52, 42).into(),
            on_primary_container: Rgb::new(255, 218, 212).into(),
            inverse_primary: Rgb::new(144, 75, 64).into(),
            secondary: Rgb::new(231, 189, 182).into(),
            on_secondary: Rgb::new(68, 41, 37).into(),
            secondary_container: Rgb::new(93, 63, 59).into(),
            on_secondary_container: Rgb::new(255, 218, 212).into(),
            tertiary: Rgb::new(222, 196, 140).into(),
            on_tertiary: Rgb::new(62, 46, 4).into(),
            tertiary_container: Rgb::new(86, 68, 25).into(),
            on_tertiary_container: Rgb::new(251, 223, 166).into(),
            error: Rgb::new(255, 180, 171).into(),
            on_error: Rgb::new(105, 0, 5).into(),
            error_container: Rgb::new(147, 0, 10).into(),
            on_error_container: Rgb::new(255, 218, 214).into(),
            primary_fixed: Rgb::new(255, 218, 212).into(),
            on_primary_fixed: Rgb::new(58, 9, 5).into(),
            primary_fixed_dim: Rgb::new(255, 180, 168).into(),
            on_primary_fixed_variant: Rgb::new(115, 52, 42).into(),
            secondary_fixed: Rgb::new(255, 218, 212).into(),
            on_secondary_fixed: Rgb::new(44, 21, 18).into(),
            secondary_fixed_dim: Rgb::new(231, 189, 182).into(),
            on_secondary_fixed_variant: Rgb::new(93, 63, 59).into(),
            tertiary_fixed: Rgb::new(251, 223, 166).into(),
            on_tertiary_fixed: Rgb::new(37, 26, 0).into(),
            tertiary_fixed_dim: Rgb::new(222, 196, 140).into(),
            on_tertiary_fixed_variant: Rgb::new(86, 68, 25).into(),
            surface: Rgb::new(26, 17, 16).into(),
            on_surface: Rgb::new(241, 223, 220).into(),
            surface_variant: Rgb::new(83, 67, 65).into(),
            on_surface_variant: Rgb::new(216, 194, 190).into(),
            inverse_surface: Rgb::new(241, 223, 220).into(),
            inverse_on_surface: Rgb::new(57, 46, 44).into(),
            outline: Rgb::new(160, 140, 137).into(),
            outline_variant: Rgb::new(83, 67, 65).into(),
            surface_dim: Rgb::new(26, 17, 16).into(),
            surface_tint: Rgb::new(255, 180, 168).into(),
            surface_bright: Rgb::new(66, 55, 53).into(),
            surface_container_lowest: Rgb::new(20, 12, 11).into(),
            surface_container_low: Rgb::new(35, 25, 24).into(),
            surface_container: Rgb::new(39, 29, 28).into(),
            surface_container_high: Rgb::new(50, 40, 38).into(),
            surface_container_highest: Rgb::new(61, 50, 48).into(),
            background: Rgb::new(26, 17, 16).into(),
            on_background: Rgb::new(241, 223, 220).into(),
            shadow: Rgb::new(0, 0, 0).into(),
            scrim: Rgb::new(0, 0, 0).into(),
        }
    );

    Ok(())
}
