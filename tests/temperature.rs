use float_cmp::assert_approx_eq;
use material_colors::{color::Rgb, hct::Hct, temperature::TemperatureCache};

#[test]
fn test_raw_temperature() {
    let blue_hct = Hct::new(Rgb::from_u32(0x0000FF));
    let red_hct = Hct::new(Rgb::from_u32(0xFF0000));
    let green_hct = Hct::new(Rgb::from_u32(0x00FF00));
    let white_hct = Hct::new(Rgb::from_u32(0xFFFFFF));
    let black_hct = Hct::new(Rgb::from_u32(0x000000));

    let blue_temp = TemperatureCache::raw_temperature(&blue_hct);
    let red_temp = TemperatureCache::raw_temperature(&red_hct);
    let green_temp = TemperatureCache::raw_temperature(&green_hct);
    let white_temp = TemperatureCache::raw_temperature(&white_hct);
    let black_temp = TemperatureCache::raw_temperature(&black_hct);

    assert_approx_eq!(f64, -1.393, blue_temp, epsilon = 0.001);
    assert_approx_eq!(f64, 2.351, red_temp, epsilon = 0.001);
    assert_approx_eq!(f64, -0.267, green_temp, epsilon = 0.001);
    assert_approx_eq!(f64, -0.5, white_temp, epsilon = 0.001);
    assert_approx_eq!(f64, -0.5, black_temp, epsilon = 0.001);
}

#[test]
fn test_complement() {
    let blue_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0x0000FF))).complement().into();
    let red_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFF0000))).complement().into();
    let green_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0x00FF00))).complement().into();
    let white_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFFFFFF))).complement().into();
    let black_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0x000000))).complement().into();

    assert_eq!(Rgb::from_u32(0x9D0002), blue_complement);
    assert_eq!(Rgb::from_u32(0x007BFC), red_complement);
    assert_eq!(Rgb::from_u32(0xFFD2C9), green_complement);
    assert_eq!(Rgb::from_u32(0xFFFFFF), white_complement);
    assert_eq!(Rgb::from_u32(0x000000), black_complement);
}

#[test]
fn test_blue_analogous() {
    let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0x0000FF))).analogous();

    assert_eq!(Rgb::from_u32(0x00590C), analogous[0].into());
    assert_eq!(Rgb::from_u32(0x00564E), analogous[1].into());
    assert_eq!(Rgb::from_u32(0x0000FF), analogous[2].into());
    assert_eq!(Rgb::from_u32(0x6700CC), analogous[3].into());
    assert_eq!(Rgb::from_u32(0x81009F), analogous[4].into());
    assert_eq!(5, analogous.len());
}

#[test]
fn test_red_analogous() {
    let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFF0000))).analogous();

    assert_eq!(Rgb::from_u32(0xF60082), analogous[0].into());
    assert_eq!(Rgb::from_u32(0xFC004C), analogous[1].into());
    assert_eq!(Rgb::from_u32(0xFF0000), analogous[2].into());
    assert_eq!(Rgb::from_u32(0xD95500), analogous[3].into());
    assert_eq!(Rgb::from_u32(0xAF7200), analogous[4].into());
    assert_eq!(5, analogous.len());
}

#[test]
fn test_green_analogous() {
    let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0x00FF00))).analogous();

    assert_eq!(Rgb::from_u32(0xCEE900), analogous[0].into());
    assert_eq!(Rgb::from_u32(0x92F500), analogous[1].into());
    assert_eq!(Rgb::from_u32(0x00FF00), analogous[2].into());
    assert_eq!(Rgb::from_u32(0x00FD6F), analogous[3].into());
    assert_eq!(Rgb::from_u32(0x00FAB3), analogous[4].into());
    assert_eq!(5, analogous.len());
}

#[test]
fn test_white_analogous() {
    let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFFFFFF))).analogous();

    assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[0].into());
    assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[1].into());
    assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[2].into());
    assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[3].into());
    assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[4].into());
    assert_eq!(5, analogous.len());
}

#[test]
fn test_black_analogous() {
    let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0x000000))).analogous();

    assert_eq!(Rgb::from_u32(0x000000), analogous[0].into());
    assert_eq!(Rgb::from_u32(0x000000), analogous[1].into());
    assert_eq!(Rgb::from_u32(0x000000), analogous[2].into());
    assert_eq!(Rgb::from_u32(0x000000), analogous[3].into());
    assert_eq!(Rgb::from_u32(0x000000), analogous[4].into());
    assert_eq!(5, analogous.len());
}
