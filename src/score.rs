use indexmap::IndexMap;

use crate::{
    color::Argb,
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
        scored_hcts.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

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
    use std::str::FromStr;

    use indexmap::IndexMap;

    use crate::color::Argb;
    use crate::Error;

    use super::Score;

    #[test]
    fn test_prioritizes_chroma() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("000000")?, 1),
            (Argb::from_str("ffffff")?, 1),
            (Argb::from_str("0000ff")?, 1),
        ]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_str("0000ff")?);

        Ok(())
    }

    #[test]
    fn test_prioritizes_chroma_when_proportions_equal() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("ff0000")?, 1),
            (Argb::from_str("00ff00")?, 1),
            (Argb::from_str("0000ff")?, 1),
        ]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_str("ff0000")?);
        assert_eq!(ranked[1], Argb::from_str("00ff00")?);
        assert_eq!(ranked[2], Argb::from_str("0000ff")?);

        Ok(())
    }

    #[test]
    fn test_generates_gblue_when_no_colors_available() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> =
            IndexMap::from_iter([(Argb::from_str("000000")?, 1)]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_str("4285f4")?);

        Ok(())
    }

    #[test]
    fn test_dedupes_nearby_hues() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("008772")?, 1),
            (Argb::from_str("318477")?, 1),
        ]);

        let ranked = Score::score(&argb_to_population, None, None, None);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_str("008772")?);

        Ok(())
    }

    #[test]
    fn test_maximizes_hue_distance() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("008772")?, 1),
            (Argb::from_str("008587")?, 1),
            (Argb::from_str("007ebc")?, 1),
        ]);

        let ranked = Score::score(&argb_to_population, Some(2), None, None);

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_str("007ebc")?);
        assert_eq!(ranked[1], Argb::from_str("008772")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_one() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("7ea16d")?, 67),
            (Argb::from_str("d8ccae")?, 67),
            (Argb::from_str("835c0d")?, 49),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_str("8d3819")?),
            Some(false),
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_str("7ea16d")?);
        assert_eq!(ranked[1], Argb::from_str("d8ccae")?);
        assert_eq!(ranked[2], Argb::from_str("835c0d")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_two() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("d33881")?, 14),
            (Argb::from_str("3205cc")?, 77),
            (Argb::from_str("0b48cf")?, 36),
            (Argb::from_str("a08f5d")?, 81),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            None,
            Some(Argb::from_str("7d772b")?),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_str("3205cc")?);
        assert_eq!(ranked[1], Argb::from_str("a08f5d")?);
        assert_eq!(ranked[2], Argb::from_str("d33881")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_three() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("be94a6")?, 23),
            (Argb::from_str("c33fd7")?, 42),
            (Argb::from_str("899f36")?, 90),
            (Argb::from_str("94c574")?, 82),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_str("aa79a4")?),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_str("94c574")?);
        assert_eq!(ranked[1], Argb::from_str("c33fd7")?);
        assert_eq!(ranked[2], Argb::from_str("be94a6")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_four() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("df241c")?, 85),
            (Argb::from_str("685859")?, 44),
            (Argb::from_str("d06d5f")?, 34),
            (Argb::from_str("561c54")?, 27),
            (Argb::from_str("713090")?, 88),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(5),
            Some(Argb::from_str("58c19c")?),
            Some(false),
        );

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_str("df241c")?);
        assert_eq!(ranked[1], Argb::from_str("561c54")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_five() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("be66f8")?, 41),
            (Argb::from_str("4bbda9")?, 88),
            (Argb::from_str("80f6f9")?, 44),
            (Argb::from_str("ab8017")?, 43),
            (Argb::from_str("e89307")?, 65),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_str("916691")?),
            Some(false),
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_str("ab8017")?);
        assert_eq!(ranked[1], Argb::from_str("4bbda9")?);
        assert_eq!(ranked[2], Argb::from_str("be66f8")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_six() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("18ea8f")?, 93),
            (Argb::from_str("327593")?, 18),
            (Argb::from_str("066a18")?, 74),
            (Argb::from_str("fa8a23")?, 62),
            (Argb::from_str("04ca1f")?, 65),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(2),
            Some(Argb::from_str("4c377a")?),
            Some(false),
        );

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_str("18ea8f")?);
        assert_eq!(ranked[1], Argb::from_str("fa8a23")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_seven() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("2e05ed")?, 23),
            (Argb::from_str("153e55")?, 90),
            (Argb::from_str("9ab220")?, 23),
            (Argb::from_str("153379")?, 66),
            (Argb::from_str("68bcc3")?, 81),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(2),
            Some(Argb::from_str("f588dc")?),
            None,
        );

        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0], Argb::from_str("2e05ed")?);
        assert_eq!(ranked[1], Argb::from_str("9ab220")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_eight() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("816ec5")?, 24),
            (Argb::from_str("6dcb94")?, 19),
            (Argb::from_str("3cae91")?, 98),
            (Argb::from_str("5b542f")?, 25),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(1),
            Some(Argb::from_str("84b0fd")?),
            Some(false),
        );

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], Argb::from_str("3cae91")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_nine() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("206f86")?, 52),
            (Argb::from_str("4a620d")?, 96),
            (Argb::from_str("f51401")?, 85),
            (Argb::from_str("2b8ebf")?, 3),
            (Argb::from_str("277766")?, 59),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            Some(3),
            Some(Argb::from_str("02b415")?),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_str("f51401")?);
        assert_eq!(ranked[1], Argb::from_str("4a620d")?);
        assert_eq!(ranked[2], Argb::from_str("2b8ebf")?);

        Ok(())
    }

    #[test]
    fn test_generated_scenario_ten() -> Result<(), Error> {
        let argb_to_population: IndexMap<Argb, u32> = IndexMap::from_iter([
            (Argb::from_str("8b1d99")?, 54),
            (Argb::from_str("27effe")?, 43),
            (Argb::from_str("6f558d")?, 2),
            (Argb::from_str("77fdf2")?, 78),
        ]);

        let ranked = Score::score(
            &argb_to_population,
            None,
            Some(Argb::from_str("5e7a10")?),
            None,
        );

        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], Argb::from_str("27effe")?);
        assert_eq!(ranked[1], Argb::from_str("8b1d99")?);
        assert_eq!(ranked[2], Argb::from_str("6f558d")?);

        Ok(())
    }
}
