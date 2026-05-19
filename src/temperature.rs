#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::{Lab, Rgb},
    hct::Hct,
    utils::{FromRef, math::sanitize_degrees_double},
};

#[derive(Debug, Clone, Copy)]
struct HctWithTemp {
    color: Hct,
    temp: f64,
}

/// Design utilities using color temperature theory.
///
/// Analogous colors, complementary color, and cache to efficiently, lazily,
/// generate data for calculations when needed.
pub struct TemperatureCache {
    input: Hct,

    warmest: HctWithTemp,
    coldest: HctWithTemp,

    /// HCTs for all hues, with the same chroma/tone as the input.
    /// Sorted ascending, hue 0 to 360.
    hcts_by_hue: [HctWithTemp; 362],
    /// Relative temperature of the input color. See [`relative_temperature`].
    ///
    /// [`relative_temperature`]: Self::relative_temperature
    input_relative_temperature: f64,
    _complement: Option<Hct>,
}

impl TemperatureCache {
    pub fn new(input: Hct) -> Self {
        let chroma = input.get_chroma();
        let tone = input.get_tone();

        let input_temp = Self::raw_temperature(&input);
        let mut hcts_by_hue = [HctWithTemp {
            color: input,
            temp: input_temp,
        }; 362];

        for (index, item) in hcts_by_hue.iter_mut().enumerate().take(361) {
            let color = Hct::from(index as f64, chroma, tone);
            let temp = Self::raw_temperature(&color);

            *item = HctWithTemp { color, temp };
        }

        let mut hcts_by_temp = hcts_by_hue;
        let depth = hcts_by_temp.len().ilog2() * 2;

        introsort_slice(&mut hcts_by_temp, depth);

        let mut cache = Self {
            input,
            warmest: hcts_by_temp[361],
            coldest: hcts_by_temp[0],
            hcts_by_hue,
            input_relative_temperature: -1.0,
            _complement: None,
        };

        cache.input_relative_temperature = cache.relative_temperature(input_temp);

        cache
    }

    pub fn analogous(&self) -> [Hct; 5] {
        self.analogous_generic::<5, 12>()
    }

    /// A set of colors with differing hues, equidistant in temperature.
    ///
    /// In art, this is usually described as a set of 5 colors on a color wheel
    /// divided into 12 sections. This method allows provision of either of
    /// those values.
    ///
    /// Behavior is undefined when `count` or `divisions` is 0.
    /// When `divisions` < `count`, colors repeat.
    ///
    /// - `count`: The number of colors to return, includes the input color.
    /// - `divisions`: The number of divisions on the color wheel.
    pub fn analogous_generic<const C: usize, const D: usize>(&self) -> [Hct; C] {
        // let count = count.unwrap_or(5);
        // let divisions = divisions.unwrap_or(12);
        let start_hue = self.input.get_hue().round() as usize;

        let start_hct = self.hcts_by_hue[start_hue];
        let mut last_temp = self.relative_temperature(self.hcts_by_hue[start_hue].temp);
        let mut all_colors = [start_hct; D];
        let mut all_colors_len = 1;

        let mut absolute_total_temp_delta = 0.0;

        for i in 0..360 {
            let hue = sanitize_degrees_double((start_hue + i) as f64);
            let temp = self.relative_temperature(self.hcts_by_hue[hue as usize].temp);
            let temp_delta = (temp - last_temp).abs();

            last_temp = temp;
            absolute_total_temp_delta += temp_delta;
        }

        let mut hue_addend = 1;
        let temp_step = absolute_total_temp_delta / D as f64;

        let mut total_temp_delta = 0.0;

        last_temp = self.relative_temperature(self.hcts_by_hue[start_hue].temp);

        while all_colors_len < D {
            let hue = sanitize_degrees_double((start_hue + hue_addend) as f64);
            let hct = self.hcts_by_hue[hue as usize];
            let temp = self.relative_temperature(self.hcts_by_hue[hue as usize].temp);
            let temp_delta = (temp - last_temp).abs();

            total_temp_delta += temp_delta;

            let desired_total_temp_delta_for_index = all_colors_len as f64 * temp_step;

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
            while index_satisfied && all_colors_len < D {
                all_colors[all_colors_len] = hct;
                all_colors_len += 1;

                let desired_total_temp_delta_for_index = (all_colors_len + index_addend) as f64 * temp_step;

                index_satisfied = total_temp_delta >= desired_total_temp_delta_for_index;
                index_addend += 1;
            }

            last_temp = temp;
            hue_addend += 1;

            if hue_addend > 360 {
                while all_colors_len < D {
                    all_colors[all_colors_len] = hct;
                    all_colors_len += 1;
                }

                break;
            }
        }

        let mut answers = [self.input; C];
        let mut answers_len = 1;

        // First, generate analogues from rotating counter-clockwise.
        let increase_hue_count = (C as isize - 1) / 2;

        for i in 1..=increase_hue_count {
            let mut index = 0_isize - i;

            while index < 0 {
                index += all_colors.len() as isize;
            }

            if index >= all_colors.len() as isize {
                index %= all_colors.len() as isize;
            }

            answers[increase_hue_count as usize - answers_len] = all_colors[index as usize].color;
            answers_len += 1;
        }

        // Second, generate analogues from rotating clockwise.
        let decrease_hue_count = C as isize - increase_hue_count - 1;

        for i in 1..=decrease_hue_count {
            let mut index = i;

            while index < 0 {
                index += all_colors.len() as isize;
            }

            if index >= all_colors.len() as isize {
                index %= all_colors.len() as isize;
            }

            answers[answers_len] = all_colors[index as usize].color;
            answers_len += 1;
        }

        answers
    }

