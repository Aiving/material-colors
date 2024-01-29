use std::io::Cursor;

use image::imageops::resize;
use image::imageops::FilterType;
use material_colors::hex_from_argb;
use material_colors::source_color_from_image;
use material_colors::theme_from_source_color;
use material_colors::Argb;

use image::io::Reader as ImageReader;

#[tokio::test]
async fn main() -> Result<(), reqwest::Error> {
    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let data = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .expect("failed to guess format")
        .decode()
        .expect("failed to decode image")
        .into_rgba8();

    let data = resize(&data, 128, 128, FilterType::Gaussian);
    let pixels: Vec<Argb> = data
        .pixels()
        .map(|pixel| u32::from_be_bytes(pixel.0).rotate_right(8).to_be_bytes())
        .collect();

    let color = source_color_from_image(&pixels);

    println!("{}", hex_from_argb(color));

    _ = theme_from_source_color(color, Default::default());

    // Do whatever you want...

    Ok(())
}
