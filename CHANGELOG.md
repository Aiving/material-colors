# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 (Oct 1st, 2026)

- **breaking**: Replace `struct:Argb` with `struct:Rgb` across the whole API; `func:from_u32` now takes `0xRRGGBB`, and `func:as_argb_u32` returns an opaque ARGB value
- **breaking**: Replace `func:to_hex` and `func:to_hex_with_pound` with `func:as_hex`, which returns `struct:HexFmt` (implements `Display`) instead of a `String`
- **breaking**: Replace the `image` feature with `quantize`: `mod:image` now only provides `func:extract_color` for already decoded pixels, and `struct:ImageReader`, `struct:Image`, `trait:AsPixels` and the `FilterType` re-export are removed
- **breaking**: `mod:quantize` and `mod:score` now require the `quantize` feature, and the submodules of `mod:quantize` are private (their types are still re-exported from `mod:quantize`)
- **breaking**: Rewrite `mod:dynamic_color`:
  - `struct:DynamicColor` is now `enum:DynamicColor`, either a Material `enum:Role` or a `trait:CustomColor`; `func:new` and the `name` field are removed, `func:get_argb` is replaced with `func:get_rgb`, and `func:get_hct` takes `&self`
  - `func:new` in `struct:DynamicScheme` takes a `struct:Hct` source color, and the `source_color_argb` field is removed
  - `enum:TonePolarity` variants `Nearer` and `Farther` are replaced with `RelativeDarker`, `RelativeLighter` and `enum:DeltaConstraint`
  - the `subject` and `basis` fields of `struct:ToneDeltaPair` are renamed to `role_a` and `role_b`, and `func:new` takes 4 arguments, with `func:with_stay_together` and `func:with_constraint` for the rest
  - `CONTENT_ACCENT_TONE_DELTA` is removed from `struct:MaterialDynamicColors`
  - `enum:Variant` gets the `Cmf` variant
- **breaking**: Remove custom colors from `struct:ThemeBuilder` and `struct:Theme`, and the `name` field from `struct:CustomColor`; use `func:new` in `struct:CustomColorGroup` instead
- **breaking**: Remove `struct:SchemeFromPalette` and `From<Scheme> for Map<String, String>`; `IntoIterator` for `struct:Scheme` now yields `(&'static str, Rgb)`
- **breaking**: Rename the `j`, `q`, `m` and `s` fields of `struct:Cam16` to `lightness`, `brightness`, `colorfulness` and `saturation`, rename `func:fromi32_in_viewing_conditions` to `func:from_rgb_in_viewing_conditions`, and make `func:from_xyz_in_viewing_conditions` take `struct:Xyz`
- **breaking**: Rename `func:solve_to_argb` in `struct:HctSolver` to `func:solve_to_rgb`
- **breaking**: Remove `func:standard` from `struct:ViewingConditions` (use `func:s_rgb`)
- **breaking**: `func:lighter` and `func:darker` in `mod:contrast` now return `Option<f64>` instead of `-1.0`
- **breaking**: `func:score` in `struct:Score` now takes `desired` as `Option<usize>`
- **breaking**: Rework `struct:TemperatureCache` to avoid allocations: `func:analogous` returns `[Hct; 5]` (see `func:analogous_generic` for other sizes), `func:relative_temperature` takes a temperature, and `func:coldest`, `func:warmest`, `func:hcts_by_hue`, `func:hcts_by_temp`, `func:temps_by_hct` and `func:input_relative_temperature` are removed
- **breaking**: Remove `func:create_key_color` from `struct:TonalPalette` (use `struct:KeyColor`) and `SIZE` from `struct:CorePalette`
- **breaking**: Rename the serialized fields of `struct:Hct` from `_hue`, `_chroma`, `_tone` and `_argb` to `hue`, `chroma`, `tone` and `rgb`
- **added**: Add `struct:CorePalettes`
- **added**: Add unit tests for `struct:SchemeContent`
- **added**: Add unit tests for `struct:SchemeExpressive`
- **added**: Add unit tests for `struct:SchemeFidelity`
- **added**: Add unit tests for `struct:SchemeFruitSalad`
- **added**: Add unit tests for `struct:SchemeMonochrome`
- **added**: Add unit tests for `struct:SchemeNeutral`
- **added**: Add unit tests for `struct:SchemeRainbow`
- **added**: Add unit tests for `struct:SchemeVibrant`
- **added**: Add color spec versions 2021, 2025 and 2026 (`enum:SpecVersion`, `trait:ColorSpec`, `struct:ColorSpec2021`, `struct:ColorSpec2025`, `struct:ColorSpec2026`, `func:color_spec`) and the phone/watch `enum:Platform`
- **added**: Add `struct:SchemeCmf`
- **added**: Add `func:with_spec` and `VARIANT` to every scheme, for building schemes of any spec version and platform
- **added**: Add to `struct:DynamicScheme`:
  - `platform`, `spec_version` and `secondary_source_color_hct` fields
  - `func:from_spec`, `func:spec_palette` and `func:fallback_spec_version`
  - `func:with_spec_version`, `func:with_platform`, `func:with_secondary_source_color_hct` and `func:with_mode`
  - `func:palette`, `func:secondary_source_or_primary`, `func:resolver`, `func:get_hct`, `func:get_rgb` and `func:get_piecewise_value`
  - `primary_dim`, `secondary_dim`, `tertiary_dim`, `error_dim` and `error_palette_key_color` colors
