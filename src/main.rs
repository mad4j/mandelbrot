mod compute_mono;
mod compute_ocl;
mod compute_rayon;
mod field_map;
mod strategy;
mod utils;

use std::time::Duration;

use clap::{Parser, Subcommand};
use compute_mono::MandelbrotMono;
use strategy::{ComputationContext, ComputationParams, ComputationStrategy};

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
    Ocl {
        /// OpenCL platform index
        #[arg(short, long, default_value_t = 0)]
        platform: usize,
    },
    /// computing with Rayon
    Rayon {},
}

fn main() -> Result<()> {
    // parse command line arguments
    let cli = Cli::parse();

    // set up the computation parameters
    let params = ComputationParams {
        // image dimensions
        width: 4000,
        height: 3000,
        // max iterations
        max_iters: 255,
        // area of the complex plane to visualize
        upper_left: Complex::new(-1.20, 0.35),
        lower_right: Complex::new(-1.00, 0.20),
    };

    // create the computation context based on the command line argument
    let mut context = match &cli.command {
        Commands::Mono {} => ComputationContext::new(Box::new(MandelbrotMono::new())),
        Commands::Ocl { platform } => ComputationContext::new(Box::new(MandelbrotOcl::new(*platform))),
        Commands::Rayon {} => ComputationContext::new(Box::new(MandelbrotRayon::new())),
    };

    // dump computation strategy info
    context.dump_info().expect("Failed to dump strategy info");

    // initialize setup timer
    let mut setup_time = Duration::ZERO;

    timeit!(&mut setup_time, {
        // initialize the computation context
        context
            .init(&params)
            .expect("Failed to initialize the computation context");

        // setup the computation context
        context
            .setup()
            .expect("Failed to setup the computation context");
    });

    // initialize computation timer
    let mut core_time = Duration::ZERO;

    let values = timeit!(&mut core_time, {
        // perform the computation
        context
            .compute()
            .expect("Failed to compute the Mandelbrot set")
    });

    // print the elapsed time for the computation
    println!("Setup time: {:.02?}", setup_time);
    println!("Core  time: {:.02?}", core_time);

    // create and save the image
    let image: GrayImage = ImageBuffer::from_raw(params.width as u32, params.height as u32, values)
        .expect("Failed to create image buffer");

    image.save("mandelbrot.png")?;

    // that's all folks
    Ok(())
}
