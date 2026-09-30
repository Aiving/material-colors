# Material colors

[![crates.io: material-colors](https://img.shields.io/crates/v/material-colors.svg?style=for-the-badge)](https://crates.io/crates/material-colors)
[![Documentation](https://img.shields.io/docsrs/material-colors.svg?style=for-the-badge)](https://docs.rs/material-colors)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Aiving/material-colors/CI.yml.svg?style=for-the-badge)](https://github.com/Aiving/material-colors/actions)
[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT_or_Apache_2.0-634f7d.svg?style=for-the-badge)](LICENSE-APACHE)

An unofficial port of the `material-color-utilities` library for creating Material You themes and color schemes.

## Features

- `std`: enabled by default, uses the floating-point functions of std; enables `alloc`
- `libm`: registers the floating-point backend based on [`libm`](https://github.com/rust-lang/libm) for builds without `std` (or register your own, see below); can't be combined with `std`
- `quantize`: adds support for extracting colors from images (`quantize`, `score`, `image::extract_color`); pulls in `alloc`, `indexmap` and `ahash`
- `serde`: adds support for JSON serialization of themes and color schemes

## Examples

From HEX color:

```rust
use material_colors::{color::Rgb, theme::ThemeBuilder};

let theme = ThemeBuilder::with_source(Rgb::from_u32(0xaae5a4)).build();

// Do whatever you want...
```

From image:

> ⚠️ Before obtaining an array of RGB pixels for the image, **it is recommended** (but not necessary if your image is already small in size or you just don't mind about execution time) to downscale it to fit in 128x128 with [`DynamicImage::resize`](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.resize) (it keeps the aspect ratio, so a 1920x1080 image becomes 128x72): quantization time grows with the number of pixels. The reason is described [**here**](https://github.com/material-foundation/material-color-utilities/blob/main/dev_guide/extracting_colors.md).

```rust,ignore
use std::io::Cursor;

use image::{ImageReader, imageops::FilterType};
use material_colors::{
    color::Rgb, image::extract_color,
    theme::ThemeBuilder
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let data = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .expect("failed to guess image format")
        .decode()
        .expect("failed to decode image")
        // Downscaling leaves far fewer pixels to quantize, so color
        // extraction finishes much faster.
        //
        // The filter only decides how each smaller pixel is computed:
        // `Nearest` is the fastest and keeps the image's original colors,
        // while smoothing filters like `Triangle` or `Lanczos3` (the slowest)
        // blend neighboring pixels. If you don't like the results, try
        // another `FilterType`.
        .resize(128, 128, FilterType::Lanczos3)
        .into_rgb8()
        .into_raw()
        .chunks_exact(3)
        .map(|color| {
            let &[red, green, blue] = color else {
                unreachable!();
            };

            Rgb::new(red, green, blue)
        })
        .collect::<Vec<_>>();

    let theme = ThemeBuilder::with_source(extract_color(&data)).build();

    // Do whatever you want...

    Ok(())
}
```

## Current status of `no-std` support

Only the `quantize` feature requires `alloc`, because `Quantizer` and `Score` make heavy use of `Vec`. Everything else (HCT, palettes, dynamic colors, schemes, themes) works with plain `core`:

```toml
# no allocator
material-colors = { version = "*", default-features = false, features = ["libm"] }
# with an allocator, including color extraction
material-colors = { version = "*", default-features = false, features = ["libm", "quantize"] }
```

The library also makes heavy use of various floating point functions. Without `std`, they come from the `libm` feature, which gives the same results everywhere (all tests pass in both modes on x86-64). With `std`, functions like `powf`, `sin` or `cbrt` come from the platform's system math library, so results may differ in the last bits between platforms. `libm` can be slower on platforms where `std` uses hardware instructions (for example `mul_add`, `sqrt` or `floor`).

If your platform has faster floating-point functions, build crate without `libm` and register your own backend in your binary. Only 8 functions are required; the rest have defaults:

```rust,ignore
struct HardwareFloats;

impl material_colors::utils::no_std::FloatBackend for HardwareFloats {
    fn powf(x: f64, n: f64) -> f64 { /* ... */ }
    fn sqrt(x: f64) -> f64 { /* ... */ }
    fn cbrt(x: f64) -> f64 { /* ... */ }
    fn exp(x: f64) -> f64 { /* ... */ }
    fn ln(x: f64) -> f64 { /* ... */ }
    fn sin(x: f64) -> f64 { /* ... */ }
    fn cos(x: f64) -> f64 { /* ... */ }
    fn atan2(y: f64, x: f64) -> f64 { /* ... */ }
}

material_colors::set_float_backend!(HardwareFloats);
```

Exactly one backend must be registered in the final binary: with none, linking fails with undefined `__material_colors_float_v1_*` symbols; with two (for example `libm` plus your own), it fails with duplicate ones. Libraries depending on this crate should never register a backend.

## MSRV

The Minimum Supported Rust Version is currently 1.97.0.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option. This project may not be copied, modified, or distributed except according to those terms.