    /// A color that complements the input color aesthetically.
    ///
    /// In art, this is usually described as being across the color wheel.
    /// History of this shows intent as a color that is just as cool-warm as the
    /// input color is warm-cool.
    ///
    /// # Panics
    ///
    /// Will panic if there is no coldest or warmest HCT
    pub fn complement(&mut self) -> Hct {
        if let Some(complement) = self._complement {
            return complement;
        }

        let (coldest_hue, coldest_temp) = (self.coldest.color.get_hue(), self.coldest.temp);
        let (warmest_hue, warmest_temp) = (self.warmest.color.get_hue(), self.warmest.temp);

        let temp_range = warmest_temp - coldest_temp;
        let warmest_to_coldest = Self::is_between(self.input.get_hue(), coldest_hue, warmest_hue);

        let [start_hue, end_hue] = if warmest_to_coldest {
            [warmest_hue, coldest_hue]
        } else {
            [coldest_hue, warmest_hue]
        };

        let direction_of_rotation = 1.0_f64;
        let mut smallest_error = 1000.0;
        let mut answer = &self.hcts_by_hue[self.input.get_hue().round() as usize];

        let complement_relative_temp = 1.0 - self.input_relative_temperature;

        // Find the color in the other section, closest to the inverse percentile
        // of the input color. This is the complement.
        for hue_addend in 0..=360 {
            let hue = sanitize_degrees_double(direction_of_rotation.mul_add(f64::from(hue_addend), start_hue));

            if !Self::is_between(hue, start_hue, end_hue) {
                continue;
            }

            let possible_answer = &self.hcts_by_hue[hue.round() as usize];
            let relative_temp = (possible_answer.temp - coldest_temp) / temp_range;
            let error = (complement_relative_temp - relative_temp).abs();

            if error < smallest_error {
                smallest_error = error;
                answer = possible_answer;
            }
        }

        let answer = answer.color;

        self._complement = Some(answer);

        answer
    }

    /// Temperature relative to all colors with the same chroma and tone.
    /// Value on a scale from 0 to 1.
    pub const fn relative_temperature(&self, temp: f64) -> f64 {
        let coldest = self.coldest;
        let warmest = self.warmest;

        let range = warmest.temp - coldest.temp;
        let difference_from_coldest = temp - coldest.temp;

        // Handle when there's no difference in temperature between warmest and
        // coldest: for example, at T100, only one color is available, white.
        if range == 0.0 { 0.5 } else { difference_from_coldest / range }
    }

