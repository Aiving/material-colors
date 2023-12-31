#![allow(clippy::too_many_arguments)]

use ahash::HashMap;
use std::fmt::Display;

use crate::utils::color::argb_from_rgb;
use crate::utils::color::blue_from_argb;
use crate::utils::color::green_from_argb;
use crate::utils::color::red_from_argb;
use crate::utils::color::Argb;
use crate::utils::color::Rgb;

use super::quantizer::Quantizer;
use super::quantizer::QuantizerResult;
use super::quantizer_map::QuantizerMap;

// A histogram of all the input colors is constructed. It has the shape of a
//  The cube would be too large if it contained all 16 million colors:
// historical best practice is to use 5 bits  of the 8 in each channel,
// reducing the histogram to a volume of ~32,000.
const INDEX_BITS: u8 = 5;
const BITS_TO_REMOVE: u8 = 8 - INDEX_BITS;
const MAX_INDEX: u8 = 32;
const SIDE_LENGTH: usize = 33;
const TOTAL_SIZE: usize = 35937;

#[derive(Default)]
pub struct QuantizerWu {
    weights: Vec<u32>,
    moments_r: Vec<u32>,
    moments_g: Vec<u32>,
    moments_b: Vec<u32>,
    moments: Vec<f64>,
    cubes: Vec<Cube>,
}

impl Quantizer for QuantizerWu {
    fn quantize(
        &mut self,
        pixels: &[Argb],
        max_colors: i32,
        _return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult {
        let result = QuantizerMap.quantize(pixels, max_colors, None);

        self.construct_histogram(result.color_to_count);
        self.compute_moments();

        let create_boxes_result = self.create_boxes(max_colors as usize);
        let results = self.create_result(create_boxes_result.result_count as usize);
        let mut color_to_count: HashMap<Argb, u32> = Default::default();

        for e in results {
            color_to_count.insert(e, 0);
        }

        QuantizerResult {
            color_to_count,
            input_pixel_to_cluster_pixel: Default::default(),
        }
    }
}

impl QuantizerWu {
    pub fn get_index(r: u8, g: u8, b: u8) -> usize {
        let red = (r as usize) << (INDEX_BITS * 2);
        let green = (r as usize) << (INDEX_BITS + 1);
        let blue = (g as usize) << INDEX_BITS;

        red + green + blue + r as usize + g as usize + b as usize
    }

    pub fn construct_histogram(&mut self, pixels: HashMap<Argb, u32>) {
        self.weights = vec![0; TOTAL_SIZE];
        self.moments_r = vec![0; TOTAL_SIZE];
        self.moments_g = vec![0; TOTAL_SIZE];
        self.moments_b = vec![0; TOTAL_SIZE];
        self.moments = vec![0.0; TOTAL_SIZE];

        for (argb, count) in pixels {
            let red = red_from_argb(argb);
            let green = green_from_argb(argb);
            let blue = blue_from_argb(argb);

            let i_r = (red >> BITS_TO_REMOVE) + 1;
            let i_g = (green >> BITS_TO_REMOVE) + 1;
            let i_b = (blue >> BITS_TO_REMOVE) + 1;

            let index = Self::get_index(i_r, i_g, i_b);

            self.weights[index] += count;

            self.moments_r[index] += red as u32 * count;
            self.moments_g[index] += green as u32 * count;
            self.moments_b[index] += blue as u32 * count;

            self.moments[index] +=
                count as f64 * ((red * red) + (green * green) + (blue * blue)) as f64;
        }
    }

