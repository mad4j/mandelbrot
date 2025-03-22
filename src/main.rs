mod compute_mono;
mod compute_ocl;
mod compute_rayon;
mod mandelbrot_utils;

use clap::{Parser, Subcommand};
use compute_mono::MandelbrotMono;
use mandelbrot_utils::MandelbrotComputation;

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


fn processing_values<T: MandelbrotComputation>(
    width: u32,
    height: u32,
    max_iters: usize,
    upper_left: Complex<f32>,
    lower_right: Complex<f32>,
) -> Result<()> {

    T::dump_info()?;

    let start_time = Instant::now();

    let result = T::compute(width, height, max_iters, upper_left, lower_right)?;

    let elapsed_time = start_time.elapsed();
    println!("Total Elapsed time: {:.02?}", elapsed_time);
    println!("Core  Elapsed time: {:.02?}", result.elapsed_time);

    // Crea e salva l'immagine
    let image: GrayImage =
        ImageBuffer::from_raw(width as u32, height as u32, result.values).unwrap();

    image.save("mandelbrot.png")?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Dimensioni dell'immagine
    let width = 4000;
    let height = 3000;
    let max_iters = 255;

    // Area del piano complesso da visualizzare
    let upper_left = Complex::<f32>::new(-1.20, 0.35);
    let lower_right = Complex::<f32>::new(-1.00, 0.20);

    match &cli.command {
        Commands::Mono {} => processing_values::<MandelbrotMono>(width, height, max_iters, upper_left, lower_right)?,
        Commands::Ocl {} => processing_values::<MandelbrotOcl>(width, height, max_iters, upper_left, lower_right)?,
        Commands::Rayon {} => processing_values::<MandelbrotRayon>(width, height, max_iters, upper_left, lower_right)?,
    }  

    Ok(())
}
