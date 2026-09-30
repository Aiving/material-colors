use material_colors::{color::Rgb, scheme::variant::SchemeVibrant};

#[test]
fn test_key_colors() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary_palette_key_color(), Rgb::from_u32(0x080CFF));
    assert_eq!(scheme.secondary_palette_key_color(), Rgb::from_u32(0x7B7296));
    assert_eq!(scheme.tertiary_palette_key_color(), Rgb::from_u32(0x886C9D));
    assert_eq!(scheme.neutral_palette_key_color(), Rgb::from_u32(0x777682));

    assert_eq!(scheme.neutral_variant_palette_key_color(), Rgb::from_u32(0x767685));
}

#[test]
fn test_light_theme_min_contrast_primary() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x5660FF));
}

#[test]
fn test_light_theme_standard_contrast_primary() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x343DFF));
}

#[test]
fn test_light_theme_max_contrast_primary() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x00019F));
}

#[test]
fn test_light_theme_min_contrast_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0xD5D6FF));
}

#[test]
fn test_light_theme_standard_contrast_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0xE0E0FF));
}

#[test]
fn test_light_theme_max_contrast_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000F6));
}

#[test]
fn test_light_theme_min_contrast_on_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x5E68FF));
}

#[test]
fn test_light_theme_standard_contrast_on_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x0000EF));
}

#[test]
fn test_light_theme_max_contrast_on_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xFFFFFF));
}

#[test]
fn test_light_theme_min_contrast_surface() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
}

#[test]
fn test_light_theme_standard_contrast_surface() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
}

#[test]
fn test_light_theme_max_contrast_surface() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
}

#[test]
fn test_dark_theme_min_contrast_primary() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x7C84FF));
}

#[test]
fn test_dark_theme_standard_contrast_primary() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0xBEC2FF));
}

#[test]
fn test_dark_theme_max_contrast_primary() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0xF0EEFF));
}

#[test]
fn test_dark_theme_min_contrast_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0001C9));
}

#[test]
fn test_dark_theme_standard_contrast_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000EF));
}

#[test]
fn test_dark_theme_max_contrast_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0xBABDFF));
}

#[test]
fn test_dark_theme_min_contrast_on_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x6B75FF));
}

#[test]
fn test_dark_theme_standard_contrast_on_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xE0E0FF));
}

#[test]
fn test_dark_theme_max_contrast_on_primary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x00003D));
}

#[test]
fn test_dark_theme_min_contrast_on_tertiary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x9679AB));
}

#[test]
fn test_dark_theme_standard_contrast_on_tertiary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xF2DAFF));
}

#[test]
fn test_dark_theme_max_contrast_on_tertiary_container() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x16002A));
}

#[test]
fn test_dark_theme_min_contrast_surface() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x12131C));
}

#[test]
fn test_dark_theme_standard_contrast_surface() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x12131C));
}

#[test]
fn test_dark_theme_max_contrast_surface() {
    let scheme = SchemeVibrant::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x12131C));
}
