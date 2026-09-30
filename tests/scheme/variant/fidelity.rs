use material_colors::{color::Rgb, scheme::variant::SchemeFidelity};

#[test]
fn test_key_colors() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary_palette_key_color(), Rgb::from_u32(0x080CFF));
    assert_eq!(scheme.secondary_palette_key_color(), Rgb::from_u32(0x656DD3));
    assert_eq!(scheme.tertiary_palette_key_color(), Rgb::from_u32(0x9D0002));
    assert_eq!(scheme.neutral_palette_key_color(), Rgb::from_u32(0x767684));

    assert_eq!(scheme.neutral_variant_palette_key_color(), Rgb::from_u32(0x757589));
}

#[test]
fn test_light_theme_min_contrast_primary() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x5660FF));
}

#[test]
fn test_light_theme_standard_contrast_primary() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x0001BB));
}

#[test]
fn test_light_theme_max_contrast_primary() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x00019F));
}

#[test]
fn test_light_theme_min_contrast_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0xD5D6FF));
}

#[test]
fn test_light_theme_standard_contrast_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000FF));
}

#[test]
fn test_light_theme_max_contrast_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000F6));
}

#[test]
fn test_light_theme_min_contrast_tertiary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xFFCDC6));
}

#[test]
fn test_light_theme_standard_contrast_tertiary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x9D0002));
}

#[test]
fn test_light_theme_max_contrast_tertiary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x980002));
}

#[test]
fn test_light_theme_min_contrast_objectionable_tertiary_container_lightens() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x850096).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xEBD982));
}

#[test]
fn test_light_theme_standard_contrast_objectionable_tertiary_container_lightens() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x850096).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0xBCAC5A));
}

#[test]
fn test_light_theme_max_contrast_objectionable_tertiary_container_darkens() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x850096).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.tertiary_container(), Rgb::from_u32(0x544900));
}

#[test]
fn test_light_theme_min_contrast_on_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x5E68FF));
}

#[test]
fn test_light_theme_standard_contrast_on_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xB3B7FF));
}

#[test]
fn test_light_theme_max_contrast_on_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xFFFFFF));
}

#[test]
fn test_light_theme_min_contrast_surface() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(-1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
}

#[test]
fn test_light_theme_standard_contrast_surface() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(0.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
}

#[test]
fn test_light_theme_max_contrast_surface() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), false, Some(1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0xFBF8FF));
}

#[test]
fn test_dark_theme_min_contrast_primary() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0x7C84FF));
}

#[test]
fn test_dark_theme_standard_contrast_primary() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0xBEC2FF));
}

#[test]
fn test_dark_theme_max_contrast_primary() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.primary(), Rgb::from_u32(0xF0EEFF));
}

#[test]
fn test_dark_theme_min_contrast_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0001C9));
}

#[test]
fn test_dark_theme_standard_contrast_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0x0000FF));
}

#[test]
fn test_dark_theme_max_contrast_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.primary_container(), Rgb::from_u32(0xBABDFF));
}

#[test]
fn test_dark_theme_min_contrast_on_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x6B75FF));
}

#[test]
fn test_dark_theme_standard_contrast_on_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0xB3B7FF));
}

#[test]
fn test_dark_theme_max_contrast_on_primary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.on_primary_container(), Rgb::from_u32(0x00003D));
}

#[test]
fn test_dark_theme_min_contrast_on_tertiary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xEF4635));
}

#[test]
fn test_dark_theme_standard_contrast_on_tertiary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0xFFA598));
}

#[test]
fn test_dark_theme_max_contrast_on_tertiary_container() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.on_tertiary_container(), Rgb::from_u32(0x220000));
}

#[test]
fn test_dark_theme_min_contrast_surface() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(-1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x12121D));
}

#[test]
fn test_dark_theme_standard_contrast_surface() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(0.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x12121D));
}

#[test]
fn test_dark_theme_max_contrast_surface() {
    let scheme = SchemeFidelity::new(Rgb::from_u32(0x0000FF).into(), true, Some(1.0)).scheme;

    assert_eq!(scheme.surface(), Rgb::from_u32(0x12121D));
}
