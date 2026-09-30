scheme! {
    /// A scheme that places the source color in `primary_container`.
    ///
    /// `primary_container` is the source color, adjusted for color relativity. It maintains constant
    /// appearance in light mode and dark mode. This adds ~5 tone in light mode, and subtracts ~5 tone in
    /// dark mode.
    ///
    /// `tertiary_container` is an analogous color, specifically, the analog of a color wheel divided into
    /// 6, and the precise analog is the one found by increasing hue. This is a scientifically grounded
    /// equivalent to rotating hue clockwise by 60 degrees. It also maintains constant appearance.
    SchemeContent => Content
}
