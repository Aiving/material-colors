use crate::{blend::harmonize, Argb, CorePalette, Scheme, SchemeTonalSpot, TonalPalette};

#[cfg(feature = "serde")]
use serde::Serialize;

/// Custom color used to pair with a theme
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CustomColor {
    pub value: Argb,
    pub name: String,
    pub blend: bool,
}

/// Color group
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ColorGroup {
    pub color: Argb,
    pub on_color: Argb,
    pub color_container: Argb,
    pub on_color_container: Argb,
}

/// Custom Color Group
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CustomColorGroup {
    pub color: CustomColor,
    pub value: Argb,
    pub light: ColorGroup,
    pub dark: ColorGroup,
}

impl CustomColorGroup {
    /// Generate custom color group from source and target color
    ///

    ///
    /// @link https://m3.material.io/styles/color/the-color-system/color-roles
    fn new(source: Argb, color: CustomColor) -> Self {
        let mut value = color.value;

        let from = value;
        let to = source;

        if color.blend {
            value = harmonize(from, to);
        }

        let palette = CorePalette::of(value);
        let tones = palette.primary;

        Self {
            color,
            value,
            light: ColorGroup {
                color: tones.tone(40),
                on_color: tones.tone(100),
                color_container: tones.tone(90),
                on_color_container: tones.tone(10),
            },
            dark: ColorGroup {
                color: tones.tone(80),
                on_color: tones.tone(20),
                color_container: tones.tone(30),
                on_color_container: tones.tone(90),
            },
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Schemes {
    pub light: Scheme,
    pub dark: Scheme,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Palettes {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
    pub error: TonalPalette,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Theme {
    pub source: Argb,
    pub schemes: Schemes,
    pub palettes: Palettes,
    pub custom_colors: Vec<CustomColorGroup>,
}

impl Theme {
    /// Generatse a theme from a source color
    pub fn from_source_color(source: Argb, custom_colors: Vec<CustomColor>) -> Self {
        let palette = CorePalette::of(source);

        Self {
            source,
            schemes: Schemes {
                light: SchemeTonalSpot::new(source.into(), false, None)
                    .scheme
                    .into(),
                dark: SchemeTonalSpot::new(source.into(), true, None)
                    .scheme
                    .into(),
            },
            palettes: Palettes {
                primary: palette.primary,
                secondary: palette.secondary,
                tertiary: palette.tertiary,
                neutral: palette.neutral,
                neutral_variant: palette.neutral_variant,
                error: palette.error,
            },
            custom_colors: custom_colors
                .into_iter()
                .map(|c| CustomColorGroup::new(source, c))
                .collect(),
        }
    }
}