- **added**: Add `primary_dim`, `secondary_dim`, `tertiary_dim`, `error_dim`, `error_palette_key_color`, `func:highest_surface` and `ALL` to `struct:MaterialDynamicColors`
- **added**: Add `enum:Role`, `struct:RoleMap`, `struct:ColorDefinition`, `enum:Entry`, `type:SpecTable` and `enum:SchemePalette`
- **added**: Add `trait:CustomColor` and `struct:ArgbColor` for user-defined dynamic colors
- **added**: Add `struct:SchemeResolver`, `struct:ToneCache` and `struct:Context` for resolving many colors of one scheme with a shared memo
- **added**: Add `trait:FloatBackend`, `macro:set_float_backend` and `struct:Libm` for custom floating-point backends in `no_std` builds
- **added**: Add `struct:KeyColor`
- **added**: Add `func:as_u32` and `func:as_argb_u32` to `struct:Rgb`, and implement `Hash`, `PartialEq`, `Eq`, `PartialOrd` and `Ord` for it
- **added**: Add `func:is_blue`, `func:is_yellow` and `func:is_cyan` to `struct:Hct`, and implement `Default` for it
- **added**: Add `trait:FromRef`
- **added**: Add `spec_version` and `platform` fields and functions to `struct:ThemeBuilder`
- **added**: Implement `Deserialize` for every type that implements `Serialize`, and `Clone` for the theme and scheme types
- **added**: Add the `alloc` and `quantize` features
- **added**: Add `image` and `theme` examples
- **added**: Add tests for:
  - alias resolution, memoization, spec fallback and the 2025 error hue in `mod:dynamic_color`
  - hue rotation in `struct:DynamicScheme`
  - key colors in `struct:TonalPalette`
  - `struct:SchemeCmf`
  - the rounding defaults of `trait:FloatBackend`
- **changed**: Deprecate `struct:CorePalette` & `palettes` field in `struct:Theme`
- **changed**: Update `struct:MaterialDynamicColors` to use the expressive on-colors spec
- **changed**: Update `struct:TonalPalette` to use new key color algorithm
- **changed**: Resolve dynamic colors through the color spec of the scheme, with the definitions of each spec version merged at compile time and tones memoized per scheme
- **changed**: Build the palettes of every scheme from `trait:ColorSpec`
- **changed**: Only the `quantize` feature needs an allocator; everything else works with plain `core`
- **changed**: Floating-point functions in `no_std` builds now come from a registered backend: the `libm` feature registers `struct:Libm`, and `no_std` builds no longer require it
- **changed**: Implement `core::error::Error` for `enum:Error` without the `std` feature
- **changed**: `func:get_rotated_hue` in `struct:DynamicScheme` now takes `n + 1` breakpoints for `n` rotations and no longer panics on slices of different lengths
- **changed**: Make `func:ratio_of_tones`, `func:y_from_lstar`, `func:next_range`, the functions of `mod:utils/math` and several `struct:DynamicColor` functions const
- **changed**: Move unit tests to `tests/`, except for private items (quantizers, float backend defaults)
- **changed**: Move `tests/image.rs` and `tests/theme.rs` to examples
- **changed**: Update dependencies
  - **`ahash`**: `v0.8.11` -> `v0.8.12` (now optional)
  - **`indexmap`**: `v2.3.0` -> `v2.14.2` (now optional)
  - **`serde`**: `v1.0.205` -> `v1.0.229` (without default features)
  - **`libm`**: `v0.2.8` -> `v0.2.16`
