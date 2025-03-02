use image::{
    Rgb, 
    RgbImage
};

use palette::{
    color_difference::{Ciede2000, EuclideanDistance}, 
    white_point::D65, 
    FromColor, 
    Lab, 
    Srgb
};

pub fn mix_color_channel(
    mix_factor: f32, 
    from_value: u8,
    to_value: u8
) -> u8 {
    let mix_factor = mix_factor.clamp(0.0, 1.0);
    let mixed_value = (1.0 - mix_factor) * (from_value as f32) + mix_factor * (to_value as f32);
    mixed_value.round().clamp(0.0, 255.0) as u8 
}

pub fn mix_rgb_colors(
    mix_factor: f32, 
    from_color: Rgb<u8>,
    to_color: Rgb<u8>
) -> Rgb<u8> {
    Rgb([
        mix_color_channel(mix_factor, from_color[0], to_color[0]),
        mix_color_channel(mix_factor, from_color[1], to_color[1]),
        mix_color_channel(mix_factor, from_color[2], to_color[2])
    ])
}

pub fn rgb_to_lab(rgb_color: &Rgb<u8>) -> Lab {
    let srgb = rgb_to_srgb(rgb_color);
    Lab::from_color(srgb)
}

pub fn rgb_to_srgb(rgb_color: &Rgb<u8>) -> palette::rgb::Srgb {
    Srgb::new(
        rgb_color[0] as f32 / 255.0,
        rgb_color[1] as f32 / 255.0,
        rgb_color[2] as f32 / 255.0
    )
}

pub fn lab_to_rgb(lab_color: &Lab) -> Rgb<u8> {
    let srgb = Srgb::from_color(*lab_color);
    srgb_to_rgb(&srgb)
}

pub fn srgb_to_rgb(srgb: &Srgb) -> Rgb<u8> {
    Rgb([
        (srgb.red * 255.0).round() as u8,
        (srgb.green * 255.0).round() as u8,
        (srgb.blue * 255.0).round() as u8
    ])
}

pub fn rgb_image_to_float_srgb_vec(source_image: RgbImage) -> (usize, usize, Vec<Vec<Srgb>>) {
    let (width, height) = (source_image.width() as usize, source_image.height() as usize);
    let mut lab_image = vec![vec![Srgb::new(0.0, 0.0, 0.0); width]; height];
    
    source_image.enumerate_pixels()
        .for_each(|(x, y, rgb_pixel)| {
            lab_image[y as usize][x as usize] = rgb_to_srgb(rgb_pixel)
        });

    (width, height, lab_image)
}

pub fn rgb_image_to_lab_vec(source_image: RgbImage) -> (usize, usize, Vec<Vec<Lab<D65,f32>>>) {
    let (width, height) = (source_image.width() as usize, source_image.height() as usize);
    let mut lab_image = vec![vec![Lab::new(0.0, 0.0, 0.0); width]; height];
    
    source_image.enumerate_pixels()
        .for_each(|(x, y, rgb_pixel)| {
            lab_image[y as usize][x as usize] = rgb_to_lab(rgb_pixel)
        });

    (width, height, lab_image)
}

pub fn lab_vec_to_rgb_iamge(width: usize, height: usize, lab_vec: Vec<Vec<Lab>>) -> RgbImage {
    RgbImage::from_fn(width as u32, height as u32, |x, y| {
        let lab_color = &lab_vec[y as usize][x as usize];
        lab_to_rgb(lab_color)
    })
}

pub fn srgb_vec_to_rgb_iamge(width: usize, height: usize, rgb_vec: Vec<Vec<Srgb>>) -> RgbImage {
    RgbImage::from_fn(width as u32, height as u32, |x, y| {
        let srgb_color = &rgb_vec[y as usize][x as usize];
        srgb_to_rgb(srgb_color)
    })
}

pub fn sub_lab_colors(lab_left: &Lab, lab_right: &Lab) -> Lab {
    let mut tmp = *lab_left;
    tmp.l -= lab_right.l;
    tmp.a -= lab_right.a;
    tmp.b -= lab_right.b;
    tmp
}

pub fn add_lab_colors(lab_left: &Lab, lab_right: &Lab) -> Lab {
    let mut tmp = *lab_left;
    tmp.l += lab_right.l;
    tmp.a += lab_right.a;
    tmp.b += lab_right.b;
    tmp
}

pub fn mul_lab_colors_by_scalar(lab_left: &Lab, scalar: f32) -> Lab {
    let mut tmp = *lab_left;
    tmp.l *= scalar;
    tmp.a *= scalar;
    tmp.b *= scalar;
    tmp
}

pub fn sub_srgb_colors(left: &Srgb, right: &Srgb) -> Srgb {
    let mut tmp = *left;
    tmp.red -= right.red;
    tmp.green -= right.green;
    tmp.blue -= right.blue;
    tmp
}

pub fn add_srgb_colors(left: &Srgb, right: &Srgb) -> Srgb {
    let mut tmp = *left;
    tmp.red += right.red;
    tmp.green += right.green;
    tmp.blue += right.blue;
    tmp
}

pub fn mul_srgb_colors_by_scalar(left: &Srgb, scalar: f32) -> Srgb {
    let mut tmp = *left;
    tmp.red *= scalar;
    tmp.green *= scalar;
    tmp.blue *= scalar;
    tmp
}

// pub fn find_closest_rgb_color(rgb_color: &Rgb<u8>, palette: &PaletteRGB) -> (ColorRGB, ColorRGB) {
//     let lab_color = rgb_to_lab(rgb_color);
//     let lab_pallete = palette.to_lab();

//     let (_, &closest_palette_color) = lab_pallete.into_iter()
//         .zip(palette.iter())
//         .map(|(lab_palette_color, palette_color)| {
//             let diff = lab_color.difference(lab_palette_color);
//             (diff, palette_color)
//         })
//         .min_by(|(diff_a, _), (diff_b, _)| diff_a.partial_cmp(diff_b)
//             .unwrap_or(std::cmp::Ordering::Equal)
//         )
//         .unwrap();

//     let quant_err = sub_rgb_colors(lab_color, &closest_palette_color);
//     (closest_palette_color, quant_err)
// }

pub fn find_closest_lab_color(lab_color: &Lab, palette: &[Lab]) -> (Lab, Lab) {
    let (_, &closest_palette_color) = palette.iter()
        .map(|palette_color| {
            let diff = lab_color.difference(*palette_color);
            (diff, palette_color)
        })
        .min_by(|(diff_a, _), (diff_b, _)| diff_a.partial_cmp(diff_b)
            .unwrap_or(std::cmp::Ordering::Equal)
        )
        .unwrap();

    let quant_err = sub_lab_colors(lab_color, &closest_palette_color);
    (closest_palette_color, quant_err)
}

pub fn find_closest_srgb_color(srgb_color: &Srgb, palette: &[Srgb]) -> Srgb {
    let (_, &closest_palette_color) = palette.iter()
        .map(|palette_color| {
            let diff = srgb_color.distance_squared(*palette_color);
            (diff, palette_color)
        })
        .min_by(|(diff_a, _), (diff_b, _)| diff_a.partial_cmp(diff_b)
            .unwrap_or(std::cmp::Ordering::Equal)
        )
        .unwrap();

    closest_palette_color
}

#[test]
fn test_channel_mix() {
    let mix_factor = 0.25;
    let from_value = 0;
    let to_value = 100;
    let result = mix_color_channel(mix_factor, from_value, to_value);
    assert_eq!(result, 25);
}