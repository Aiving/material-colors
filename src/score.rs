#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::Argb,
    hct::Hct,
    utils::math::{difference_degrees, sanitize_degrees_int},
    IndexMap,
};
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
#[cfg(feature = "std")]
use std::{vec, vec::Vec};

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
        colors_to_population: &IndexMap<Argb, u32>,
        desired: Option<i32>,
        fallback_color_argb: Option<Argb>,
        filter: Option<bool>,
    ) -> Vec<Argb> {
        let desired = desired.unwrap_or(4);
        let fallback_color_argb = fallback_color_argb.unwrap_or(Argb::new(255, 66, 133, 244));
        let filter = filter.unwrap_or(true);
        // Get the HCT color for each Argb value, while finding the per hue count and
        // total count.
        let mut colors_hct = vec![];
        let mut hue_population = [0; 360];
        let mut population_sum = 0.0;

        for (argb, population) in colors_to_population {
            let hct: Hct = (*argb).into();

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
        // SAFETY: The score will never be NAN, so using `unwrap_unchecked` is completely safe
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

                if !chosen_colors.iter().any(|color| {
                    difference_degrees(entry.hct.get_hue(), color.get_hue())
                        < f64::from(difference_degree)
                }) {
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
            colors.push(fallback_color_argb);
        }

        for chosen_hct in chosen_colors {
            colors.push(Argb::from(chosen_hct));
        }

        colors
    }
}

#[cfg(test)]
mod tests {
    use super::Score;
    use crate::{color::Argb, IndexMap};

