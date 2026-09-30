pub use cmf::SchemeCmf;
pub use content::SchemeContent;
pub use expressive::SchemeExpressive;
pub use fidelity::SchemeFidelity;
pub use fruit_salad::SchemeFruitSalad;
pub use monochrome::SchemeMonochrome;
pub use neutral::SchemeNeutral;
pub use rainbow::SchemeRainbow;
pub use tonal_spot::SchemeTonalSpot;
pub use vibrant::SchemeVibrant;

macro_rules! scheme {
    ($(#[$meta:meta])* $name:ident => $variant:ident) => {
        $(#[$meta])*
        pub struct $name {
            pub scheme: $crate::dynamic_color::DynamicScheme,
        }

        impl $name {
            pub const VARIANT: $crate::dynamic_color::Variant = $crate::dynamic_color::Variant::$variant;

            /// Spec 2021 on phone (the original signature).
            pub fn new(source_color_hct: $crate::hct::Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
                Self::with_spec(
                    source_color_hct,
                    is_dark,
                    contrast_level,
                    $crate::dynamic_color::dynamic_scheme::DEFAULT_SPEC_VERSION,
                    $crate::dynamic_color::dynamic_scheme::DEFAULT_PLATFORM,
                )
            }

            /// Any spec version and platform. The effective spec version follows
            /// `DynamicScheme::fallback_spec_version`: only Neutral, TonalSpot,
            /// Vibrant and Expressive use 2025 (2026 is treated as 2025 for them).
            pub fn with_spec(
                source_color_hct: $crate::hct::Hct,
                is_dark: bool,
                contrast_level: Option<f64>,
                spec_version: $crate::dynamic_color::SpecVersion,
                platform: $crate::dynamic_color::Platform,
            ) -> Self {
                Self {
                    scheme: $crate::dynamic_color::DynamicScheme::from_spec(
                        source_color_hct,
                        Self::VARIANT,
                        is_dark,
                        contrast_level,
                        platform,
                        spec_version,
                    ),
                }
            }

            /// One palette as built for spec 2021 on phone, light mode, standard
            /// contrast (used by `TonalPalette::by_variant`).
            pub fn palette(source_color_hct: &$crate::hct::Hct, palette: &$crate::palette::Palette) -> $crate::palette::TonalPalette {
                $crate::dynamic_color::DynamicScheme::spec_palette(
                    Self::VARIANT,
                    *source_color_hct,
                    palette,
                    false,
                    $crate::dynamic_color::dynamic_scheme::DEFAULT_PLATFORM,
                    0.0,
                    $crate::dynamic_color::dynamic_scheme::DEFAULT_SPEC_VERSION,
                )
            }
        }
    };
}

mod cmf;
mod content;
mod expressive;
mod fidelity;
mod fruit_salad;
mod monochrome;
mod neutral;
mod rainbow;
mod tonal_spot;
mod vibrant;
