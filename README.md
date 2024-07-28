# Material colors

An unofficial port of the `material-color-utilities` library for creating Material You themes and color schemes.

## Examples

From HEX color:

```rust
use std::str::FromStr;
use material_colors::{color::Argb, theme::ThemeBuilder};

fn main() {
    let theme = ThemeBuilder::with_source(Argb::from_32(0xffaae5a4)).build();

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
