#![allow(clippy::too_many_arguments)]
#[cfg(feature = "serde")]
use serde::Serialize;

use core::{array::IntoIter, fmt};

use ahash::HashMap;

use crate::{dynamic_color::DynamicScheme, Argb};

pub mod variant;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Scheme {
    pub primary: Argb,
    pub on_primary: Argb,
    pub primary_container: Argb,
    pub on_primary_container: Argb,
    pub inverse_primary: Argb,
    pub primary_fixed: Argb,
    pub primary_fixed_dim: Argb,
    pub on_primary_fixed: Argb,
    pub on_primary_fixed_variant: Argb,
    pub secondary: Argb,
    pub on_secondary: Argb,
    pub secondary_container: Argb,
    pub on_secondary_container: Argb,
    pub secondary_fixed: Argb,
    pub secondary_fixed_dim: Argb,
    pub on_secondary_fixed: Argb,
    pub on_secondary_fixed_variant: Argb,
    pub tertiary: Argb,
    pub on_tertiary: Argb,
    pub tertiary_container: Argb,
    pub on_tertiary_container: Argb,
    pub tertiary_fixed: Argb,
    pub tertiary_fixed_dim: Argb,
    pub on_tertiary_fixed: Argb,
    pub on_tertiary_fixed_variant: Argb,
    pub error: Argb,
    pub on_error: Argb,
    pub error_container: Argb,
    pub on_error_container: Argb,
    pub surface_dim: Argb,
    pub surface: Argb,
    pub surface_bright: Argb,
    pub surface_container_lowest: Argb,
    pub surface_container_low: Argb,
    pub surface_container: Argb,
    pub surface_container_high: Argb,
    pub surface_container_highest: Argb,
    pub on_surface: Argb,
    pub on_surface_variant: Argb,
    pub outline: Argb,
    pub outline_variant: Argb,
    pub inverse_surface: Argb,
    pub inverse_on_surface: Argb,
    pub surface_variant: Argb,
    pub background: Argb,
    pub on_background: Argb,
    pub shadow: Argb,
    pub scrim: Argb,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scheme")
            .field("primary", &self.primary)
            .field("on_primary", &self.on_primary)
            .field("primary_container", &self.primary_container)
            .field("on_primary_container", &self.on_primary_container)
            .field("inverse_primary", &self.inverse_primary)
            .field("primary_fixed", &self.primary_fixed)
            .field("primary_fixed_dim", &self.primary_fixed_dim)
            .field("on_primary_fixed", &self.on_primary_fixed)
            .field("on_primary_fixed_variant", &self.on_primary_fixed_variant)
            .field("secondary", &self.secondary)
            .field("on_secondary", &self.on_secondary)
            .field("secondary_container", &self.secondary_container)
            .field("on_secondary_container", &self.on_secondary_container)
            .field("secondary_fixed", &self.secondary_fixed)
            .field("secondary_fixed_dim", &self.secondary_fixed_dim)
            .field("on_secondary_fixed", &self.on_secondary_fixed)
            .field(
                "on_secondary_fixed_variant",
                &self.on_secondary_fixed_variant,
            )
            .field("tertiary", &self.tertiary)
            .field("on_tertiary", &self.on_tertiary)
            .field("tertiary_container", &self.tertiary_container)
            .field("on_tertiary_container", &self.on_tertiary_container)
            .field("tertiary_fixed", &self.tertiary_fixed)
            .field("tertiary_fixed_dim", &self.tertiary_fixed_dim)
            .field("on_tertiary_fixed", &self.on_tertiary_fixed)
            .field("on_tertiary_fixed_variant", &self.on_tertiary_fixed_variant)
            .field("error", &self.error)
            .field("on_error", &self.on_error)
            .field("error_container", &self.error_container)
            .field("on_error_container", &self.on_error_container)
            .field("surface_dim", &self.surface_dim)
            .field("surface", &self.surface)
            .field("surface_bright", &self.surface_bright)
            .field("surface_container_lowest", &self.surface_container_lowest)
            .field("surface_container_low", &self.surface_container_low)
            .field("surface_container", &self.surface_container)
            .field("surface_container_high", &self.surface_container_high)
            .field("surface_container_highest", &self.surface_container_highest)
            .field("on_surface", &self.on_surface)
            .field("on_surface_variant", &self.on_surface_variant)
            .field("outline", &self.outline)
            .field("outline_variant", &self.outline_variant)
            .field("inverse_surface", &self.inverse_surface)
            .field("inverse_on_surface", &self.inverse_on_surface)
            .field("surface_variant", &self.surface_variant)
            .field("background", &self.background)
            .field("on_background", &self.on_background)
            .field("shadow", &self.shadow)
            .field("scrim", &self.scrim)
            .finish()
    }
}

