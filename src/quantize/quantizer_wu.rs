#![allow(clippy::too_many_arguments)]

use super::{Quantizer, QuantizerMap, QuantizerResult};
#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;
use crate::{
    color::{Argb, Rgb},
    IndexMap,
};
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
use core::fmt;
#[cfg(feature = "std")]
use std::{vec, vec::Vec};

// A histogram of all the input colors is constructed. It has the shape of a
//  The cube would be too large if it contained all 16 million colors:
// historical best practice is to use 5 bits  of the 8 in each channel,
// reducing the histogram to a volume of ~32,000.
const INDEX_BITS: u8 = 5;
const BITS_TO_REMOVE: u8 = 8 - INDEX_BITS;
const SIDE_LENGTH: usize = (1 << INDEX_BITS) + 1;
const TOTAL_SIZE: usize = SIDE_LENGTH.pow(3);

pub struct QuantizerWu {
    weights: Vec<i64>,
    moments_r: Vec<i64>,
    moments_g: Vec<i64>,
    moments_b: Vec<i64>,
    moments: Vec<f64>,
    cubes: Vec<Cube>,
}

impl QuantizerWu {
    fn new(max_colors: usize) -> Self {
        Self {
            weights: vec![0; TOTAL_SIZE],
            moments_r: vec![0; TOTAL_SIZE],
            moments_g: vec![0; TOTAL_SIZE],
            moments_b: vec![0; TOTAL_SIZE],
            moments: vec![0.0; TOTAL_SIZE],
            cubes: vec![
                Cube {
                    pixels: [Rgb::default(), Rgb::default()],
                    vol: 0
                };
                max_colors
            ],
        }
    }
}

impl Quantizer for QuantizerWu {
    fn quantize(pixels: &[Argb], max_colors: usize) -> QuantizerResult {
        let mut result = QuantizerMap::quantize(pixels, max_colors);

        result.color_to_count.sort_by(|_, a, _, b| a.cmp(b));

        let mut quantizer = Self::new(max_colors);

        quantizer.construct_histogram(result.color_to_count);
        quantizer.compute_moments();

        let create_boxes_result = quantizer.create_boxes(max_colors);
        let color_to_count = quantizer.create_result(create_boxes_result.result_count);

        QuantizerResult {
            color_to_count,
            input_pixel_to_cluster_pixel: IndexMap::default(),
        }
    }
}

impl QuantizerWu {
    pub fn get_index<T: Into<usize>>(r: T, g: T, b: T) -> usize {
        let r: usize = r.into();
        let g: usize = g.into();
        let b: usize = b.into();

        (r << (INDEX_BITS * 2)) + (r << (INDEX_BITS + 1)) + (g << INDEX_BITS) + r + g + b
    }

    pub fn construct_histogram(&mut self, pixels: IndexMap<Argb, u32>) {
        for (pixel, count) in pixels {
            let red = pixel.red;
            let green = pixel.green;
            let blue = pixel.blue;

            let i_r = (red >> BITS_TO_REMOVE) + 1;
            let i_g = (green >> BITS_TO_REMOVE) + 1;
            let i_b = (blue >> BITS_TO_REMOVE) + 1;

            let index = Self::get_index(i_r, i_g, i_b);

            self.weights[index] += i64::from(count);

            self.moments_r[index] += i64::from(red) * i64::from(count);
            self.moments_g[index] += i64::from(green) * i64::from(count);
            self.moments_b[index] += i64::from(blue) * i64::from(count);

            self.moments[index] += f64::from(count)
                * f64::from(blue).mul_add(
                    f64::from(blue),
                    f64::from(red).mul_add(f64::from(red), f64::from(green) * f64::from(green)),
                );
        }
    }

    pub fn compute_moments(&mut self) {
        for r in 1..SIDE_LENGTH {
            let mut area = [0; SIDE_LENGTH];
            let mut area_r = [0; SIDE_LENGTH];
            let mut area_g = [0; SIDE_LENGTH];
            let mut area_b = [0; SIDE_LENGTH];
            let mut area2 = [0.0; SIDE_LENGTH];

            for g in 1..SIDE_LENGTH {
                let mut line = 0;
                let mut line_r = 0;
                let mut line_g = 0;
                let mut line_b = 0;
                let mut line2 = 0.0;

                for b in 1..SIDE_LENGTH {
                    let index = Self::get_index(r, g, b);

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

                    let previous_index = Self::get_index(r - 1, g, b);

                    self.weights[index] = self.weights[previous_index] + area[b];
                    self.moments_r[index] = self.moments_r[previous_index] + area_r[b];
                    self.moments_g[index] = self.moments_g[previous_index] + area_g[b];
                    self.moments_b[index] = self.moments_b[previous_index] + area_b[b];
                    self.moments[index] = self.moments[previous_index] + area2[b];
                }
            }
        }
    }

