use std::{collections::HashSet, fs::File, io::{BufReader, BufWriter}, ops::Deref, path::Path, vec};
use errors::PaletteError;
use palette::{color_difference::Ciede2000, FromColor, Lab, Srgb};
use image::{Rgb, RgbImage};
use serde::{Serialize, Deserialize};
use super::utils::algorithms;

pub mod errors {
    use crate::utils::algorithms::CentroidsFindError;

    #[derive(Debug, thiserror::Error)]
    pub enum PaletteError {
        #[error("Not enough colors to be converted to. Expected={expected} but actual={actual}.")]
        NotEnoughColors {
            expected: usize,
            actual: usize
        },

        #[error("Faild to convert, reason={0}")]
        ConvertionErrot(CentroidsFindError),

        #[error("I/O error, reason={0}")]
        IoError(std::io::Error),

        #[error("JSON parsing failed, reason={0}")]
        JsonParsingFailed(serde_json::error::Error),
    }

    impl From<CentroidsFindError> for PaletteError {
        fn from(value: CentroidsFindError) -> Self {
            Self::ConvertionErrot(value)
        }
    }

    impl From<std::io::Error> for PaletteError {
        fn from(value: std::io::Error) -> Self {
            Self::IoError(value)
        }
    }

    impl From<serde_json::error::Error> for PaletteError {
        fn from(value: serde_json::error::Error) -> Self {
            Self::JsonParsingFailed(value)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColorRGB([u8; 3]);

impl ColorRGB {
    pub fn red(&self) -> u8 {
        self[0]
    }
    
    pub fn green(&self) -> u8 {
        self[1]
    }
    
    pub fn blue(&self) -> u8 {
        self[2]
    }
}

impl Deref for ColorRGB {
    type Target = [u8; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Rgb<u8>> for ColorRGB {
    fn from(value: Rgb<u8>) -> Self {
        ColorRGB(value.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PaletteRGB(Vec<ColorRGB>);

impl PaletteRGB {
    /// Constructs a palette from a `HashSet` of `Rgb<u8>` colors.
    pub fn from_hashset(input_set: HashSet<Rgb<u8>>) -> Self {
        PaletteRGB(input_set.into_iter().map(|c| c.into()).collect())
    }
    
    /// Extracts a palette from an image by collecting unique pixel colors.
    pub fn from_image(img: &RgbImage) -> Self {
        let mut palette_set = HashSet::new();

        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                palette_set.insert(*pixel);
            }
        }

        Self::from_hashset(palette_set)
    }

    /// Returns a palette containing only black and white.
    pub fn black_and_white() -> Self {
        PaletteRGB(vec![
            ColorRGB([0, 0, 0]),
            ColorRGB([255, 255, 255]),
        ])
    }

    /// Returns a palette of primary colors: red, green, and blue.
    pub fn primary() -> Self {
        PaletteRGB(vec![
            ColorRGB([255, 0, 0]),
            ColorRGB([0, 255, 0]),
            ColorRGB([0, 0, 255]),
        ])
    }

    /// Returns a grayscale palette with the specified number of steps.
    ///
    /// # Example
    ///
    /// ```
    /// use ditherum::palette::PaletteRGB;
    /// 
    /// let palette = PaletteRGB::grayscale(5);
    /// 
    /// println!("{palette:?}");
    /// // Produces: [black, dark gray, medium gray, light gray, white]
    /// ```
    pub fn grayscale(steps: usize) -> PaletteRGB {
        assert!(steps >= 2, "Grayscale palette requires at least two steps.");

        let colors = (0..steps)
            .map(|step| {
                let channel_value = ((255 * step) / (steps - 1)) as u8;
                ColorRGB([channel_value, channel_value, channel_value])
            })
            .collect::<Vec<_>>();

        PaletteRGB(colors)
    }

    /// Attempts to reduce the number of colors in the palette to a specified target count.
    ///
    /// This method is useful when you want to simplify a color palette by reducing the number
    /// of distinct colors while preserving the overall color harmony as much as possible. It 
    /// uses a clustering technique to find the best fitting centroids that represent the reduced 
    /// color set.
    ///
    /// # Parameters
    /// - `target_colors_count`: The desired number of colors in the reduced palette.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the palette was successfully reduced to the target number of colors.
    /// - `Err(PaletteError::NotEnoughColors)`: If the requested number of colors is greater than 
    ///   the current number of colors in the palette.
    ///
    /// # Errors
    /// - `PaletteError::NotEnoughColors`: Returned when the requested number of colors is greater 
    ///   than the available number of colors in the palette.
    ///
    /// # Panics
    /// This method does not panic.
    ///
    /// # Example
    /// ```
    /// use ditherum::palette::PaletteRGB;
    /// 
    /// let palette = PaletteRGB::primary();
    ///
    /// let reduced_palette = palette.try_reduce(2).expect("Failed to reduce colors");
    /// println!("{:?}", reduced_palette);
    /// ```
    ///
    /// In this example, the palette is reduced to 2 colors while maintaining the color balance
    /// using a clustering algorithm to find the best fitting centroids.
    pub fn try_reduce(self, target_colors_count: usize) -> Result<Self, self::errors::PaletteError> {
        match self.len().cmp(&target_colors_count) {

            // Cannot obtain bigger pallete than the input pallet size
            std::cmp::Ordering::Less => Err(self::errors::PaletteError::NotEnoughColors { 
                expected: target_colors_count, 
                actual: self.len() 
            }),

            // Te same pallet
            std::cmp::Ordering::Equal => Ok(self),

            // Reduce colors count
            std::cmp::Ordering::Greater => {

                let lab_colors: Vec<Lab> = self.into();

                // Apply clusterization to find best fitting centroids
                let new_lab_colors = find_lab_colors_centroids(
                    &lab_colors, 
                    target_colors_count
                )?;

                Ok(new_lab_colors.into())
            },
        }
    }

    /// Saves the palette to a JSON file at the specified path.
    ///
    /// # Parameters
    /// - `path`: The file path where the JSON data should be saved.
    ///
    /// # Errors
    /// - Returns an `io::Error` if there is an issue creating or writing to the file.
    ///
    /// # Example
    /// ```
    /// use ditherum::palette::PaletteRGB;
    /// 
    /// let palette = PaletteRGB::primary();
    /// 
    /// palette.save_to_json("tmp_palette.json").expect("Failed to save palette");
    /// ```
    pub fn save_to_json<P>(&self, path: P) -> Result<(), PaletteError> 
    where 
        P: AsRef<Path>
    {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
    
    /// Loads the palette from a JSON file at the specified path.
    ///
    /// # Parameters
    /// - `path`: The file path from which to read the JSON data.
    ///
    /// # Returns
    /// - `Ok(PaletteRGB)`: If the JSON data is successfully parsed into a `PaletteRGB`.
    /// - `Err(io::Error)`: If there is an issue reading the file.
    /// - `Err(serde_json::Error)`: If there is an issue parsing the JSON data.
    ///
    /// # Example
    /// ```
    /// use ditherum::palette::PaletteRGB;
    /// 
    /// let palette = PaletteRGB::load_from_json("tmp_palette.json").expect("Failed to load palette");
    /// println!("{:?}", palette);
    /// ```
    pub fn load_from_json<P>(path: P) -> Result<Self, PaletteError> 
    where 
        P: AsRef<Path>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let pallete = serde_json::from_reader(reader)?;
        Ok(pallete)
    }

}

impl From<PaletteRGB> for Vec<Lab> {
    fn from(value: PaletteRGB) -> Self {
        let srgb_colors = value.iter().map(|c| {
            Srgb::new(
                c[0] as f32 / 255.0,
                c[1] as f32 / 255.0,
                c[2] as f32 / 255.0
            )
        }).collect::<Vec<_>>();

        Vec::<Lab>::from_color(srgb_colors)
    }
}

/// Allows conversion from a vector of Lab colors into a `PaletteRGB`.
impl From<Vec<Lab>> for PaletteRGB {
    fn from(value: Vec<Lab>) -> Self {
        let new_rgb_colors = Vec::<Srgb>::from_color(value);

        let result_rgb_colors = new_rgb_colors.into_iter().map(|c| {
            ColorRGB([
                (c.red * 255.0).round() as u8,
                (c.green * 255.0).round() as u8,
                (c.blue * 255.0).round() as u8
            ])
        }).collect::<Vec<_>>();

        PaletteRGB(result_rgb_colors)
    }
}

/// Allows treating `PaletteRGB` as a slice of `Rgb<u8>`.
impl Deref for PaletteRGB {
    type Target = Vec<ColorRGB>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Clusters Lab colors using k-means and returns the new Lab centroids.
///
/// # Parameters
///
/// - `input`: A slice of Lab colors.
/// - `centroids_count`: The number of centroids to compute.
///
/// # Returns
///
/// A `Result` containing a vector of new Lab centroids or an error if clustering fails.
fn find_lab_colors_centroids(
    input: &[Lab], 
    centroids_count: usize
) -> Result<Vec<Lab>, algorithms::CentroidsFindError> {
    let lab_distance_measure = |a: &Lab, b: &Lab| {
        a.difference(*b)
    };

    let calculate_lab_mean = |arr: &[Lab]| {
        let mut accumulator = arr.iter()
            .fold(Lab::new(0.0, 0.0, 0.0), |mut acc, item| {
                acc.l += item.l;
                acc.a += item.a;
                acc.b += item.b;
                acc
            });
        accumulator.l /= arr.len() as f32;
        accumulator.a /= arr.len() as f32;
        accumulator.b /= arr.len() as f32;
        accumulator
    };

    algorithms::find_centroids(
        input, 
        centroids_count, 
        lab_distance_measure, 
        calculate_lab_mean
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale_palette() {
        let steps = 113;
        let palette = PaletteRGB::grayscale(steps);
        assert_eq!(palette.len(), steps);

        // Check endpoints are black and white.
        assert_eq!(palette[0], ColorRGB([0, 0, 0]));
        assert_eq!(palette[steps - 1], ColorRGB([255, 255, 255]));
    }

    #[test]
    fn test_try_reduce_not_enough_colors() {
        // Create a palette with only three colors.
        let palette = PaletteRGB::primary();

        // Trying to reduce to 4 colors should fail.
        let result = palette.clone().try_reduce(4);
        assert!(result.is_err());

        if let Err(errors::PaletteError::NotEnoughColors { expected, actual }) = result {
            assert_eq!(expected, 4);
            assert_eq!(actual, palette.len());
        } else {
            panic!("Expected NotEnoughColors error.");
        }
    }

    #[test]
    fn test_reduce_bn_w_palette() {
        let palette = PaletteRGB::black_and_white();
        assert_eq!(palette.len(), 2);

        let reduced_palette = palette.try_reduce(1);
        assert!(reduced_palette.is_ok());
        let reduced_palette = reduced_palette.unwrap();
        let reduced_color = reduced_palette[0];
        assert_eq!(reduced_color, ColorRGB([119, 119, 119]));
    }
}