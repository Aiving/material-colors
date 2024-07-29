# Material colors

[![crates.io: material-colors](https://img.shields.io/crates/v/material-colors.svg?style=for-the-badge)](https://crates.io/crates/material-colors)
[![Documentation](https://img.shields.io/docsrs/material-colors.svg?style=for-the-badge)](https://docs.rs/material-colors)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Aiving/material-colors/CI.yml.svg?style=for-the-badge)](https://github.com/Aiving/material-colors/actions)
[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT_or_Apache_2.0-634f7d.svg?style=for-the-badge)](LICENSE-APACHE)

An unofficial port of the `material-color-utilities` library for creating Material You themes and color schemes.

## Features

- `std`: enabled by default, disabling makes it possible to use the crate in `no_std` environments, provided there is an allocator available
  - **NOTE**: Currently, the crate uses [`libm`](https://github.com/rust-lang/libm) in `no_std` environments for all math operations for floating point numbers. This is likely to significantly reduce its speed, but there are no other alternatives at the moment.
- `image`: adds support for extracting colors from images, requires `std` feature enabled
- `serde`: adds support for JSON serialization of themes and color schemes

## Examples

From HEX color:

```rust
use material_colors::{color::Argb, theme::ThemeBuilder};

fn main() {
    let theme = ThemeBuilder::with_source(Argb::from_u32(0xffaae5a4)).build();

    // Do whatever you want...
}
```

From image:

> ⚠️ Before obtaining an array of ARGB pixels for the image, **it is recommended** (but not necessary if your image is already small in size or you just don't mind about execution time) to adjust its dimensions to 128x128 by `func:resize` from `struct:Image` provided by `struct:ImageReader`. The reason is described [**here**](https://github.com/material-foundation/material-color-utilities/blob/main/extract_colors.md).

```rust
use material_colors::{
    image::{FilterType, ImageReader},
    theme::ThemeBuilder,
};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let mut data = ImageReader::read(image).expect("failed to read image");

    // Lancsoz3 takes a little longer, but provides the best pixels for color extraction.
    // However, if you don't like the results, you can always try other FilterType values.
    data.resize(128, 128, FilterType::Lanczos3);

    let theme = ThemeBuilder::with_source(ImageReader::extract_color(&data)).build();

    // Do whatever you want...

    Ok(())
}
```

## MSRV

The Minimum Supported Rust Version is currently 1.63.0.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option. This project may not be copied, modified, or distributed except according to those terms.