    pub fn compute_moments(&mut self) {
        for r in 1..SIDE_LENGTH {
            let mut area = [0; SIDE_LENGTH];
            let mut area_r = [0; SIDE_LENGTH];
            let mut area_g = [0; SIDE_LENGTH];
            let mut area_b = [0; SIDE_LENGTH];
            let mut area2 = vec![0.0; SIDE_LENGTH];

            for g in 1..SIDE_LENGTH {
                let mut line = 0;
                let mut line_r = 0;
                let mut line_g = 0;
                let mut line_b = 0;
                let mut line2 = 0.0;

                for b in 1..SIDE_LENGTH {
                    let index = Self::get_index(r as u8, g as u8, b as u8);

                    line += self.weights[index];
                    line_r += self.moments_r[index];
                    line_g += self.moments_g[index];
                    line_b += self.moments_b[index];
                    line2 += self.moments[index];

                    area[b] += line;
                    area_r[b] += line_r;
                    area_g[b] += line_g;
                    area_b[b] += line_b;
                    area2[b] += line2;

                    let previous_index = Self::get_index((r - 1) as u8, g as u8, b as u8);

                    self.weights[index] = self.weights[previous_index] + area[b];

                    self.moments_r[index] = self.moments_r[previous_index] + area_r[b];
                    self.moments_g[index] = self.moments_g[previous_index] + area_g[b];
                    self.moments_b[index] = self.moments_b[previous_index] + area_b[b];

                    self.moments[index] = self.moments[previous_index] + area2[b]
                }
            }
        }
    }

    pub fn create_boxes(&mut self, max_color_count: usize) -> CreateBoxesResult {
        self.cubes = vec![];

        for _ in 0..max_color_count {
            self.cubes.push(Cube {
                pixels: [[0, 0, 0], [0, 0, 0]],
                vol: 0,
            })
        }

        self.cubes[0] = Cube {
            pixels: [[0, 0, 0], [MAX_INDEX, MAX_INDEX, MAX_INDEX]],
            vol: 0,
        };

        let mut volume_variance = vec![0.0; max_color_count];
        let mut next = 0;
        let mut generated_color_count = max_color_count;
        let mut i = 1;

        while i < max_color_count {
            if self.cut(self.cubes[next], self.cubes[i]) {
                volume_variance[next] = if self.cubes[next].vol > 1 {
                    self.variance(self.cubes[next])
                } else {
                    0.0
                };
                volume_variance[i] = if self.cubes[i].vol > 1 {
                    self.variance(self.cubes[i])
                } else {
                    0.0
                };
            } else {
                volume_variance[next] = 0.0;
                i -= 1;
            }

            next = 0;

            let mut temp = volume_variance[0];
            let mut j = 1;

            while j <= i {
                if volume_variance[j] > temp {
                    temp = volume_variance[j];
                    next = j;
                }

                j += 1;
            }

            if temp <= 0.0 {
                generated_color_count = i + 1;

                break;
            }

            i += 1;
        }

        CreateBoxesResult {
            requested_count: max_color_count as i32,
            result_count: generated_color_count as i32,
        }
    }

    pub fn create_result(&self, color_count: usize) -> Vec<Argb> {
        Vec::from_iter((0..color_count).filter_map(|i| {
            let cube = self.cubes[i];
            let weight = Self::volume(cube, &self.weights);

            if weight > 0 {
                let r = (Self::volume(cube, &self.moments_r) / weight) as u8;
                let g = (Self::volume(cube, &self.moments_g) / weight) as u8;
                let b = (Self::volume(cube, &self.moments_b) / weight) as u8;

                let color = argb_from_rgb([r, g, b]);

                Some(color)
            } else {
                None
            }
        }))
    }

    pub fn variance(&self, cube: Cube) -> f64 {
        let dr = Self::volume(cube, &self.moments_r);
        let dg = Self::volume(cube, &self.moments_g);
        let db = Self::volume(cube, &self.moments_b);

        let [[r0, g0, b0], [r1, g1, b1]] = cube.pixels;

        let xx = self.moments[Self::get_index(r1, g1, b1)]
            - self.moments[Self::get_index(r1, g1, b0)]
            - self.moments[Self::get_index(r1, g0, b1)]
            + self.moments[Self::get_index(r1, g0, b0)]
            - self.moments[Self::get_index(r0, g1, b1)]
            + self.moments[Self::get_index(r0, g1, b0)]
            + self.moments[Self::get_index(r0, g0, b1)]
            - self.moments[Self::get_index(r0, g0, b0)];

        let hypotenuse = dr * dr + dg * dg + db * db;
        let volume_ = Self::volume(cube, &self.weights);

        xx - (hypotenuse / volume_) as f64
    }

