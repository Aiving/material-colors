use ahash::HashMap;
use core::cmp::Ordering;

use crate::{color::Lab, utils::math::sanitize_degrees_double, Argb, Hct};

/// Design utilities using color temperature theory.
///
/// Analogous colors, complementary color, and cache to efficiently, lazily,
/// generate data for calculations when needed.
pub struct TemperatureCache {
    input: Hct,

    _hcts_by_temp: Vec<Hct>,
    _hcts_by_hue: Vec<Hct>,
    _temps_by_hct: HashMap<Hct, f64>,
    _input_relative_temperature: f64,
    _complement: Option<Hct>,
}

impl TemperatureCache {
    pub fn warmest(&mut self) -> Hct {
        let hcts = self.hcts_by_temp();

        return *hcts.last().unwrap();
    }

    pub fn coldest(&mut self) -> Hct {
        let hcts = self.hcts_by_temp();

        return *hcts.first().unwrap();
    }

    pub fn new(input: Hct) -> Self {
        Self {
            input,
            _hcts_by_temp: vec![],
            _hcts_by_hue: vec![],
            _temps_by_hct: Default::default(),
            _input_relative_temperature: -1.0,
            _complement: None,
        }
    }

    /// A set of colors with differing hues, equidistant in temperature.
    ///
    /// In art, this is usually described as a set of 5 colors on a color wheel
    /// divided into 12 sections. This method allows provision of either of those
    /// values.
    ///
    /// Behavior is undefined when [count] or [divisions] is 0.
    /// When divisions < count, colors repeat.
    ///
    /// [count] The number of colors to return, includes the input color.
    /// [divisions] The number of divisions on the color wheel.
    pub fn analogous(&mut self, count: Option<i32>, divisions: Option<i32>) -> Vec<Hct> {
        let count = count.unwrap_or(5);
        let divisions = divisions.unwrap_or(12);
        let start_hue = self.input.get_hue().round() as i32;
        let start_hct = &self.hcts_by_hue()[start_hue as usize];
        let mut last_temp = self.relative_temperature(start_hct);
        let mut all_colors = vec![*start_hct];

        let mut absolute_total_temp_delta = 0.0;

        for i in 0..360 {
            let hue = sanitize_degrees_double((start_hue + i).into());
            let hct = &self.hcts_by_hue()[hue as usize];
            let temp = self.relative_temperature(hct);
            let temp_delta = (temp - last_temp).abs();

            last_temp = temp;
            absolute_total_temp_delta += temp_delta;
        }

        let mut hue_addend = 1;
        let temp_step = absolute_total_temp_delta / divisions as f64;

        let mut total_temp_delta = 0.0;

        last_temp = self.relative_temperature(start_hct);

        while all_colors.len() < divisions as usize {
            let hue = sanitize_degrees_double((start_hue + hue_addend).into());
            let hct = &self.hcts_by_hue()[hue as usize];
            let temp = self.relative_temperature(hct);
            let temp_delta = (temp - last_temp).abs();

            total_temp_delta += temp_delta;

            let desired_total_temp_delta_for_index = all_colors.len() as f64 * temp_step;

            let mut index_satisfied = total_temp_delta >= desired_total_temp_delta_for_index;
            let mut index_addend = 1;

            // Keep adding this hue to the answers until its temperature is
            // insufficient. This ensures consistent behavior when there aren't
            // [divisions] discrete steps between 0 and 360 in hue with [tempStep]
            // delta in temperature between them.
            //
            // For example, white and black have no analogues: there are no other
            // colors at T100/T0. Therefore, they should just be added to the array
            // as answers.
            while index_satisfied && all_colors.len() < divisions as usize {
                all_colors.push(*hct);

                let desired_total_temp_delta_for_index =
                    (all_colors.len() + index_addend) as f64 * temp_step;

                index_satisfied = total_temp_delta >= desired_total_temp_delta_for_index;
                index_addend += 1;
            }

            last_temp = temp;
            hue_addend += 1;

            if hue_addend > 360 {
                while all_colors.len() < divisions as usize {
                    all_colors.push(*hct);
                }

                break;
            }
        }

        let mut answers = vec![self.input];

        // First, generate analogues from rotating counter-clockwise.
        let increase_hue_count = ((count as f64 - 1.0) / 2.0).floor() as isize;

        for i in 1..=increase_hue_count {
            let mut index = 0_isize - i;

            while index < 0 {
                index += all_colors.len() as isize;
            }

            if index >= all_colors.len() as isize {
                index %= all_colors.len() as isize;
            }

            answers.insert(0, all_colors[index as usize]);
        }

        // Second, generate analogues from rotating clockwise.
        let decrease_hue_count = (count - (increase_hue_count as i32) - 1) as isize;

        for i in 1..=decrease_hue_count {
            let mut index = i;

            while index < 0 {
                index += all_colors.len() as isize;
            }

            if index >= all_colors.len() as isize {
                index %= all_colors.len() as isize;
            }

            answers.push(all_colors[index as usize]);
        }

        answers
    }

