use std::cmp::Ordering;
use ahash::HashMap;
use std::time::Instant;

use rand::Rng;

use crate::quantize::point_provider_lab::PointProviderLab;
use crate::utils::color::Argb;

use super::point_provider::PointProvider;
use super::quantizer::QuantizerResult;

pub struct DistanceAndIndex {
    pub distance: f64,
    pub index: usize,
}

impl PartialEq for DistanceAndIndex {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for DistanceAndIndex {}

impl PartialOrd for DistanceAndIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistanceAndIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

macro_rules! default_value {
    ($argument_name:ident:$argument_ty:ty = $default_value:expr) => {
        let $argument_name: $argument_ty = $argument_name.unwrap_or($default_value);
    };
    ($($argument_name:ident:$argument_ty:ty = $default_value:expr);*;) => {
        $(
            let default_value = $default_value;
            let $argument_name: $argument_ty = $argument_name.unwrap_or(default_value);
        )*
    };
}

#[derive(Default)]
pub struct QuantizerWsmeans;

impl QuantizerWsmeans {
    const DEBUG: bool = false;

    pub fn debug_log<T: Into<String>>(log: T) {
        let log: String = log.into();

        if QuantizerWsmeans::DEBUG {
            println!("{log}");
        }
    }

    pub fn quantize(
        input_pixels: &[Argb],
        max_colors: i32,
        starting_clusters: Option<Vec<Argb>>,
        point_provider: Option<PointProviderLab>,
        max_iterations: Option<i32>,
        return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult {
        default_value! {
            starting_clusters: Vec<Argb> = vec![];
            point_provider: PointProviderLab = PointProviderLab::new();
            max_iterations: i32 = 5;
            return_input_pixel_to_cluster_pixel: bool = false;
        };

        let mut pixel_to_count: HashMap<Argb, u32> = Default::default();
        let mut points: Vec<[f64; 3]> = vec![];
        let mut pixels: Vec<Argb> = vec![];
        let mut point_count = 0;

        for input_pixel in input_pixels {
            let mut pixel_count = pixel_to_count.get(input_pixel).copied();

            if let Some(value) = pixel_count {
                pixel_count = Some(value + 1);
                pixel_to_count.insert(*input_pixel, pixel_count.unwrap());
            } else {
                pixel_count = Some(1);
                pixel_to_count.insert(*input_pixel, pixel_count.unwrap());
            }

            if pixel_count.is_some_and(|count| count == 1) {
                point_count += 1;

                points.push(point_provider.lab_from_int(*input_pixel));
                pixels.push(*input_pixel);
            }
        }

        let mut counts = vec![0; point_count];

        for i in 0..point_count {
            let pixel = pixels[i];
            let count = pixel_to_count.get(&pixel);

            counts[i] = *count.unwrap();
        }

        let cluster_count = (max_colors as usize).min(point_count);

        let mut clusters = starting_clusters
            .iter()
            .map(|cluster| point_provider.lab_from_int(*cluster))
            .collect::<Vec<_>>();
        let additional_clusters_needed = cluster_count - clusters.len();

        if additional_clusters_needed > 0 {
            let mut indices: Vec<usize> = vec![];

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

                let mut index = rand::thread_rng().gen_range(0..points.len());

                while indices.contains(&index) {
                    index = rand::thread_rng().gen_range(0..points.len());
                }

                indices.push(index);
            }

            for index in indices {
                clusters.push(points[index]);
            }
        }

        QuantizerWsmeans::debug_log(format!(
            "have {} starting clusters, {} points",
            clusters.len(),
            points.len()
        ));

        let mut cluster_indices = fill_array(point_count, |index| index % cluster_count);

        let mut index_matrix = vec![vec![0; cluster_count]; cluster_count];
        let mut distance_to_index_matrix = fill_array(cluster_count, |_| {
            fill_array(cluster_count, |index| DistanceAndIndex {
                index,
                distance: 0.0,
            })
        });
        let mut pixel_count_sums = vec![0; cluster_count];

        for iteration in 0..max_iterations {
            if QuantizerWsmeans::DEBUG {
                for item in pixel_count_sums.iter_mut().take(cluster_count) {
                    *item = 0;
                }

                for i in 0..point_count {
                    let cluster_index = cluster_indices[i];
                    let count = counts[i];

                    pixel_count_sums[cluster_index] += count;
                }

                let mut empty_clusters = 0;

                for cluster in pixel_count_sums.iter().take(cluster_count) {
                    if cluster == &0 {
                        empty_clusters += 1
                    }
                }

                QuantizerWsmeans::debug_log(format!(
                    "starting iteration {}; {empty_clusters} clusters are empty of {cluster_count}",
                    iteration + 1
                ));
            }

            let mut points_moved = 0;

            for i in 0..cluster_count {
                for j in (i + 1)..cluster_count {
                    let distance = point_provider.distance(clusters[i], clusters[j]);

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

            for i in 0..point_count {
                let point = points[i];
                let previous_cluster_index = cluster_indices[i];
                let previous_cluster = clusters[previous_cluster_index];
                let previous_distance = point_provider.distance(point, previous_cluster);

                let mut minimum_distance = previous_distance;
                let mut new_cluster_index: isize = -1;

                for (j, item) in clusters.iter().enumerate().take(cluster_count) {
                    if distance_to_index_matrix[previous_cluster_index][j].distance
                        >= 4.0 * previous_distance
                    {
                        continue;
                    }

                    let distance = point_provider.distance(point, *item);

                    if distance < minimum_distance {
                        minimum_distance = distance;
                        new_cluster_index = j as isize;
                    }
                }

                if new_cluster_index != -1 {
                    points_moved += 1;
                    cluster_indices[i] = new_cluster_index as usize;
                }
            }

            if points_moved == 0 && iteration > 0 {
                QuantizerWsmeans::debug_log(format!(
                    "terminated after {iteration} k-means iterations"
                ));

                break;
            }

            QuantizerWsmeans::debug_log(format!(
                "iteration {} moved {points_moved}",
                iteration + 1
            ));

            let mut component_asums = vec![0.0; cluster_count];
            let mut component_bsums = vec![0.0; cluster_count];
            let mut component_csums = vec![0.0; cluster_count];

            for sum in pixel_count_sums.iter_mut().take(cluster_count) {
                *sum = 0;
            }

            for i in 0..point_count {
                let cluster_index = cluster_indices[i] as usize;
                let point = points[i];
                let count = counts[i];

                pixel_count_sums[cluster_index] += count;
                component_asums[cluster_index] += point[0] * count as f64;
                component_bsums[cluster_index] += point[1] * count as f64;
                component_csums[cluster_index] += point[2] * count as f64;
            }

            for i in 0..cluster_count {
                let count = pixel_count_sums[i];

                if count == 0 {
                    clusters[i] = [0.0, 0.0, 0.0];

                    continue;
                }

                let a = component_asums[i] / count as f64;
                let b = component_bsums[i] / count as f64;
                let c = component_csums[i] / count as f64;

                clusters[i] = [a, b, c];
            }
        }

        let mut cluster_argbs: Vec<Argb> = vec![];
        let mut cluster_populations: Vec<u32> = vec![];

        for i in 0..cluster_count {
            let count = pixel_count_sums[i];

            if count == 0 {
                continue;
            }

            let possible_new_cluster = point_provider.lab_to_int(clusters[i]);

            if cluster_argbs.contains(&possible_new_cluster) {
                continue;
            }

            cluster_argbs.push(possible_new_cluster);
            cluster_populations.push(count);
        }

        QuantizerWsmeans::debug_log(format!(
            "kmeans finished and generated {} clusters; {cluster_count} were requested",
            cluster_argbs.len()
        ));

        let mut input_pixel_to_cluster_pixel: HashMap<Argb, Argb> = Default::default();

        if return_input_pixel_to_cluster_pixel {
            let start_time = Instant::now();

            for i in 0..pixels.len() {
                let input_pixel = pixels[i];
                let cluster_index = cluster_indices[i];
                let cluster = clusters[cluster_index];
                let cluster_pixel = point_provider.lab_to_int(cluster);

                input_pixel_to_cluster_pixel.insert(input_pixel, cluster_pixel);
            }

            let time_elapsed = start_time.elapsed().as_millis();

            QuantizerWsmeans::debug_log(format!(
                "took {time_elapsed} ms to create input to cluster map"
            ));
        }

        let mut color_to_count: HashMap<Argb, u32> = Default::default();

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

fn fill_array<T, F>(count: usize, callback: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    let mut results: Vec<T> = vec![];

    for index in 0..count {
        results.push(callback(index));
    }

    results
}
