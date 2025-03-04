use image::RgbImage;
use palette::Lab;

use crate::{color, palette::PaletteRGB};

struct Kernel2x2<T> {
    tl: T,
    tr: T,
    bl: T,
    br: T,
}

impl<T> Kernel2x2<T> 
where 
    T: Copy
{
    fn from_matrix(tl_position: (usize, usize), image_size: (usize, usize), src_matrix: &[Vec<T>]) -> Self {
        let (x, y) = tl_position;
        let (width, height) = image_size;

        let get_wrapped = |translated_x: usize, translated_y: usize| {
            let target_x = if translated_x < width { translated_x } else { width - 1 };
            let target_y = if translated_y < height { translated_y } else { height - 1 };
            src_matrix[target_y][target_x]
        };

        Self {
            tl: get_wrapped(x    , y    ),
            tr: get_wrapped(x + 1, y    ),
            bl: get_wrapped(x    , y + 1),
            br: get_wrapped(x + 1, y + 1),
        }
    }
    
    fn apply_to_matrix(self, tl_position: (usize, usize), image_size: (usize, usize), target_matrix: &mut [Vec<T>]) {
        let (x, y) = tl_position;
        let (width, height) = image_size;

        let mut set_omitting = |translated_x: usize, translated_y: usize, value: T| {
            if (translated_x < width) && (translated_y < height) {
                target_matrix[translated_y][translated_x] = value;
            }
        };

        set_omitting(x    , y    , self.tl);
        set_omitting(x + 1, y    , self.tr);
        set_omitting(x    , y + 1, self.bl);
        set_omitting(x + 1, y + 1, self.br);
    }
}

type Kernel2x2Lab = Kernel2x2<Lab>;

fn apply_2x2_dithering_lab_kernel(
    _x: usize,
    _y: usize,
    mut kernel: Kernel2x2Lab,
    palette: &[Lab]
) -> Kernel2x2Lab {
    let (closest_tl_color, quant_error) = color::manip::find_closest_lab_color(&kernel.tl , palette);
    kernel.tl = closest_tl_color;

    // Spread quantisation error over remaining 3 pixels
    let err_weights: [f32; 3] = [
        9.0 / 18.0,
        5.0 / 18.0,
        4.0 / 18.0,
    ];

    let fuzz = 0; //x
    let err_w_tr = err_weights[(fuzz) % 3];
    let err_w_bl = err_weights[(fuzz + 1) % 3];
    let err_w_br = err_weights[(fuzz + 2) % 3];

    kernel.tr = color::manip::lab_add(&kernel.tr, &color::manip::lab_mul_scalar(&quant_error, err_w_tr));
    kernel.bl = color::manip::lab_add(&kernel.bl, &color::manip::lab_mul_scalar(&quant_error, err_w_bl));
    kernel.br = color::manip::lab_add(&kernel.br, &color::manip::lab_mul_scalar(&quant_error, err_w_br));

    kernel
}

pub fn dithering_floyd_steinberg_lab(source_image: RgbImage, palette: PaletteRGB) -> RgbImage {
    let (width, height, mut lab_vec) = crate::image::manip::rgb_image_to_lab_vec(source_image);
    let lab_palette = palette.to_lab();

    for y in 0..height {
        for x in 0..width {
            let kernel = Kernel2x2Lab::from_matrix((x, y), (width, height), &lab_vec);
            let result_kernel = apply_2x2_dithering_lab_kernel(x, y, kernel, &lab_palette);
            result_kernel.apply_to_matrix((x, y), (width, height), &mut lab_vec);
        }
    }

    crate::image::manip::lab_vec_to_rgb_iamge(width, height, lab_vec)
}

type Kernel2x2Srgb = Kernel2x2<palette::Srgb>;

fn apply_2x2_dithering_srgb_kernel(
    _x: usize,
    _y: usize,
    mut kernel: Kernel2x2Srgb,
    palette: &[palette::Srgb]
) -> Kernel2x2Srgb {
    let closest_tl_color = color::manip::find_closest_srgb_color(&kernel.tl , palette);
    let quant_error = color::manip::srgb_sub(&kernel.tl, &closest_tl_color);
    kernel.tl = closest_tl_color;

    // Spread quantisation error over remaining 3 pixels
    let err_weights: [f32; 3] = [
        9.0 / 18.0,
        5.0 / 18.0,
        4.0 / 18.0,
    ];

    let fuzz = 0; //x
    let err_w_tr = err_weights[(fuzz) % 3];
    let err_w_bl = err_weights[(fuzz + 1) % 3];
    let err_w_br = err_weights[(fuzz + 2) % 3];

    kernel.tr = color::manip::srgb_add(&kernel.tr, &color::manip::srgb_mul_scalar(&quant_error, err_w_tr));
    kernel.bl = color::manip::srgb_add(&kernel.bl, &color::manip::srgb_mul_scalar(&quant_error, err_w_bl));
    kernel.br = color::manip::srgb_add(&kernel.br, &color::manip::srgb_mul_scalar(&quant_error, err_w_br));

    kernel
}

pub fn dithering_floyd_steinberg_rgb(source_image: RgbImage, palette: PaletteRGB) -> RgbImage {
    let (width, height, mut rgb_matrix) = crate::image::manip::rgb_image_to_float_srgb_vec(source_image);
    let srgb_palette = palette.to_srgb();

    for y in 0..height {
        for x in 0..width {
            let kernel = Kernel2x2Srgb::from_matrix(
                (x, y), 
                (width, height), 
                &rgb_matrix
            );
            let result_kernel = apply_2x2_dithering_srgb_kernel(x, y, kernel, &srgb_palette);
            result_kernel.apply_to_matrix((x, y), (width, height), &mut rgb_matrix);
        }
    }

    crate::image::manip::srgb_vec_to_rgb_iamge(width, height, rgb_matrix)
}

