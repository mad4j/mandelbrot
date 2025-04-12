use human_bytes::human_bytes;

use anyhow::Result;
use ocl::{
    core::{DeviceInfo, DeviceInfoResult},
    Device, Platform, ProQue,
};

use crate::strategy::{ComputationParams, ComputationStrategy};

pub struct MandelbrotOcl {
    platform: usize,
    params: Option<ComputationParams>,
    buffer: Option<ocl::Buffer<u8>>,
    kernel: Option<ocl::Kernel>,
}

impl MandelbrotOcl {
    pub fn new(platform: usize) -> Self {
        MandelbrotOcl {
            platform,
            params: None,
            buffer: None,
            kernel: None,
        }
    }
}

impl ComputationStrategy for MandelbrotOcl {
    fn setup(&mut self) -> Result<()> {
        Ok(())
    }

    fn init(&mut self, params: &ComputationParams) -> Result<()> {
        self.params = Some(params.clone());

        let params = self
            .params
            .as_ref()
            .expect("Computation parameters not initialized.");

        // Convert the parameters to OpenCL types
        let width = params.width as i32;
        let height = params.height as i32;
        let max_iters = params.max_iters as i32;

        // Calcola i valori di xmin, xmax, ymin e ymax
        let xmin = params.upper_left.re;
        let xmax = params.lower_right.re;

        // Inverti ymin e ymax per mantenere la coerenza con il piano cartesiano
        let ymin = params.upper_left.im;
        let ymax = params.lower_right.im;

        // Codice kernel OpenCL
        let kernel_src = r#"
            __kernel void mandelbrot(
                __global uchar* output,
                int width,
                int height,
                int max_iters,
                double xmin,
                double xmax,
                double ymin,
                double ymax
            ) {
                int x = get_global_id(0);
                int y = get_global_id(1);
                
                if (x >= width || y >= height) return;

                double cx = xmin + (xmax - xmin) * x / width;
                double cy = ymin + (ymax - ymin) * y / height;

                double zx = 0.0f;
                double zy = 0.0f;
                int i = 0;
                
                while (zx * zx + zy * zy <= 4.0f && i < max_iters) {
                    double xtemp = zx * zx - zy * zy + cx;
                    zy = 2.0f * zx * zy + cy;
                    zx = xtemp;
                    i++;
                }

                output[y * width + x] = max_iters-i;
            }
        "#;

        // initialize the OpenCL environment
        let pro_que = ProQue::builder()
            //.platform(Platform::default())
            .platform(Platform::list()[self.platform])
            .src(kernel_src)
            .dims((width as usize, height as usize))
            .build()?;

        // create output buffer
        let buffer = pro_que.create_buffer::<u8>()?;

        // Prepara il kernel con gli argomenti
        let kernel = pro_que
            .kernel_builder("mandelbrot")
            .arg(&buffer)
            .arg(width)
            .arg(height)
            .arg(max_iters)
            .arg(xmin)
            .arg(xmax)
            .arg(ymin)
            .arg(ymax)
            .build()?;

        // set strcut parameters
        self.buffer = Some(buffer);
        self.kernel = Some(kernel);

        Ok(())
    }

    fn compute(&self) -> Result<Vec<u8>> {
        let params = self
            .params
            .as_ref()
            .expect("Computation parameters not initialized.");

        let buffer = self.buffer.as_ref().expect("Buffer not initialized.");

        let kernel = self.kernel.as_ref().expect("Kernel not initialized.");

        // execute the kernel
        unsafe {
            kernel.enq()?;
        }

        // retrive results from the device
        let mut result_vec = vec![0u8; (params.width * params.height) as usize];
        buffer.read(&mut result_vec).enq()?;

        Ok(result_vec)
    }

    fn dump_info(&self) -> Result<()> {
        println!("MandelbrotOCL computation info: Parallelized using OpenCL.");
        println!("------------------------------------------------------------");

        let platforms = Platform::list();

        for (p_idx, platform) in platforms.iter().enumerate() {
            println!("Platform #{}: {}", p_idx, platform.name()?);
            println!("  Profile: {} {}", platform.version()?, platform.profile()?);

            let devices = Device::list_all(platform)?;

            for (d_idx, device) in devices.iter().enumerate() {
                println!(
                    "  Device {}: [{}] {}",
                    d_idx,
                    device.info(DeviceInfo::Type)?,
                    device.name()?
                );
                println!(
                    "    Max Compute Units: {} @ {}",
                    device.info(DeviceInfo::MaxComputeUnits)?,
                    device.info(DeviceInfo::MaxClockFrequency)?
                );
                println!(
                    "    Max Work Group Size: {} x {}",
                    device.info(DeviceInfo::MaxWorkGroupSize)?,
                    device.info(DeviceInfo::MaxWorkItemSizes)?
                );
                println!(
                    "    Memory Size: {} (global) {} (local)",
                    human_bytes(
                        extract_u64(device.info(DeviceInfo::GlobalMemSize)?).unwrap_or(0) as f64
                    ),
                    human_bytes(
                        extract_u64(device.info(DeviceInfo::LocalMemSize)?).unwrap_or(0) as f64
                    )
                );
            }
        }

        println!("Selected Platform #{}", self.platform);
        println!("------------------------------------------------------------");

        Ok(())
    }
}

fn extract_u64(i: DeviceInfoResult) -> Option<u64> {
    match i {
        DeviceInfoResult::MaxComputeUnits(v) => Some(v as u64),
        DeviceInfoResult::MaxWorkGroupSize(v) => Some(v as u64),
        DeviceInfoResult::MaxWorkItemDimensions(v) => Some(v as u64),
        DeviceInfoResult::MaxWorkItemSizes(_) => None,
        DeviceInfoResult::MaxClockFrequency(v) => Some(v as u64),
        DeviceInfoResult::MaxMemAllocSize(v) => Some(v),
        DeviceInfoResult::GlobalMemSize(v) => Some(v),
        DeviceInfoResult::LocalMemSize(v) => Some(v),
        _ => None,
    }
}
