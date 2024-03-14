#[cfg(feature = "image")]
#[tokio::test]
async fn main() -> Result<(), reqwest::Error> {
    use material_colors::utils::theme::Theme;
    use material_colors::FilterType;
    use material_colors::ImageReader;

    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let mut data = ImageReader::read(image).expect("failed to read image");

    data.resize(128, 128, FilterType::Lanczos3);

    let color = ImageReader::extract_color(&data);

    println!("{}", color);

    _ = Theme::from_source_color(color, Default::default());

    // Do whatever you want...

    Ok(())
}
