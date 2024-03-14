use crate::{
    dynamic_color::{DynamicScheme, Variant},
    utils::math::sanitize_degrees_double,
    Hct, TonalPalette,
};

pub struct SchemeFruitSalad {
    pub scheme: DynamicScheme,
}

impl SchemeFruitSalad {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                None,
                Variant::FruitSalad,
                is_dark,
                contrast_level,
                TonalPalette::of(
                    sanitize_degrees_double(source_color_hct.get_hue() - 50.0),
                    48.0,
                ),
                TonalPalette::of(
                    sanitize_degrees_double(source_color_hct.get_hue() - 50.0),
                    36.0,
                ),
                TonalPalette::of(source_color_hct.get_hue(), 36.0),
                TonalPalette::of(source_color_hct.get_hue(), 10.0),
                TonalPalette::of(source_color_hct.get_hue(), 16.0),
                None,
            ),
        }
    }
}