impl Scheme {
    pub fn new(
        primary: Argb,
        on_primary: Argb,
        primary_container: Argb,
        on_primary_container: Argb,
        inverse_primary: Argb,
        primary_fixed: Argb,
        primary_fixed_dim: Argb,
        on_primary_fixed: Argb,
        on_primary_fixed_variant: Argb,
        secondary: Argb,
        on_secondary: Argb,
        secondary_container: Argb,
        on_secondary_container: Argb,
        secondary_fixed: Argb,
        secondary_fixed_dim: Argb,
        on_secondary_fixed: Argb,
        on_secondary_fixed_variant: Argb,
        tertiary: Argb,
        on_tertiary: Argb,
        tertiary_container: Argb,
        on_tertiary_container: Argb,
        tertiary_fixed: Argb,
        tertiary_fixed_dim: Argb,
        on_tertiary_fixed: Argb,
        on_tertiary_fixed_variant: Argb,
        error: Argb,
        on_error: Argb,
        error_container: Argb,
        on_error_container: Argb,
        surface_dim: Argb,
        surface: Argb,
        surface_bright: Argb,
        surface_container_lowest: Argb,
        surface_container_low: Argb,
        surface_container: Argb,
        surface_container_high: Argb,
        surface_container_highest: Argb,
        on_surface: Argb,
        on_surface_variant: Argb,
        outline: Argb,
        outline_variant: Argb,
        inverse_surface: Argb,
        inverse_on_surface: Argb,
        surface_variant: Argb,
        background: Argb,
        on_background: Argb,
        shadow: Argb,
        scrim: Argb,
    ) -> Self {
        Self {
            primary,
            on_primary,
            primary_container,
            on_primary_container,
            inverse_primary,
            primary_fixed,
            primary_fixed_dim,
            on_primary_fixed,
            on_primary_fixed_variant,
            secondary,
            on_secondary,
            secondary_container,
            on_secondary_container,
            secondary_fixed,
            secondary_fixed_dim,
            on_secondary_fixed,
            on_secondary_fixed_variant,
            tertiary,
            on_tertiary,
            tertiary_container,
            on_tertiary_container,
            tertiary_fixed,
            tertiary_fixed_dim,
            on_tertiary_fixed,
            on_tertiary_fixed_variant,
            error,
            on_error,
            error_container,
            on_error_container,
            surface_dim,
            surface,
            surface_bright,
            surface_container_lowest,
            surface_container_low,
            surface_container,
            surface_container_high,
            surface_container_highest,
            on_surface,
            on_surface_variant,
            outline,
            outline_variant,
            inverse_surface,
            inverse_on_surface,
            surface_variant,
            background,
            on_background,
            shadow,
            scrim,
        }
    }
}

impl From<DynamicScheme> for Scheme {
    fn from(scheme: DynamicScheme) -> Self {
        Self::new(
            scheme.primary(),
            scheme.on_primary(),
            scheme.primary_container(),
            scheme.on_primary_container(),
            scheme.inverse_primary(),
            scheme.primary_fixed(),
            scheme.primary_fixed_dim(),
            scheme.on_primary_fixed(),
            scheme.on_primary_fixed_variant(),
            scheme.secondary(),
            scheme.on_secondary(),
            scheme.secondary_container(),
            scheme.on_secondary_container(),
            scheme.secondary_fixed(),
            scheme.secondary_fixed_dim(),
            scheme.on_secondary_fixed(),
            scheme.on_secondary_fixed_variant(),
            scheme.tertiary(),
            scheme.on_tertiary(),
            scheme.tertiary_container(),
            scheme.on_tertiary_container(),
            scheme.tertiary_fixed(),
            scheme.tertiary_fixed_dim(),
            scheme.on_tertiary_fixed(),
            scheme.on_tertiary_fixed_variant(),
            scheme.error(),
            scheme.on_error(),
            scheme.error_container(),
            scheme.on_error_container(),
            scheme.surface_dim(),
            scheme.surface(),
            scheme.surface_bright(),
            scheme.surface_container_lowest(),
            scheme.surface_container_low(),
            scheme.surface_container(),
            scheme.surface_container_high(),
            scheme.surface_container_highest(),
            scheme.on_surface(),
            scheme.on_surface_variant(),
            scheme.outline(),
            scheme.outline_variant(),
            scheme.inverse_surface(),
            scheme.inverse_on_surface(),
            scheme.surface_variant(),
            scheme.background(),
            scheme.on_background(),
            scheme.shadow(),
            scheme.scrim(),
        )
    }
}

