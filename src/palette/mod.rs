pub use self::core::CorePalette;
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
