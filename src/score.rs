#[cfg(not(feature = "std"))] use alloc::{vec, vec::Vec};
#[cfg(feature = "std")] use std::{vec, vec::Vec};

#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    IndexMap,
    color::Rgb,
    hct::Hct,
    utils::math::{difference_degrees, sanitize_degrees_int},
};

#[derive(Debug)]
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
    const CUTOFF_CHROMA: f64 = 5.0;
    const CUTOFF_EXCITED_PROPORTION: f64 = 0.01;
    const TARGET_CHROMA: f64 = 48.0;
    const WEIGHT_CHROMA_ABOVE: f64 = 0.3;
    const WEIGHT_CHROMA_BELOW: f64 = 0.1;
    // A1 Chroma
    const WEIGHT_PROPORTION: f64 = 0.7;

    /// Given a map with keys of colors and values of how often the color
    /// appears, rank the colors based on suitability for being used for a
    /// UI theme.
    ///
    /// - Parameters: `colorsToPopulation`: is a map with keys of colors and
    ///   values of often the color appears, usually from a source image.
    ///   `desired`: Max count of colors to be returned in the list.
    ///   `fallbackColorRgb`: Color to be returned if no other options
    ///   available. `filter`: Whether to filter out undesirable combinations.
    ///
    /// - Returns: A list of color `Int` that can be used when generating a
    ///   theme. The list returned is of length <= `desired`. The recommended
    ///   color is the first item, the least suitable is the last. There will
    ///   always be at least one color returned. If all the input colors were
    ///   not suitable for a theme, a default fallback color will be provided,
    ///   Google Blue. The default number of colors returned is 4, simply
    ///   because thats the # of colors display in Android 12's wallpaper
    ///   picker.
    pub fn score(colors_to_population: &IndexMap<Rgb, u32>, desired: Option<i32>, fallback_color_rgb: Option<Rgb>, filter: Option<bool>) -> Vec<Rgb> {
        let desired = desired.unwrap_or(4);
        let fallback_color_rgb = fallback_color_rgb.unwrap_or(Rgb::new(66, 133, 244));
        let filter = filter.unwrap_or(true);
        // Get the HCT color for each Rgb value, while finding the per hue count and
        // total count.
        let mut colors_hct = vec![];
        let mut hue_population = [0; 360];
        let mut population_sum = 0.0;

        for (rgb, population) in colors_to_population {
            let hct: Hct = (*rgb).into();

            let hue = hct.get_hue().floor() as i32;

            colors_hct.push(hct);

            hue_population[hue as usize] += population;
            population_sum += f64::from(*population);
        }

        // Hues with more usage in neighboring 30 degree slice get a larger number.
        let mut hue_excited_proportions = [0.0; 360];

        for (hue, population) in hue_population.into_iter().enumerate().take(360) {
            let proportion = f64::from(population) / population_sum;

            for i in ((hue as i32) - 14)..((hue as i32) + 16) {
                let neighbor_hue = sanitize_degrees_int(i);

                hue_excited_proportions[neighbor_hue as usize] += proportion;
            }
        }

        // Scores each HCT color based on usage and chroma, while optionally
        // filtering out values that do not have enough chroma or usage.
        let mut scored_hcts = vec![];

        for hct in colors_hct {
            let hue = hct.get_hue().round() as i32;

            let hue = sanitize_degrees_int(hue);
            let proportion = hue_excited_proportions[hue as usize];

            if filter && (hct.get_chroma() < Self::CUTOFF_CHROMA || proportion <= Self::CUTOFF_EXCITED_PROPORTION) {
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
        // SAFETY: The score will never be NAN, so using `unwrap_unchecked` is
        // completely safe
        scored_hcts.sort_by(|a, b| unsafe { b.score.partial_cmp(&a.score).unwrap_unchecked() });

        // Iterates through potential hue differences in degrees in order to select
        // the colors with the largest distribution of hues possible. Starting at
        // 90 degrees(maximum difference for 4 colors) then decreasing down to a
        // 15 degree minimum.
        let mut chosen_colors: Vec<Hct> = vec![];

        for difference_degree in (15..=90).rev() {
            chosen_colors.clear();

            for entry in &scored_hcts {
                let hct = entry.hct;

                if !chosen_colors
                    .iter()
                    .any(|color| difference_degrees(entry.hct.get_hue(), color.get_hue()) < f64::from(difference_degree))
                {
                    chosen_colors.push(hct);
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
            colors.push(fallback_color_rgb);
        }

        for chosen_hct in chosen_colors {
            colors.push(Rgb::from(chosen_hct));
        }

        colors
    }
}

#[cfg(test)]
mod tests {
    use super::Score;
    use crate::{IndexMap, color::Rgb};

    #[test]
    fn test_prioritizes_chroma() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x000000), 1),
            (Rgb::from_u32(0xFFFFFF), 1),
            (Rgb::from_u32(0x0000FF), 1),
        ]);

        let ranked = Score::score(&rgb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Rgb::from_u32(0x0000FF));
    }

    #[test]
    fn test_prioritizes_chroma_when_proportions_equal() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0xFF0000), 1),
            (Rgb::from_u32(0x00FF00), 1),
            (Rgb::from_u32(0x0000FF), 1),
        ]);

        let ranked = Score::score(&rgb_to_population, None, None, None);

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Rgb::from_u32(0xFF0000));
        assert_eq!(ranked[1], Rgb::from_u32(0x00FF00));
        assert_eq!(ranked[2], Rgb::from_u32(0x0000FF));
    }

    #[test]
    fn test_generates_gblue_when_no_colors_available() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([(Rgb::from_u32(0x000000), 1)]);

        let ranked = Score::score(&rgb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Rgb::from_u32(0x4285F4));
    }

    #[test]
    fn test_dedupes_nearby_hues() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([(Rgb::from_u32(0x008772), 1), (Rgb::from_u32(0x318477), 1)]);

        let ranked = Score::score(&rgb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Rgb::from_u32(0x008772));
    }

    #[test]
    fn test_maximizes_hue_distance() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x008772), 1),
            (Rgb::from_u32(0x008587), 1),
            (Rgb::from_u32(0x007EBC), 1),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(2), None, None);

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Rgb::from_u32(0x007EBC));
        assert_eq!(ranked[1], Rgb::from_u32(0x008772));
    }

    #[test]
    fn test_generated_scenario_one() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x7EA16D), 67),
            (Rgb::from_u32(0xD8CCAE), 67),
            (Rgb::from_u32(0x835C0D), 49),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0x8D3819)), Some(false));

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Rgb::from_u32(0x7EA16D));
        assert_eq!(ranked[1], Rgb::from_u32(0xD8CCAE));
        assert_eq!(ranked[2], Rgb::from_u32(0x835C0D));
    }

    #[test]
    fn test_generated_scenario_two() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0xD33881), 14),
            (Rgb::from_u32(0x3205CC), 77),
            (Rgb::from_u32(0x0B48CF), 36),
            (Rgb::from_u32(0xA08F5D), 81),
        ]);

        let ranked = Score::score(&rgb_to_population, None, Some(Rgb::from_u32(0x7D772B)), None);

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Rgb::from_u32(0x3205CC));
        assert_eq!(ranked[1], Rgb::from_u32(0xA08F5D));
        assert_eq!(ranked[2], Rgb::from_u32(0xD33881));
    }

    #[test]
    fn test_generated_scenario_three() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0xBE94A6), 23),
            (Rgb::from_u32(0xC33FD7), 42),
            (Rgb::from_u32(0x899F36), 90),
            (Rgb::from_u32(0x94C574), 82),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0xAA79A4)), None);

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Rgb::from_u32(0x94C574));
        assert_eq!(ranked[1], Rgb::from_u32(0xC33FD7));
        assert_eq!(ranked[2], Rgb::from_u32(0xBE94A6));
    }

    #[test]
    fn test_generated_scenario_four() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0xDF241C), 85),
            (Rgb::from_u32(0x685859), 44),
            (Rgb::from_u32(0xD06D5F), 34),
            (Rgb::from_u32(0x561C54), 27),
            (Rgb::from_u32(0x713090), 88),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(5), Some(Rgb::from_u32(0x58C19C)), Some(false));

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Rgb::from_u32(0xDF241C));
        assert_eq!(ranked[1], Rgb::from_u32(0x561C54));
    }

    #[test]
    fn test_generated_scenario_five() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0xBE66F8), 41),
            (Rgb::from_u32(0x4BBDA9), 88),
            (Rgb::from_u32(0x80F6F9), 44),
            (Rgb::from_u32(0xAB8017), 43),
            (Rgb::from_u32(0xE89307), 65),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0x916691)), Some(false));

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Rgb::from_u32(0xAB8017));
        assert_eq!(ranked[1], Rgb::from_u32(0x4BBDA9));
        assert_eq!(ranked[2], Rgb::from_u32(0xBE66F8));
    }

    #[test]
    fn test_generated_scenario_six() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x18EA8F), 93),
            (Rgb::from_u32(0x327593), 18),
            (Rgb::from_u32(0x066A18), 74),
            (Rgb::from_u32(0xFA8A23), 62),
            (Rgb::from_u32(0x04CA1F), 65),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(2), Some(Rgb::from_u32(0x4C377A)), Some(false));

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Rgb::from_u32(0x18EA8F));
        assert_eq!(ranked[1], Rgb::from_u32(0xFA8A23));
    }

    #[test]
    fn test_generated_scenario_seven() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x2E05ED), 23),
            (Rgb::from_u32(0x153E55), 90),
            (Rgb::from_u32(0x9AB220), 23),
            (Rgb::from_u32(0x153379), 66),
            (Rgb::from_u32(0x68BCC3), 81),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(2), Some(Rgb::from_u32(0xF588DC)), None);

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Rgb::from_u32(0x2E05ED));
        assert_eq!(ranked[1], Rgb::from_u32(0x9AB220));
    }

    #[test]
    fn test_generated_scenario_eight() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x816EC5), 24),
            (Rgb::from_u32(0x6DCB94), 19),
            (Rgb::from_u32(0x3CAE91), 98),
            (Rgb::from_u32(0x5B542F), 25),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(1), Some(Rgb::from_u32(0x84B0FD)), Some(false));

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Rgb::from_u32(0x3CAE91));
    }

    #[test]
    fn test_generated_scenario_nine() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x206F86), 52),
            (Rgb::from_u32(0x4A620D), 96),
            (Rgb::from_u32(0xF51401), 85),
            (Rgb::from_u32(0x2B8EBF), 3),
            (Rgb::from_u32(0x277766), 59),
        ]);

        let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0x02B415)), None);

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Rgb::from_u32(0xF51401));
        assert_eq!(ranked[1], Rgb::from_u32(0x4A620D));
        assert_eq!(ranked[2], Rgb::from_u32(0x2B8EBF));
    }

    #[test]
    fn test_generated_scenario_ten() {
        let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
            (Rgb::from_u32(0x8B1D99), 54),
            (Rgb::from_u32(0x27EFFE), 43),
            (Rgb::from_u32(0x6F558D), 2),
            (Rgb::from_u32(0x77FDF2), 78),
        ]);

        let ranked = Score::score(&rgb_to_population, None, Some(Rgb::from_u32(0x5E7A10)), None);

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Rgb::from_u32(0x27EFFE));
        assert_eq!(ranked[1], Rgb::from_u32(0x8B1D99));
        assert_eq!(ranked[2], Rgb::from_u32(0x6F558D));
    }
}
