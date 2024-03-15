#![allow(clippy::too_many_arguments)]

use core::fmt;
use indexmap::IndexMap;

use crate::{Argb, Rgb};

use super::{Quantizer, QuantizerMap, QuantizerResult};

// A histogram of all the input colors is constructed. It has the shape of a
//  The cube would be too large if it contained all 16 million colors:
// historical best practice is to use 5 bits  of the 8 in each channel,
// reducing the histogram to a volume of ~32,000.
const INDEX_BITS: u8 = 5;
const BITS_TO_REMOVE: u8 = 8 - INDEX_BITS;
const SIDE_LENGTH: usize = (1 << INDEX_BITS) + 1;
const TOTAL_SIZE: usize = SIDE_LENGTH.pow(3);

pub struct QuantizerWu {
    weights: [u32; TOTAL_SIZE],
    moments_r: [u32; TOTAL_SIZE],
    moments_g: [u32; TOTAL_SIZE],
    moments_b: [u32; TOTAL_SIZE],
    moments: [f64; TOTAL_SIZE],
    cubes: Vec<Cube>,
}

impl Default for QuantizerWu {
    fn default() -> Self {
        Self {
            weights: [0; TOTAL_SIZE],
            moments_r: [0; TOTAL_SIZE],
            moments_g: [0; TOTAL_SIZE],
            moments_b: [0; TOTAL_SIZE],
            moments: [0.0; TOTAL_SIZE],
            cubes: vec![],
        }
    }
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
        let color_to_count = self.create_result(create_boxes_result.result_count as usize);

        QuantizerResult {
            color_to_count,
            input_pixel_to_cluster_pixel: Default::default(),
        }
    }
}

impl QuantizerWu {
    pub fn get_index<T: Into<usize>>(r: T, g: T, b: T) -> usize {
        let r: usize = r.into();
        let g: usize = g.into();
        let b: usize = b.into();

        (r << (INDEX_BITS * 2)) + (r << (INDEX_BITS + 1)) + r + (g << INDEX_BITS) + g + b
    }

    pub fn construct_histogram(&mut self, pixels: IndexMap<Argb, u32>) {
        self.weights = [0; TOTAL_SIZE];
        self.moments_r = [0; TOTAL_SIZE];
        self.moments_g = [0; TOTAL_SIZE];
        self.moments_b = [0; TOTAL_SIZE];
        self.moments = [0.0; TOTAL_SIZE];

        for (argb, count) in pixels {
            let red = argb.red;
            let green = argb.green;
            let blue = argb.blue;

            let i_r = (red >> BITS_TO_REMOVE) + 1;
            let i_g = (green >> BITS_TO_REMOVE) + 1;
            let i_b = (blue >> BITS_TO_REMOVE) + 1;

            let index = Self::get_index(i_r, i_g, i_b);

            self.weights[index] += count;

            self.moments_r[index] += u32::from(red) * count;
            self.moments_g[index] += u32::from(green) * count;
            self.moments_b[index] += u32::from(blue) * count;

            self.moments[index] += f64::from(count)
                * (f64::from(red).powi(2) + f64::from(green).powi(2) + f64::from(blue)).powi(2);
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
        self.cubes = vec![
            Cube {
                pixels: [Rgb::default(), Rgb::default()],
                vol: 0
            };
            max_color_count
        ];

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
            requested_count: max_color_count as i32,
            result_count: generated_color_count as i32,
        }
    }

    pub fn create_result(&self, color_count: usize) -> IndexMap<Argb, u32> {
        let mut result = IndexMap::new();

        for i in 0..color_count {
            let cube = &self.cubes[i];
            let weight = Self::volume(cube, &self.weights);

            if weight > 0 {
                let r = (Self::volume(cube, &self.moments_r) / weight) as u8;
                let g = (Self::volume(cube, &self.moments_g) / weight) as u8;
                let b = (Self::volume(cube, &self.moments_b) / weight) as u8;

                let color = Rgb::new(r, g, b).into();

                result.insert(color, 0);
            }
        }

        result
    }

