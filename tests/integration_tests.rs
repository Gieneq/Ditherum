mod common;

use common::{
    get_test_image_absolute_path,
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
    image::{
        self, 
        generate_test_gradient_image, 
        ImageProcessor,
        ProcessingAlgorithm
    }, 
    palette::{
        errors::PaletteError, 
        PaletteRGB
    }
};
use ::image::Rgb;

/// Tests opening an image file and verifying its dimensions.
#[test]
fn test_image_opening() {
    let test_image = load_test_image(COLOR_PINK300_IMAGE_FILENAME);
    assert!(test_image.width() > 0);
    assert!(test_image.height() > 0);
}

/// Tests saving an image to disk.
#[test]
fn test_image_saving() {
    tests_setup();
    let test_image = load_test_image(COLOR_YELLOW600_IMAGE_FILENAME);
    let save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join("test_result.png");
    let result = image::save_image(save_path, &test_image);
    assert!(result.is_ok());
}

/// Tests generating a black-and-white palette from an image.
#[test]
fn test_obtaining_palette_from_bn_w_image() {
    tests_setup();
    let test_image = load_test_image(BNW_IMAGE_FILENAME);
    let palette = PaletteRGB::from_rgbu8_image(&test_image);

    // Expecting two colors: black and white.
    assert_eq!(palette.len(), 2);
}

