use material_colors::{color::Rgb, scheme::variant::SchemeExpressive};

#[test]
fn test_key_colors() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary_palette_key_color(), Rgb::from_u32(0x35855F));
    assert_eq!(scheme.secondary_palette_key_color(), Rgb::from_u32(0x8C6D8C));
    assert_eq!(scheme.tertiary_palette_key_color(), Rgb::from_u32(0x806EA1));
    assert_eq!(scheme.neutral_palette_key_color(), Rgb::from_u32(0x79757F));

    assert_eq!(scheme.neutral_variant_palette_key_color(), Rgb::from_u32(0x7A7585));
}

#[test]
fn test_light_theme_min_contrast_primary() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x32835D));
}

#[test]
fn test_light_theme_standard_contrast_primary() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x146C48));
}

#[test]
fn test_light_theme_max_contrast_primary() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x00341F));
}

#[test]
fn test_light_theme_min_contrast_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x99EABD));
}

#[test]
fn test_light_theme_standard_contrast_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0xA2F4C6));
}

#[test]
fn test_light_theme_max_contrast_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x005436));
}

#[test]
fn test_light_theme_min_contrast_on_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x388862));
}

#[test]
fn test_light_theme_standard_contrast_on_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x005234));
}

#[test]
fn test_light_theme_max_contrast_on_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xFFFFFF));
}

#[test]
fn test_light_theme_min_contrast_surface() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFDF7FF));
}

#[test]
fn test_light_theme_standard_contrast_surface() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFDF7FF));
}

#[test]
fn test_light_theme_max_contrast_surface() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFDF7FF));
}

#[test]
fn test_dark_theme_min_contrast_primary() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x51A078));
}

#[test]
fn test_dark_theme_standard_contrast_primary() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x87D7AB));
}

#[test]
fn test_dark_theme_max_contrast_primary() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0xBBFFD7));
}

#[test]
fn test_dark_theme_min_contrast_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x00432A));
}

#[test]
fn test_dark_theme_standard_contrast_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x005234));
}

#[test]
fn test_dark_theme_max_contrast_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x83D3A8));
}

#[test]
fn test_dark_theme_min_contrast_on_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x43936C));
}

#[test]
fn test_dark_theme_standard_contrast_on_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xA2F4C6));
}

#[test]
fn test_dark_theme_max_contrast_on_primary_container() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x000E06));
}

#[test]
fn test_dark_theme_min_contrast_surface() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x14121A));
}

#[test]
fn test_dark_theme_standard_contrast_surface() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x14121A));
}

#[test]
fn test_dark_theme_max_contrast_surface() {
    let scheme = SchemeExpressive::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x14121A));
}