    /// A color that complements the input color aesthetically.
    ///
    /// In art, this is usually described as being across the color wheel.
    /// History of this shows intent as a color that is just as cool-warm as the
    /// input color is warm-cool.
    pub fn complement(&mut self) -> Hct {
        if let Some(_complement) = &self._complement {
            return *_complement;
        }

        let coldest_hue = self.coldest().get_hue();
        let coldest_temp = self.temps_by_hct()[&self.coldest()];

        let warmest_hue = self.warmest().get_hue();
        let warmest_temp = self.temps_by_hct()[&self.warmest()];

        let range = warmest_temp - coldest_temp;
        let start_hue_is_coldest_to_warmest =
            TemperatureCache::is_between(self.input.get_hue(), coldest_hue, warmest_hue);
        let start_hue = if start_hue_is_coldest_to_warmest {
            warmest_hue
        } else {
            coldest_hue
        };
        let end_hue = if start_hue_is_coldest_to_warmest {
            coldest_hue
        } else {
            warmest_hue
        };
        let direction_of_rotation = 1.0_f64;
        let mut smallest_error = 1000.0;
        let mut answer = self.hcts_by_hue()[self.input.get_hue().round() as usize];

        let complement_relative_temp = 1.0 - self.input_relative_temperature();

        // Find the color in the other section, closest to the inverse percentile
        // of the input color. This is the complement.
        for hue_addend in 0..=360 {
            let hue = sanitize_degrees_double(
                direction_of_rotation.mul_add(hue_addend as f64, start_hue),
            );

            if !TemperatureCache::is_between(hue, start_hue, end_hue) {
                continue;
            }

            let possible_answer = &self.hcts_by_hue()[hue.round() as usize];
            let relative_temp = (self._temps_by_hct[possible_answer] - coldest_temp) / range;
            let error = (complement_relative_temp - relative_temp).abs();

            if error < smallest_error {
                smallest_error = error;
                answer = *possible_answer;
            }
        }

        self._complement = Some(answer);

        self._complement.unwrap()
    }

    /// Temperature relative to all colors with the same chroma and tone.
    /// Value on a scale from 0 to 1.
    pub fn relative_temperature(&mut self, hct: &Hct) -> f64 {
        let range = self.temps_by_hct()[&self.warmest()] - self.temps_by_hct()[&self.coldest()];
        let difference_from_coldest =
            self.temps_by_hct()[hct] - self.temps_by_hct()[&self.coldest()];

        // Handle when there's no difference in temperature between warmest and
        // coldest: for example, at T100, only one color is available, white.
        if range == 0.0 {
            return 0.5;
        }

        difference_from_coldest / range
    }

    /// Relative temperature of the input color. See [relativeTemperature].
    pub fn input_relative_temperature(&mut self) -> f64 {
        if self._input_relative_temperature >= 0.0 {
            return self._input_relative_temperature;
        }

        let coldest_temp = self.temps_by_hct()[&self.coldest()];

        let range = self.temps_by_hct()[&self.warmest()] - coldest_temp;
        let difference_from_coldest = self.temps_by_hct()[&self.input] - coldest_temp;
        let input_relative_temp = if range == 0.0 {
            0.5
        } else {
            difference_from_coldest / range
        };

        self._input_relative_temperature = input_relative_temp;

        self._input_relative_temperature
    }

