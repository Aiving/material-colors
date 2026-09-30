pub mod cam16;
pub mod viewing_conditions;

use float_cmp::{approx_eq, assert_approx_eq};
use material_colors::{
    color::{Rgb, y_from_lstar},
    hct::{Cam16, Hct, ViewingConditions},
};

const BLACK: Rgb = Rgb::from_u32(0x000000);
const WHITE: Rgb = Rgb::from_u32(0xFFFFFF);
const RED: Rgb = Rgb::from_u32(0xFF0000);
const GREEN: Rgb = Rgb::from_u32(0x00FF00);
const BLUE: Rgb = Rgb::from_u32(0x0000FF);
const MIDGRAY: Rgb = Rgb::from_u32(0x777777);

const fn color_is_on_boundary(rgb: Rgb) -> bool {
    rgb.red == 0 || rgb.red == 255 || rgb.green == 0 || rgb.green == 255 || rgb.blue == 0 || rgb.blue == 255
}

#[test]
fn test_hash_code() {
    let a: Hct = Rgb::from_u32(123).into();
    let b: Hct = Rgb::from_u32(123).into();

    assert_eq!(a, b);
}

#[test]
fn test_conversions_are_reflexive() {
    let cam = Cam16::from(RED);
    let color = cam.viewed(&ViewingConditions::s_rgb());

    assert_eq!(color, RED);
}

#[test]
fn test_ymidgray() {
    assert_approx_eq!(f64, 18.418, y_from_lstar(50.0), epsilon = 0.001);
}

#[test]
fn test_yblack() {
    assert_approx_eq!(f64, 0.0, y_from_lstar(0.0), epsilon = 0.001);
}

#[test]
fn test_ywhite() {
    assert_approx_eq!(f64, 100.0, y_from_lstar(100.0), epsilon = 0.001);
}

#[test]
fn test_cam_red() {
    let cam = Cam16::from(RED);

    assert_approx_eq!(f64, 46.445, cam.lightness, epsilon = 0.001);
    assert_approx_eq!(f64, 113.357, cam.chroma, epsilon = 0.001);
    assert_approx_eq!(f64, 27.408, cam.hue, epsilon = 0.001);
    assert_approx_eq!(f64, 89.494, cam.colorfulness, epsilon = 0.001);
    assert_approx_eq!(f64, 91.889, cam.saturation, epsilon = 0.001);
    assert_approx_eq!(f64, 105.988, cam.brightness, epsilon = 0.001);
}

#[test]
fn test_cam_green() {
    let cam = Cam16::from(GREEN);

    assert_approx_eq!(f64, 79.331, cam.lightness, epsilon = 0.001);
    assert_approx_eq!(f64, 108.410, cam.chroma, epsilon = 0.001);
    assert_approx_eq!(f64, 142.139, cam.hue, epsilon = 0.001);
    assert_approx_eq!(f64, 85.587, cam.colorfulness, epsilon = 0.001);
    assert_approx_eq!(f64, 78.604, cam.saturation, epsilon = 0.001);
    assert_approx_eq!(f64, 138.520, cam.brightness, epsilon = 0.001);
}

#[test]
fn test_cam_blue() {
    let cam = Cam16::from(BLUE);

    assert_approx_eq!(f64, 25.465, cam.lightness, epsilon = 0.001);
    assert_approx_eq!(f64, 87.230, cam.chroma, epsilon = 0.001);
    assert_approx_eq!(f64, 282.788, cam.hue, epsilon = 0.001);
    assert_approx_eq!(f64, 68.867, cam.colorfulness, epsilon = 0.001);
    assert_approx_eq!(f64, 93.674, cam.saturation, epsilon = 0.001);
    assert_approx_eq!(f64, 78.481, cam.brightness, epsilon = 0.001);
}

#[test]
fn test_cam_black() {
    let cam = Cam16::from(BLACK);

    assert_approx_eq!(f64, 0.0, cam.lightness, epsilon = 0.001);
    assert_approx_eq!(f64, 0.0, cam.chroma, epsilon = 0.001);
    assert_approx_eq!(f64, 0.0, cam.hue, epsilon = 0.001);
    assert_approx_eq!(f64, 0.0, cam.colorfulness, epsilon = 0.001);
    assert_approx_eq!(f64, 0.0, cam.saturation, epsilon = 0.001);
    assert_approx_eq!(f64, 0.0, cam.brightness, epsilon = 0.001);
}

#[test]
fn test_cam_white() {
    let cam = Cam16::from(WHITE);

    assert_approx_eq!(f64, 100.0, cam.lightness, epsilon = 0.001);
    assert_approx_eq!(f64, 2.869, cam.chroma, epsilon = 0.001);
    assert_approx_eq!(f64, 209.492, cam.hue, epsilon = 0.001);
    assert_approx_eq!(f64, 2.265, cam.colorfulness, epsilon = 0.001);
    assert_approx_eq!(f64, 12.068, cam.saturation, epsilon = 0.001);
    assert_approx_eq!(f64, 155.521, cam.brightness, epsilon = 0.001);
}

