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


use std::time::Instant;

use anyhow::Result;
use ocl::ProQue;
use image::{GrayImage, ImageBuffer};

fn main() -> Result<()> {
    // Dimensioni dell'immagine
    let width = 4000i32;
    let height = 3000i32;
    let max_iters = 255i32;

    // Area del piano complesso da visualizzare
    let xmin = -1.20f32;
    let xmax = -1.00f32;
    let ymin = 0.35f32;
    let ymax = 0.20f32;

    // Codice kernel OpenCL
    let kernel_src = r#"
        __kernel void mandelbrot(
            __global uchar* output,
            int width,
            int height,
            int max_iters,
            float xmin,
            float xmax,
            float ymin,
            float ymax
        ) {
            int x = get_global_id(0);
            int y = get_global_id(1);
            
            if (x >= width || y >= height) return;

            float cx = xmin + (xmax - xmin) * x / (width - 1);
            float cy = ymin + (ymax - ymin) * y / (height - 1);

            float zx = 0.0f;
            float zy = 0.0f;
            int i = 0;
            
            while (zx * zx + zy * zy <= 4.0f && i < max_iters) {
                float xtemp = zx * zx - zy * zy + cx;
                zy = 2.0f * zx * zy + cy;
                zx = xtemp;
                i++;
            }

            output[y * width + x] = i == max_iters ? 0 : 255-i;
        }
    "#;

    // Inizializza l'ambiente OpenCL
    let pro_que = ProQue::builder()
        .src(kernel_src)
        .dims((width as usize, height as usize))
        .build()?;

    // Crea buffer per l'output
    let buffer = pro_que.create_buffer::<u8>()?;

    // Prepara il kernel con gli argomenti
    let kernel = pro_que.kernel_builder("mandelbrot")
        .arg(&buffer)
        .arg(&width)
        .arg(&height)
        .arg(&max_iters)
        .arg(xmin)
        .arg(xmax)
        .arg(ymin)
        .arg(ymax)
        .build()?;


    let start_time = Instant::now();

    // Esegui il kernel
    unsafe { kernel.enq()?; }

    // Leggi i risultati dal dispositivo
    let mut result_vec = vec![0u8; (width * height) as usize];
    buffer.read(&mut result_vec).enq()?;

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Crea e salva l'immagine
    let img: GrayImage = ImageBuffer::from_raw(
        width as u32,
        height as u32,
        result_vec
    ).unwrap();

    img.save("mandelbrot.png")?;

    Ok(())
}