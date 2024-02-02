#[cfg(feature = "image")]
#[tokio::test]
async fn main() -> Result<(), reqwest::Error> {
    use material_colors::hex_from_argb;
    use material_colors::theme_from_source_color;
    use material_colors::FilterType;
    use material_colors::ImageReader;

    let image = reqwest::get("https://picsum.photos/id/866/1920/1080")
        .await?
        .bytes()
        .await?
        .to_vec();

    let mut data = ImageReader::read(image).expect("failed to read image");

    data.resize(FilterType::Lanczos3);

    let color = ImageReader::extract_color(&data);

    println!("{}", hex_from_argb(&color));

    _ = theme_from_source_color(color, Default::default());

    // Do whatever you want...

    Ok(())
}