    pub fn create_boxes(&mut self, max_color_count: usize) -> CreateBoxesResult {
        self.cubes[0] = Cube {
            pixels: [
                Rgb::default(),
                Rgb::new(
                    SIDE_LENGTH as u8 - 1,
                    SIDE_LENGTH as u8 - 1,
                    SIDE_LENGTH as u8 - 1,
                ),
            ],
            vol: 0,
        };

        let mut volume_variance = vec![0.0; max_color_count];
        let mut next = 0;
        let mut generated_color_count = max_color_count;
        let mut i = 1;

        while i < max_color_count {
            if self.cut(next, i) {
                volume_variance[next] = if self.cubes[next].vol > 1 {
                    self.variance(&self.cubes[next])
                } else {
                    0.0
                };

                volume_variance[i] = if self.cubes[i].vol > 1 {
                    self.variance(&self.cubes[i])
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
            requested_count: max_color_count,
            result_count: generated_color_count,
        }
    }

    pub fn create_result(&self, color_count: usize) -> IndexMap<Argb, u32> {
        let mut result = IndexMap::default();

        for i in 0..color_count {
            let cube = &self.cubes[i];
            let weight = Self::volume(cube, &self.weights);

            if weight > 0 {
                let r = ((Self::volume(cube, &self.moments_r)) / weight) as u8;
                let g = ((Self::volume(cube, &self.moments_g)) / weight) as u8;
                let b = ((Self::volume(cube, &self.moments_b)) / weight) as u8;

                let color = Rgb::new(r, g, b).into();

                result.insert(color, 0);
            }
        }

        result
    }

    pub fn variance(&self, cube: &Cube) -> f64 {
        let dr = Self::volume(cube, &self.moments_r) as f64;
        let dg = Self::volume(cube, &self.moments_g) as f64;
        let db = Self::volume(cube, &self.moments_b) as f64;

        let xx = self.moments[Self::get_index::<u8>(cube.r(1), cube.g(1), cube.b(1))]
            - self.moments[Self::get_index::<u8>(cube.r(1), cube.g(1), cube.b(0))]
            - self.moments[Self::get_index::<u8>(cube.r(1), cube.g(0), cube.b(1))]
            + self.moments[Self::get_index::<u8>(cube.r(1), cube.g(0), cube.b(0))]
            - self.moments[Self::get_index::<u8>(cube.r(0), cube.g(1), cube.b(1))]
            + self.moments[Self::get_index::<u8>(cube.r(0), cube.g(1), cube.b(0))]
            + self.moments[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(1))]
            - self.moments[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(0))];

        let hypotenuse = db.mul_add(db, dr.mul_add(dr, dg * dg));
        let volume = Self::volume(cube, &self.weights) as f64;

        xx - (hypotenuse / volume)
    }

