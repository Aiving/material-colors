/// Set of themes supported by Dynamic Color.
/// Instantiate the corresponding subclass, ex. [`SchemeTonalSpot`], to create
/// colors corresponding to the theme.
///
/// [`SchemeTonalSpot`]: crate::scheme::variant::SchemeTonalSpot
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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
}
