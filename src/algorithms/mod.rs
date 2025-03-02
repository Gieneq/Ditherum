pub mod kmean;
pub mod color_manip;
pub mod image_proc;

use image_proc::*;
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
        ProcessingAlgorithm::FloydSteinbergRgb => f_s_rgb::dithering_floyd_steinberg_rgb(source_image, palette),
        ProcessingAlgorithm::FloydSteinbergLab => f_s_lab::dithering_floyd_steinberg_lab(source_image, palette),
    }
}