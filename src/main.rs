mod compute_mono;
mod compute_ocl;
mod compute_rayon;
mod field_map;

use clap::{Parser, Subcommand};
use compute_mono::MandelbrotMono;
use ocl::{core::DeviceInfo, Device, Platform};

use std::time::Instant;

use anyhow::Result;
use compute_ocl::MandelbrotOcl;
use compute_rayon::MandelbrotRayon;
use image::{GrayImage, ImageBuffer};
use num::Complex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// computing with a single core
    Mono {},
    /// computing with OpenCL
    Ocl {},
    /// computing with Rayon
    Rayon {},
}

pub trait MandelbrotComputation {
    fn compute(
        width: u32,
        height: u32,
        max_iters: usize,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<MandelbrotComputationResult>;
}

pub struct MandelbrotComputationResult {
    values: Vec<u8>,
    elapsed_time: std::time::Duration,
}

pub fn dump_opencl_info() -> Result<()> {
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    dump_opencl_info()?;

    // Dimensioni dell'immagine
    let width = 4000;
    let height = 3000;
    let max_iters = 255;

    // Area del piano complesso da visualizzare
    let upper_left = Complex::<f32>::new(-1.20, 0.35);
    let lower_right = Complex::<f32>::new(-1.00, 0.20);

    let start_time = Instant::now();

    let result = match &cli.command {
        Commands::Ocl {} => {
            MandelbrotOcl::compute(width, height, max_iters, upper_left, lower_right)?
        }
        Commands::Rayon {} => {
            MandelbrotRayon::compute(width, height, max_iters, upper_left, lower_right)?
        }
        Commands::Mono {} => {
            MandelbrotMono::compute(width, height, max_iters, upper_left, lower_right)?
        }
    };

    let elapsed_time = start_time.elapsed();
    println!("Total Elapsed time: {:.02?}", elapsed_time);
    println!("Core  Elapsed time: {:.02?}", result.elapsed_time);

    // Crea e salva l'immagine
    let image: GrayImage =
        ImageBuffer::from_raw(width as u32, height as u32, result.values).unwrap();

    image.save("mandelbrot.png")?;

    Ok(())
}
