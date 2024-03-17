#[cfg(feature = "image")]
#[tokio::test]
async fn main() -> Result<(), reqwest::Error> {
    use material_colors::{
        image::{FilterType, ImageReader},
        theme::ThemeBuilder,
    };

    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let mut data = ImageReader::read(image).expect("failed to read image");

    data.resize(128, 128, FilterType::Lanczos3);

    _ = ThemeBuilder::with_source(ImageReader::extract_color(&data)).build();

    // Do whatever you want...

    Ok(())
}
