mod complex;
mod field_map;

use std::time::Instant;

use complex::Z;
use field_map::FieldMap;

use rayon::prelude::*;

fn escape_time(c: Z) -> u8 {
    let mut z = Z::ZERO;
    for i in 0..=255 {
        if z.norm_sqr() > 4.0 {
            return 255 - i;
        }
        z = z * z + c;
    }

    0
}

fn main() {
    let field_map = FieldMap::new(Z::new(-1.20, 0.35), Z::new(-1.00, 0.20), 1000, 750);

    let start_time = Instant::now();

    let pixels: Vec<u8> = (0..field_map.get_limit())
        .into_par_iter()
        .map(|p| {
            let c = field_map.get_point(p);
            escape_time(c)
        })
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
