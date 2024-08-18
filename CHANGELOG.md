# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- **added**: Add `struct:CorePalettes`
- **added**: Add unit tests for `struct:SchemeContent`
- **added**: Add unit tests for `struct:SchemeExpressive`
- **added**: Add unit tests for `struct:SchemeFidelity`
- **added**: Add unit tests for `struct:SchemeFruitSalad`
- **added**: Add unit tests for `struct:SchemeMonochrome`
- **added**: Add unit tests for `struct:SchemeNeutral`
- **added**: Add unit tests for `struct:SchemeRainbow`
- **added**: Add unit tests for `struct:SchemeVibrant`
- **changed**: Deprecate `struct:CorePalette`
- **changed**: Update `struct:MaterialDynamicColors` to use the expressive on-colors spec
- **changed**: Update `struct:TonalPalette` to use new key color algorithm

## 0.4.2 (Apr 8th, 2024)

- **fixed**: Fix markdown in `README`

## 0.4.1 (Apr 8th, 2024)

- **added**: Add badges to the `README`
- **added**: Add the Features, Status of no-std & License headings to the `README`
- **added**: Add compile errors for `std` with `libm` & `no_std` without `libm`
- **fixed**: Fix HEX color example in `README`
- **changed**: Move to dual-license to be compatible with the Rust project
- **changed**: Make all math operations on floating point numbers use their equivalent in `trait:FloatExt` for compatibility with `no_std` environments

## 0.4.0 (July 29th, 2024)

- **added**: Add support for `no_std` environments
- **added**: Add `surface_tint` color
- **added**: Add MSRV in `README`
- **changed**: Code cleanup, imports organizing
- **changed**: Now the use of `image` feature requires `std` feature
- **changed**: Update dependencies
  - **`image`**: `v0.25.1` -> `v0.25.2`
  - **`serde`**: `v1.0.203` -> `v1.0.204`
- **changed**: Update dev-dependencies
  - **`reqwest`**: `v0.12.4` -> `v0.12.5`
  - **`tokio`**: `v1.37.0` -> `v1.39.2`
- **changed**: Remove `image` from dev-dependencies
- **changed**: Update tests

## 0.3.3 (May 25th, 2024)

- **added**: Add tests for:
  - `mod:blend`
  - `mod:contrast`
  - `mod:dislike`
  - `mod:dynamic_color/dynamic_scheme`
  - `mod:dynamic_color`
  - `mod:hct/cam16`
  - `mod:hct`
  - `mod:hct/viewing_conditions`
  - `mod:palette/core`
  - `mod:palette/tonal`
  - `mod:quantize/quantizer_celebi`
  - `mod:quantize/quantizer_wsmeans`
  - `mod:quantize/quantizer_wu`
  - `mod:scheme/content`
  - `mod:utils/math`

- **changed**: Allow `clippy:while_float` lint
- **fixed**: Fix stack overflow in `struct:QuantizerWu` creation
- **fixed**: Fix `func:on_secondary_container` in `struct:MaterialDynamicColors`
- **fixed**: Fix `func:critical_plane_below` and `func:critical_plane_above` in `struct:HctSolver`
- **fixed**: Fix `clippy:doc_lazy_continuation` lint in `mod:hct/solver` and `mod:palette/tonal`

## 0.3.2 (May 8th, 2024)

- **added**: Add tests for `mod:color`
- **changed**: Change type of `arg:max_colors` in `trait:Quantizer`'s `func:quantize` from `i32` to `usize`;
- **changed**: Rewrite `struct:QuantierWsmeans` in `mod:quantize/quantizer_wsmeans`;
- **changed**: Rewrite `struct:QuantierWu` in `mod:quantize/quantizer_wu`;
- **fixed**: Fix color extraction from images (FULLY)

## 0.3.1 (May 4th, 2024)

- **fixed**: Fix color extraction from images (PARTIALLY)