    /// Determines if an angle is between two other angles, rotating clockwise.
    pub const fn is_between(angle: f64, a: f64, b: f64) -> bool {
        if a < b { a <= angle && angle <= b } else { a <= angle || angle <= b }
    }

    /// Value representing cool-warm factor of a color.
    /// Values below 0 are considered cool, above, warm.
    ///
    /// Color science has researched emotion and harmony, which art uses to
    /// select colors. Warm-cool is the foundation of analogous and
    /// complementary colors. See:
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
    pub fn raw_temperature(color: &Hct) -> f64 {
        let lab = Lab::from(Rgb::from_ref(color));
        let hue = sanitize_degrees_double(lab.b.atan2(lab.a).to_degrees());

        let chroma = lab.a.hypot(lab.b);

        (0.02 * chroma.powf(1.07)).mul_add((sanitize_degrees_double(hue - 50.0).to_radians()).cos(), -0.5)
    }
}

const fn insertion_sort_slice(slice: &mut [HctWithTemp]) {
    let n = slice.len();

    if n <= 1 {
        return;
    }

    let mut i = 1;

    while i < n {
        let mut j = i;

        while j > 0 && (slice[j - 1].temp > slice[j].temp) {
            (slice[j - 1], slice[j]) = (slice[j], slice[j - 1]);

            j -= 1;
        }

        i += 1;
    }
}

const fn max_heapify_slice(slice: &mut [HctWithTemp], n: usize, i: usize) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = l + 1;

    if l < n && (slice[l].temp > slice[largest].temp) {
        largest = l;
    }

    if r < n && (slice[r].temp > slice[largest].temp) {
        largest = r;
    }

    if largest != i {
        (slice[i], slice[largest]) = (slice[largest], slice[i]);

        max_heapify_slice(slice, n, largest);
    }
}

const fn heapsort_slice(slice: &mut [HctWithTemp]) {
    let n = slice.len();

    if n <= 1 {
        return;
    }

    let mut i = n / 2 - 1;

    while i > 0 {
        max_heapify_slice(slice, n, i);

        i -= 1;
    }

    max_heapify_slice(slice, n, i);

    let mut i = n - 1;

    while i > 0 {
        (slice[0], slice[i]) = (slice[i], slice[0]);

        max_heapify_slice(slice, i, 0);

        i -= 1;
    }
}

