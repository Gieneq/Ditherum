use image::RgbImage;

use crate::{color::ColorRGB, palette::PaletteRGB};

pub fn thresohlding_rgb(mut source_image: RgbImage, palette: PaletteRGB) -> RgbImage {
    source_image.enumerate_pixels_mut()
        .for_each(|(_, _, pixel)| {
            *pixel = palette.find_closest_by_rgb(&ColorRGB::from_rgbu8(*pixel)).to_rgbu8()
        });

    source_image
}

pub fn thresohlding_lab(mut source_image: RgbImage, palette: PaletteRGB) -> RgbImage {
    source_image.enumerate_pixels_mut()
        .for_each(|(_, _, pixel)| {
            *pixel = palette.find_closest_by_lab(&ColorRGB::from_rgbu8(*pixel)).to_rgbu8()
        });

    source_image
}