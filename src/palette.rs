use image::Rgb;

pub type PaletteRGB = Vec<Rgb<u8>>;

pub fn black_n_white() -> PaletteRGB {
    vec![
        Rgb::<u8>([0, 0, 0]),
        Rgb::<u8>([255, 255, 255])
    ]
}