    /// HCTs for all hues, with the same chroma/tone as the input.
    /// Sorted from coldest first to warmest last.
    pub fn hcts_by_temp(&mut self) -> Vec<Hct> {
        if !self._hcts_by_temp.is_empty() {
            return self._hcts_by_temp.clone();
        }

        let mut hcts = self.hcts_by_hue();

        hcts.push(self.input);
        hcts.sort_by(|a, b| self.sort_by_temp(a, b));

        self._hcts_by_temp = hcts;

        self._hcts_by_temp.clone()
    }

    fn sort_by_temp(&mut self, this: &Hct, that: &Hct) -> Ordering {
        let a = self.temps_by_hct()[this];
        let b = self.temps_by_hct()[that];

        a.partial_cmp(&b).unwrap()
    }

    /// A Map with keys of HCTs in [hctsByTemp], values of raw temperature.
    pub fn temps_by_hct(&mut self) -> HashMap<Hct, f64> {
        if !self._temps_by_hct.is_empty() {
            return self._temps_by_hct.clone();
        }

        let mut all_hcts = self.hcts_by_hue();

        all_hcts.push(self.input);

        let mut temperatures_by_hct: HashMap<Hct, f64> = Default::default();

        for e in all_hcts {
            temperatures_by_hct.insert(e, TemperatureCache::raw_temperature(e));
        }

        self._temps_by_hct = temperatures_by_hct;

        self._temps_by_hct.clone()
    }

    /// HCTs for all hues, with the same chroma/tone as the input.
    /// Sorted ascending, hue 0 to 360.
    pub fn hcts_by_hue(&mut self) -> Vec<Hct> {
        if !self._hcts_by_hue.is_empty() {
            return self._hcts_by_hue.clone();
        }

        let mut hcts = vec![];

        for hue in 0..=360 {
            let color_at_hue =
                Hct::from(hue as f64, self.input.get_chroma(), self.input.get_tone());

            hcts.push(color_at_hue);
        }

        self._hcts_by_hue = hcts;

        self._hcts_by_hue.clone()
    }

    /// Determines if an angle is between two other angles, rotating clockwise.
    pub fn is_between(angle: f64, a: f64, b: f64) -> bool {
        if a < b {
            a <= angle && angle <= b
        } else {
            a <= angle || angle <= b
        }
    }

    /// Value representing cool-warm factor of a color.
    /// Values below 0 are considered cool, above, warm.
    ///
    /// Color science has researched emotion and harmony, which art uses to select
    /// colors. Warm-cool is the foundation of analogous and complementary colors.
    /// See:
    /// - Li-Chen Ou's Chapter 19 in Handbook of Color Psychology (2015).
    /// - Josef Albers' Interaction of Color chapters 19 and 21.
    ///
    /// Implementation of Ou, Woodcock and Wright's algorithm, which uses
    /// L*a*b*/LCH color space.
    /// Return value has these properties:
    /// - Values below 0 are cool, above 0 are warm.
    /// - Lower bound: -0.52 - (chroma ^ 1.07 / 20). L*a*b* chroma is infinite.
    ///   Assuming max of 130 chroma, -9.66.
    /// - Upper bound: -0.52 + (chroma ^ 1.07 / 20). L*a*b* chroma is infinite.
    ///   Assuming max of 130 chroma, 8.61.
    pub fn raw_temperature(color: Hct) -> f64 {
        let lab = Lab::from(Argb::from(color));
        let hue = sanitize_degrees_double(lab.b.atan2(lab.a).to_degrees());
        let chroma = ((lab.a * lab.a) + (lab.b * lab.b)).sqrt();

        -0.5 + 0.02 * chroma.powf(1.07) * (sanitize_degrees_double(hue - 50.0).to_radians()).cos()
    }
}
