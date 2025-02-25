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


#[cfg(test)]
mod tests_image {
    const TEST_IMAGES_DIR: &str = "./res/test_images";
    const COLOR_IMAGE_FILENAME: &str = "karambola.PNG";
    const BNW_IMAGE_FILENAME: &str = "test_ok_image.jpg";

    const SAVE_TEST_IMAGE_DIR: &str = "./res/test_results";
    const SAVE_TEST_FILENAME: &str = "test_result.png";

    use image::{ImageResult, RgbImage};

    use std::{fmt::Debug, path::Path};

    fn load_test_image<P>(path: P) -> RgbImage 
    where 
        P: AsRef<Path> + Debug
    {
        let absolute_test_imge_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(TEST_IMAGES_DIR).join(&path);
        
        println!("Loading test file '{:?}' at absolute path '{:?}' ...", path, absolute_test_imge_path);

        let img = crate::image::load_image(absolute_test_imge_path).unwrap_or_else(|e| {
            panic!("Failed to open test image, reason {e}")
        });
        println!("Loading image done! Img width={}, height={}", img.width(), img.height());
        img
    }

    fn save_test_image(img: &RgbImage) -> ImageResult<()> {
        let path = Path::new(SAVE_TEST_IMAGE_DIR).join(SAVE_TEST_FILENAME);
        let result = crate::image::save_image(path, img);
        println!("{result:?}");
        result
    }

    fn prepare_resource_directory() {
        let dirpath = Path::new(env!("CARGO_MANIFEST_DIR")).join(SAVE_TEST_IMAGE_DIR);

        // Clear content of results dir
        if dirpath.exists() {
            std::fs::remove_dir_all(&dirpath).unwrap_or_else(|e| {
                panic!("Failed to remove content of '{SAVE_TEST_IMAGE_DIR}', reason {e}")
            });
        }
        assert!(!dirpath.exists());

        // Check if results dir exists
        std::fs::create_dir_all(&dirpath).unwrap_or_else(|e| {
            panic!("Failed to create results dir, reason {e}")
        });
        assert!(dirpath.exists());
    }

    #[test]
    fn test_image_opening() {
        let test_image = load_test_image(COLOR_IMAGE_FILENAME);
        assert!(test_image.width() > 0);
        assert!(test_image.height() > 0);
    }

    #[test]
    fn test_image_saving() {
        prepare_resource_directory();
        let test_image = load_test_image(COLOR_IMAGE_FILENAME);
        assert!(matches!(save_test_image(&test_image), Ok(())));
    }

    #[test]
    fn test_obtaining_pallet_from_b_n_w_image() {
        prepare_resource_directory();
        let test_image = load_test_image(BNW_IMAGE_FILENAME);
        assert!(matches!(save_test_image(&test_image), Ok(())));
    }
}
