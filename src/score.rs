use ahash::HashMap;

use crate::hct::Hct;
use crate::utils::color::Argb;
use crate::utils::math::difference_degrees;
use crate::utils::math::sanitize_degrees_int;

struct ScoredHCT {
    hct: Hct,
    score: f64,
}

/// Given a large set of colors, remove colors that are unsuitable for a UI
/// theme, and rank the rest based on suitability.
///
/// Enables use of a high cluster count for image quantization, thus ensuring
/// colors aren't muddied, while curating the high cluster count to a much
///  smaller number of appropriate choices.
pub struct Score;

impl Score {
    const TARGET_CHROMA: f64 = 48.0; // A1 Chroma
    const WEIGHT_PROPORTION: f64 = 0.7;
    const WEIGHT_CHROMA_ABOVE: f64 = 0.3;
    const WEIGHT_CHROMA_BELOW: f64 = 0.1;
    const CUTOFF_CHROMA: f64 = 5.0;
    const CUTOFF_EXCITED_PROPORTION: f64 = 0.01;
    /// Given a map with keys of colors and values of how often the color appears,
    /// rank the colors based on suitability for being used for a UI theme.
    ///
    /// - Parameters:
    ///   `colorsToPopulation`: is a map with keys of colors and values of often
    ///     the color appears, usually from a source image.
    ///   `desired`: Max count of colors to be returned in the list.
    ///   `fallbackColorArgb`: Color to be returned if no other options available.
    ///   `filter`: Whether to filter out undesireable combinations.
    ///
    /// - Returns: A list of color `Int` that can be used when generating a theme.
    ///   The list returned is of length <= `desired`. The recommended color is
    ///   the first item, the least suitable is the last. There will always be at
    ///   least one color returned. If all the input colors were not suitable for
    ///   a theme, a default fallback color will be provided, Google Blue. The
    ///   default number of colors returned is 4, simply because thats the # of
    ///   colors display in Android 12's wallpaper picker.
    pub fn score(
        colors_to_population: &HashMap<Argb, u32>,
        desired: Option<i32>,
        fallback_color_argb: Option<Argb>,
        filter: Option<bool>,
    ) -> Vec<Argb> {
        let desired = desired.unwrap_or(4);
        let fallback_color_argb = fallback_color_argb.unwrap_or([255, 66, 133, 244]);
        let filter = filter.unwrap_or(true);
        // Get the HCT color for each Argb value, while finding the per hue count and
        // total count.
        let mut colors_hct = vec![];
        let mut hue_population = [0; 360];
        let mut population_sum = 0.0;

        for argb in colors_to_population.keys() {
            let population = colors_to_population[argb];
            let hct: Hct = (*argb).into();

            let hue = hct.get_hue().floor() as i32;

            colors_hct.push(hct);

            hue_population[hue as usize] += population;
            population_sum += population as f64;
        }

        // Hues with more usage in neighboring 30 degree slice get a larger number.
        let mut hue_excited_proportions = [0.0; 360];

        for hue in hue_population.into_iter().take(360) {
            let proportion = hue as f64 / population_sum;

            for i in ((hue as i32) - 14)..((hue as i32) + 16) {
                let neighbor_hue = sanitize_degrees_int(i as i32);

                hue_excited_proportions[neighbor_hue as usize] += proportion;
            }
        }

        // Scores each HCT color based on usage and chroma, while optionally
        // filtering out values that do not have enough chroma or usage.
        let mut scored_hcts = vec![];

        for hct in colors_hct {
            let hue = sanitize_degrees_int(hct.get_hue().round() as i32);
            let proportion = hue_excited_proportions[hue as usize];

            if filter
                && (hct.get_chroma() < Self::CUTOFF_CHROMA
                    || proportion <= Self::CUTOFF_EXCITED_PROPORTION)
            {
                continue;
            }

            let proportion_score = proportion * 100.0 * Self::WEIGHT_PROPORTION;
            let chroma_weight = if hct.get_chroma() < Self::TARGET_CHROMA {
                Self::WEIGHT_CHROMA_BELOW
            } else {
                Self::WEIGHT_CHROMA_ABOVE
            };
            let chroma_score = (hct.get_chroma() - Self::TARGET_CHROMA) * chroma_weight;
            let score = proportion_score + chroma_score;

            scored_hcts.push(ScoredHCT { hct, score });
        }

        // Sorted so that colors with higher scores come first.
        scored_hcts.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        // Iterates through potential hue differences in degrees in order to select
        // the colors with the largest distribution of hues possible. Starting at
        // 90 degrees(maximum difference for 4 colors) then decreasing down to a
        // 15 degree minimum.
        let mut chosen_colors: Vec<Hct> = vec![];

        for difference_degree in (15..=90).rev() {
            chosen_colors.clear();

            for entry in &scored_hcts {
                if !chosen_colors.iter().any(|color| {
                    difference_degrees(entry.hct.get_hue(), color.get_hue())
                        < difference_degree as f64
                }) {
                    chosen_colors.push(entry.hct)
                }

                if chosen_colors.len() >= desired as usize {
                    break;
                }
            }

            if chosen_colors.len() >= desired as usize {
                break;
            }
        }

        let mut colors = vec![];

        if chosen_colors.is_empty() {
            colors.push(fallback_color_argb)
        }

        for chosen_hct in chosen_colors {
            colors.push(Argb::from(chosen_hct))
        }

        colors
    }
}
