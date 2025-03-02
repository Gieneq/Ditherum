use std::path::Path;

use image::{ImageResult, Rgb, RgbImage};

use crate::{algorithms::{color_manip, processing, ProcessingAlgorithm}, palette::PaletteRGB};

pub fn load_image<P>(path: P) -> ImageResult<RgbImage> 
where 
    P: AsRef<Path>
{
    let img = image::open(path)?;
    Ok(img.to_rgb8())
}

pub fn save_image<P>(path: P, img: &RgbImage) -> ImageResult<()>
where 
    P: AsRef<Path>
{
    img.save(path)
}

pub fn generate_test_gradient_image(
    width: u32, 
    height: u32,
    from_color: Rgb<u8>,
    to_color: Rgb<u8>
) -> RgbImage {
    if width == 0 {
        panic!("Width should be > 0");
    }

    if height == 0 {
        panic!("Height should be > 0");
    }

    let mut img = RgbImage::new(width, height);

    for x in 0..width {
        let mix_factor = (x as f32) / (width - 1) as f32;
        let pixel_color = color_manip::mix_rgb_colors(mix_factor, from_color, to_color);
        (0..height).for_each(|y| {
            *img.get_pixel_mut(x, y) = pixel_color;
        });
    }

    img
}

pub struct ImageProcessor {
    source_image: RgbImage,
    palette: PaletteRGB,
    algorithm: ProcessingAlgorithm,
}

#[derive(Debug)]
pub enum ImageProcessingError {

}

impl ImageProcessor {
    pub fn new(source_image: RgbImage, palette: PaletteRGB) -> Self {
        Self {
            source_image,
            palette,
            algorithm: ProcessingAlgorithm::ThresholdingRgb
        }
    }

    pub fn with_algorithm(mut self, algorithm: ProcessingAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn run(self) -> Result<RgbImage, ImageProcessingError>{
        let result_image = processing(self.source_image, self.palette, self.algorithm);
        Ok(result_image)
    }
}

#[test]
fn test_processing_gradient_image() {
    let (width, height) = (200, 80);
    let source_image = generate_test_gradient_image(
        width, 
        height, 
        Rgb::<u8>([0,0,0]), 
        Rgb::<u8>([0,0,255]), 
    );
    let palette = PaletteRGB::primary();

    let processing_result = ImageProcessor::new(source_image, palette)
        .run();
    assert!(processing_result.is_ok());

    let processing_result = processing_result.unwrap();
    assert_eq!(processing_result.width(), width);
    assert_eq!(processing_result.height(), height);
}