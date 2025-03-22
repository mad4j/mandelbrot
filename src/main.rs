mod compute_mono;
mod compute_ocl;
mod compute_rayon;
mod field_map;

use clap::{Parser, Subcommand};
use compute_mono::MandelbrotMono;

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

    fn dump_info() -> Result<()>;
}

pub struct MandelbrotComputationResult {
    values: Vec<u8>,
    elapsed_time: std::time::Duration,
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
        Commands::Ocl {} => MandelbrotOcl::dump_info()?,
        Commands::Rayon {} => MandelbrotRayon::dump_info()?,
        Commands::Mono {} => MandelbrotMono::dump_info()?,
    }

    
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
