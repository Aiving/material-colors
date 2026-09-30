#![allow(deprecated)]

use material_colors::{color::Rgb, palette::CorePalette};

#[test]
fn test_equals_and_hash() {
    let core_palette_a = CorePalette::of(Rgb::from_u32(0x0000FF));
    let core_palette_b = CorePalette::of(Rgb::from_u32(0x0000FF));
    let core_palette_c = CorePalette::of(Rgb::from_u32(0x123456));

    assert_eq!(core_palette_a, core_palette_b);
    assert_ne!(core_palette_b, core_palette_c);
}

#[test]
fn test_of_blue() {
    let core = CorePalette::of(Rgb::from_u32(0x0000FF));

    assert_eq!(core.primary.tone(100), Rgb::from_u32(0xFFFFFF));
    assert_eq!(core.primary.tone(95), Rgb::from_u32(0xF1EFFF));
    assert_eq!(core.primary.tone(90), Rgb::from_u32(0xE0E0FF));
    assert_eq!(core.primary.tone(80), Rgb::from_u32(0xBEC2FF));
    assert_eq!(core.primary.tone(70), Rgb::from_u32(0x9DA3FF));
    assert_eq!(core.primary.tone(60), Rgb::from_u32(0x7C84FF));
    assert_eq!(core.primary.tone(50), Rgb::from_u32(0x5A64FF));
    assert_eq!(core.primary.tone(40), Rgb::from_u32(0x343DFF));
    assert_eq!(core.primary.tone(30), Rgb::from_u32(0x0000EF));
    assert_eq!(core.primary.tone(20), Rgb::from_u32(0x0001AC));
    assert_eq!(core.primary.tone(10), Rgb::from_u32(0x00006E));
    assert_eq!(core.primary.tone(0), Rgb::from_u32(0x000000));
    assert_eq!(core.secondary.tone(100), Rgb::from_u32(0xFFFFFF));
    assert_eq!(core.secondary.tone(95), Rgb::from_u32(0xF1EFFF));
    assert_eq!(core.secondary.tone(90), Rgb::from_u32(0xE1E0F9));
    assert_eq!(core.secondary.tone(80), Rgb::from_u32(0xC5C4DD));
    assert_eq!(core.secondary.tone(70), Rgb::from_u32(0xA9A9C1));
    assert_eq!(core.secondary.tone(60), Rgb::from_u32(0x8F8FA6));
    assert_eq!(core.secondary.tone(50), Rgb::from_u32(0x75758B));
    assert_eq!(core.secondary.tone(40), Rgb::from_u32(0x5C5D72));
    assert_eq!(core.secondary.tone(30), Rgb::from_u32(0x444559));
    assert_eq!(core.secondary.tone(20), Rgb::from_u32(0x2E2F42));
    assert_eq!(core.secondary.tone(10), Rgb::from_u32(0x191A2C));
    assert_eq!(core.secondary.tone(0), Rgb::from_u32(0x000000));
}

#[test]
fn test_content_of_blue() {
    let core = CorePalette::content_of(Rgb::from_u32(0x0000FF));

    assert_eq!(core.primary.tone(100), Rgb::from_u32(0xFFFFFF));
    assert_eq!(core.primary.tone(95), Rgb::from_u32(0xF1EFFF));
    assert_eq!(core.primary.tone(90), Rgb::from_u32(0xE0E0FF));
    assert_eq!(core.primary.tone(80), Rgb::from_u32(0xBEC2FF));
    assert_eq!(core.primary.tone(70), Rgb::from_u32(0x9DA3FF));
    assert_eq!(core.primary.tone(60), Rgb::from_u32(0x7C84FF));
    assert_eq!(core.primary.tone(50), Rgb::from_u32(0x5A64FF));
    assert_eq!(core.primary.tone(40), Rgb::from_u32(0x343DFF));
    assert_eq!(core.primary.tone(30), Rgb::from_u32(0x0000EF));
    assert_eq!(core.primary.tone(20), Rgb::from_u32(0x0001AC));
    assert_eq!(core.primary.tone(10), Rgb::from_u32(0x00006E));
    assert_eq!(core.primary.tone(0), Rgb::from_u32(0x000000));
    assert_eq!(core.secondary.tone(100), Rgb::from_u32(0xFFFFFF));
    assert_eq!(core.secondary.tone(95), Rgb::from_u32(0xF1EFFF));
    assert_eq!(core.secondary.tone(90), Rgb::from_u32(0xE0E0FF));
    assert_eq!(core.secondary.tone(80), Rgb::from_u32(0xC1C3F4));
    assert_eq!(core.secondary.tone(70), Rgb::from_u32(0xA5A7D7));
    assert_eq!(core.secondary.tone(60), Rgb::from_u32(0x8B8DBB));
    assert_eq!(core.secondary.tone(50), Rgb::from_u32(0x7173A0));
    assert_eq!(core.secondary.tone(40), Rgb::from_u32(0x585B86));
    assert_eq!(core.secondary.tone(30), Rgb::from_u32(0x40436D));
    assert_eq!(core.secondary.tone(20), Rgb::from_u32(0x2A2D55));
    assert_eq!(core.secondary.tone(10), Rgb::from_u32(0x14173F));
    assert_eq!(core.secondary.tone(0), Rgb::from_u32(0x000000));
}
