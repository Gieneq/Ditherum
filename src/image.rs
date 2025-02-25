use std::path::Path;

use image::{ImageResult, RgbImage};

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
