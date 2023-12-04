use crate::dislike::fix_if_disliked;
use crate::hct::Hct;

use super::contrast_curve::ContrastCurve;
use super::dynamic_scheme::DynamicScheme;
use super::tone_delta_pair::ToneDeltaPair;
use super::tone_delta_pair::TonePolarity;
use super::variant::Variant;
use super::DynamicColor;

fn _is_fidelity(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Fidelity) || matches!(scheme.variant, Variant::Content)
}

fn _is_monochrome(scheme: &DynamicScheme) -> bool {
    matches!(scheme.variant, Variant::Monochrome)
}

macro_rules! define_key {
    ($name:ident => $palette:ident; [tone, $scheme_argument:ident] => $tone:expr;) => {
        pub fn $name() -> DynamicColor {
            DynamicColor::new(
                stringify!($name),
                |scheme| &scheme.$palette,
                |$scheme_argument| $tone,
                false,
                None,
                None,
                None,
                None,
            )
        }
    };

    ($name:ident => $palette:ident; [tone, $tone_scheme_argument:ident] => $tone:expr; [background, $background_scheme_argument:ident] => $background:expr;) => {
        pub fn $name() -> DynamicColor {
            DynamicColor::new(
                stringify!($name),
                |scheme| &scheme.$palette,
                |$tone_scheme_argument| $tone,
                false,
                Some(|$background_scheme_argument| $background),
                None,
                None,
                None,
            )
        }
    };

    ($name:ident => $palette:ident; [tone, $tone_scheme_argument:ident] => $tone:expr; [background, $background_scheme_argument:ident] => $background:expr; [contrast_curve] => $contrast_curve:expr;) => {
        pub fn $name() -> DynamicColor {
            DynamicColor::new(
                stringify!($name),
                |scheme| &scheme.$palette,
                |$tone_scheme_argument| $tone,
                false,
                Some(|$background_scheme_argument| $background),
                None,
                Some($contrast_curve),
                None,
            )
        }
    };

    ($name:ident => $palette:ident; [tone, $tone_scheme_argument:ident] => $tone:expr; [background, $background_scheme_argument:ident] => $background:expr; [second_background, $second_background_scheme_argument:ident] => $second_background:expr; [contrast_curve] => $contrast_curve:expr;) => {
        pub fn $name() -> DynamicColor {
            DynamicColor::new(
                stringify!($name),
                |scheme| &scheme.$palette,
                |$tone_scheme_argument| $tone,
                false,
                Some(|$background_scheme_argument| $background),
                Some(|$second_background_scheme_argument| $second_background),
                Some($contrast_curve),
                None,
            )
        }
    };

    ($name:ident => $palette:ident; [tone, $tone_scheme_argument:ident] => $tone:expr; [background, $background_scheme_argument:ident] => $background:expr; [contrast_curve] => $contrast_curve:expr; [tone_delta_pair, $tone_delta_pair_argument:ident] => $tone_delta_pair:expr;) => {
        pub fn $name() -> DynamicColor {
            DynamicColor::new(
                stringify!($name),
                |scheme| &scheme.$palette,
                |$tone_scheme_argument| $tone,
                false,
                Some(|$background_scheme_argument| $background),
                None,
                Some($contrast_curve),
                Some(|$tone_delta_pair_argument| $tone_delta_pair),
            )
        }
    };

    (background $name:ident => $palette:ident; [tone, $tone_scheme_argument:ident] => $tone:expr; [background, $background_scheme_argument:ident] => $background:expr; [contrast_curve] => $contrast_curve:expr; [tone_delta_pair, $tone_delta_pair_argument:ident] => $tone_delta_pair:expr;) => {
        pub fn $name() -> DynamicColor {
            DynamicColor::new(
                stringify!($name),
                |scheme| &scheme.$palette,
                |$tone_scheme_argument| $tone,
                true,
                Some(|$background_scheme_argument| $background),
                None,
                Some($contrast_curve),
                Some(|$tone_delta_pair_argument| $tone_delta_pair),
            )
        }
    };

    (background $name:ident => $palette:ident; [tone, $scheme_argument:ident] => $tone:expr;) => {
        pub fn $name() -> DynamicColor {
            DynamicColor::new(
                stringify!($name),
                |scheme| &scheme.$palette,
                |$scheme_argument| $tone,
                true,
                None,
                None,
                None,
                None,
            )
        }
    };
}

/// Tokens, or named colors, in the Material Design system.
pub struct MaterialDynamicColors;

impl MaterialDynamicColors {
    pub(crate) const CONTENT_ACCENT_TONE_DELTA: f64 = 15.0;

