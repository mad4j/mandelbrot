/*
mod field_map;

use field_map::FieldMap;
use num::Complex;

use rayon::prelude::*;
use std::time::Instant;

#[inline(always)]
fn escape_time(c: Complex<f64>) -> u8 {
    let mut z = Complex::new(0.0, 0.0);
    let mut i = 0;
    while i < 255 && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        i += 1;
    }
    255 - i
}

fn main() -> Result<(), image::ImageError> {
    let field_map = FieldMap::new(
        Complex::new(-1.20, 0.35),
        Complex::new(-1.00, 0.20),
        4000,
        3000,
    );

    let start_time = Instant::now();

    let pixels: Vec<u8> = (0..field_map.get_limit())
        .into_par_iter()
        .map(|i| escape_time(field_map.get_point(i)))
        .collect();

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    image::save_buffer(
        "mandelbrot.png",
        &pixels,
        field_map.re_resolution as u32,
        field_map.im_resolution as u32,
        image::ColorType::L8,
    )
}
*/

mod compute_ocl;

use std::time::Instant;

use anyhow::Result;
use compute_ocl::MandelbrotOcl;
use image::{GrayImage, ImageBuffer};

pub trait MandelbrotComputation {
    fn compute(width: u32, height: u32, max_iters: usize, xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> Result<MandelbrotComputationResult>;
}

pub struct MandelbrotComputationResult {
    values: Vec<u8>,
    elapsed_time: std::time::Duration,
}

fn main() -> Result<()> {
    // Dimensioni dell'immagine
    let width = 4000;
    let height = 3000;
    let max_iters = 255;

    // Area del piano complesso da visualizzare
    let xmin = -1.20f32;
    let xmax = -1.00f32;
    let ymin = 0.35f32;
    let ymax = 0.20f32;

    let start_time = Instant::now();

    let result = MandelbrotOcl::compute(width, height, max_iters, xmin, xmax, ymin, ymax)?;

    let elapsed_time = start_time.elapsed();
    println!("Total Elapsed time: {:.02?}", elapsed_time);
    println!("Core  Elapsed time: {:.02?}", result.elapsed_time);

     // Crea e salva l'immagine
     let image: GrayImage = ImageBuffer::from_raw(
        width as u32,
        height as u32,
        result.values
    ).unwrap();

    image.save("mandelbrot.png")?;

    Ok(())
}