/// Tests reducing a black-and-white palette to a single color.
#[test]
fn test_reducing_bn_w_palette() {
    tests_setup();
    let test_image = load_test_image(BNW_IMAGE_FILENAME);
    let palette = PaletteRGB::from_rgbu8_image(&test_image);
    assert_eq!(palette.len(), 2);
    let reduced_palette = palette.try_reduce(1);
    assert!(reduced_palette.is_ok());
}
/// Tests reducing a color palette while maintaining a certain number of colors.
#[test]
fn test_reducing_color_palette() {
    tests_setup();
    let test_image = load_test_image(COLOR_PINK300_IMAGE_FILENAME);
    let palette = PaletteRGB::from_rgbu8_image(&test_image);
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

/// Tests loading a valid primary color palette.
#[test]
fn test_load_primary_palette() {
    tests_setup();
    let palette_path = get_palette_absolute_path(PRIMARY_PALETTE_FILENAME);
    let palette = PaletteRGB::load_from_json(palette_path);
    assert!(palette.is_ok());

    let palette = palette.unwrap();
    assert_eq!(palette, PaletteRGB::primary());
}

/// Tests loading a corrupted palette file and handling parsing failure.
#[test]
fn test_load_corrupted_palette() {
    tests_setup();
    let palette_path = get_palette_absolute_path(CORRUPTED_PALETTE_FILENAME);
    let palette = PaletteRGB::load_from_json(palette_path);
    assert!(matches!(palette, Err(PaletteError::JsonParsingFailed(_))));
}

/// Tests handling an attempt to load a nonexistent palette file.
#[test]
fn test_load_not_existing_palette_palette() {
    tests_setup();
    let palette_path = get_palette_absolute_path("not_existing_file.json");
    let palette = PaletteRGB::load_from_json(palette_path);
    assert!(matches!(palette, Err(PaletteError::IoError(_))));
}

/// Tests saving and reloading a reduced color palette.
#[test]
fn test_saving_reduced_color_palette_and_loading_back() {
    tests_setup();
    let test_image = load_test_image(COLOR_GRASS300_IMAGE_FILENAME);
    let palette = PaletteRGB::from_rgbu8_image(&test_image);
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

/// Tests generating and saving a gradient image.
#[test]
fn test_gradient_generated_image_saving() {
    tests_setup();
    let test_image = generate_test_gradient_image(
        200, 
        80, 
        Rgb::<u8>([0,0,0]), 
        Rgb::<u8>([255,255,255]), 
    );

    let save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join("test_gradient_image_result.png");
    let result = image::save_image(save_path, &test_image);
    assert!(result.is_ok());
}

/// Tests thresholding in RGB space on a gradient image.
#[test]
fn test_thresholding_rgb_gradient_image() {
    tests_setup();
    let (width, height) = (200, 80);
    let gradient_image = generate_test_gradient_image(
        width, 
        height, 
        Rgb::<u8>([0,0,0]), 
        Rgb::<u8>([0,0,255]), 
    );
    // let palette = PaletteRGB::primary_bw();
    let palette = PaletteRGB::grayscale(30);

    // Reference image before dithering
    let save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join("test_gradient_image_before_dither.png");
    let result = image::save_image(save_path, &gradient_image);
    assert!(result.is_ok());

    // Processing
    let processing_result = ImageProcessor::new(gradient_image, palette)
        .with_algorithm(ProcessingAlgorithm::ThresholdingRgb)
        .run();
    assert_eq!(processing_result.width(), width);
    assert_eq!(processing_result.height(), height);
    
    // Saving processing results
    let thresh_rgb_save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join("test_gradient_image_thresholded_rgb.png");
    let result = image::save_image(thresh_rgb_save_path, &processing_result);
    assert!(result.is_ok());
}

/// Tests full processing of a pink image using an automatically extracted palette.
#[test]
fn test_full_processing_with_auto_palette_pink_image() {
    tests_setup();
    let test_image = load_test_image(COLOR_PINK300_IMAGE_FILENAME);

    let palette = PaletteRGB::from_rgbu8_image(&test_image)
        .try_reduce(12)
        .unwrap();
    let save_palette_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join("full_proc_palette.json");
    palette.save_to_json(save_palette_path).expect("Could not save palette");

    // Processing setup
    let processing_setup = [
        (ProcessingAlgorithm::ThresholdingRgb, "full_proc_thrsh_rgb.png"),
        (ProcessingAlgorithm::ThresholdingLab, "full_proc_thrsh_lab.png"),
        (ProcessingAlgorithm::FloydSteinbergRgb, "full_proc_dith_fs_rgb.png"),
    ];

    for (algorithm, filename) in processing_setup {
        let save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join(filename);
        let processing_result_rgb = ImageProcessor::new(test_image.clone(), palette.clone())
            .with_algorithm(algorithm)
            .run();
        
        let recreated_palette = PaletteRGB::from_rgbu8_image(&processing_result_rgb);
        assert_eq!(recreated_palette.len(), palette.len());

        // Saving processing results
        let result = image::save_image(&save_path, &processing_result_rgb);
        assert!(result.is_ok(), "Failed saving to {save_path:?}");
    }
}

#[test]
fn test_full_processing_with_auto_palette_grass_image() {
    tests_setup();
    let test_image = load_test_image(COLOR_GRASS300_IMAGE_FILENAME);

    let palette = PaletteRGB::from_rgbu8_image(&test_image)
        .try_reduce(12)
        .unwrap();
    let save_palette_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join("full_proc_grass_palette.json");
    palette.save_to_json(save_palette_path).expect("Could not save palette");

    // Processing setup
    let processing_setup = [
        (ProcessingAlgorithm::ThresholdingRgb, "full_proc_grass_thrsh_rgb.png"),
        (ProcessingAlgorithm::ThresholdingLab, "full_proc_grass_thrsh_lab.png"),
        (ProcessingAlgorithm::FloydSteinbergRgb, "full_proc_grass_dith_fs_rgb.png"),
    ];

    for (algorithm, filename) in processing_setup {
        let save_path = std::path::Path::new(SAVE_TEST_IMAGE_DIR).join(filename);
        let processing_result_rgb = ImageProcessor::new(test_image.clone(), palette.clone())
            .with_algorithm(algorithm)
            .run();

        let recreated_palette = PaletteRGB::from_rgbu8_image(&processing_result_rgb);
        assert_eq!(recreated_palette.len(), palette.len());
        
        // Saving processing results
        let result = image::save_image(&save_path, &processing_result_rgb);
        assert!(result.is_ok(), "Failed saving to {save_path:?}");
    }
}

#[cfg(test)]
mod tests_cli {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_palette_black_and_white_extraction() {
        tests_setup();
        let test_palette_filename = "sample_bw_palette.json";
        let absolute_input_path = get_test_image_absolute_path(BNW_IMAGE_FILENAME);
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
        let absolute_input_path = get_test_image_absolute_path(GRAY300_IMAGE_FILENAME);
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

    #[test]
    fn test_dither_simple() {
        // cargo test --test integration_tests test_dither_simple -- --nocapture
        tests_setup();
        let colors_count = 2;
        let test_output_image_filename = "simple_dithered_pink_image.png";
        let test_output_palette_filename = "simple_dithered_pink_palette.json";
        let absolute_input_path = get_test_image_absolute_path(COLOR_PINK300_IMAGE_FILENAME);
        let absolute_output_path = get_test_save_absolute_path(test_output_image_filename);
        let absolute_output_palette_path = get_test_save_absolute_path(test_output_palette_filename);

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("dither")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-c")
            .arg(colors_count.to_string())
            .arg("-o")
            .arg(&absolute_output_path)
            .arg("-r")
            .arg(&absolute_output_palette_path);
        let output = cmd.output();
        assert!(output.is_ok());

        let output = output.unwrap();
        assert!(output.status.success());
        
        // Load palette back, it shoudl have 2 colors
        let loaded_palette = PaletteRGB::load_from_json(absolute_output_palette_path);
        assert!(loaded_palette.is_ok());

        let loaded_palette = loaded_palette.unwrap();
        assert_eq!(loaded_palette.len(), colors_count);

        // Load image, it shoudl:
        // - has the samme width & height
        // - consist of 2 colors
        let base_image = image::load_image(absolute_input_path);
        assert!(base_image.is_ok());
        let base_image = base_image.unwrap();

        let loaded_image = image::load_image(absolute_output_path);
        assert!(loaded_image.is_ok());
        let loaded_image = loaded_image.unwrap();
        assert_eq!(base_image.width(), loaded_image.width());
        assert_eq!(base_image.height(), loaded_image.height());

        let palette_from_loaded_image = PaletteRGB::from_rgbu8_image(&loaded_image);
        assert_eq!(palette_from_loaded_image.len(), loaded_palette.len());
        assert_eq!(palette_from_loaded_image, loaded_palette);
    }
    
    #[test]
    fn test_dither_resize_width() {
        // cargo test --test integration_tests test_dither_resize_width -- --nocapture
        tests_setup();
        let target_width = 90;
        let test_output_image_filename = "resize_width_dithered_grass_image.png";
        let absolute_input_path = get_test_image_absolute_path(COLOR_GRASS300_IMAGE_FILENAME);
        let absolute_output_path = get_test_save_absolute_path(test_output_image_filename);

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("dither")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-W")
            .arg(target_width.to_string())
            .arg("-o")
            .arg(&absolute_output_path);
        let output = cmd.output();
        assert!(output.is_ok());

        let output = output.unwrap();
        assert!(output.status.success(), "cmd output={output:?}.");
        
        let (base_img_width, base_img_height) = {
            let base_img = image::load_image(absolute_input_path).unwrap();
            (base_img.width(), base_img.height())
        };
        let expected_height = (target_width as f32 * base_img_height as f32 / base_img_width as f32).round() as u32;

        let loaded_image = image::load_image(absolute_output_path);
        assert!(loaded_image.is_ok());
        let loaded_image = loaded_image.unwrap();
        assert_eq!(target_width, loaded_image.width());
        assert_eq!(expected_height, loaded_image.height());
    }
    
    #[test]
    fn test_dither_resize_height() {
        // cargo test --test integration_tests test_dither_resize_height -- --nocapture
        tests_setup();
        let target_height = 123;
        let test_output_image_filename = "resize_height_dithered_grass_image.png";
        let absolute_input_path = get_test_image_absolute_path(COLOR_GRASS300_IMAGE_FILENAME);
        let absolute_output_path = get_test_save_absolute_path(test_output_image_filename);

        // Generate black and white colors palette
        let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
        cmd
            .arg("dither")
            .arg("-i")
            .arg(&absolute_input_path)
            .arg("-H")
            .arg(target_height.to_string())
            .arg("-o")
            .arg(&absolute_output_path);
        let output = cmd.output();
        assert!(output.is_ok());

        let output = output.unwrap();
        assert!(output.status.success(), "cmd output={output:?}.");
        
        let (base_img_width, base_img_height) = {
            let base_img = image::load_image(absolute_input_path).unwrap();
            (base_img.width(), base_img.height())
        };
        let expected_width = (target_height as f32 * base_img_width as f32 / base_img_height as f32).round() as u32;

        let loaded_image = image::load_image(absolute_output_path);
        assert!(loaded_image.is_ok());
        let loaded_image = loaded_image.unwrap();
        assert_eq!(expected_width, loaded_image.width());
        assert_eq!(target_height, loaded_image.height());
    }
    
    #[test]
    fn test_dither_resize_multiple_widths() {
        // cargo test --test integration_tests test_dither_resize_multiple_widths -- --nocapture
        tests_setup();
        let target_width_range = (1..20).map(|idx| idx * 11);
        let absolute_input_path = get_test_image_absolute_path(COLOR_GRASS300_IMAGE_FILENAME);
        
        let (base_img_width, base_img_height) = {
            let base_img = image::load_image(&absolute_input_path).unwrap();
            (base_img.width(), base_img.height())
        };

        target_width_range.for_each(|target_width| {
            let test_output_image_filename = format!("resize_multiwidth_{target_width}px_dithered_grass_image.png");
            let absolute_output_path = get_test_save_absolute_path(test_output_image_filename);
    
            // Generate black and white colors palette
            let mut cmd: Command = Command::cargo_bin("ditherum").unwrap();
            cmd
                .arg("dither")
                .arg("-i")
                .arg(&absolute_input_path)
                .arg("-W")
                .arg(target_width.to_string())
                .arg("-o")
                .arg(&absolute_output_path);
            let output = cmd.output();
            assert!(output.is_ok());
    
            let output = output.unwrap();
            assert!(output.status.success(), "cmd output={output:?}.");
            let expected_height = (target_width as f32 * base_img_height as f32 / base_img_width as f32).round() as u32;
    
            let loaded_image = image::load_image(absolute_output_path);
            assert!(loaded_image.is_ok());
            let loaded_image = loaded_image.unwrap();
            assert_eq!(target_width, loaded_image.width());
            assert_eq!(expected_height, loaded_image.height());
        });
    }
    
}