    #[test]
    fn test_prioritizes_chroma() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff000000), 1),
            (Argb::from_u32(0xffffffff), 1),
            (Argb::from_u32(0xff0000ff), 1),
        ]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_u32(0xff0000ff));
    }

    #[test]
    fn test_prioritizes_chroma_when_proportions_equal() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xffff0000), 1),
            (Argb::from_u32(0xff00ff00), 1),
            (Argb::from_u32(0xff0000ff), 1),
        ]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_u32(0xffff0000));
        assert_eq!(ranked[1], Argb::from_u32(0xff00ff00));
        assert_eq!(ranked[2], Argb::from_u32(0xff0000ff));
    }

    #[test]
    fn test_generates_gblue_when_no_colors_available() {
        let argb_to_population: IndexMap<Argb, u32> =
            IndexMap::from_iter([(Argb::from_u32(0xff000000), 1)]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_u32(0xff4285f4));
    }

    #[test]
    fn test_dedupes_nearby_hues() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff008772), 1),
            (Argb::from_u32(0xff318477), 1),
        ]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_u32(0xff008772));
    }

    #[test]
    fn test_maximizes_hue_distance() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff008772), 1),
            (Argb::from_u32(0xff008587), 1),
            (Argb::from_u32(0xff007ebc), 1),
        ]);

        let ranked = Score::score(&argb_to_population, Some(2), None, None);

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_u32(0xff007ebc));
        assert_eq!(ranked[1], Argb::from_u32(0xff008772));
    }

    #[test]
    fn test_generated_scenario_one() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff7ea16d), 67),
            (Argb::from_u32(0xffd8ccae), 67),
            (Argb::from_u32(0xff835c0d), 49),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_u32(0xff8d3819)),
            Some(false),
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_u32(0xff7ea16d));
        assert_eq!(ranked[1], Argb::from_u32(0xffd8ccae));
        assert_eq!(ranked[2], Argb::from_u32(0xff835c0d));
    }

    #[test]
    fn test_generated_scenario_two() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xffd33881), 14),
            (Argb::from_u32(0xff3205cc), 77),
            (Argb::from_u32(0xff0b48cf), 36),
            (Argb::from_u32(0xffa08f5d), 81),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            None,
            Some(Argb::from_u32(0xff7d772b)),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_u32(0xff3205cc));
        assert_eq!(ranked[1], Argb::from_u32(0xffa08f5d));
        assert_eq!(ranked[2], Argb::from_u32(0xffd33881));
    }

    #[test]
    fn test_generated_scenario_three() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xffbe94a6), 23),
            (Argb::from_u32(0xffc33fd7), 42),
            (Argb::from_u32(0xff899f36), 90),
            (Argb::from_u32(0xff94c574), 82),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_u32(0xffaa79a4)),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_u32(0xff94c574));
        assert_eq!(ranked[1], Argb::from_u32(0xffc33fd7));
        assert_eq!(ranked[2], Argb::from_u32(0xffbe94a6));
    }

    #[test]
    fn test_generated_scenario_four() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xffdf241c), 85),
            (Argb::from_u32(0xff685859), 44),
            (Argb::from_u32(0xffd06d5f), 34),
            (Argb::from_u32(0xff561c54), 27),
            (Argb::from_u32(0xff713090), 88),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(5),
            Some(Argb::from_u32(0xff58c19c)),
            Some(false),
        );

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_u32(0xffdf241c));
        assert_eq!(ranked[1], Argb::from_u32(0xff561c54));
    }

    #[test]
    fn test_generated_scenario_five() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xffbe66f8), 41),
            (Argb::from_u32(0xff4bbda9), 88),
            (Argb::from_u32(0xff80f6f9), 44),
            (Argb::from_u32(0xffab8017), 43),
            (Argb::from_u32(0xffe89307), 65),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_u32(0xff916691)),
            Some(false),
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_u32(0xffab8017));
        assert_eq!(ranked[1], Argb::from_u32(0xff4bbda9));
        assert_eq!(ranked[2], Argb::from_u32(0xffbe66f8));
    }

    #[test]
    fn test_generated_scenario_six() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff18ea8f), 93),
            (Argb::from_u32(0xff327593), 18),
            (Argb::from_u32(0xff066a18), 74),
            (Argb::from_u32(0xfffa8a23), 62),
            (Argb::from_u32(0xff04ca1f), 65),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(2),
            Some(Argb::from_u32(0xff4c377a)),
            Some(false),
        );

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_u32(0xff18ea8f));
        assert_eq!(ranked[1], Argb::from_u32(0xfffa8a23));
    }

    #[test]
    fn test_generated_scenario_seven() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff2e05ed), 23),
            (Argb::from_u32(0xff153e55), 90),
            (Argb::from_u32(0xff9ab220), 23),
            (Argb::from_u32(0xff153379), 66),
            (Argb::from_u32(0xff68bcc3), 81),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(2),
            Some(Argb::from_u32(0xfff588dc)),
            None,
        );

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_u32(0xff2e05ed));
        assert_eq!(ranked[1], Argb::from_u32(0xff9ab220));
    }

    #[test]
    fn test_generated_scenario_eight() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff816ec5), 24),
            (Argb::from_u32(0xff6dcb94), 19),
            (Argb::from_u32(0xff3cae91), 98),
            (Argb::from_u32(0xff5b542f), 25),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(1),
            Some(Argb::from_u32(0xff84b0fd)),
            Some(false),
        );

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_u32(0xff3cae91));
    }

    #[test]
    fn test_generated_scenario_nine() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff206f86), 52),
            (Argb::from_u32(0xff4a620d), 96),
            (Argb::from_u32(0xfff51401), 85),
            (Argb::from_u32(0xff2b8ebf), 3),
            (Argb::from_u32(0xff277766), 59),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_u32(0xff02b415)),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_u32(0xfff51401));
        assert_eq!(ranked[1], Argb::from_u32(0xff4a620d));
        assert_eq!(ranked[2], Argb::from_u32(0xff2b8ebf));
    }

    #[test]
    fn test_generated_scenario_ten() {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_u32(0xff8b1d99), 54),
            (Argb::from_u32(0xff27effe), 43),
            (Argb::from_u32(0xff6f558d), 2),
            (Argb::from_u32(0xff77fdf2), 78),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            None,
            Some(Argb::from_u32(0xff5e7a10)),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_u32(0xff27effe));
        assert_eq!(ranked[1], Argb::from_u32(0xff8b1d99));
        assert_eq!(ranked[2], Argb::from_u32(0xff6f558d));
    }
}