#[test]
fn test_camut_map_red() {
    let color_to_test = RED;
    let cam = Cam16::from(color_to_test);
    let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

    assert_eq!(color_to_test, color);
}

#[test]
fn test_camut_map_green() {
    let color_to_test = GREEN;
    let cam = Cam16::from(color_to_test);
    let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

    assert_eq!(color_to_test, color);
}

#[test]
fn test_camut_map_blue() {
    let color_to_test = BLUE;
    let cam = Cam16::from(color_to_test);
    let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

    assert_eq!(color_to_test, color);
}

#[test]
fn test_camut_map_white() {
    let color_to_test = WHITE;
    let cam = Cam16::from(color_to_test);
    let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

    assert_eq!(color_to_test, color);
}

#[test]
fn test_camut_map_midgray() {
    let color_to_test = MIDGRAY;
    let cam = Cam16::from(color_to_test);
    let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

    assert_eq!(color_to_test, color);
}

#[test]
fn test_camut_map_black() {
    let color_to_test = BLACK;
    let cam = Cam16::from(color_to_test);
    let color = Hct::from(cam.hue, cam.chroma, color_to_test.as_lstar()).into();

    assert_eq!(color_to_test, color);
}

#[test]
fn test_hct_returns_sufficiently_close_color() {
    for hue in (15..361).step_by(30) {
        for chroma in (0..100).step_by(10) {
            for tone in (20..80).step_by(10) {
                let hct_color = Hct::from(f64::from(hue), f64::from(chroma), f64::from(tone));

                if chroma > 0 {
                    assert!(
                        approx_eq!(f64, hct_color.get_hue(), f64::from(hue), epsilon = 4.0),
                        "Hue should be close for H{hue} C{chroma} T{tone}"
                    );
                }

                assert!(
                    (0.0..(f64::from(chroma) + 2.5)).contains(&hct_color.get_chroma()),
                    "Chroma should be close or less for H{hue} C{chroma} T{tone}"
                );

                if hct_color.get_chroma() < f64::from(chroma) - 2.5 {
                    assert!(
                        color_is_on_boundary(hct_color.into()),
                        "HCT request for non-sRGB color should return a color on the boundary of the sRGB cube for H{hue} C{chroma} T{tone}, but got #{} instead",
                        Rgb::from(hct_color).as_hex()
                    );
                }

                assert!(
                    approx_eq!(f64, hct_color.get_tone(), f64::from(tone), epsilon = 0.5),
                    "Tone should be close for H{hue} C{chroma} T{tone}"
                );
            }
        }
    }
}

#[test]
fn test_cam16_to_xyz_without_array() {
    let color_to_test = RED;
    let cam = Cam16::from(color_to_test);
    let xyz = cam.xyz_in_viewing_conditions(&ViewingConditions::s_rgb());

    assert_approx_eq!(f64, xyz.x, 41.23, epsilon = 0.01);
    assert_approx_eq!(f64, xyz.y, 21.26, epsilon = 0.01);
    assert_approx_eq!(f64, xyz.z, 1.93, epsilon = 0.01);
}

#[test]
fn test_color_relativity_red_in_black() {
    let color_to_test = RED;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x9F5C51));
}

#[test]
fn test_color_relativity_red_in_white() {
    let color_to_test = RED;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(100.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0xFF5D48));
}

#[test]
fn test_color_relativity_green_in_black() {
    let color_to_test = GREEN;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0xACD69D));
}

#[test]
fn test_color_relativity_green_in_white() {
    let color_to_test = GREEN;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(100.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x8EFF77));
}

#[test]
fn test_color_relativity_blue_in_black() {
    let color_to_test = BLUE;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x343654));
}

#[test]
fn test_color_relativity_blue_in_white() {
    let color_to_test = BLUE;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(100.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x3F49FF));
}

#[test]
fn test_color_relativity_white_in_black() {
    let color_to_test = WHITE;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0xFFFFFF));
}

#[test]
fn test_color_relativity_white_in_white() {
    let color_to_test = WHITE;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(100.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0xFFFFFF));
}

#[test]
fn test_color_relativity_midgray_in_black() {
    let color_to_test = MIDGRAY;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x605F5F));
}

#[test]
fn test_color_relativity_midgray_in_white() {
    let color_to_test = MIDGRAY;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(100.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x8E8E8E));
}

#[test]
fn test_color_relativity_black_in_black() {
    let color_to_test = BLACK;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(0.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x000000));
}

#[test]
fn test_color_relativity_black_in_white() {
    let color_to_test = BLACK;
    let hct: Hct = color_to_test.into();

    let result = hct.in_viewing_conditions(&ViewingConditions::make(None, None, Some(100.0), None, None));

    assert_eq!(Rgb::from(result), Rgb::from_u32(0x000000));
}
