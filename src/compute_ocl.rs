use std::time::Instant;

use anyhow::Result;
use num::Complex;
use ocl::ProQue;

use crate::{MandelbrotComputation, MandelbrotComputationResult};

pub struct MandelbrotOcl {}

impl MandelbrotComputation for MandelbrotOcl {
    fn compute(
        width: u32,
        height: u32,
        max_iters: usize,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<MandelbrotComputationResult> {
        // Converti i parametri in tipi OpenCL
        let width = width as i32;
        let height = height as i32;
        let max_iters = max_iters as i32;

        // Calcola i valori di xmin, xmax, ymin e ymax
        let xmin = upper_left.re;
        let xmax = lower_right.re;

        // Inverti ymin e ymax per mantenere la coerenza con il piano cartesiano
        let ymin = upper_left.im;
        let ymax = lower_right.im;

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

        let start_time = Instant::now();

        // Prepara il kernel con gli argomenti
        let kernel = pro_que
            .kernel_builder("mandelbrot")
            .arg(&buffer)
            .arg(&width)
            .arg(&height)
            .arg(&max_iters)
            .arg(xmin)
            .arg(xmax)
            .arg(ymin)
            .arg(ymax)
            .build()?;

        // Esegui il kernel
        unsafe {
            kernel.enq()?;
        }

        // Leggi i risultati dal dispositivo
        let mut result_vec = vec![0u8; (width * height) as usize];
        buffer.read(&mut result_vec).enq()?;

        let elapsed_time = start_time.elapsed();

        Ok(MandelbrotComputationResult {
            values: result_vec,
            elapsed_time,
        })
    }
}
