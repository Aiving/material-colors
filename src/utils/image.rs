use std::io::Cursor;
use std::io::Result;
use std::path::Path;

use image::imageops::resize;

pub use image::imageops::FilterType;

use image::io::Reader;
use image::RgbaImage;

use crate::quantize::quantizer::Quantizer;
use crate::quantize::quantizer_celebi::QuantizerCelebi;
use crate::score::Score;

use super::color::Argb;

pub struct Image {
    image: RgbaImage,
}

impl Image {
    pub fn new(image: RgbaImage) -> Self {
        Self { image }
    }

    pub fn resize(&mut self, width: u32, height: u32, filter_type: FilterType) -> &mut Self {
        self.image = resize(&self.image, width, height, filter_type);

        self
    }
}

pub trait AsPixels {
    fn as_pixels(&self) -> Vec<Argb>;
}

impl AsPixels for Image {
    fn as_pixels(&self) -> Vec<Argb> {
        self.image
            .pixels()
            .map(|pixel| u32::from_be_bytes(pixel.0).rotate_right(8).to_be_bytes())
            .collect()
    }
}

pub struct ImageReader;

impl ImageReader {
    pub fn read<T>(data: T) -> Result<Image>
    where
        T: AsRef<[u8]>,
    {
        let data = Reader::new(Cursor::new(data))
            .with_guessed_format()?
            .decode()
            .expect("failed to decode image")
            .into_rgba8();

        Ok(Image::new(data))
    }

    pub fn open<P>(path: P) -> Result<Image>
    where
        P: AsRef<Path>,
    {
        let data = Reader::open(path)?
            .with_guessed_format()?
            .decode()
            .expect("failed to decode image")
            .into_rgba8();

        Ok(Image::new(data))
    }

    /// Get the source color from an image.
    ///
    /// [`image`] A struct that implements the AsPixels trait
    ///
    /// Returns source color - the color most suitable for creating a UI theme
    pub fn extract_color<I>(image: &I) -> Argb
    where
        I: AsPixels,
    {
        let pixels = image.as_pixels();
        let result = QuantizerCelebi.quantize(&pixels, 128, None);
        let ranked = Score::score(&result.color_to_count, None, None, None);

        ranked[0]
    }
}