    fn highest_surface(scheme: &DynamicScheme) -> DynamicColor {
        if scheme.is_dark {
            Self::surface_bright()
        } else {
            Self::surface_dim()
        }
    }

    define_key! {
      primary_palette_key_color => primary_palette;
      [tone, scheme] => scheme.primary_palette.key_color().get_tone();
    }

    define_key! {
      secondary_palette_key_color => secondary_palette;
      [tone, scheme] => scheme.secondary_palette.key_color().get_tone();
    }

    define_key! {
      tertiary_palette_key_color => tertiary_palette;
      [tone, scheme] => scheme.tertiary_palette.key_color().get_tone();
    }

    define_key! {
      neutral_palette_key_color => neutral_palette;
      [tone, scheme] => scheme.neutral_palette.key_color().get_tone();
    }

    define_key! {
      neutral_variant_palette_key_color => neutral_variant_palette;
      [tone, scheme] => scheme.neutral_variant_palette.key_color().get_tone();
    }

    define_key! {
      background background => neutral_palette;
      [tone, scheme] => if scheme.is_dark { 6.0 } else { 98.0 };
    }

    define_key! {
      on_background => neutral_palette;
      [tone, scheme] => if scheme.is_dark { 90.0 } else { 10.0 };
      [background, _scheme] => MaterialDynamicColors::background();
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 3.0, medium: 4.5, high: 7.0 };
    }

    define_key! {
      background surface => neutral_palette;
      [tone, scheme] => if scheme.is_dark { 6.0 } else { 98.0 };
    }

    define_key! {
      background surface_dim => neutral_palette;
      [tone, scheme] => if scheme.is_dark { 6.0 } else { ContrastCurve { low: 87.0, normal: 87.0, medium: 80.0, high: 75.0 }.get(scheme.contrast_level) };
    }

    define_key! {
      background surface_bright => neutral_palette;
      [tone, scheme] => if scheme.is_dark { ContrastCurve { low: 24.0, normal: 24.0, medium: 29.0, high: 34.0 }.get(scheme.contrast_level) } else { 98.0 };
    }

    define_key! {
      background surface_container_lowest => neutral_palette;
      [tone, scheme] => if scheme.is_dark { ContrastCurve { low: 4.0, normal: 4.0, medium: 2.0, high: 0.0 }.get(scheme.contrast_level) } else { 100.0 };
    }

    define_key! {
      background surface_container_low => neutral_palette;
      [tone, scheme] => if scheme.is_dark { ContrastCurve { low: 10.0, normal: 10.0, medium: 11.0, high: 12.0 }.get(scheme.contrast_level) } else { ContrastCurve { low: 96.0, normal: 96.0, medium: 96.0, high: 95.0 }.get(scheme.contrast_level) };
    }

    define_key! {
      background surface_container => neutral_palette;
      [tone, scheme] => if scheme.is_dark { ContrastCurve { low: 12.0, normal: 12.0, medium: 16.0, high: 20.0 }.get(scheme.contrast_level) } else { ContrastCurve { low: 94.0, normal: 94.0, medium: 92.0, high: 90.0 }.get(scheme.contrast_level) };
    }

    define_key! {
      background surface_container_high => neutral_palette;
      [tone, scheme] => if scheme.is_dark { ContrastCurve { low: 17.0, normal: 17.0, medium: 21.0, high: 25.0 }.get(scheme.contrast_level) } else { ContrastCurve { low: 92.0, normal: 92.0, medium: 88.0, high: 85.0 }.get(scheme.contrast_level) };
    }

    define_key! {
      background surface_container_highest => neutral_palette;
      [tone, scheme] => if scheme.is_dark { ContrastCurve { low: 22.0, normal: 22.0, medium: 26.0, high: 30.0 }.get(scheme.contrast_level) } else { ContrastCurve { low: 90.0, normal: 90.0, medium: 84.0, high: 80.0 }.get(scheme.contrast_level) };
    }

    define_key! {
      on_surface => neutral_palette;
      [tone, scheme] => if scheme.is_dark { 90.0 } else { 10.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background surface_variant => neutral_variant_palette;
      [tone, scheme] => if scheme.is_dark { 30.0 } else { 90.0 };
    }

    define_key! {
      on_surface_variant => neutral_variant_palette;
      [tone, scheme] => if scheme.is_dark { 80.0 } else { 30.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 11.0 };
    }

    define_key! {
      inverse_surface => neutral_palette;
      [tone, scheme] => if scheme.is_dark { 90.0 } else { 20.0 };
    }

    define_key! {
      inverse_on_surface => neutral_palette;
      [tone, scheme] => if scheme.is_dark { 20.0 } else { 95.0 };
      [background, _scheme] => Self::inverse_surface();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      outline => neutral_variant_palette;
      [tone, scheme] => if scheme.is_dark { 60.0 } else { 50.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.5, normal: 3.0, medium: 4.5, high: 7.0 };
    }

    define_key! {
      outline_variant => neutral_variant_palette;
      [tone, scheme] => if scheme.is_dark { 30.0 } else { 80.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
    }

    define_key! {
      shadow => neutral_palette;
      [tone, _scheme] => 0.0;
    }

    define_key! {
      scrim => neutral_palette;
      [tone, _scheme] => 0.0;
    }

    define_key! {
      background surface_tint => primary_palette;
      [tone, scheme] => if scheme.is_dark { 80.0 } else { 40.0 };
    }

    define_key! {
      background primary => primary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 100.0 } else { 0.0 } } else if scheme.is_dark { 80.0 } else { 40.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 7.0 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::primary_container(), Self::primary(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_primary => primary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 10.0 } else { 90.0 } } else if scheme.is_dark { 20.0 } else { 100.0 };
      [background, _scheme] => Self::primary();
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background primary_container => primary_palette;
      [tone, scheme] => if _is_fidelity(scheme) { scheme.source_color_hct.get_tone() } else if _is_monochrome(scheme) { if scheme.is_dark { 85.0 } else { 25.0 } } else if scheme.is_dark { 30.0 } else { 90.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::primary_container(), Self::primary(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_primary_container => primary_palette;
      [tone, scheme] => if _is_fidelity(scheme) { DynamicColor::foreground_tone(Self::primary_container().get_tone(scheme), 4.5) } else if _is_monochrome(scheme) { if scheme.is_dark { 0.0 } else { 100.0 } } else if scheme.is_dark { 90.0 } else { 10.0 };
      [background, _scheme] => Self::primary_container();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      inverse_primary => primary_palette;
      [tone, scheme] => if scheme.is_dark { 40.0 } else { 80.0 };
      [background, _scheme] => Self::inverse_surface();
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 7.0 };
    }

    define_key! {
      background secondary => secondary_palette;
      [tone, scheme] => if scheme.is_dark { 80.0 } else { 40.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 7.0 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::secondary_container(), Self::secondary(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_secondary => secondary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 10.0 } else { 100.0 } } else if scheme.is_dark { 20.0 } else { 100.0 };
      [background, _scheme] => Self::secondary();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background secondary_container => secondary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 30.0 } else { 85.0 } } else if _is_fidelity(scheme) { if scheme.is_dark { 30.0 } else { 90.0 } } else { Self::_find_desired_chroma_by_tone(scheme.secondary_palette.hue(), scheme.secondary_palette.chroma(), if scheme.is_dark { 30.0 } else { 90.0 }, !scheme.is_dark) };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::primary_container(), Self::primary(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_secondary_container => secondary_palette;
      [tone, scheme] => if _is_fidelity(scheme) { DynamicColor::foreground_tone(MaterialDynamicColors::secondary_container().get_tone(scheme), 4.5) } else if scheme.is_dark { 90.0 } else { 10.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background tertiary => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 90.0 } else { 25.0 } } else if scheme.is_dark { 80.0 } else { 40.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 7.0 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::tertiary_container(), Self::tertiary(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_tertiary => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 10.0 } else { 90.0 } } else if scheme.is_dark { 20.0 } else { 100.0 };
      [background, _scheme] => Self::tertiary();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background tertiary_container => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 60.0 } else { 49.0 } } else if !_is_fidelity(scheme) { if scheme.is_dark { 30.0 } else { 90.0 } } else { fix_if_disliked(scheme.tertiary_palette.get_hct(scheme.source_color_hct.get_tone())).get_tone() };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::tertiary_container(), Self::tertiary(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_tertiary_container => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { if scheme.is_dark { 0.0 } else { 100.0 } } else if !_is_fidelity(scheme) { if scheme.is_dark { 90.0 } else { 10.0 } } else { DynamicColor::foreground_tone(Self::tertiary_container().get_tone(scheme), 4.5) };
      [background, _scheme] => Self::tertiary_container();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background error => error_palette;
      [tone, scheme] => if scheme.is_dark { 80.0 } else { 40.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 7.0 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::error_container(), Self::error(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_error => error_palette;
      [tone, scheme] => if scheme.is_dark { 20.0 } else { 100.0 };
      [background, _scheme] => Self::error();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background error_container => error_palette;
      [tone, scheme] => if scheme.is_dark { 30.0 } else { 90.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::error_container(), Self::error(), 10.0, TonePolarity::Nearer, false);
    }

    define_key! {
      on_error_container => error_palette;
      [tone, scheme] => if scheme.is_dark { 90.0 } else { 10.0 };
      [background, _scheme] => Self::error_container();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      background primary_fixed => primary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 40.0 } else { 90.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::primary_fixed(), Self::primary_fixed_dim(), 10.0, TonePolarity::Lighter, true);
    }

    define_key! {
      background primary_fixed_dim => primary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 30.0 } else { 80.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::primary_fixed(), Self::primary_fixed_dim(), 10.0, TonePolarity::Lighter, true);
    }

    define_key! {
      on_primary_fixed => primary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 100.0 } else { 10.0 };
      [background, _scheme] => Self::primary_fixed_dim();
      [second_background, _scheme] => Self::primary_fixed();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      on_primary_fixed_variant => primary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 90.0 } else { 30.0 };
      [background, _scheme] => Self::primary_fixed_dim();
      [second_background, _scheme] => Self::primary_fixed();
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 11.0 };
    }

    define_key! {
      background secondary_fixed => secondary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 80.0 } else { 90.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::secondary_fixed(), Self::secondary_fixed_dim(), 10.0, TonePolarity::Lighter, true);
    }

    define_key! {
      background secondary_fixed_dim => secondary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 70.0 } else { 80.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::secondary_fixed(), Self::secondary_fixed_dim(), 10.0, TonePolarity::Lighter, true);
    }

    define_key! {
      on_secondary_fixed => secondary_palette;
      [tone, _scheme] => 10.0;
      [background, _scheme] => Self::secondary_fixed_dim();
      [second_background, _scheme] => Self::secondary_fixed();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      on_secondary_fixed_variant => secondary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 25.0 } else { 30.0 };
      [background, _scheme] => Self::secondary_fixed_dim();
      [second_background, _scheme] => Self::secondary_fixed();
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 11.0 };
    }

    define_key! {
      background tertiary_fixed => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 40.0 } else { 90.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::tertiary_fixed(), Self::tertiary_fixed_dim(), 10.0, TonePolarity::Lighter, true);
    }

    define_key! {
      background tertiary_fixed_dim => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 30.0 } else { 80.0 };
      [background, scheme] => Self::highest_surface(scheme);
      [contrast_curve] => ContrastCurve { low: 1.0, normal: 1.0, medium: 3.0, high: 4.5 };
      [tone_delta_pair, _scheme] => ToneDeltaPair::new(Self::tertiary_fixed(), Self::tertiary_fixed_dim(), 10.0, TonePolarity::Lighter, true);
    }

    define_key! {
      on_tertiary_fixed => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 100.0 } else { 10.0 };
      [background, _scheme] => Self::tertiary_fixed_dim();
      [second_background, _scheme] => Self::tertiary_fixed();
      [contrast_curve] => ContrastCurve { low: 4.5, normal: 7.0, medium: 11.0, high: 21.0 };
    }

    define_key! {
      on_tertiary_fixed_variant => tertiary_palette;
      [tone, scheme] => if _is_monochrome(scheme) { 90.0 } else { 30.0 };
      [background, _scheme] => Self::tertiary_fixed_dim();
      [second_background, _scheme] => Self::tertiary_fixed();
      [contrast_curve] => ContrastCurve { low: 3.0, normal: 4.5, medium: 7.0, high: 11.0 };
    }

    fn _find_desired_chroma_by_tone(
        hue: f64,
        chroma: f64,
        tone: f64,
        by_decreasing_tone: bool,
    ) -> f64 {
        let mut answer = tone;

        let mut closest_to_chroma = Hct::from(hue, chroma, tone);

        if closest_to_chroma.get_chroma() < chroma {
            let mut chroma_peak = closest_to_chroma.get_chroma();

            while closest_to_chroma.get_chroma() < chroma {
                answer += if by_decreasing_tone { -1.0 } else { 1.0 };

                let potential_solution = Hct::from(hue, chroma, answer);

                if chroma_peak > potential_solution.get_chroma() {
                    break;
                }

                if (potential_solution.get_chroma() - chroma).abs() < 0.4 {
                    break;
                }

                let potential_delta = (potential_solution.get_chroma() - chroma).abs();
                let current_delta = (closest_to_chroma.get_chroma() - chroma).abs();

                if potential_delta < current_delta {
                    closest_to_chroma = potential_solution;
                }

                chroma_peak = chroma_peak.max(potential_solution.get_chroma());
            }
        }

        answer
    }
}
