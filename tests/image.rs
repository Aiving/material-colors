#[cfg(feature = "quantize")]
#[tokio::test(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    use std::io::Cursor;

    use image::{ImageReader, imageops::FilterType};
    use material_colors::{color::Rgb, image::extract_color, theme::ThemeBuilder};

    let image = reqwest::get("https://picsum.photos/id/866/1920/1080").await?.bytes().await?.to_vec();
    let data = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .expect("failed to guess image format")
        .decode()
        .expect("failed to decode image")
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

    _ = ThemeBuilder::with_source(extract_color(&data)).build();

    // Do whatever you want...

    Ok(())
}
