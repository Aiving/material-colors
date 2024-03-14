# Material colors

A Rust library for generating Material You themes (as well as color schemes)

Most of the code was taken from the Swift version of material-color-utilities, as its code is the easiest to rewrite.

There are also a relatively large number of unused variables and functions (45 warnings!). I do not know for what reason they are not used in the original code (apart from a few tests), but I decided to leave them for the future.

## Examples

From HEX color:

```rust
use std::str::FromStr;
use material_colors::utils::theme::Theme;
use material_colors::Argb;

fn main() {
    let theme = Theme::from_source_color(Argb::from_str("#AAE5A4").unwrap(), Default::default());

    // Do whatever you want...
}
```

From image:

> ⚠️ Before obtaining an array of ARGB pixels for the image, **it is recommended** (but not necessary if your image is already small in size or you just don't mind about execution time) to adjust its dimensions to 128x128 (by `resize` function from `image` crate, for example). The reason is described [**here**](https://github.com/material-foundation/material-color-utilities/blob/main/extract_colors.md).

```rust
use material_colors::utils::theme::Theme;
use material_colors::FilterType;
use material_colors::ImageReader;

#[tokio::main]
async fn _main() -> Result<(), reqwest::Error> {
    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let mut data = ImageReader::read(image).expect("failed to read image");

    // Lancsoz3 takes a little longer, but provides the best pixels for color extraction.
    // However, if you don't like the results, you can always try other FilterType values.
    data.resize(128, 128, FilterType::Lanczos3);

    let theme = Theme::from_source_color(ImageReader::extract_color(&data), Default::default());

    // Do whatever you want...

    Ok(())
}
```
