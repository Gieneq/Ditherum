use std::{collections::HashSet, ops::Deref, vec};
use palette::{color_difference::Ciede2000, FromColor, Lab, Srgb};
use rand::seq::IndexedRandom;

use image::{Rgb, RgbImage};

#[derive(Debug, Clone)]
pub struct PaletteRGB(Vec<Rgb<u8>>);

#[derive(Debug, thiserror::Error)]
pub enum PaletteError {

    #[error("Not enough colors to be converted to. Expected={expected} but actual={actual}.")]
    NotEnoughColors {
        expected: usize,
        actual: usize
    }
}

impl PaletteRGB {
    pub fn from_hashset(input_set: HashSet<Rgb<u8>>) -> Self {
        PaletteRGB(input_set.into_iter().collect())
    }
    
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

    pub fn black_n_white() -> Self {
        PaletteRGB(vec![
            Rgb::<u8>([0, 0, 0]),
            Rgb::<u8>([255, 255, 255])
        ])
    }

    fn find_centroids(input: &[Lab], centroids_count: usize) -> Vec<Lab> {
        const THRESHOLD_FACTOR: f32 = 0.01;
        const THRESHOLD_MIN: usize = 2;
        
        // Apply K-mean clustering
        let mut rng = rand::rng();
        
        // Centorids - centers of cluester. At first choosen randomly
        let mut centroids = input.choose_multiple(&mut rng, centroids_count).cloned().collect::<Vec<_>>();
        let mut clusters: Vec<Vec<Lab>> = vec![vec![]; centroids_count];
        assert_eq!(centroids.len(), clusters.len());
        
        // Helper vectors to store values used in convergence check
        let mut last_counts;
        let mut recent_counts: Vec<usize> = vec![0; centroids_count];
        let threshold = ((input.len() as f32 * THRESHOLD_FACTOR).round() as usize).max(THRESHOLD_MIN);

        fn check_converges(last_counts: &[usize], recent_counts: &[usize], threshold: usize) -> bool {
            last_counts.iter()
                .zip(recent_counts.iter())
                .all(|(last, recent)| last.abs_diff(*recent) < threshold)
        }

        let mut iterations_count = 0;
        loop {
            iterations_count += 1;

            // Clear last clusters
            clusters.iter_mut().for_each(|c| c.clear());

            // Populate clusters based on distance from centroids
            input.iter().for_each(|color| {
                // Find closest controid and place value to coresponding cluster
                let (selected_centroid_idx, _) = centroids.iter()
                    .enumerate()
                    .map(|(centroid_idx, centroid_color)| (centroid_idx, centroid_color.difference(*color)))
                    .min_by(|(_, color_distance), (_, other_color_distance)| color_distance.partial_cmp(other_color_distance).unwrap() )
                    .unwrap();
                clusters[selected_centroid_idx].push(*color);

            });

            // For each cluster find new centroids
            centroids = clusters.iter().map(|cluster| {
                let items_count = cluster.len();
                // For sure there is at least 1 element in each cluster

                let accumulator = cluster.iter().fold([0f32; 3], {|acc, color|
                    [
                        acc[0] + color.l,
                        acc[1] + color.a,
                        acc[2] + color.b
                    ]
                });

                // Mean LAB value
                Lab::new(
                    accumulator[0] / items_count as f32,
                    accumulator[1] / items_count as f32,
                    accumulator[2] / items_count as f32
                )
            }).collect::<Vec<_>>();

            // Check convergence
            last_counts = recent_counts;
            recent_counts = clusters.iter().map(|cluster| cluster.len()).collect();
            log::debug!("Iteration {}: centroids={:?}", iterations_count, centroids);

            if check_converges(&last_counts, &recent_counts, threshold) {
                log::debug!("Find K-mean clustering solution in {iterations_count} iterations!");
                break;
            }
        }

        centroids
    } 

    pub fn reduce_to(self, target_colors_count: usize) -> Result<Self, PaletteError> {
        match self.len().cmp(&target_colors_count) {

            // Cannot obtain bigger pallete than the input pallet size
            std::cmp::Ordering::Less => Err(PaletteError::NotEnoughColors { 
                expected: target_colors_count, 
                actual: self.len() 
            }),

            // Te same pallet
            std::cmp::Ordering::Equal => Ok(self),

            // Reduce colors count
            std::cmp::Ordering::Greater => {

                // Glue code to stick image crate with palette crate
                let srgb_colors = self.iter().map(|c| {
                    Srgb::new(
                        c[0] as f32 / 255.0,
                        c[1] as f32 / 255.0,
                        c[2] as f32 / 255.0
                    )
                }).collect::<Vec<_>>();

                // Use LAB for better percetion based matching
                let lab_colors = Vec::<Lab>::from_color(srgb_colors);

                // Apply clusterization to find best fitting centroids
                let new_lab_colors = Self::find_centroids(&lab_colors, target_colors_count);

                // Back to sRGB color space
                let new_rgb_colors = Vec::<Srgb>::from_color(new_lab_colors);

                // Glue code to stick palette crate with image crate
                let result_rgb_colors = new_rgb_colors.into_iter().map(|c| {
                    Rgb([
                        (c.red * 255.0).round() as u8,
                        (c.green * 255.0).round() as u8,
                        (c.blue * 255.0).round() as u8
                    ])
                }).collect::<Vec<_>>();

                Ok(PaletteRGB(result_rgb_colors))
            },
        }

    }
}

impl Deref for PaletteRGB {
    type Target = Vec<Rgb<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