## 0.3.0 (May 1st, 2024)

- **breaking:** Move the functions within `mod:theme` into structs for a more idiomatic approach
- **breaking:** Update exports
- **breaking:** Apply more strictly Clippy lints
- **breaking:** Replace `func:from_source_color` in `struct:Theme` with `struct:ThemeBuilder`
- **breaking:** Replace `struct:ParseRgbError` with `enum:Error` `ParseRGB` variant in `mod:error`
- **breaking:** Replace `func:as_hex` in `struct:Argb` with `func:to_hex` and `func:to_hex_with_pound`
- **breaking:** Rename `mod:palettes` to `mod:palette`
- **added:** Add `CHANGELOG`
- **added:** Add documentation above `struct:Argb`
- **added:** Add `check` job to `actions:CI.yml`
- **added:** Implement tests from C++ for `mod:scheme`, `mod:score` and `mod:temperature`
- **changed:** Rename `actions:tests.yml` to `actions:CI.yml`
- **changed:** Rename `build` job to `test` in `actions:CI.yml`
- **changed:** Remove unnecessary comments about warnings count from the `README`
- **changed:** Update tests
- **changed:** Update examples in `README`
- **changed:** Update dependencies
- **fixed:** Restore `trait:Serialize` on `struct:Argb` and implement it for a new color types

## 0.2.1 (March 14th, 2024)

- **breaking:** Introduce new structures in `mod:utils/color` as a replacement for the color type aliases
- **breaking:** Merge `mod:utils/string` into `mod:utils/color`
- **breaking:** Rewrite `mod:utils/image` for using `image` crate
- **breaking:** `mod:utils/image` is now optional and available with the `image` feature
- **added:** Add support for the serde with the `serde` feature
- **changed:** Update dependencies
- **changed:** Update tests
- **fixed:** Fix `func:secondary_container` in `struct:MaterialDynamicColors`

## 0.1.6 (February 2nd, 2024)

- **added:** Implement `trait:IntoIterator` for the `struct:Scheme`
- **fixed:** Fix incorrect proportion calcuation in `mod:score`
- **fixed:** Update the description of `struct:Random` in `mod:utils/random`

## 0.1.5 (February 2nd, 2024)

- **added:** Add partial LCG algorithm implementation in `mod:utils/random`
- **changed:** Remove small F.A.Q. from `README`
- **changed:** Now functions in `mod:utils/color` and `mod:utils/string` require only references to colors
- **fixed:** Fix how cluster indicies fill in `struct:QuantizerWu`
- **fixed:** Fix sorting of scored colors in `mod:score`

## 0.1.4 (January 30th, 2024)

- **added:** Add a test for the image color extraction
- **changed:** Cleanup code in quantizers
- **changed:** Remove random color filling from `struct:QuantizerWsmeans`
- **changed:** Update constants in `struct:QuantizerWu`
- **changed:** Update `func:get_index` in `struct:QuantizerWu`
- **changed:** Add a warning for the image color extraction example in `README`
- **fixed:** Update broken example of extracting colors from image in `README`
- **fixed:** Replace`struct:HashMap` with `struct:IndexMap`
- **fixed:** Fix different palettes for the same image

## 0.1.3 (January 25th, 2024)

- **fixed:** Fix `func:sanitize_degrees_int` in `mod:utils/math`

## 0.1.2 (January 30th, 2024)

- **breaking:** `func:source_color_from_image` now accepts an ARGB color array instead of a byte array
- **fixed:** Change visibility in `struct:QuantizerWsmeans`
- **fixed:** Fix some issues with integers in `struct:QuantizerWu` and `mod:score`

## 0.1.1 (December 31st, 2023)

- **added:** Add a small F.A.Q. about std to `README`
- **changed:** Update for visibility for a lot of mods, structs, functions, etc.
- **fixed:** Fix incorrect code highlighting of examples in `README`
- **fixed:** Update to a valid license