- **changed**: Remove `image` from dependencies
- **changed**: Update dev-dependencies
  - **`float-cmp`**: `v0.9` -> `v0.10.0`
  - **`reqwest`**: `v0.12.5` -> `v0.13.5`
  - **`tokio`**: `v1.39.2` -> `v1.53.1`
- **changed**: Add `image` (`v0.25.10`) to dev-dependencies
- **changed**: Move to Rust 2024 edition and bump MSRV to 1.97.0
- **changed**: Allow `clippy:unnecessary_wraps` lint, and stop allowing `clippy:negative_feature_names`
- **changed**: Update `README`: features, examples, color extraction advice and `no_std` status
- **fixed**: Fix `func:neutral_variant_palette_key_color` in `struct:DynamicScheme` returning the neutral key color
- **fixed**: Fix `func:get_rotated_hue` in `struct:DynamicScheme` not rotating hues that are exactly on a breakpoint
- **fixed**: Fix `func:lighter` and `func:darker` in `mod:contrast` not rejecting out-of-range luminance
- **fixed**: Fix the neutral hue of `struct:SchemeExpressive` not wrapping around 360 degrees
- **fixed**: Fix documentation of `struct:TonalPalette`

## 0.4.2 (Apr 8th, 2024)

- **fixed**: Fix markdown in `README`

## 0.4.1 (Apr 8th, 2024)

- **added**: Add badges to the `README`
- **added**: Add the Features, Status of no-std & License headings to the `README`
- **added**: Add compile errors for `std` with `libm` & `no_std` without `libm`
- **fixed**: Fix HEX color example in `README`
- **changed**: Move to dual-license to be compatible with the Rust project
- **changed**: Make all math operations on floating point numbers use their equivalent in `trait:FloatExt` for compatibility with `no_std` environments

## 0.4.0 (Jul 29th, 2024)

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

## 0.2.1 (Mar 14th, 2024)

- **breaking:** Introduce new structures in `mod:utils/color` as a replacement for the color type aliases
- **breaking:** Merge `mod:utils/string` into `mod:utils/color`
- **breaking:** Rewrite `mod:utils/image` for using `image` crate
- **breaking:** `mod:utils/image` is now optional and available with the `image` feature
- **added:** Add support for the serde with the `serde` feature
- **changed:** Update dependencies
- **changed:** Update tests
- **fixed:** Fix `func:secondary_container` in `struct:MaterialDynamicColors`

## 0.1.6 (Feb 2nd, 2024)

- **added:** Implement `trait:IntoIterator` for the `struct:Scheme`
- **fixed:** Fix incorrect proportion calculation in `mod:score`
- **fixed:** Update the description of `struct:Random` in `mod:utils/random`

## 0.1.5 (Feb 2nd, 2024)

- **added:** Add partial LCG algorithm implementation in `mod:utils/random`
- **changed:** Remove small F.A.Q. from `README`
- **changed:** Now functions in `mod:utils/color` and `mod:utils/string` require only references to colors
- **fixed:** Fix how cluster indices fill in `struct:QuantizerWu`
- **fixed:** Fix sorting of scored colors in `mod:score`

## 0.1.4 (Jan 30th, 2024)

- **added:** Add a test for the image color extraction
- **changed:** Cleanup code in quantizers
- **changed:** Remove random color filling from `struct:QuantizerWsmeans`
- **changed:** Update constants in `struct:QuantizerWu`
- **changed:** Update `func:get_index` in `struct:QuantizerWu`
- **changed:** Add a warning for the image color extraction example in `README`
- **fixed:** Update broken example of extracting colors from image in `README`
- **fixed:** Replace`struct:HashMap` with `struct:IndexMap`
- **fixed:** Fix different palettes for the same image

## 0.1.3 (Jan 25th, 2024)

- **fixed:** Fix `func:sanitize_degrees_int` in `mod:utils/math`

## 0.1.2 (Jan 30th, 2024)

- **breaking:** `func:source_color_from_image` now accepts an ARGB color array instead of a byte array
- **fixed:** Change visibility in `struct:QuantizerWsmeans`
- **fixed:** Fix some issues with integers in `struct:QuantizerWu` and `mod:score`

## 0.1.1 (Dec 31st, 2023)

- **added:** Add a small F.A.Q. about std to `README`
- **changed:** Update for visibility for a lot of mods, structs, functions, etc.
- **fixed:** Fix incorrect code highlighting of examples in `README`
- **fixed:** Update to a valid license