impl IntoIterator for Scheme {
    type Item = (String, Argb);

    type IntoIter = IntoIter<(String, Argb), 48>;

    fn into_iter(self) -> Self::IntoIter {
        [
            ("primary".into(), self.primary),
            ("on_primary".into(), self.on_primary),
            ("primary_container".into(), self.primary_container),
            ("on_primary_container".into(), self.on_primary_container),
            ("inverse_primary".into(), self.inverse_primary),
            ("primary_fixed".into(), self.primary_fixed),
            ("primary_fixed_dim".into(), self.primary_fixed_dim),
            ("on_primary_fixed".into(), self.on_primary_fixed),
            (
                "on_primary_fixed_variant".into(),
                self.on_primary_fixed_variant,
            ),
            ("secondary".into(), self.secondary),
            ("on_secondary".into(), self.on_secondary),
            ("secondary_container".into(), self.secondary_container),
            ("on_secondary_container".into(), self.on_secondary_container),
            ("secondary_fixed".into(), self.secondary_fixed),
            ("secondary_fixed_dim".into(), self.secondary_fixed_dim),
            ("on_secondary_fixed".into(), self.on_secondary_fixed),
            (
                "on_secondary_fixed_variant".into(),
                self.on_secondary_fixed_variant,
            ),
            ("tertiary".into(), self.tertiary),
            ("on_tertiary".into(), self.on_tertiary),
            ("tertiary_container".into(), self.tertiary_container),
            ("on_tertiary_container".into(), self.on_tertiary_container),
            ("tertiary_fixed".into(), self.tertiary_fixed),
            ("tertiary_fixed_dim".into(), self.tertiary_fixed_dim),
            ("on_tertiary_fixed".into(), self.on_tertiary_fixed),
            (
                "on_tertiary_fixed_variant".into(),
                self.on_tertiary_fixed_variant,
            ),
            ("error".into(), self.error),
            ("on_error".into(), self.on_error),
            ("error_container".into(), self.error_container),
            ("on_error_container".into(), self.on_error_container),
            ("surface_dim".into(), self.surface_dim),
            ("surface".into(), self.surface),
            ("surface_bright".into(), self.surface_bright),
            (
                "surface_container_lowest".into(),
                self.surface_container_lowest,
            ),
            ("surface_container_low".into(), self.surface_container_low),
            ("surface_container".into(), self.surface_container),
            ("surface_container_high".into(), self.surface_container_high),
            (
                "surface_container_highest".into(),
                self.surface_container_highest,
            ),
            ("on_surface".into(), self.on_surface),
            ("on_surface_variant".into(), self.on_surface_variant),
            ("outline".into(), self.outline),
            ("outline_variant".into(), self.outline_variant),
            ("inverse_surface".into(), self.inverse_surface),
            ("inverse_on_surface".into(), self.inverse_on_surface),
            ("surface_variant".into(), self.surface_variant),
            ("background".into(), self.background),
            ("on_background".into(), self.on_background),
            ("shadow".into(), self.shadow),
            ("scrim".into(), self.scrim),
        ]
        .into_iter()
    }
}

impl From<Scheme> for HashMap<String, String> {
    fn from(value: Scheme) -> Self {
        let map: HashMap<String, Argb> = HashMap::from_iter(value);

        map.into_iter()
            .map(|(key, value)| (key, value.as_hex()))
            .collect()
    }
}
