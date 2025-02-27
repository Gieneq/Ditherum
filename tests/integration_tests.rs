mod common;

use common::{get_palette_absolute_path, get_test_save_absolute_path, load_test_image, tests_setup, BNW_IMAGE_FILENAME, COLOR_IMAGE_FILENAME, CORRUPTED_PALETTE_FILENAME, PRIMARY_PALETTE_FILENAME, SAVE_TEST_FILENAME, SAVE_TEST_IMAGE_DIR};
use ditherum::{image, palette::{errors::PaletteError, PaletteRGB}};

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
    let reduced_palette = palette.try_reduce(1);
    assert!(reduced_palette.is_ok());
}

#[test]
fn test_reducing_color_palette() {
    tests_setup();
    let test_image = load_test_image(COLOR_IMAGE_FILENAME);
    let palette = PaletteRGB::from_image(&test_image);
    let original_len = palette.len();
    let reduced_palette = palette.try_reduce(10);
    assert!(reduced_palette.is_ok(), "failed result={:?}", reduced_palette);
    let reduced_palette = reduced_palette.unwrap();
    log::debug!(
        "Reduced a palette of {} colors to {} colors: {:?}",
        original_len,
        reduced_palette.len(),
        reduced_palette
    );
}

#[test]
fn test_load_primary_palette() {
    tests_setup();
    let palette_path = get_palette_absolute_path(PRIMARY_PALETTE_FILENAME);
    let palette = PaletteRGB::load_from_json(palette_path);
    assert!(palette.is_ok());

    let palette = palette.unwrap();
    assert_eq!(palette, PaletteRGB::primary());
}

#[test]
fn test_load_corrupted_palette() {
    tests_setup();
    let palette_path = get_palette_absolute_path(CORRUPTED_PALETTE_FILENAME);
    let palette = PaletteRGB::load_from_json(palette_path);
    assert!(matches!(palette, Err(PaletteError::JsonParsingFailed(_))));
}

#[test]
fn test_load_not_existing_palette_palette() {
    tests_setup();
    let palette_path = get_palette_absolute_path("not_existing_file.json");
    let palette = PaletteRGB::load_from_json(palette_path);
    assert!(matches!(palette, Err(PaletteError::IoError(_))));
}

#[test]
fn test_saving_reduced_color_palette_and_loading_back() {
    tests_setup();
    let test_image = load_test_image(COLOR_IMAGE_FILENAME);
    let palette = PaletteRGB::from_image(&test_image);
    let target_colors_count = 20;
    let reduced_palette = palette.try_reduce(target_colors_count);
    assert!(reduced_palette.is_ok(), "failed result={:?}", reduced_palette);
    let reduced_palette = reduced_palette.unwrap();

    let result_path = get_test_save_absolute_path("reduced_palette_color.json");
    let saving_result = reduced_palette.save_to_json(&result_path);
    assert!(saving_result.is_ok());

    let loaded_palette = PaletteRGB::load_from_json(result_path);
    assert!(loaded_palette.is_ok());

    let loaded_palette = loaded_palette.unwrap();
    assert_eq!(loaded_palette.len(), target_colors_count);
}