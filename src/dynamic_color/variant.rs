/// Set of themes supported by Dynamic Color.
/// Instantiate the corresponding subclass, ex. SchemeTonalSpot, to create
/// colors corresponding to the theme.
#[derive(PartialEq, Eq, PartialOrd, Clone)]
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
