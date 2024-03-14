use crate::{
    dynamic_color::{DynamicScheme, Variant},
    Hct, TonalPalette,
};

pub struct SchemeNeutral {
    pub scheme: DynamicScheme,
}

impl SchemeNeutral {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct.into(),
                None,
                Variant::Neutral,
                is_dark,
                contrast_level,
                TonalPalette::of(source_color_hct.get_hue(), 12.0),
                TonalPalette::of(source_color_hct.get_hue(), 8.0),
                TonalPalette::of(source_color_hct.get_hue(), 16.0),
                TonalPalette::of(source_color_hct.get_hue(), 2.0),
                TonalPalette::of(source_color_hct.get_hue(), 2.0),
                None,
            ),
        }
    }
}
