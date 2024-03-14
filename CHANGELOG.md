# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- **breaking:** Move the functions within `mod:utils/theme` into structs for a more idiomatic approach ([#11])
- **breaking**: Update exports ([#13])
- **added:** Add `CHANGELOG` ([#10])
- **changed:** Remove unnecessary comment about warnings count from the `README`
- **fixed:** Update `README` and `tests:image` for renamed color functions ([#12])
- **fixed:** Restore `trait:Serialize` on `struct:Argb` and implements it for new colour
types

## 0.2.1 (March 14th, 2024)

- **breaking:** Introduce new structures in `mod:utils/color` as a replacement for the color type aliases ([#35e58a0])
- **breaking:** Merge `mod:utils/string` into `mod:utils/color` ([#35e58a0])
- **breaking:** Rewrite `mod:utils/image` for using `image` crate ([#713b178])
- **breaking:** `mod:utils/image` now optional and available with `image` feature ([#713b178])
- **added:** Add support for the serde with optional `serde` feature ([#8])
- **changed:** Update dependencies
- **changed:** Update tests
- **fixed:** Fix `secondary_container` color in dynamic colors ([#6])

## 0.1.6 (February 2nd, 2024)

- **added:** Implement `trait:IntoIterator` for the `struct:Scheme`
- **fixed:** Fix incorrect proportion calcuation in `mod:score`
- **fixed:** Update description of `struct:Random` in `utils/random.rs`

## 0.1.5 (February 2nd, 2024)

- **added:** Add partial LCG algorithm implementation in `mod:utils/random`
- **changed:** Remove small F.A.Q from `README.md`
- **changed:** Now functions in `mod:utils/color` and `mod:utils/string` requires only references to colors
- **fixed:** Fix how cluster indicies filling in `struct:QuantizerWu`
- **fixed:** Fix sorting of scored colors in `mod:score`

## 0.1.4 (January 30th, 2024)

- **added:** Add test for the image color extraction
- **changed:** Cleanup code in quantizers
- **changed:** Remove random color filling from `struct:QuantizerWsmeans`
- **changed:** Update constants in `struct:QuantizerWu`
- **changed:** Update `func:get_index` in `struct:QuantizerWu`
- **changed:** Add warning for the image color extraction example in `README`
- **fixed:** Update broken example of extracting colors from image in `README`
- **fixed:** Replace`struct:HashMap` with `struct:IndexMap`
- **fixed:** Fix different palettes for the same image

## 0.1.3 (January 25th, 2024)

- **fixed:** Fix `func:sanitize_degrees_int` in `mod:utils/math`

## 0.1.2 (January 30th, 2024)

- **breaking:** `func:source_color_from_image` now accepts ARGB color array instead of byte array
- **fixed:** Change visibility in `struct:QuantizerWsmeans`
- **fixed:** Fix some issue with integers in `struct:QuantizerWu` and `mod:score`

## 0.1.1 (Decemeber 31st, 2023)

- **added:** Add small F.A.Q about std to `README`
- **changed:** Update for visibility for a lot of mods, structs, functions, etc.
- **fixed:** Fix incorrect code highlighting of examples in `README`
- **fixed:** Update to valid license
