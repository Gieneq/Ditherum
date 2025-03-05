use std::path::Path;

use image::{ImageResult, RgbImage};

use crate::{algorithms::{dithering, thresholding}, palette::PaletteRGB};

#[derive(Debug)]
pub enum ProcessingAlgorithm {
    ThresholdingRgb,
    ThresholdingLab,
    FloydSteinbergRgb,
}

#[derive(Debug)]
pub enum ImageProcessingError {

}

#[derive(Debug)]
pub struct ImageProcessor {
    source_image: RgbImage,
    palette: PaletteRGB,
    algorithm: ProcessingAlgorithm,
}

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
    from_color: image::Rgb<u8>,
    to_color: image::Rgb<u8>
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
        let pixel_color = super::color::manip::mix_rgb_colors(mix_factor, from_color, to_color);
        (0..height).for_each(|y| {
            *img.get_pixel_mut(x, y) = pixel_color;
        });
    }

    img
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
        let result_image = match self.algorithm {
            ProcessingAlgorithm::ThresholdingRgb => thresholding::thresohlding_rgb(self.source_image, self.palette),
            ProcessingAlgorithm::ThresholdingLab => thresholding::thresohlding_lab(self.source_image, self.palette),
            ProcessingAlgorithm::FloydSteinbergRgb => dithering::dithering_floyd_steinberg_rgb(self.source_image, self.palette),
        };
        Ok(result_image)
    }
}

pub mod manip {
    use palette::white_point::D65;

    use crate::color;

    use super::*;
    
    pub fn rgb_image_to_float_srgb_vec(source_image: RgbImage) -> (usize, usize, Vec<Vec<palette::Srgb>>) {
        let (width, height) = (source_image.width() as usize, source_image.height() as usize);
        let mut lab_image = vec![vec![palette::Srgb::new(0.0, 0.0, 0.0); width]; height];
        
        source_image.enumerate_pixels()
            .for_each(|(x, y, rgb_pixel)| {
                lab_image[y as usize][x as usize] = color::manip::rgbu8_to_srgb(*rgb_pixel)
            });

        (width, height, lab_image)
    }

    pub fn rgb_image_to_lab_vec(source_image: RgbImage) -> (usize, usize, Vec<Vec<palette::Lab<D65,f32>>>) {
        let (width, height) = (source_image.width() as usize, source_image.height() as usize);
        let mut lab_image = vec![vec![palette::Lab::new(0.0, 0.0, 0.0); width]; height];
        
        source_image.enumerate_pixels()
            .for_each(|(x, y, rgb_pixel)| {
                lab_image[y as usize][x as usize] = color::manip::rgbu8_to_lab(*rgb_pixel)
            });

        (width, height, lab_image)
    }

    pub fn lab_vec_to_rgb_iamge(width: usize, height: usize, lab_vec: Vec<Vec<palette::Lab>>) -> RgbImage {
        RgbImage::from_fn(width as u32, height as u32, |x, y| {
            let lab_color = &lab_vec[y as usize][x as usize];
            color::manip::lab_to_rgbu8(*lab_color)
        })
    }

    pub fn srgb_vec_to_rgb_iamge(width: usize, height: usize, rgb_vec: Vec<Vec<palette::Srgb>>) -> RgbImage {
        RgbImage::from_fn(width as u32, height as u32, |x, y| {
            let srgb_color = &rgb_vec[y as usize][x as usize];
            color::manip::srgb_to_rgbu8(*srgb_color)
        })
    }

    pub fn srgb_vec_to_rgb_image_using_palette(width: usize, height: usize, rgb_vec: Vec<Vec<palette::Srgb>>, palette: &PaletteRGB) -> RgbImage {
        RgbImage::from_fn(width as u32, height as u32, |x, y| {
            let srgb_color = &rgb_vec[y as usize][x as usize];
            palette.find_closest_by_srgb(srgb_color).into()
        })
    }
}

#[test]
fn test_processing_gradient_image() {
    let (width, height) = (200, 80);
    let source_image = generate_test_gradient_image(
        width, 
        height, 
        image::Rgb::<u8>([0,0,0]), 
        image::Rgb::<u8>([0,0,255]), 
    );
    let palette = PaletteRGB::primary();

    let processing_result = ImageProcessor::new(source_image, palette)
        .run();
    assert!(processing_result.is_ok());

    let processing_result = processing_result.unwrap();
    assert_eq!(processing_result.width(), width);
    assert_eq!(processing_result.height(), height);
}