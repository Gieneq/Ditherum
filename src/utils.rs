pub mod algorithms {
    use std::fmt::Debug;
    use rand::seq::IndexedRandom;
    
    const CONVERGE_THRESHOLD: f32 = 0.01;
    const ITERATION_MAX_COUNT: usize = 40;

    /// Errors that can occur while finding centroids using the K-means algorithm.
    #[derive(Debug, thiserror::Error)]
    pub enum CentroidsFindError {
        /// The algorithm exceeded the maximum allowed number of iterations without converging.
        #[error("TooManyIterations")]
        TooManyIterations,

        /// The input contains fewer elements than the requested number of centroids.
        #[error("TooManyCentroids expected={expected}, actual={actual}")]
        TooManyCentroids {
            expected: usize,
            actual: usize
        },

        /// The input data is empty.
        #[error("InputEmpty")]
        InputEmpty,
    }

    /// Validates the input data for the K-means clustering algorithm.
    ///
    /// The function checks two conditions:
    /// - The input slice must not be empty.
    /// - The number of input elements must be at least as many as `centroids_count`.
    ///
    /// # Parameters
    ///
    /// * `input` - A slice of input data points.
    /// * `centroids_count` - The number of centroids requested.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the input is valid, or a corresponding [`CentroidsFindError`] otherwise.
    fn validate_input<T>(input: &[T], centroids_count: usize) -> Result<(), CentroidsFindError> 
    where 
        T: Debug + Copy + Clone
    {
        if input.is_empty() {
            Err(CentroidsFindError::InputEmpty)
        } else if input.len() < centroids_count {
            Err(CentroidsFindError::TooManyCentroids { expected: centroids_count, actual: input.len() })
        } else {
            Ok(())
        }
    }

    /// Finds the index of the centroid closest to the given item.
    ///
    /// # Parameters
    ///
    /// * `item` - A reference to the data point for which the nearest centroid is required.
    /// * `centroids` - A slice of candidate centroid points.
    /// * `distance_measure` - A function or closure that computes the distance between two points.
    ///
    /// # Returns
    ///
    /// The index (in `centroids`) of the centroid that is closest to `item`.
    fn find_closest_centroid_idx<T, D>(
        item: &T,
        centroids: &[T],
        distance_measure: &D
    ) -> usize 
    where 
        T: Debug + Copy + Clone,
        D: Fn(&T, &T) -> f32,
    {
        let distances_to_centroids = centroids.iter()
            .enumerate()
            .map(|(centroid_idx, centroid_value)| {
                (centroid_idx, distance_measure(item, centroid_value))
            });

        // Unwrape should be safe here, distance shoul be valid not-negative value.
        let closest_centroid = distances_to_centroids
            .min_by(|(_, a_dist), (_, b_dist)| {
                a_dist.partial_cmp(b_dist).expect("Distance comparison failed")
            });
        
        // Safe unwrap, assignment is not empty
        let (closest_centroid_idx, _) = closest_centroid.unwrap();
        closest_centroid_idx
    }

    /// Assigns each item in the input slice to the closest centroid.
    ///
    /// # Parameters
    ///
    /// * `input` - A slice of data points.
    /// * `centroids` - A slice of current centroid points.
    /// * `distance_measure` - A function or closure that calculates the distance between two points.
    ///
    /// # Returns
    ///
    /// A vector of clusters, where each cluster is a vector of data points assigned to one centroid.
    fn create_clusters_assignment<T, D>(
        input: &[T],
        centroids: &[T],
        distance_measure: &D
    ) -> Vec<Vec<T>>
    where
        T: Debug + Copy + Clone,
        D: Fn(&T, &T) -> f32
    {
        let centroids_count = centroids.len();
        let mut clusters = vec![vec![]; centroids_count];

        // Fill clusters based on distance to centroids
        input.iter().for_each(|item| {
            let closest_centroid_idx = find_closest_centroid_idx(
                item, 
                centroids, 
                distance_measure
            );
            clusters[closest_centroid_idx].push(*item);
        });
        clusters
    }
    
    /// Checks whether the centroids have converged.
    ///
    /// Convergence is determined by comparing the positions of centroids between iterations.
    /// If the distance between each corresponding pair of centroids is below the specified
    /// `distance_threshold`, the centroids are considered converged.
    ///
    /// # Parameters
    ///
    /// * `last_centroids` - The centroids from the previous iteration.
    /// * `recent_centroids` - The centroids from the current iteration.
    /// * `distance_threshold` - The maximum allowed change for convergence.
    /// * `distance_measure` - A function or closure to calculate the distance between two points.
    ///
    /// # Returns
    ///
    /// `true` if all centroids have converged (i.e., the distance change is below the threshold), else `false`.
    fn check_converges<T, D>(
        last_centroids: &[T], 
        recent_centroids: &[T],
        distance_threshold: f32,
        distance_measure: &D
    ) -> bool
    where 
        T: Debug + Copy + Clone,
        D: Fn(&T, &T) -> f32,
    {
        last_centroids.iter()
            .zip(recent_centroids.iter())
            .all(|(last, recent)| distance_measure(last, recent) < distance_threshold)
    }

    /// Computes new centroids by calculating the mean of each cluster.
    ///
    /// # Parameters
    ///
    /// * `clusters` - A slice of clusters, each represented as a vector of data points.
    /// * `calculate_mean` - A function or closure that computes the mean of a slice of points.
    ///
    /// # Returns
    ///
    /// A vector of new centroids, each computed as the mean of the corresponding cluster.
    fn create_centroids_from_clusters<T, M>(
        clusters: &[Vec<T>],
        calculate_mean: &M
    ) -> Vec<T>
    where 
        T: Debug + Copy + Clone,
        M: Fn(&[T]) -> T
    {
        clusters.iter()
            .map(|cluster| calculate_mean(cluster))
            .collect()
    }

    /// Performs K-means clustering to find a set of centroids for the input data.
    ///
    /// This function implements a K-means clustering algorithm that repeatedly assigns data
    /// points to the nearest centroid and then updates centroids by computing the mean of
    /// the assigned points. Iteration continues until the centroids converge (i.e., change less
    /// than a specified threshold) or the maximum number of iterations is reached.
    ///
    /// # Parameters
    ///
    /// * `input` - A slice of input data points.
    /// * `centroids_count` - The number of centroids (clusters) to compute.
    /// * `distance_measure` - A closure that computes the distance between two points.
    /// * `calculate_mean` - A closure that computes the mean of a slice of data points.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<T>)` containing the computed centroids if the algorithm converges,
    /// or a [`CentroidsFindError`] if an error occurs (e.g., too many iterations, input is empty).
    ///
    /// # Examples
    ///
    /// The following example demonstrates how to use `find_centroids` with floating-point data:
    ///
    /// ```
    /// use ditherum::algorithms::{find_centroids, CentroidsFindError};
    ///
    ///  // Define input data.
    ///  let input_data: Vec<f32> = vec![1.0, 2.0, 9.0, 7.0, 8.0, 22.0, 24.0, 3.0];
    ///  let centroids_count = 3;
    ///
    ///  // Define a simple absolute difference as the distance measure.
    ///  let distance_measure = |a: &f32, b: &f32| (a - b).abs();
    ///
    ///  // Define the mean calculation as the arithmetic mean.
    ///  let calculate_mean = |arr: &[f32]| arr.iter().sum::<f32>() / arr.len() as f32;
    ///
    ///  // Run the K-means clustering algorithm.
    ///  let centroids = find_centroids(
    ///      &input_data,
    ///      centroids_count,
    ///      distance_measure,
    ///      calculate_mean
    ///  );
    ///
    ///  println!("Computed centroids: {:?}", centroids);
    /// ```
    pub fn find_centroids<T, D, M>(
        input: &[T], 
        centroids_count: usize,
        distance_measure: D,
        calculate_mean: M
    
    ) -> Result<Vec<T>, CentroidsFindError>
    where 
        T: Debug + Copy + Clone,
        D: Fn(&T, &T) -> f32,
        M: Fn(&[T]) -> T
    {
        validate_input(input, centroids_count)?;

        // If the number of input points equals the requested centroids count,
        // return the input data as the centroids.
        if input.len() == centroids_count {
            return Ok(input.to_vec());
        }

        let mut rng = rand::rng();

        let mut last_centroids;
        let mut centroids = input
            .choose_multiple(&mut rng, centroids_count)
            .copied()
            .collect::<Vec<_>>();
        let mut clusters;
        let mut iterations_count = 0;

        loop {
            iterations_count += 1;

            if iterations_count > ITERATION_MAX_COUNT {
                return Err(CentroidsFindError::TooManyIterations);
            }

            // Assign each input point to the nearest centroid.
            clusters = create_clusters_assignment(input, &centroids, &distance_measure);
            log::debug!("Clusters: {clusters:?}");

            // Compute new centroids as the mean of the clusters.
            last_centroids = centroids;
            centroids = create_centroids_from_clusters(&clusters, &calculate_mean);

            // Check if the centroids have converged.
            if check_converges(
                &last_centroids, 
                &centroids, 
                CONVERGE_THRESHOLD,
                &distance_measure
            ) {
                log::debug!("Found solution after {iterations_count} iterations!");
                break;
            }
        }

        Ok(centroids)
    }         
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_centroid_float() {
            let input_data: Vec<f32> = vec![1.0, 2.0, 9.0, 7.0, 8.0, 22.0, 24.0, 3.0];
            let centroids_count = 3;
            let distance_measure = |a: &f32, b: &f32| { (a - b).abs() };
            let calculate_mean = |arr: &[f32]| { arr.iter().sum::<f32>() / arr.len() as f32 };

            let centroids = find_centroids(
                &input_data, 
                centroids_count, 
                distance_measure, 
                calculate_mean
            );

            assert!(matches!(centroids, Ok(_)));
            let centroids = centroids.unwrap();
            assert_eq!(centroids.len(), 3);
        }
    }
}


