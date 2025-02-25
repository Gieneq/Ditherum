mod common;

use common::{tests_setup, load_test_image, BNW_IMAGE_FILENAME, COLOR_IMAGE_FILENAME, SAVE_TEST_IMAGE_DIR, SAVE_TEST_FILENAME};
use ditherum::{image, palette::PaletteRGB};

#[test]
fn test_image_opening() {
    let test_image = load_test_image(COLOR_IMAGE_FILENAME);
    assert!(test_image.width() > 0);
    assert!(test_image.height() > 0);
}

#[test]
fn test_image_saving() {
    tests_setup();
    let test_image = load_test_image(COLOR_IMAGE_FILENAME);
    let save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join(SAVE_TEST_FILENAME);
    let result = image::save_image(save_path, &test_image);
    assert!(result.is_ok());
}

#[test]
fn test_obtaining_palette_from_bn_w_image() {
    tests_setup();
    let test_image = load_test_image(BNW_IMAGE_FILENAME);
    let palette = PaletteRGB::from_image(&test_image);

    // Expecting two colors: black and white.
    assert_eq!(palette.len(), 2);
}

#[test]
fn test_reducing_bn_w_palette() {
    tests_setup();
    let test_image = load_test_image(BNW_IMAGE_FILENAME);
    let palette = PaletteRGB::from_image(&test_image);
    assert_eq!(palette.len(), 2);
    let reduced_palette = palette.reduce_to(1);
    assert!(reduced_palette.is_ok());
}

#[test]
fn test_reducing_color_palette() {
    tests_setup();
    let test_image = load_test_image(COLOR_IMAGE_FILENAME);
    let palette = PaletteRGB::from_image(&test_image);
    let original_len = palette.len();
    let reduced_palette = palette.reduce_to(10);
    assert!(reduced_palette.is_ok());
    let reduced_palette = reduced_palette.unwrap();
    log::debug!(
        "Reduced a palette of {} colors to {} colors: {:?}",
        original_len,
        reduced_palette.len(),
        reduced_palette
    );
}