    pub fn variance(&self, cube: &Cube) -> f64 {
        let dr = Self::volume(cube, &self.moments_r);
        let dg = Self::volume(cube, &self.moments_g);
        let db = Self::volume(cube, &self.moments_b);

        let [Rgb {
            red: r0,
            green: g0,
            blue: b0,
        }, Rgb {
            red: r1,
            green: g1,
            blue: b1,
        }] = cube.pixels;

        let xx = self.moments[Self::get_index(r1, g1, b1)]
            - self.moments[Self::get_index(r1, g1, b0)]
            - self.moments[Self::get_index(r1, g0, b1)]
            + self.moments[Self::get_index(r1, g0, b0)]
            - self.moments[Self::get_index(r0, g1, b1)]
            + self.moments[Self::get_index(r0, g1, b0)]
            + self.moments[Self::get_index(r0, g0, b1)]
            - self.moments[Self::get_index(r0, g0, b0)];

        let hypotenuse = dr
            .wrapping_pow(2)
            .wrapping_add(dg.wrapping_pow(2))
            .wrapping_add(db.wrapping_pow(2));
        let volume_ = Self::volume(cube, &self.weights);

        xx - f64::from(hypotenuse / volume_)
    }

    pub fn cut(&mut self, next: usize, i: usize) -> bool {
        let (mut one, mut two) = (self.cubes[next].clone(), self.cubes[i].clone());

        let whole_r = Self::volume(&one, &self.moments_r);
        let whole_g = Self::volume(&one, &self.moments_g);
        let whole_b = Self::volume(&one, &self.moments_b);
        let whole_w = Self::volume(&one, &self.weights);

        let max_rresult = self.maximize(
            &one,
            Direction::Red,
            one.r::<u8>(0) + 1,
            one.r::<u8>(1),
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_gresult = self.maximize(
            &one,
            Direction::Green,
            one.g::<u8>(0) + 1,
            one.g::<u8>(1),
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_bresult = self.maximize(
            &one,
            Direction::Blue,
            one.b::<u8>(0) + 1,
            one.b::<u8>(1),
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

            if max_rresult.cut_location.is_none() {
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
                one.pixels[1].red = max_rresult.cut_location.unwrap_or_default();
                two.pixels[0].red = one.pixels[1].red;
                two.pixels[0].green = one.pixels[0].green;
                two.pixels[0].blue = one.pixels[0].blue;
            }
            Direction::Green => {
                one.pixels[1].blue = max_gresult.cut_location.unwrap_or_default();
                two.pixels[0].red = one.pixels[0].red;
                two.pixels[0].green = one.pixels[1].green;
                two.pixels[0].blue = one.pixels[0].blue;
            }
            Direction::Blue => {
                one.pixels[1].green = max_bresult.cut_location.unwrap_or_default();
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
        direction: Direction,
        first: u8,
        last: u8,
        whole_r: i32,
        whole_g: i32,
        whole_b: i32,
        whole_w: i32,
    ) -> MaximizeResult {
        let bottom_r = Self::bottom(cube, &direction, &self.moments_r);
        let bottom_g = Self::bottom(cube, &direction, &self.moments_g);
        let bottom_b = Self::bottom(cube, &direction, &self.moments_b);
        let bottom_w = Self::bottom(cube, &direction, &self.weights);

        let mut max = 0.0;
        let mut cut = None;

        for i in first..last {
            let mut half_r = bottom_r.wrapping_add(Self::top(cube, &direction, i, &self.moments_r));
            let mut half_g = bottom_g.wrapping_add(Self::top(cube, &direction, i, &self.moments_g));
            let mut half_b = bottom_b.wrapping_add(Self::top(cube, &direction, i, &self.moments_b));
            let mut half_w = bottom_w.wrapping_add(Self::top(cube, &direction, i, &self.weights));

            if half_w == 0 {
                continue;
            }

            let mut temp_numerator = f64::from(
                half_r
                    .wrapping_pow(2)
                    .wrapping_add(half_g.wrapping_pow(2))
                    .wrapping_add(half_b.wrapping_pow(2)),
            );
            let mut temp_denominator = f64::from(half_w);
            let mut temp = temp_numerator / temp_denominator;

            half_r = whole_r.wrapping_sub(half_r);
            half_g = whole_g.wrapping_sub(half_g);
            half_b = whole_b.wrapping_sub(half_b);
            half_w = whole_w.wrapping_sub(half_w);

            if half_w == 0 {
                continue;
            }

            temp_numerator = f64::from(
                half_r
                    .wrapping_pow(2)
                    .wrapping_add(half_g.wrapping_pow(2))
                    .wrapping_add(half_b.wrapping_pow(2)),
            );
            temp_denominator = f64::from(half_w);
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

    pub fn volume(cube: &Cube, moment: &[u32]) -> i32 {
        let [Rgb {
            red: r0,
            green: g0,
            blue: b0,
        }, Rgb {
            red: r1,
            green: g1,
            blue: b1,
        }] = cube.pixels;

        (moment[Self::get_index(r1, g1, b1)] as i32)
            .wrapping_sub(moment[Self::get_index(r1, g1, b0)] as i32)
            .wrapping_sub(moment[Self::get_index(r1, g0, b1)] as i32)
            .wrapping_add(moment[Self::get_index(r1, g0, b0)] as i32)
            .wrapping_sub(moment[Self::get_index(r0, g1, b1)] as i32)
            .wrapping_add(moment[Self::get_index(r0, g1, b0)] as i32)
            .wrapping_add(moment[Self::get_index(r0, g0, b1)] as i32)
            .wrapping_sub(moment[Self::get_index(r0, g0, b0)] as i32)
    }

    pub fn bottom(cube: &Cube, direction: &Direction, moment: &[u32]) -> i32 {
        let [Rgb {
            red: r0,
            green: g0,
            blue: b0,
        }, Rgb {
            red: r1,
            green: g1,
            blue: b1,
        }] = cube.pixels;

        match direction {
            Direction::Red => (moment[Self::get_index(r0, g1, b1)] as i32)
                .wrapping_neg()
                .wrapping_add(moment[Self::get_index(r0, g1, b0)] as i32)
                .wrapping_add(moment[Self::get_index(r0, g0, b1)] as i32)
                .wrapping_sub(moment[Self::get_index(r0, g0, b0)] as i32),
            Direction::Green => (moment[Self::get_index(r1, g0, b1)] as i32)
                .wrapping_neg()
                .wrapping_add(moment[Self::get_index(r1, g0, b0)] as i32)
                .wrapping_add(moment[Self::get_index(r0, g0, b1)] as i32)
                .wrapping_sub(moment[Self::get_index(r0, g0, b0)] as i32),
            Direction::Blue => (moment[Self::get_index(r1, g1, b0)] as i32)
                .wrapping_neg()
                .wrapping_add(moment[Self::get_index(r1, g0, b0)] as i32)
                .wrapping_add(moment[Self::get_index(r0, g1, b0)] as i32)
                .wrapping_sub(moment[Self::get_index(r0, g0, b0)] as i32),
        }
    }

    pub fn top(cube: &Cube, direction: &Direction, position: u8, moment: &[u32]) -> i32 {
        let [Rgb {
            red: r0,
            green: g0,
            blue: b0,
        }, Rgb {
            red: r1,
            green: g1,
            blue: b1,
        }] = cube.pixels;

        match direction {
            Direction::Red => (moment[Self::get_index(position, g1, b1)] as i32)
                .wrapping_sub(moment[Self::get_index(position, g1, b0)] as i32)
                .wrapping_sub(moment[Self::get_index(position, g0, b1)] as i32)
                .wrapping_add(moment[Self::get_index(position, g0, b0)] as i32),
            Direction::Green => (moment[Self::get_index(r1, position, b1)] as i32)
                .wrapping_sub(moment[Self::get_index(r1, position, b0)] as i32)
                .wrapping_sub(moment[Self::get_index(r0, position, b1)] as i32)
                .wrapping_add(moment[Self::get_index(r0, position, b0)] as i32),
            Direction::Blue => (moment[Self::get_index(r1, g1, position)] as i32)
                .wrapping_sub(moment[Self::get_index(r1, g0, position)] as i32)
                .wrapping_sub(moment[Self::get_index(r0, g1, position)] as i32)
                .wrapping_add(moment[Self::get_index(r0, g0, position)] as i32),
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
    pub cut_location: Option<u8>,
    pub maximum: f64,
}

pub struct CreateBoxesResult {
    pub requested_count: i32,
    pub result_count: i32,
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
            "Box R {} -> {} G {} -> {} B {} -> {} VOL = {}",
            self.pixels[0].red,
            self.pixels[0].green,
            self.pixels[0].blue,
            self.pixels[1].red,
            self.pixels[1].green,
            self.pixels[1].blue,
            self.vol
        )
    }
}
