scheme! {
    /// A scheme that places the source color in `primary_container`.
    ///
    /// `primary_container` is the source color, adjusted for color relativity. It maintains constant
    /// appearance in light mode and dark mode. This adds ~5 tone in light mode, and subtracts ~5 tone in
    /// dark mode.
    ///
    /// `tertiary_container` is the complement to the source color, using `TemperatureCache`. It also
    /// maintains constant appearance.
    SchemeFidelity => Fidelity
}
