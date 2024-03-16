use crate::{
    blend::harmonize,
    color::Argb,
    dynamic_color::{DynamicScheme, Variant},
    palette::{CorePalette, Palette, TonalPalette},
    scheme::Scheme,
};

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
    /// @link <https://m3.material.io/styles/color/the-color-system/color-roles>
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

pub struct ThemeBuilder {
    source: Argb,
    variant: Variant,
    primary: Option<Argb>,
    secondary: Option<Argb>,
    tertiary: Option<Argb>,
    error: Option<Argb>,
    neutral: Option<Argb>,
    neutral_variant: Option<Argb>,
    custom_colors: Vec<CustomColor>,
}

impl ThemeBuilder {

    /// Creates a new theme builder with a custom source color.
    #[must_use]
    pub const fn with_source(source: Argb) -> Self {
        Self {
            source,
            variant: Variant::TonalSpot,
            primary: None,
            secondary: None,
            tertiary: None,
            error: None,
            neutral: None,
            neutral_variant: None,
            custom_colors: Vec::new(),
        }
    }

    #[must_use]
    pub const fn override_palette(mut self, palette: &Palette, color: Argb) -> Self {
        match palette {
            Palette::Primary => self.primary = Some(color),
            Palette::Secondary => self.secondary = Some(color),
            Palette::Tertiary => self.tertiary = Some(color),
            Palette::Error => self.error = Some(color),
            Palette::Neutral => self.neutral = Some(color),
            Palette::NeutralVariant => self.neutral_variant = Some(color),
        }

        self
    }

    #[must_use]
    pub fn custom_color<N: Into<String>>(mut self, name: N, value: Argb, blend: bool) -> Self {
        self.custom_colors.push(CustomColor {
            value,
            name: name.into(),
            blend,
        });

        self
    }

    pub fn build(self) -> Theme {
        let palette = CorePalette::of(self.source);

        let mut light = DynamicScheme::by_variant(self.source, &self.variant, false, None);
        let mut dark = DynamicScheme::by_variant(self.source, &self.variant, true, None);

        if let Some(color) = self.primary {
            let palette = TonalPalette::by_variant(&color.into(), &self.variant, &Palette::Primary);

            light.primary_palette = palette;
            dark.primary_palette = palette;
        }

        if let Some(color) = self.secondary {
            let palette =
                TonalPalette::by_variant(&color.into(), &self.variant, &Palette::Secondary);

            light.secondary_palette = palette;
            dark.secondary_palette = palette;
        }

        if let Some(color) = self.tertiary {
            let palette =
                TonalPalette::by_variant(&color.into(), &self.variant, &Palette::Tertiary);

            light.tertiary_palette = palette;
            dark.tertiary_palette = palette;
        }

        if let Some(color) = self.error {
            let palette = TonalPalette::by_variant(&color.into(), &self.variant, &Palette::Error);

            light.error_palette = palette;
            dark.error_palette = palette;
        }

        if let Some(color) = self.neutral {
            let palette = TonalPalette::by_variant(&color.into(), &self.variant, &Palette::Neutral);

            light.neutral_palette = palette;
            dark.neutral_palette = palette;
        }

        if let Some(color) = self.neutral_variant {
            let palette =
                TonalPalette::by_variant(&color.into(), &self.variant, &Palette::NeutralVariant);

            light.neutral_variant_palette = palette;
            dark.neutral_variant_palette = palette;
        }

        Theme {
            source: self.source,
            schemes: Schemes {
                light: light.into(),
                dark: dark.into(),
            },
            palettes: Palettes {
                primary: palette.primary,
                secondary: palette.secondary,
                tertiary: palette.tertiary,
                neutral: palette.neutral,
                neutral_variant: palette.neutral_variant,
                error: palette.error,
            },
            custom_colors: self
                .custom_colors
                .into_iter()
                .map(|c| CustomColorGroup::new(self.source, c))
                .collect(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Theme {
    pub source: Argb,
    pub schemes: Schemes,
    pub palettes: Palettes,
    pub custom_colors: Vec<CustomColorGroup>,
}
