use std::ops;

use num::complex::Complex;
use rayon::prelude::*;


struct Z {
    re: f64,
    im: f64,
}


impl Z {
    fn new(re: f64, im: f64) -> Self {
        Z { re, im}
    }
    
    fn norm_square(self) -> f64 {
        self.re*self.re + self.im*self.im
    }
}

impl ops::Add for Z {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl ops::Mul for Z {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}


/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(c: Complex<f64>) -> u8 {
    match escape_time(c, 255) {
        None => 0,
        Some(count) => 255 - count as u8,
    }
}

#[derive(Clone, Copy)]
struct FieldMap {
    pub upper_left: Complex<f64>,
    //pub lower_right: Complex<f64>,
    pub re_resolution: usize,
    pub im_resolution: usize,
    pub re_delta: f64,
    pub im_delta: f64,
}

impl FieldMap {
    fn new(
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
        re_resolution: usize,
        im_resolution: usize,
    ) -> Self {
        FieldMap {
            upper_left,
            //lower_right,
            re_resolution,
            im_resolution,
            re_delta: (lower_right.re - upper_left.re) / re_resolution as f64,
            im_delta: (upper_left.im - lower_right.im) / im_resolution as f64,
        }
    }

    fn get_point(self, index: usize) -> Complex<f64> {
        let (x, y) = (index % self.re_resolution, index / self.re_resolution);

        Complex {
            re: self.upper_left.re + x as f64 * self.re_delta,
            im: self.upper_left.im - y as f64 * self.im_delta,
        }
    }

    fn get_limit(self) -> usize {
        self.re_resolution * self.im_resolution
    }
}

fn main() {
    let filename = "mandelbrot.png";

    let field_map = FieldMap::new(
        Complex {
            re: -1.20f64,
            im: 0.35f64,
        },
        Complex {
            re: -1f64,
            im: 0.20f64,
        },
        1000,
        750,
    );

    let pixels: Vec<u8> = (0..field_map.get_limit())
        .into_par_iter()
        .map(|p| render(field_map.get_point(p)))
        .collect();

    image::save_buffer(
        filename,
        &pixels,
        field_map.re_resolution as u32,
        field_map.im_resolution as u32,
        image::ColorType::L8,
    )
    .unwrap();
}
