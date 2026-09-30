/// Set of themes supported by Dynamic Color.
/// Build a scheme with the corresponding type, e.g. [`SchemeTonalSpot`], to get
/// colors for the theme.
///
/// [`SchemeTonalSpot`]: crate::scheme::variant::SchemeTonalSpot
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Variant {
    Monochrome,
    Neutral,
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Rainbow,
    FruitSalad,
    /// Two-source-color theme; only supported by spec 2026.
    Cmf,
}
