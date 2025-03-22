use std::time::Instant;

use anyhow::Result;
use num::Complex;
use ocl::{core::DeviceInfo, Device, Platform, ProQue};

use crate::mandelbrot_utils::{MandelbrotComputation, MandelbrotComputationResult};


pub struct MandelbrotOcl {}

impl MandelbrotComputation for MandelbrotOcl {
    fn compute(
        width: u32,
        height: u32,
        max_iters: usize,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<MandelbrotComputationResult> {
        // Convert the parameters to OpenCL types
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

                float cx = xmin + (xmax - xmin) * x / width;
                float cy = ymin + (ymax - ymin) * y / height;

                float zx = 0.0f;
                float zy = 0.0f;
                int i = 0;
                
                while (zx * zx + zy * zy <= 4.0f && i < max_iters) {
                    float xtemp = zx * zx - zy * zy + cx;
                    zy = 2.0f * zx * zy + cy;
                    zx = xtemp;
                    i++;
                }

                output[y * width + x] = max_iters-i;
            }
        "#;

        // Initialize the OpenCL environment
        let pro_que = ProQue::builder()
            .platform(Platform::default())
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

    fn dump_info() -> Result<()> {

        println!("MandelbrotRayon computation info: Parallelized using OpenCL.");

        let platforms = Platform::list();

        for (p_idx, platform) in platforms.iter().enumerate() {
            println!("Platform #{}: {}", p_idx, platform.name()?);
            println!("  Vendor: {}", platform.vendor()?);
            println!("  Version: {}", platform.version()?);
            println!("  Profile: {}", platform.profile()?);

            let devices = Device::list_all(platform)?;

            for (d_idx, device) in devices.iter().enumerate() {
                println!("  Device {}: {}", d_idx, device.name()?);
                println!("    Vendor: {}", device.vendor()?);
                println!("    Version: {}", device.version()?);
                println!("    Type: {}", device.info(DeviceInfo::Type)?);
                println!("    Profile: {}", device.info(DeviceInfo::Profile)?);
                println!(
                    "    Max Compute Units: {}",
                    device.info(DeviceInfo::MaxComputeUnits)?
                );
                println!(
                    "    Max Work Group Size: {}",
                    device.info(DeviceInfo::MaxWorkGroupSize)?
                );
                println!(
                    "    Max Work Item Dimensions: {}",
                    device.info(DeviceInfo::MaxWorkItemDimensions)?
                );
                println!(
                    "    Max Work Item Sizes: {}",
                    device.info(DeviceInfo::MaxWorkItemSizes)?
                );
                println!(
                    "    Max Clock Frequency: {}",
                    device.info(DeviceInfo::MaxClockFrequency)?
                );
                println!(
                    "    Max Memory Allocation Size: {}",
                    device.info(DeviceInfo::MaxMemAllocSize)?
                );
                println!(
                    "    Global Memory Size: {}",
                    device.info(DeviceInfo::GlobalMemSize)?
                );
                println!(
                    "    Local Memory Size: {}",
                    device.info(DeviceInfo::LocalMemSize)?
                );
            }
        }

        Ok(())
    }
}