const fn introsort_slice(slice: &mut [HctWithTemp], recursion_depth: u32) {
    if slice.len() <= 1 {
    } else if slice.len() <= 16 {
        insertion_sort_slice(slice);
    } else if recursion_depth == 0 {
        heapsort_slice(slice);
    } else {
        let (pivot, rest) = slice.split_first_mut().expect("slice is not empty, as verified above");
        let mut left = 0;
        let mut right = rest.len() - 1;

        while left <= right {
            if rest[left].temp <= pivot.temp {
                left += 1;
            } else if rest[right].temp > pivot.temp {
                if right == 0 {
                    break;
                }

                right -= 1;
            } else {
                (rest[left], rest[right]) = (rest[right], rest[left]);

                left += 1;

                if right == 0 {
                    break;
                }

                right -= 1;
            }
        }

        (slice[0], slice[left]) = (slice[left], slice[0]);

        let (left, right) = slice.split_at_mut(left);

        introsort_slice(left, recursion_depth - 1);

        if let Some((_pivot, right)) = right.split_first_mut() {
            introsort_slice(right, recursion_depth - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::TemperatureCache;
    use crate::{color::Rgb, hct::Hct};

    #[test]
    fn test_raw_temperature() {
        let blue_hct = Hct::new(Rgb::from_u32(0x0000FF));
        let red_hct = Hct::new(Rgb::from_u32(0xFF0000));
        let green_hct = Hct::new(Rgb::from_u32(0x00FF00));
        let white_hct = Hct::new(Rgb::from_u32(0xFFFFFF));
        let black_hct = Hct::new(Rgb::from_u32(0x000000));

        let blue_temp = TemperatureCache::raw_temperature(&blue_hct);
        let red_temp = TemperatureCache::raw_temperature(&red_hct);
        let green_temp = TemperatureCache::raw_temperature(&green_hct);
        let white_temp = TemperatureCache::raw_temperature(&white_hct);
        let black_temp = TemperatureCache::raw_temperature(&black_hct);

        assert_approx_eq!(f64, -1.393, blue_temp, epsilon = 0.001);
        assert_approx_eq!(f64, 2.351, red_temp, epsilon = 0.001);
        assert_approx_eq!(f64, -0.267, green_temp, epsilon = 0.001);
        assert_approx_eq!(f64, -0.5, white_temp, epsilon = 0.001);
        assert_approx_eq!(f64, -0.5, black_temp, epsilon = 0.001);
    }

    #[test]
    fn test_complement() {
        let blue_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0x0000FF))).complement().into();
        let red_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFF0000))).complement().into();
        let green_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0x00FF00))).complement().into();
        let white_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFFFFFF))).complement().into();
        let black_complement: Rgb = TemperatureCache::new(Hct::new(Rgb::from_u32(0x000000))).complement().into();

        assert_eq!(Rgb::from_u32(0x9D0002), blue_complement);
        assert_eq!(Rgb::from_u32(0x007BFC), red_complement);
        assert_eq!(Rgb::from_u32(0xFFD2C9), green_complement);
        assert_eq!(Rgb::from_u32(0xFFFFFF), white_complement);
        assert_eq!(Rgb::from_u32(0x000000), black_complement);
    }

    #[test]
    fn test_blue_analogous() {
        let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0x0000FF))).analogous();

        assert_eq!(Rgb::from_u32(0x00590C), analogous[0].into());
        assert_eq!(Rgb::from_u32(0x00564E), analogous[1].into());
        assert_eq!(Rgb::from_u32(0x0000FF), analogous[2].into());
        assert_eq!(Rgb::from_u32(0x6700CC), analogous[3].into());
        assert_eq!(Rgb::from_u32(0x81009F), analogous[4].into());
        assert_eq!(5, analogous.len());
    }

    #[test]
    fn test_red_analogous() {
        let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFF0000))).analogous();

        assert_eq!(Rgb::from_u32(0xF60082), analogous[0].into());
        assert_eq!(Rgb::from_u32(0xFC004C), analogous[1].into());
        assert_eq!(Rgb::from_u32(0xFF0000), analogous[2].into());
        assert_eq!(Rgb::from_u32(0xD95500), analogous[3].into());
        assert_eq!(Rgb::from_u32(0xAF7200), analogous[4].into());
        assert_eq!(5, analogous.len());
    }

    #[test]
    fn test_green_analogous() {
        let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0x00FF00))).analogous();

        assert_eq!(Rgb::from_u32(0xCEE900), analogous[0].into());
        assert_eq!(Rgb::from_u32(0x92F500), analogous[1].into());
        assert_eq!(Rgb::from_u32(0x00FF00), analogous[2].into());
        assert_eq!(Rgb::from_u32(0x00FD6F), analogous[3].into());
        assert_eq!(Rgb::from_u32(0x00FAB3), analogous[4].into());
        assert_eq!(5, analogous.len());
    }

    #[test]
    fn test_white_analogous() {
        let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0xFFFFFF))).analogous();

        assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[0].into());
        assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[1].into());
        assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[2].into());
        assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[3].into());
        assert_eq!(Rgb::from_u32(0xFFFFFF), analogous[4].into());
        assert_eq!(5, analogous.len());
    }

    #[test]
    fn test_black_analogous() {
        let analogous = TemperatureCache::new(Hct::new(Rgb::from_u32(0x000000))).analogous();

        assert_eq!(Rgb::from_u32(0x000000), analogous[0].into());
        assert_eq!(Rgb::from_u32(0x000000), analogous[1].into());
        assert_eq!(Rgb::from_u32(0x000000), analogous[2].into());
        assert_eq!(Rgb::from_u32(0x000000), analogous[3].into());
        assert_eq!(Rgb::from_u32(0x000000), analogous[4].into());
        assert_eq!(5, analogous.len());
    }
}
