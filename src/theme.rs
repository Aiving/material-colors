#[allow(deprecated)]
use crate::{
    blend::harmonize,
    color::Argb,
    dynamic_color::{DynamicScheme, Variant},
    palette::{CorePalette, Palette, TonalPalette},
    scheme::Scheme,
};
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "serde")]
use serde::Serialize;
#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

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

        if color.blend {
            value = harmonize(value, source);
        }

        #[allow(deprecated)]
        let palette = CorePalette::of(value);
        #[allow(deprecated)]
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
    color_match: bool,
    primary: Option<Argb>,
    secondary: Option<Argb>,
    tertiary: Option<Argb>,
    error: Option<Argb>,
    neutral: Option<Argb>,
    neutral_variant: Option<Argb>,
    custom_colors: Vec<CustomColor>,
}

impl ThemeBuilder {
    /// Creates a theme builder with a custom source color.
    #[must_use]
    pub const fn with_source(source: Argb) -> Self {
        Self {
            source,
            variant: Variant::TonalSpot,
            color_match: false,
            primary: None,
            secondary: None,
            tertiary: None,
            error: None,
            neutral: None,
            neutral_variant: None,
            custom_colors: Vec::new(),
        }
    }

    /// Sets the theme variant.
    #[must_use]
    pub const fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;

        self
    }

    /// Sets the primary color of the theme.
    #[must_use]
    pub const fn primary(mut self, color: Argb) -> Self {
        self.primary = Some(color);

        self
    }

    /// Sets the secondary color of the theme.
    #[must_use]
    pub const fn secondary(mut self, color: Argb) -> Self {
        self.secondary = Some(color);

        self
    }

    /// Sets the tertiary color of the theme.
    #[must_use]
    pub const fn tertiary(mut self, color: Argb) -> Self {
        self.tertiary = Some(color);

        self
    }

    /// Sets the error color of the theme.
    #[must_use]
    pub const fn error(mut self, color: Argb) -> Self {
        self.error = Some(color);

        self
    }

    /// Sets the neutral color, used for background and surfaces.
    #[must_use]
    pub const fn neutral(mut self, color: Argb) -> Self {
        self.neutral = Some(color);

        self
    }

    /// Sets the neutral variant color, used for for medium emphasis and variants.
    #[must_use]
    pub const fn neutral_variant(mut self, color: Argb) -> Self {
        self.neutral_variant = Some(color);

        self
    }

    /// Sets the custom colors, used as complementary tones.
    ///
    /// Custom colors are also known as extended colors.
    #[must_use]
    pub fn custom_colors(mut self, custom_colors: Vec<CustomColor>) -> Self {
        self.custom_colors = custom_colors;

        self
    }

    #[must_use]
    pub const fn color_match(mut self, enabled: bool) -> Self {
        self.color_match = enabled;

        self
    }

    #[must_use]
    pub fn build(mut self) -> Theme {
        #[allow(deprecated)]
        let palette = CorePalette::of(self.source);

        if self.color_match {
            self.variant = Variant::Fidelity;
        }

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
            #[allow(deprecated)]
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
                .map(|color| CustomColorGroup::new(self.source, color))
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
