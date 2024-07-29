use super::{PointProvider, PointProviderLab, QuantizerResult};
use crate::{
    color::{Argb, Lab},
    utils::random::Random,
    IndexMap,
};
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
use core::cmp::Ordering;
#[cfg(feature = "std")]
use std::{
    format,
    string::String,
    time::Instant,
    {vec, vec::Vec},
};

struct DistanceAndIndex {
    distance: f64,
    index: usize,
}

impl DistanceAndIndex {
    pub const fn new(distance: f64, index: usize) -> Self {
        Self { distance, index }
    }
}

impl Eq for DistanceAndIndex {}
impl PartialEq for DistanceAndIndex {
    fn eq(&self, other: &Self) -> bool {
        self.distance != other.distance
    }
}

impl Ord for DistanceAndIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            Ordering::Less
        } else if self.distance > other.distance {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for DistanceAndIndex {
    fn lt(&self, other: &Self) -> bool {
        self.distance < other.distance
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct QuantizerWsmeans;

impl QuantizerWsmeans {
    const DEBUG: bool = false;

    #[cfg(feature = "std")]
    fn debug_log<T: Into<String>>(log: T) {
        if Self::DEBUG {
            let log: String = log.into();

            std::println!("{log}");
        }
    }

    pub fn quantize(
        input_pixels: &[Argb],
        max_colors: usize,
        starting_clusters: &[Argb],
    ) -> QuantizerResult {
        let mut pixel_to_count: IndexMap<Argb, u32> = IndexMap::default();
        let mut points: Vec<Lab> = vec![];
        let mut pixels: Vec<Argb> = vec![];

        for input_pixel in input_pixels {
            let pixel_count = pixel_to_count.get_mut(input_pixel);

            if let Some(pixel_count) = pixel_count {
                *pixel_count += 1;
            } else {
                pixels.push(*input_pixel);
                points.push(PointProviderLab::lab_from_int(input_pixel));
                pixel_to_count.insert(*input_pixel, 1);
            }
        }

        let cluster_count = max_colors.min(points.len());

        let mut clusters = starting_clusters
            .iter()
            .map(PointProviderLab::lab_from_int)
            .collect::<Vec<_>>();
        let additional_clusters_needed = cluster_count - clusters.len();

        if additional_clusters_needed > 0 {
            let mut seed_generator = Random::new(0x42688);
            let mut indices = vec![];

            for _ in 0..additional_clusters_needed {
                // Use existing points rather than generating random centroids.
                //
                // KMeans is extremely sensitive to initial clusters. This quantizer
                // is meant to be used with a Wu quantizer that provides initial
                // centroids, but Wu is very slow on unscaled images and when extracting
                // more than 256 colors.
                //
                // Here, we can safely assume that more than 256 colors were requested
                // for extraction. Generating random centroids tends to lead to many
                // "empty" centroids, as the random centroids are nowhere near any pixels
                // in the image, and the centroids from Wu are very refined and close
                // to pixels in the image.
                //
                // Rather than generate random centroids, we'll pick centroids that
                // are actual pixels in the image, and avoid duplicating centroids.

                let mut index = seed_generator.next_range(points.len() as i32) as usize;

                while indices.contains(&index) {
                    index = seed_generator.next_range(points.len() as i32) as usize;
                }

                indices.push(index);
            }

            for index in indices {
                clusters.push(points[index]);
            }
        }

        #[cfg(feature = "std")]
        Self::debug_log(format!(
            "have {} starting clusters, {} points",
            clusters.len(),
            points.len()
        ));

        let mut cluster_indices = fill_array(points.len(), |index| index % cluster_count);
        let mut index_matrix = vec![vec![0; cluster_count]; cluster_count];

        let mut distance_to_index_matrix: Vec<Vec<DistanceAndIndex>> =
            fill_array(cluster_count, |_| {
                fill_array(cluster_count, |index| DistanceAndIndex::new(0.0, index))
            });
        let mut pixel_count_sums = vec![0; cluster_count];

        for iteration in 0..10 {
            if Self::DEBUG {
                for i in pixel_count_sums.iter_mut().take(cluster_count) {
                    *i = 0;
                }

                for i in 0..points.len() {
                    let cluster_index = cluster_indices[i];
                    let count = pixel_to_count[&pixels[i]];

                    pixel_count_sums[cluster_index] += count;
                }

                #[cfg(feature = "std")]
                {
                    let empty_clusters = pixel_count_sums
                        .iter()
                        .take(cluster_count)
                        .filter(|pixel_count_sum| pixel_count_sum == &&0)
                        .count();

                    Self::debug_log(format!(
                        "starting iteration {}; {empty_clusters} clusters are empty of {cluster_count}",
                        iteration + 1
                    ));
                }
            }

            let mut points_moved = 0;

            for i in 0..cluster_count {
                for j in (i + 1)..cluster_count {
                    let distance = PointProviderLab::distance(&clusters[i], &clusters[j]);

                    distance_to_index_matrix[j][i].distance = distance;
                    distance_to_index_matrix[j][i].index = i;
                    distance_to_index_matrix[i][j].distance = distance;
                    distance_to_index_matrix[i][j].index = j;
                }

                distance_to_index_matrix[i].sort();

                for j in 0..cluster_count {
                    index_matrix[i][j] = distance_to_index_matrix[i][j].index;
                }
            }

            for i in 0..points.len() {
                let point = points[i];
                let previous_cluster_index = cluster_indices[i];
                let previous_cluster = clusters[previous_cluster_index];
                let previous_distance = PointProviderLab::distance(&point, &previous_cluster);

                let mut minimum_distance = previous_distance;
                let mut new_cluster_index = None;

                for (j, cluster) in clusters.iter().enumerate().take(cluster_count) {
                    if distance_to_index_matrix[previous_cluster_index][j].distance
                        >= 4.0 * previous_distance
                    {
                        continue;
                    }

                    let distance = PointProviderLab::distance(&point, cluster);

                    if distance < minimum_distance {
                        minimum_distance = distance;
                        new_cluster_index = Some(j);
                    }
                }

                if let Some(new_cluster_index) = new_cluster_index {
                    points_moved += 1;
                    cluster_indices[i] = new_cluster_index;
                }
            }

            if points_moved == 0 && iteration > 0 {
                #[cfg(feature = "std")]
                Self::debug_log(format!("terminated after {iteration} k-means iterations"));

                break;
            }

            #[cfg(feature = "std")]
            Self::debug_log(format!("iteration {} moved {points_moved}", iteration + 1));

            let mut component_asums: Vec<f64> = vec![0.0; cluster_count];
            let mut component_bsums: Vec<f64> = vec![0.0; cluster_count];
            let mut component_csums: Vec<f64> = vec![0.0; cluster_count];

            for pixel_count_sum in pixel_count_sums.iter_mut().take(cluster_count) {
                *pixel_count_sum = 0;
            }

            for i in 0..points.len() {
                let cluster_index = cluster_indices[i];
                let point = points[i];
                let count = pixel_to_count[&pixels[i]];

                pixel_count_sums[cluster_index] += count;
                component_asums[cluster_index] += point.l * f64::from(count);
                component_bsums[cluster_index] += point.a * f64::from(count);
                component_csums[cluster_index] += point.b * f64::from(count);
            }

            for i in 0..cluster_count {
                let count = pixel_count_sums[i];

                if count == 0 {
                    clusters[i] = Lab::new(0.0, 0.0, 0.0);

                    continue;
                }

                let a = component_asums[i] / f64::from(count);
                let b = component_bsums[i] / f64::from(count);
                let c = component_csums[i] / f64::from(count);

                clusters[i] = Lab::new(a, b, c);
            }
        }

        let mut cluster_argbs = vec![];
        let mut cluster_populations = vec![];

        for i in 0..cluster_count {
            let count = pixel_count_sums[i];

            if count == 0 {
                continue;
            }

            let possible_new_cluster = PointProviderLab::lab_to_int(&clusters[i]);

            if cluster_argbs.contains(&possible_new_cluster) {
                continue;
            }

            cluster_argbs.push(possible_new_cluster);

            cluster_populations.push(count);
        }

        #[cfg(feature = "std")]
        Self::debug_log(format!(
            "kmeans finished and generated {} clusters; {cluster_count} were requested",
            cluster_argbs.len()
        ));

        let mut input_pixel_to_cluster_pixel: IndexMap<Argb, Argb> = IndexMap::default();

        #[cfg(feature = "std")]
        let start_time = Instant::now();

        for i in 0..pixels.len() {
            let input_pixel = pixels[i];
            let cluster_index = cluster_indices[i];
            let cluster = clusters[cluster_index];
            let cluster_pixel = PointProviderLab::lab_to_int(&cluster);

            input_pixel_to_cluster_pixel.insert(input_pixel, cluster_pixel);
        }

        #[cfg(feature = "std")]
        let time_elapsed = start_time.elapsed().as_millis();

        #[cfg(feature = "std")]
        Self::debug_log(format!(
            "took {time_elapsed} ms to create input to cluster map"
        ));

        let mut color_to_count: IndexMap<Argb, u32> = IndexMap::default();

        for i in 0..cluster_argbs.len() {
            let key = cluster_argbs[i];
            let value = cluster_populations[i];

            color_to_count.insert(key, value);
        }

        QuantizerResult {
            color_to_count,
            input_pixel_to_cluster_pixel,
        }
    }
}

fn fill_array<T>(count: usize, callback: impl Fn(usize) -> T) -> Vec<T> {
    let mut results: Vec<T> = vec![];

    for index in 0..count {
        results.push(callback(index));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::QuantizerWsmeans;
    use crate::color::Argb;
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    #[cfg(feature = "std")]
    use std::vec::Vec;

    const RED: Argb = Argb::from_u32(0xffff0000);
    const GREEN: Argb = Argb::from_u32(0xff00ff00);
    const BLUE: Argb = Argb::from_u32(0xff0000ff);
    // const WHITE: Argb = Argb::from_u32(0xffffffff);
    // const RANDOM: Argb = Argb::from_u32(0xff426088);
    const MAX_COLORS: usize = 256;

    #[test]
    fn test_1rando() {
        let result = QuantizerWsmeans::quantize(&[Argb::from_u32(0xff141216)], MAX_COLORS, &[]);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors[0], &Argb::from_u32(0xff141216));
    }

    #[test]
    fn test_1r() {
        let result = QuantizerWsmeans::quantize(&[RED], MAX_COLORS, &[]);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &RED);
    }

    #[test]
    fn test_1g() {
        let result = QuantizerWsmeans::quantize(&[GREEN], MAX_COLORS, &[]);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &GREEN);
    }

    #[test]
    fn test_1b() {
        let result = QuantizerWsmeans::quantize(&[BLUE], MAX_COLORS, &[]);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &BLUE);
    }

    #[test]
    fn test_5b() {
        let result = QuantizerWsmeans::quantize(&[BLUE, BLUE, BLUE, BLUE, BLUE], MAX_COLORS, &[]);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &BLUE);
    }
}
