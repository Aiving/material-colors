use std::{
    io::{Cursor, Result},
    path::Path,
    vec::Vec,
};

pub use images::imageops::FilterType;
use images::{ImageReader as Reader, RgbImage, imageops::resize};

use crate::{
    color::Rgb,
    quantize::{Quantizer, QuantizerCelebi},
    score::Score,
};

pub struct Image {
    image: RgbImage,
}

impl Image {
    pub const fn new(image: RgbImage) -> Self {
        Self { image }
    }

    pub fn resize(&mut self, width: u32, height: u32, filter_type: FilterType) -> &mut Self {
        self.image = resize(&self.image, width, height, filter_type);

        self
    }
}

pub trait AsPixels {
    fn as_pixels(&self) -> Vec<Rgb>;
}

impl AsPixels for Image {
    fn as_pixels(&self) -> Vec<Rgb> {
        self.image.pixels().map(|&images::Rgb([r, g, b])| Rgb::new(r, g, b)).collect()
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
            .into_rgb8();

        Ok(Image::new(data))
    }

    pub fn open<P>(path: P) -> Result<Image>
    where
        P: AsRef<Path>,
    {
        let data = Reader::open(path)?.with_guessed_format()?.decode().expect("failed to decode image").into_rgb8();

        Ok(Image::new(data))
    }

    /// Get the source color from an image.
    ///
    /// `image` A struct that implements the [`AsPixels`] trait
    ///
    /// Returns source color - the color most suitable for creating a UI theme
    pub fn extract_color<I>(image: &I) -> Rgb
    where
        I: AsPixels,
    {
        let pixels = image.as_pixels();
        let result = QuantizerCelebi::quantize(&pixels, 128);
        let ranked = Score::score(&result.color_to_count, None, None, None);

        ranked[0]
    }
}
