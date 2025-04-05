use num::Complex;

use anyhow::Result;

use rayon::prelude::*;
use std::time::Instant;

use crate::mandelbrot_utils::{FieldMap, ComputationStrategy, ComputationResult};

pub struct MandelbrotRayon {}


impl MandelbrotRayon {
    pub fn new() -> Self {
        MandelbrotRayon {}
    }
}


impl ComputationStrategy for MandelbrotRayon {
    fn compute(
        &self,
        width: u32,
        height: u32,
        max_iters: usize,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<ComputationResult> {
        compute(width, height, max_iters, upper_left, lower_right)
    }

    fn dump_info(&self) -> Result<()> {
        println!("MandelbrotRayon computation info: Parallelized using Rayon.");
        Ok(())
    }
}

#[inline(always)]
fn escape_time(c: Complex<f32>, max_iters: usize) -> u8 {
    let mut z = Complex::new(0.0, 0.0);
    let mut i = 0;
    while i < max_iters && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        i += 1;
    }
    ((max_iters - i) & 0xff) as u8
}

#[inline(always)]
fn compute(
    width: u32,
    height: u32,
    max_iters: usize,
    upper_left: Complex<f32>,
    lower_right: Complex<f32>,
) -> Result<ComputationResult> {
    let field_map = FieldMap::new(upper_left, lower_right, width as usize, height as usize);

    let start_time = Instant::now();

    let values: Vec<u8> = (0..field_map.get_limit())
        .into_par_iter()
        .map(|i| escape_time(field_map.get_point(i), max_iters))
        .collect();

    let elapsed_time = start_time.elapsed();

    Ok(ComputationResult {
        values,
        elapsed_time,
    })
}
