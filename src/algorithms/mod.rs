pub mod kmean;
pub mod processings;
pub mod thresholding;
pub mod dithering;

use image::RgbImage;
use crate::palette::PaletteRGB;

pub enum ProcessingAlgorithm {
    ThresholdingRgb,
    ThresholdingLab,
    FloydSteinbergRgb,
    FloydSteinbergLab,
}

pub fn processing(source_image: RgbImage, palette: PaletteRGB, algorithm: ProcessingAlgorithm) -> RgbImage {
    match algorithm {
        ProcessingAlgorithm::ThresholdingRgb => thresholding::thresohlding_rgb(source_image, palette),
        ProcessingAlgorithm::ThresholdingLab => thresholding::thresohlding_lab(source_image, palette),
        ProcessingAlgorithm::FloydSteinbergRgb => dithering::dithering_floyd_steinberg_rgb(source_image, palette),
        ProcessingAlgorithm::FloydSteinbergLab => dithering::dithering_floyd_steinberg_lab(source_image, palette),
    }
}