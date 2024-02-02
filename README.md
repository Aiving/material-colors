# Material colors

A Rust library for generating Material You themes (as well as color schemes)

Most of the code was taken from the Swift version of material-color-utilities, as its code is the easiest to rewrite.

There are also a relatively large number of unused variables and functions (45 warnings!). I do not know for what reason they are not used in the original code (apart from a few tests), but I decided to leave them for the future.

## Examples

From HEX color:

```rust
use material_colors::theme_from_source_color;
use material_colors::argb_from_hex;

fn main() {
    let theme = theme_from_source_color(argb_from_hex("#AAE5A4"), Default::default());

    // Do whatever you want...
}
```

From image:

> ⚠️ Before obtaining an array of ARGB pixels for the image, **it is recommended** (but not necessary if your image is already small in size or you just don't mind about execution time) to adjust its dimensions to 128x128 (by `resize` function from `image` crate, for example). The reason is described [**here**](https://github.com/material-foundation/material-color-utilities/blob/main/extract_colors.md).

```rust
use std::io::Cursor;

use material_colors::Argb;
use material_colors::theme_from_source_color;
use material_colors::source_color_from_image;

use image::io::Reader as ImageReader;
use image::imageops::resize;
use image::imageops::FilterType;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let data = ImageReader::new(Cursor::new(image))
        .expect("failed to open image")
        .with_guessed_format()
        .expect("failed to guess format")
        .decode()
        .expect("failed to decode image")
        .into_rgba8();

    // Lancsoz3 takes a little longer, but provides the best pixels for color extraction.
    // However, if you don't like the results, you can always try other FilterType values.
    let data = resize(&data, 128, 128, FilterType::Lanczos3);
    let pixels: Vec<Argb> = data
        .pixels()
        .map(|pixel| u32::from_be_bytes(pixel.0).rotate_right(8).to_be_bytes())
        .collect();

    let theme = theme_from_source_color(source_color_from_image(&pixels), Default::default());

    // Do whatever you want...

    Ok(())
}
```
