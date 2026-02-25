pub use tonal::TonalPalette;

#[allow(deprecated)]
pub use self::core::{CorePalette, CorePalettes};

mod core;
mod tonal;

#[derive(PartialEq, Eq)]
pub enum Palette {
    Primary,
    Secondary,
    Tertiary,
    Error,
    Neutral,
    NeutralVariant,
}
