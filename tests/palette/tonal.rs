use float_cmp::assert_approx_eq;
use material_colors::{color::Rgb, hct::Hct, palette::TonalPalette};

#[test]
fn test_exact_chroma_available() {
    let palette = TonalPalette::of(50.0, 60.0);
    let result = palette.key_color();

    assert_approx_eq!(f64, result.get_hue(), 50.0, epsilon = 10.0);
    assert_approx_eq!(f64, result.get_chroma(), 60.0, epsilon = 0.5);
    assert!(result.get_tone() > 0.0);
    assert!(result.get_tone() < 100.0);
}

#[test]
fn test_unusually_high_chroma() {
    let palette = TonalPalette::of(149.0, 200.0);
    let result = palette.key_color();

    assert_approx_eq!(f64, result.get_hue(), 149.0, epsilon = 10.0);
    assert!(result.get_chroma() > 89.0);
    assert!(result.get_tone() > 0.0);
    assert!(result.get_tone() < 100.0);
}

#[test]
fn test_unusually_low_chroma() {
    let palette = TonalPalette::of(50.0, 3.0);
    let result = palette.key_color();

    assert_approx_eq!(f64, result.get_hue(), 50.0, epsilon = 10.0);
    assert_approx_eq!(f64, result.get_chroma(), 3.0, epsilon = 0.5);
    assert_approx_eq!(f64, result.get_tone(), 50.0, epsilon = 0.5);
}

#[test]
fn test_of_tones_of_blue() {
    let hct: Hct = Rgb::from_u32(0x0000FF).into();
    let tones = TonalPalette::of(hct.get_hue(), hct.get_chroma());

    assert_eq!(tones.tone(0), Rgb::from_u32(0x000000));
    assert_eq!(tones.tone(10), Rgb::from_u32(0x00006E));
    assert_eq!(tones.tone(20), Rgb::from_u32(0x0001AC));
    assert_eq!(tones.tone(30), Rgb::from_u32(0x0000EF));
    assert_eq!(tones.tone(40), Rgb::from_u32(0x343DFF));
    assert_eq!(tones.tone(50), Rgb::from_u32(0x5A64FF));
    assert_eq!(tones.tone(60), Rgb::from_u32(0x7C84FF));
    assert_eq!(tones.tone(70), Rgb::from_u32(0x9DA3FF));
    assert_eq!(tones.tone(80), Rgb::from_u32(0xBEC2FF));
    assert_eq!(tones.tone(90), Rgb::from_u32(0xE0E0FF));
    assert_eq!(tones.tone(95), Rgb::from_u32(0xF1EFFF));
    assert_eq!(tones.tone(99), Rgb::from_u32(0xFFFBFF));
    assert_eq!(tones.tone(100), Rgb::from_u32(0xFFFFFF));

    // Tone not in `TonalPalette::COMMON_TONES`.
    assert_eq!(tones.tone(3), Rgb::from_u32(0x00003C));
}

#[test]
fn test_of_operator_and_hash() {
    let hct_ab: Hct = Rgb::from_u32(0x0000FF).into();
    let tones_a = TonalPalette::of(hct_ab.get_hue(), hct_ab.get_chroma());
    let tones_b = TonalPalette::of(hct_ab.get_hue(), hct_ab.get_chroma());
    let hct_c: Hct = Rgb::from_u32(0x123456).into();
    let tones_c = TonalPalette::of(hct_c.get_hue(), hct_c.get_chroma());

    assert_eq!(tones_a, tones_b);
    assert_ne!(tones_b, tones_c);
}