    pub fn cut(&mut self, next: usize, i: usize) -> bool {
        let (mut one, mut two) = (self.cubes[next].clone(), self.cubes[i].clone());

        let whole_r = Self::volume(&one, &self.moments_r);
        let whole_g = Self::volume(&one, &self.moments_g);
        let whole_b = Self::volume(&one, &self.moments_b);
        let whole_w = Self::volume(&one, &self.weights);

        let max_rresult = self.maximize(
            &one,
            &Direction::Red,
            one.r::<i32>(0) + 1,
            one.r::<i32>(1),
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_gresult = self.maximize(
            &one,
            &Direction::Green,
            one.g::<i32>(0) + 1,
            one.g::<i32>(1),
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_bresult = self.maximize(
            &one,
            &Direction::Blue,
            one.b::<i32>(0) + 1,
            one.b::<i32>(1),
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

            if max_rresult.cut_location < 0 {
                return false;
            }
        } else if max_g >= max_r && max_g >= max_b {
            cut_direction = Direction::Green;
        } else {
            cut_direction = Direction::Blue;
        }

        two.pixels[1].red = one.pixels[1].red;
        two.pixels[1].green = one.pixels[1].green;
        two.pixels[1].blue = one.pixels[1].blue;

        match cut_direction {
            Direction::Red => {
                one.pixels[1].red = max_rresult.cut_location as u8;
                two.pixels[0].red = one.pixels[1].red;
                two.pixels[0].green = one.pixels[0].green;
                two.pixels[0].blue = one.pixels[0].blue;
            }
            Direction::Green => {
                one.pixels[1].green = max_gresult.cut_location as u8;
                two.pixels[0].red = one.pixels[0].red;
                two.pixels[0].green = one.pixels[1].green;
                two.pixels[0].blue = one.pixels[0].blue;
            }
            Direction::Blue => {
                one.pixels[1].blue = max_bresult.cut_location as u8;
                two.pixels[0].red = one.pixels[0].red;
                two.pixels[0].green = one.pixels[0].green;
                two.pixels[0].blue = one.pixels[1].blue;
            }
        }

        one.vol = (one.r::<i32>(1) - one.r::<i32>(0))
            * (one.g::<i32>(1) - one.g::<i32>(0))
            * (one.b::<i32>(1) - one.b::<i32>(0));
        two.vol = (two.r::<i32>(1) - two.r::<i32>(0))
            * (two.g::<i32>(1) - two.g::<i32>(0))
            * (two.b::<i32>(1) - two.b::<i32>(0));

        self.cubes[next] = one;
        self.cubes[i] = two;

        true
    }

    pub fn maximize(
        &self,
        cube: &Cube,
        direction: &Direction,
        first: i32,
        last: i32,
        whole_r: i64,
        whole_g: i64,
        whole_b: i64,
        whole_w: i64,
    ) -> MaximizeResult {
        let bottom_r = Self::bottom(cube, direction, &self.moments_r) as f64;
        let bottom_g = Self::bottom(cube, direction, &self.moments_g) as f64;
        let bottom_b = Self::bottom(cube, direction, &self.moments_b) as f64;
        let bottom_w = Self::bottom(cube, direction, &self.weights) as f64;

        let mut max = 0.0;
        let mut cut = -1;

        for i in first..last {
            let mut half_r = bottom_r + Self::top(cube, direction, i, &self.moments_r) as f64;
            let mut half_g = bottom_g + Self::top(cube, direction, i, &self.moments_g) as f64;
            let mut half_b = bottom_b + Self::top(cube, direction, i, &self.moments_b) as f64;
            let mut half_w = bottom_w + Self::top(cube, direction, i, &self.weights) as f64;

            if half_w == 0.0 {
                continue;
            }

            let mut temp_numerator = half_b.mul_add(half_b, half_r.mul_add(half_r, half_g.powi(2)));
            let mut temp_denominator = half_w;
            let mut temp = temp_numerator / temp_denominator;

            half_r = whole_r as f64 - half_r;
            half_g = whole_g as f64 - half_g;
            half_b = whole_b as f64 - half_b;
            half_w = whole_w as f64 - half_w;

            if half_w == 0.0 {
                continue;
            }

            temp_numerator = half_b.mul_add(half_b, half_r.mul_add(half_r, half_g.powi(2)));
            temp_denominator = half_w;
            temp += temp_numerator / temp_denominator;

            if temp > max {
                max = temp;
                cut = i;
            }
        }

        MaximizeResult {
            cut_location: cut,
            maximum: max,
        }
    }

    pub fn volume(cube: &Cube, moment: &[i64]) -> i64 {
        moment[Self::get_index::<u8>(cube.r(1), cube.g(1), cube.b(1))]
            - moment[Self::get_index::<u8>(cube.r(1), cube.g(1), cube.b(0))]
            - moment[Self::get_index::<u8>(cube.r(1), cube.g(0), cube.b(1))]
            + moment[Self::get_index::<u8>(cube.r(1), cube.g(0), cube.b(0))]
            - moment[Self::get_index::<u8>(cube.r(0), cube.g(1), cube.b(1))]
            + moment[Self::get_index::<u8>(cube.r(0), cube.g(1), cube.b(0))]
            + moment[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(1))]
            - moment[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(0))]
    }

