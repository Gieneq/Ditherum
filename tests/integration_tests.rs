mod common;

use common::{
    get_image_absolute_path,
    get_palette_absolute_path, 
    get_test_save_absolute_path, 
    load_test_image, 
    tests_setup, 
    BNW_IMAGE_FILENAME, 
    GRAY300_IMAGE_FILENAME,
    COLOR_GRASS300_IMAGE_FILENAME, 
    COLOR_PINK300_IMAGE_FILENAME, 
    COLOR_YELLOW600_IMAGE_FILENAME, 
    CORRUPTED_PALETTE_FILENAME, 
    PRIMARY_PALETTE_FILENAME, 
    SAVE_TEST_IMAGE_DIR
};
use ditherum::{
    image, 
    palette::{
        errors::PaletteError, 
        PaletteRGB
    }
};

#[test]
fn test_image_opening() {
    let test_image = load_test_image(COLOR_PINK300_IMAGE_FILENAME);
    assert!(test_image.width() > 0);
    assert!(test_image.height() > 0);
}

#[test]
fn test_image_saving() {
    tests_setup();
    let test_image = load_test_image(COLOR_YELLOW600_IMAGE_FILENAME);
    let save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join("test_result.png");
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
    let test_image = load_test_image(COLOR_PINK300_IMAGE_FILENAME);
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
    let test_image = load_test_image(COLOR_GRASS300_IMAGE_FILENAME);
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

#[cfg(test)]
mod tests_cli {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_palette_black_and_white_extraction() {
        tests_setup();
        let test_palette_filename = "sample_bw_palette.json";
        let absolute_input_path = get_image_absolute_path(BNW_IMAGE_FILENAME);
        let absolute_output_path = get_test_save_absolute_path(test_palette_filename);

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("palette")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-o")
            .arg(&absolute_output_path);
        cmd.assert().success();

        // Load palette back, it shoudl have 2 colors
        let loaded_palette = PaletteRGB::load_from_json(absolute_output_path);
        assert!(loaded_palette.is_ok());

        let loaded_palette = loaded_palette.unwrap();
        assert_eq!(loaded_palette.len(), 2);
    }

    #[test]
    fn test_palette_color_reduced_9_extraction() {
        tests_setup();
        let test_palette_filename = "sample_reduced_9_colors_palette.json";
        let absolute_input_path = get_image_absolute_path(GRAY300_IMAGE_FILENAME);
        let absolute_output_path = get_test_save_absolute_path(test_palette_filename);

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("palette")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-c")
            .arg("9")
            .arg("-o")
            .arg(&absolute_output_path);
        cmd.assert().success();

        // Load palette back, it shoudl have 10 colors
        let loaded_palette = PaletteRGB::load_from_json(absolute_output_path);
        assert!(loaded_palette.is_ok());

        let loaded_palette = loaded_palette.unwrap();
        assert_eq!(loaded_palette.len(), 9);
    }

    #[test]
    fn test_palette_reduce_existing_palette() {
        tests_setup();
        let output_colors_count = 2;
        let test_palette_filename = "primary_reduced_palette.json";
        let absolute_input_path = get_palette_absolute_path(PRIMARY_PALETTE_FILENAME);
        let absolute_output_path = get_test_save_absolute_path(test_palette_filename);

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("palette")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-c")
            .arg(output_colors_count.to_string())
            .arg("-o")
            .arg(&absolute_output_path);
        cmd.assert().success();

        // Load palette back, it shoudl have 10 colors
        let loaded_palette = PaletteRGB::load_from_json(absolute_output_path);
        assert!(loaded_palette.is_ok());

        let loaded_palette = loaded_palette.unwrap();
        assert_eq!(loaded_palette.len(), output_colors_count);
    }
    
    #[test]
    fn test_palette_reduce_not_enough_colors_palette() {
        // cargo test --test integration_tests test_palette_reduce_not_enough_colors_palette -- --nocapture
        tests_setup();
        let output_colors_count = 4;
        let test_palette_filename = "primary_reduced_palette.json";
        let absolute_input_path = get_palette_absolute_path(PRIMARY_PALETTE_FILENAME);
        let absolute_output_path = get_test_save_absolute_path(test_palette_filename);

        let actual_colors_count = PaletteRGB::load_from_json(&absolute_input_path).unwrap().len();

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("palette")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-c")
            .arg(output_colors_count.to_string())
            .arg("-o")
            .arg(&absolute_output_path);
        let output = cmd.output();
        assert!(output.is_ok());
        
        let output = output.unwrap();
        assert!(matches!(output.status.code(), Some(1)));

        let stderr_text = output.stderr.iter()
            .filter_map(|v| char::from_u32(*v as u32))
            .collect::<String>();

        let expectd_err_text = PaletteError::NotEnoughColors { 
            expected: output_colors_count, 
            actual: actual_colors_count 
        }
        .to_string();
        assert!(stderr_text.trim().ends_with(&expectd_err_text), "Some other error message: '{stderr_text}'");
    }
    
    #[test]
    fn test_palette_bad_extension_colors_palette() {
        // cargo test --test integration_tests test_palette_bad_extension_colors_palette -- --nocapture
        tests_setup();
        let test_palette_filename = "primary_reduced_palette.json";
        let absolute_input_path = get_palette_absolute_path("corrupted_extension_file.meh");
        let absolute_output_path = get_test_save_absolute_path(test_palette_filename);

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("palette")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-c")
            .arg("2")
            .arg("-o")
            .arg(&absolute_output_path);
        let output = cmd.output();
        assert!(output.is_ok());
        
        let output = output.unwrap();
        assert!(matches!(output.status.code(), Some(1)));

        let stderr_text = output.stderr.iter()
            .filter_map(|v| char::from_u32(*v as u32))
            .collect::<String>();

        let expectd_err_text = "(os error 2)";
        assert!(stderr_text.contains(expectd_err_text), "Some other error message: '{stderr_text}'");
    }
    
}