mod field_map;

use field_map::FieldMap;
use num::Complex;

use rayon::prelude::*;
use std::time::Instant;

fn escape_time(c: Complex<f64>) -> u8 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..=255 {
        if z.norm_sqr() > 4.0 {
            return 255 - i;
        }
        z = z * z + c;
    }

    0
}

fn main() {
    let field_map = FieldMap::new(
        Complex::new(-1.20, 0.35),
        Complex::new(-1.00, 0.20),
        1000,
        750,
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
    .unwrap();
}
