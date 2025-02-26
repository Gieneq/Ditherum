use std::{collections::HashSet, ops::Deref, vec};
use palette::{color_difference::Ciede2000, FromColor, Lab, Srgb};
use image::{Rgb, RgbImage};
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
    }

    impl From<CentroidsFindError> for PaletteError {
        fn from(value: CentroidsFindError) -> Self {
            Self::ConvertionErrot(value)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaletteRGB(Vec<Rgb<u8>>);

impl PaletteRGB {
    /// Constructs a palette from a `HashSet` of `Rgb<u8>` colors.
    pub fn from_hashset(input_set: HashSet<Rgb<u8>>) -> Self {
        PaletteRGB(input_set.into_iter().collect())
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
            Rgb([0, 0, 0]),
            Rgb([255, 255, 255]),
        ])
    }

    /// Returns a palette of primary colors: red, green, and blue.
    pub fn primary() -> Self {
        PaletteRGB(vec![
            Rgb([255, 0, 0]),
            Rgb([0, 255, 0]),
            Rgb([0, 0, 255]),
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
                Rgb([channel_value, channel_value, channel_value])
            })
            .collect::<Vec<_>>();

        PaletteRGB(colors)
    }

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
            Rgb([
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
    type Target = Vec<Rgb<u8>>;

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
        assert_eq!(palette[0], Rgb([0, 0, 0]));
        assert_eq!(palette[steps - 1], Rgb([255, 255, 255]));
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
        assert_eq!(reduced_color, Rgb::<u8>([119, 119, 119]));
    }
}