use std::path::PathBuf;
use std::sync::OnceLock;
use std::{fmt::Debug, path::Path};

pub const TEST_IMAGES_DIR: &str = "./res/test_images";
pub const COLOR_PINK300_IMAGE_FILENAME: &str = "test_pink_300.jpg";
pub const COLOR_GRASS300_IMAGE_FILENAME: &str = "test_grass_300.png";
pub const COLOR_YELLOW600_IMAGE_FILENAME: &str = "test_yellow_600.jpg";
pub const GRAY300_IMAGE_FILENAME: &str = "test_gray_300.png";
pub const BNW_IMAGE_FILENAME: &str = "blackwhite.png";

pub const TEST_PALETTES_DIR: &str = "./res/test_palettes";
pub const PRIMARY_PALETTE_FILENAME: &str = "test_ok_palette.json";
pub const CORRUPTED_PALETTE_FILENAME: &str = "test_corrupted_palette.json";

pub const SAVE_TEST_IMAGE_DIR: &str = "./res/test_results";

/// Initializes the test environment by setting up logging and cleaning up the test results directory.
pub fn tests_setup() {
    static RESOURCE_INIT: OnceLock<()> = OnceLock::new();
    static LOGGER_INIT: OnceLock<()> = OnceLock::new();

    // Initialize logger if the logging feature is enabled.
    LOGGER_INIT.get_or_init(|| {
        if cfg!(feature = "logging") {
            env_logger::init();
        }
    });

    // Clear and recreate the results directory only once.
    RESOURCE_INIT.get_or_init(|| {
        log::info!("Initializing test resources...");
        let absolute_results_dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(SAVE_TEST_IMAGE_DIR);

        if absolute_results_dir_path.exists() {
            std::fs::remove_dir_all(&absolute_results_dir_path).unwrap_or_else(|e| {
                panic!("Failed to remove content of '{}', reason: {}", SAVE_TEST_IMAGE_DIR, e)
            });
        }
        assert!(!absolute_results_dir_path.exists());

        std::fs::create_dir_all(&absolute_results_dir_path).unwrap_or_else(|e| {
            panic!("Failed to create results directory, reason: {}", e)
        });
        assert!(absolute_results_dir_path.exists());
    });
}

/// Loads a test image from the specified filename.
/// Panics if the image cannot be loaded.
pub fn load_test_image<P>(path: P) -> image::RgbImage 
where 
    P: AsRef<Path> + Debug
{
    let absolute_test_image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(TEST_IMAGES_DIR)
        .join(&path);
    
    log::debug!(
        "Loading test file '{:?}' at absolute path '{:?}' ...", 
        path, 
        absolute_test_image_path
    );

    let img = ditherum::image::load_image(absolute_test_image_path)
        .unwrap_or_else(|e| panic!("Failed to open test image, reason: {}", e));

    log::debug!("Image loaded: width={}, height={}", img.width(), img.height());
    img
}

pub fn get_image_absolute_path<P>(filename: P) -> PathBuf 
where 
    P: AsRef<Path>
{
    Path::new(env!("CARGO_MANIFEST_DIR")).join(TEST_IMAGES_DIR).join(filename)
}

pub fn get_palette_absolute_path<P>(filename: P) -> PathBuf 
where 
    P: AsRef<Path>
{
    Path::new(env!("CARGO_MANIFEST_DIR")).join(TEST_PALETTES_DIR).join(filename)
}

pub fn get_test_save_absolute_path<P>(filename: P) -> PathBuf 
where 
    P: AsRef<Path>
{
    Path::new(env!("CARGO_MANIFEST_DIR")).join(SAVE_TEST_IMAGE_DIR).join(filename)
}