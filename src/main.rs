mod compute_mono;
mod compute_ocl;
mod compute_rayon;
mod mandelbrot_utils;

use clap::{Parser, Subcommand};
use compute_mono::MandelbrotMono;
use mandelbrot_utils::ComputationContext;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // image dimensions
    let width = 4000;
    let height = 3000;

    // max iterations
    let max_iters = 255;

    // area of the complex plane to visualize
    let upper_left = Complex::<f32>::new(-1.20, 0.35);
    let lower_right = Complex::<f32>::new(-1.00, 0.20);

    // create the computation context based on the command line argument
    let context = match &cli.command {
        Commands::Mono {} => ComputationContext::new(Box::new(MandelbrotMono::new())),
        Commands::Ocl {} => ComputationContext::new(Box::new(MandelbrotOcl::new())),
        Commands::Rayon {} => ComputationContext::new(Box::new(MandelbrotRayon::new())),
    };

    // initialize the computation context
    context
        .dump_info()
        .expect("Failed to dump strategy info");

    // take the start time
    let start_time = Instant::now();

    // perform the computation
    let result = context
        .compute(width, height, max_iters, upper_left, lower_right)
        .expect("Failed to compute the Mandelbrot set");

    // print the elapsed time for the computation
    let elapsed_time = start_time.elapsed();
    println!("Total Elapsed time: {:.02?}", elapsed_time);
    println!("Core  Elapsed time: {:.02?}", result.elapsed_time);
    
    // create and save the image
    let image: GrayImage = ImageBuffer::from_raw(width as u32, height as u32, result.values)
        .expect("Failed to create image buffer");

    image.save("mandelbrot.png")?;

    // that's all folks
    Ok(())
}
