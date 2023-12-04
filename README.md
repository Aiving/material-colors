# Material colors

A Rust library for generating Material You themes (as well as color schemes)

Most of the code was taken from the Swift version of material-color-utilities, as its code is the easiest to rewrite.

There are also a relatively large number of unused variables and functions (45 warnings!). I do not know for what reason they are not used in the original code (apart from a few tests), but I decided to leave them for the future.

## Examples

From HEX color:

```rs
use material_colors::theme_from_source_color;
use material_colors::argb_from_hex;

fn main() {
    let theme = theme_from_source_color(argb_from_hex("#AAE5A4"), Default::default());

    // Do whatever you want...
}
```

From image:

```rs
use material_colors::theme_from_source_color;
use material_colors::source_color_from_image;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();
    let theme = theme_from_source_color(source_color_from_image(&image), Default::default());

    // Do whatever you want...

    Ok(())
}
```