    pub fn cut(&self, mut one: Cube, mut two: Cube) -> bool {
        let whole_r = Self::volume(one, &self.moments_r);
        let whole_g = Self::volume(one, &self.moments_g);
        let whole_b = Self::volume(one, &self.moments_b);
        let whole_w = Self::volume(one, &self.weights);

        let max_rresult = self.maximize(
            one,
            Direction::Red,
            one.pixels[0][0] + 1,
            one.pixels[1][0],
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_gresult = self.maximize(
            one,
            Direction::Green,
            one.pixels[0][1] + 1,
            one.pixels[1][1],
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_bresult = self.maximize(
            one,
            Direction::Blue,
            one.pixels[0][2] + 1,
            one.pixels[1][2],
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );

        let cut_direction: Direction;

        let max_r = max_rresult.maximum;
        let max_g = max_gresult.maximum;
        let max_b = max_bresult.maximum;

        if max_r >= max_g && max_r >= max_b {
            cut_direction = Direction::Red;

            if max_rresult.cut_location < Some(0) {
                return false;
            }
        } else if max_g >= max_r && max_g >= max_b {
            cut_direction = Direction::Green;
        } else {
            cut_direction = Direction::Blue;
        }

        two.pixels[1][0] = one.pixels[1][0];
        two.pixels[1][1] = one.pixels[1][1];
        two.pixels[1][2] = one.pixels[1][2];

        match cut_direction {
            Direction::Red => {
                one.pixels[1][0] = max_rresult.cut_location.unwrap_or_default();
                two.pixels[0][0] = one.pixels[1][0];
                two.pixels[0][1] = one.pixels[0][1];
                two.pixels[0][2] = one.pixels[0][2];
            }
            Direction::Green => {
                one.pixels[1][1] = max_gresult.cut_location.unwrap_or_default();
                two.pixels[0][0] = one.pixels[0][0];
                two.pixels[0][1] = one.pixels[1][1];
                two.pixels[0][2] = one.pixels[0][2];
            }
            Direction::Blue => {
                one.pixels[1][2] = max_bresult.cut_location.unwrap_or_default();
                two.pixels[0][0] = one.pixels[0][0];
                two.pixels[0][1] = one.pixels[0][1];
                two.pixels[0][2] = one.pixels[1][2];
            }
        }

        one.vol = (one.pixels[1][0] - one.pixels[0][0]) as u16
            * (one.pixels[1][1] - one.pixels[0][1]) as u16
            * (one.pixels[1][2] - one.pixels[0][2]) as u16;
        two.vol = (two.pixels[1][0] - two.pixels[0][0]) as u16
            * (two.pixels[1][1] - two.pixels[0][1]) as u16
            * (two.pixels[1][2] - two.pixels[0][2]) as u16;

        true
    }

    pub fn maximize(
        &self,
        cube: Cube,
        direction: Direction,
        first: u8,
        last: u8,
        whole_r: i64,
        whole_g: i64,
        whole_b: i64,
        whole_w: i64,
    ) -> MaximizeResult {
        let bottom_r = Self::bottom(cube, &direction, &self.moments_r);
        let bottom_g = Self::bottom(cube, &direction, &self.moments_g);
        let bottom_b = Self::bottom(cube, &direction, &self.moments_b);
        let bottom_w = Self::bottom(cube, &direction, &self.weights);

        let mut max = 0.0;
        let mut cut = None;

        for i in first..last {
            let mut half_r = bottom_r + Self::top(cube, &direction, i, &self.moments_r);
            let mut half_g = bottom_g + Self::top(cube, &direction, i, &self.moments_g);
            let mut half_b = bottom_b + Self::top(cube, &direction, i, &self.moments_b);
            let mut half_w = bottom_w + Self::top(cube, &direction, i, &self.weights);

            if half_w == 0 {
                continue;
            }

            let mut temp_numerator =
                ((half_r * half_r) + (half_g * half_g) + (half_b * half_b)) as f64;
            let mut temp_denominator = half_w as f64;
            let mut temp = temp_numerator / temp_denominator;

            half_r = whole_r - half_r;
            half_g = whole_g - half_g;
            half_b = whole_b - half_b;
            half_w = whole_w - half_w;

            if half_w == 0 {
                continue;
            }

            temp_numerator = ((half_r * half_r) + (half_g * half_g) + (half_b * half_b)) as f64;
            temp_denominator = half_w as f64;
            temp += temp_numerator / temp_denominator;

            if temp > max {
                max = temp;
                cut = Some(i);
            }
        }

        MaximizeResult {
            cut_location: cut,
            maximum: max,
        }
    }

    pub fn volume(cube: Cube, moment: &[u32]) -> i64 {
        let [[r0, g0, b0], [r1, g1, b1]] = cube.pixels;

        moment[Self::get_index(r1, g1, b1)] as i64
            - moment[Self::get_index(r1, g1, b0)] as i64
            - moment[Self::get_index(r1, g0, b1)] as i64
            + moment[Self::get_index(r1, g0, b0)] as i64
            - moment[Self::get_index(r0, g1, b1)] as i64
            + moment[Self::get_index(r0, g1, b0)] as i64
            + moment[Self::get_index(r0, g0, b1)] as i64
            - moment[Self::get_index(r0, g0, b0)] as i64
    }

    pub fn bottom(cube: Cube, direction: &Direction, moment: &[u32]) -> i64 {
        let [[r0, g0, b0], [r1, g1, b1]] = cube.pixels;

        match direction {
            Direction::Red => {
                -(moment[Self::get_index(r0, g1, b1)] as i64)
                    + moment[Self::get_index(r0, g1, b0)] as i64
                    + moment[Self::get_index(r0, g0, b1)] as i64
                    - moment[Self::get_index(r0, g0, b0)] as i64
            }
            Direction::Green => {
                -(moment[Self::get_index(r1, g0, b1)] as i64)
                    + moment[Self::get_index(r1, g0, b0)] as i64
                    + moment[Self::get_index(r0, g0, b1)] as i64
                    - moment[Self::get_index(r0, g0, b0)] as i64
            }
            Direction::Blue => {
                -(moment[Self::get_index(r1, g1, b0)] as i64)
                    + moment[Self::get_index(r1, g0, b0)] as i64
                    + moment[Self::get_index(r0, g1, b0)] as i64
                    - moment[Self::get_index(r0, g0, b0)] as i64
            }
        }
    }

    pub fn top(cube: Cube, direction: &Direction, position: u8, moment: &[u32]) -> i64 {
        let [[r0, g0, b0], [r1, g1, b1]] = cube.pixels;

        match direction {
            Direction::Red => {
                moment[Self::get_index(position, g1, b1)] as i64
                    - moment[Self::get_index(position, g1, b0)] as i64
                    - moment[Self::get_index(position, g0, b1)] as i64
                    + moment[Self::get_index(position, g0, b0)] as i64
            }
            Direction::Green => {
                moment[Self::get_index(r1, position, b1)] as i64
                    - moment[Self::get_index(r1, position, b0)] as i64
                    - moment[Self::get_index(r0, position, b1)] as i64
                    + moment[Self::get_index(r0, position, b0)] as i64
            }
            Direction::Blue => {
                moment[Self::get_index(r1, g1, position)] as i64
                    - moment[Self::get_index(r1, g0, position)] as i64
                    - moment[Self::get_index(r0, g1, position)] as i64
                    + moment[Self::get_index(r0, g0, position)] as i64
            }
        }
    }
}

pub enum Direction {
    Red,
    Green,
    Blue,
}

pub struct MaximizeResult {
    // < 0 if cut impossible
    pub cut_location: Option<u8>,
    pub maximum: f64,
}

pub struct CreateBoxesResult {
    pub requested_count: i32,
    pub result_count: i32,
}

#[derive(Clone, Copy)]
pub struct Cube {
    pub pixels: [Rgb; 2],
    pub vol: u16,
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Box R {} -> {} G {} -> {} B {} -> {} VOL = {}",
            self.pixels[0][0],
            self.pixels[0][1],
            self.pixels[0][2],
            self.pixels[1][0],
            self.pixels[1][1],
            self.pixels[1][2],
            self.vol
        )
    }
}
