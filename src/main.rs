mod compute_mono;
mod compute_ocl;
mod compute_rayon;
mod field_map;
mod strategy;

use clap::{Parser, Subcommand};
use compute_mono::MandelbrotMono;
use strategy::{ComputationContext, ComputationParams, ComputationStrategy};

use anyhow::Result;
use compute_ocl::MandelbrotOcl;
use compute_rayon::MandelbrotRayon;
use image::{GrayImage, ImageBuffer};
use num::Complex;
use howlast::howlast;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    /// the width of the image
    #[arg(short='w', long, default_value_t = 4000)]
    image_width: u32,

    /// the height of the image
    #[arg(short='h', long, default_value_t = 3000)]
    image_height: u32,

    /// the number of iterations to compute
    #[arg(short='i', long, default_value_t = 256)]
    max_iters: usize,

    /// the upper left corner of the area to visualize
    #[arg(short='u', long, default_value = "-1.20+0.35i")]
    upper_left: Complex<f64>,

    /// the lower right corner of the area to visualize
    #[arg(short='l', long, default_value = "-1.00+0.20i")]
    lower_right: Complex<f64>,

    /// the name of the output file
    #[arg(short, long, default_value = "mandelbrot.png")]
    file_name: String,

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
        width: cli.image_width,
        height: cli.image_height,
        // max iterations
        max_iters: cli.max_iters-1,
        // area of the complex plane to visualize
        upper_left: cli.upper_left,
        lower_right: cli.lower_right,
    };

    // create the computation context based on the command line argument
    let mut context = match &cli.command {
        Commands::Mono {} => ComputationContext::new(Box::new(MandelbrotMono::new())),
        Commands::Ocl { platform } => ComputationContext::new(Box::new(MandelbrotOcl::new(*platform))),
        Commands::Rayon {} => ComputationContext::new(Box::new(MandelbrotRayon::new())),
    };

    // dump computation strategy info
    context.dump_info().expect("Failed to dump strategy info");

    howlast!(setup_time => {
        // initialize the computation context
        context
            .init(&params)
            .expect("Failed to initialize the computation context");

        // setup the computation context
        context
            .setup()
            .expect("Failed to setup the computation context");
    });

    howlast!(core_time, values => {
        // perform the core computation
        context
            .compute()
            .expect("Failed to compute the Mandelbrot set")
    });

    howlast!(post_time => {
        // normalize values if max_iters is less than 255
        let normalized_values: Vec<u8> = if cli.max_iters < 255 {
            values
                .iter()
                .map(|&v| ((v as f64 / cli.max_iters as f64) * 255.0).round() as u8)
                .collect()
        } else {
            values
        };

        // create and save the image
        let image: GrayImage = ImageBuffer::from_raw(params.width, params.height, normalized_values)
            .expect("Failed to create image buffer");

        image.save(cli.file_name)?;
    });

    // print the elapsed time for the computation
    println!("Setup time: {:.02?}", setup_time);
    println!("Core  time: {:.02?}", core_time);
    println!("Post  time: {:.02?}", post_time);
    println!("Total time: {:.02?}", setup_time + core_time + post_time);

    // that's all folks
    Ok(())
}