    pub fn bottom(cube: &Cube, direction: &Direction, moment: &[i64]) -> i64 {
        match direction {
            Direction::Red => {
                -moment[Self::get_index::<u8>(cube.r(0), cube.g(1), cube.b(1))]
                    + moment[Self::get_index::<u8>(cube.r(0), cube.g(1), cube.b(0))]
                    + moment[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(1))]
                    - moment[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(0))]
            }
            Direction::Green => {
                -moment[Self::get_index::<u8>(cube.r(1), cube.g(0), cube.b(1))]
                    + moment[Self::get_index::<u8>(cube.r(1), cube.g(0), cube.b(0))]
                    + moment[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(1))]
                    - moment[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(0))]
            }
            Direction::Blue => {
                -moment[Self::get_index::<u8>(cube.r(1), cube.g(1), cube.b(0))]
                    + moment[Self::get_index::<u8>(cube.r(1), cube.g(0), cube.b(0))]
                    + moment[Self::get_index::<u8>(cube.r(0), cube.g(1), cube.b(0))]
                    - moment[Self::get_index::<u8>(cube.r(0), cube.g(0), cube.b(0))]
            }
        }
    }

    pub fn top(cube: &Cube, direction: &Direction, position: i32, moment: &[i64]) -> i64 {
        match direction {
            Direction::Red => {
                moment[Self::get_index(position as usize, cube.g(1), cube.b(1))]
                    - moment[Self::get_index(position as usize, cube.g(1), cube.b(0))]
                    - moment[Self::get_index(position as usize, cube.g(0), cube.b(1))]
                    + moment[Self::get_index(position as usize, cube.g(0), cube.b(0))]
            }
            Direction::Green => {
                moment[Self::get_index(cube.r(1), position as usize, cube.b(1))]
                    - moment[Self::get_index(cube.r(1), position as usize, cube.b(0))]
                    - moment[Self::get_index(cube.r(0), position as usize, cube.b(1))]
                    + moment[Self::get_index(cube.r(0), position as usize, cube.b(0))]
            }
            Direction::Blue => {
                moment[Self::get_index(cube.r(1), cube.g(1), position as usize)]
                    - moment[Self::get_index(cube.r(1), cube.g(0), position as usize)]
                    - moment[Self::get_index(cube.r(0), cube.g(1), position as usize)]
                    + moment[Self::get_index(cube.r(0), cube.g(0), position as usize)]
            }
        }
    }
}

pub enum Direction {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct MaximizeResult {
    // < 0 if cut impossible
    pub cut_location: i32,
    pub maximum: f64,
}

pub struct CreateBoxesResult {
    pub requested_count: usize,
    pub result_count: usize,
}

#[derive(Debug, Clone)]
pub struct Cube {
    pub pixels: [Rgb; 2],
    pub vol: i32,
}

impl Cube {
    pub fn r<T: From<u8>>(&self, pixel: usize) -> T {
        self.pixels[pixel].red.into()
    }

    pub fn g<T: From<u8>>(&self, pixel: usize) -> T {
        self.pixels[pixel].green.into()
    }

    pub fn b<T: From<u8>>(&self, pixel: usize) -> T {
        self.pixels[pixel].blue.into()
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Box: R {} -> {} G {} -> {} B {} -> {} VOL = {}",
            self.pixels[0].red,
            self.pixels[1].red,
            self.pixels[0].green,
            self.pixels[1].green,
            self.pixels[0].blue,
            self.pixels[1].blue,
            self.vol
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Quantizer, QuantizerWu};
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
        let result = QuantizerWu::quantize(&[Argb::from_u32(0xff14_1216)], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &Argb::from_u32(0xff14_1216));
    }

    #[test]
    fn test_1r() {
        let result = QuantizerWu::quantize(&[RED], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &RED);
    }

    #[test]
    fn test_1g() {
        let result = QuantizerWu::quantize(&[GREEN], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &GREEN);
    }

    #[test]
    fn test_1b() {
        let result = QuantizerWu::quantize(&[BLUE], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &BLUE);
    }

    #[test]
    fn test_5b() {
        let result = QuantizerWu::quantize(&[BLUE, BLUE, BLUE, BLUE, BLUE], MAX_COLORS);
        let colors = result.color_to_count.keys().collect::<Vec<_>>();

        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], &BLUE);
    }

    #[test]
    fn test_2r_3g() {
        let result = QuantizerWu::quantize(&[RED, RED, GREEN, GREEN, GREEN], MAX_COLORS);

        assert_eq!(result.color_to_count.keys().len(), 2);

        assert!(result.color_to_count.contains_key(&GREEN));
        assert!(result.color_to_count.contains_key(&GREEN));
    }

    #[test]
    fn test_1r_1g_1b() {
        let result = QuantizerWu::quantize(&[RED, GREEN, BLUE], MAX_COLORS);

        assert_eq!(result.color_to_count.keys().len(), 3);

        assert!(result.color_to_count.contains_key(&GREEN));
        assert!(result.color_to_count.contains_key(&RED));
        assert!(result.color_to_count.contains_key(&BLUE));
    }
}
