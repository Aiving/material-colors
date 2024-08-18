#[allow(deprecated)]
pub use self::core::{CorePalette, CorePalettes};
pub use tonal::TonalPalette;

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
