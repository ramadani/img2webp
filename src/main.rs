mod converter;
mod utils;

use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, Subcommand};

use converter::{convert_bulk, convert_dir, convert_single, format_size, print_results, ConvertOptions};
use utils::build_output_path;

#[derive(Parser)]
#[command(name = "img2webp")]
#[command(version, about = "Convert images to WebP format")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a single image file to WebP
    Single {
        /// Input image file path
        input: PathBuf,

        /// Output file path (default: <input>.webp)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Quality encoding (1-100, default: 80)
        #[arg(short, long, default_value_t = 80.0)]
        quality: f32,

        /// Use lossless encoding
        #[arg(short, long)]
        lossless: bool,

        /// Resize width (pixels)
        #[arg(long)]
        width: Option<u32>,

        /// Resize height (pixels)
        #[arg(long)]
        height: Option<u32>,
    },

    /// Convert multiple image files to WebP
    Bulk {
        /// Input image file paths
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output directory (default: same directory as input)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Quality encoding (1-100, default: 80)
        #[arg(short, long, default_value_t = 80.0)]
        quality: f32,

        /// Use lossless encoding
        #[arg(short, long)]
        lossless: bool,

        /// Resize width (pixels)
        #[arg(long)]
        width: Option<u32>,

        /// Resize height (pixels)
        #[arg(long)]
        height: Option<u32>,

        /// Number of parallel threads (default: half of CPU cores)
        #[arg(short, long)]
        threads: Option<usize>,
    },

    /// Convert all images in a directory to an output directory
    Dir {
        /// Source directory
        input_dir: PathBuf,

        /// Destination directory
        output_dir: PathBuf,

        /// Quality encoding (1-100, default: 80)
        #[arg(short, long, default_value_t = 80.0)]
        quality: f32,

        /// Use lossless encoding
        #[arg(short, long)]
        lossless: bool,

        /// Scan subdirectories recursively
        #[arg(short, long)]
        recursive: bool,

        /// Resize width (pixels)
        #[arg(long)]
        width: Option<u32>,

        /// Resize height (pixels)
        #[arg(long)]
        height: Option<u32>,

        /// Number of parallel threads (default: half of CPU cores)
        #[arg(short, long)]
        threads: Option<usize>,
    },
}

/// Initialize rayon's global thread pool.
///
/// Returns the number of threads that will be used.
/// Defaults to half of available CPU cores if not specified.
fn init_thread_pool(threads: Option<usize>) -> usize {
    let available = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let num_threads = threads.unwrap_or(available / 2).max(1);

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .ok();

    num_threads
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Single {
            input,
            output,
            quality,
            lossless,
            width,
            height,
        } => {
            let output = output.unwrap_or_else(|| build_output_path(&input, None));
            let opts = ConvertOptions {
                quality,
                lossless,
                width,
                height,
            };

            let size = std::fs::metadata(&input).map(|m| m.len()).unwrap_or(0);
            println!("Converting {} ({}) ...", input.display(), format_size(size));
            let start = Instant::now();
            let result = convert_single(&input, &output, &opts);
            let elapsed = start.elapsed();

            print_results(&[result], elapsed);
        }

        Commands::Bulk {
            files,
            output,
            quality,
            lossless,
            width,
            height,
            threads,
        } => {
            let opts = ConvertOptions {
                quality,
                lossless,
                width,
                height,
            };

            let total_size: u64 = files.iter()
                .filter_map(|f| std::fs::metadata(f).ok())
                .map(|m| m.len())
                .sum();
            let num_threads = init_thread_pool(threads);
            println!("Converting {} files ({}) using {} threads ...", files.len(), format_size(total_size), num_threads);
            let start = Instant::now();
            let results = convert_bulk(&files, output.as_deref(), &opts);
            let elapsed = start.elapsed();

            print_results(&results, elapsed);
        }

        Commands::Dir {
            input_dir,
            output_dir,
            quality,
            lossless,
            recursive,
            width,
            height,
            threads,
        } => {
            let opts = ConvertOptions {
                quality,
                lossless,
                width,
                height,
            };

            let num_threads = init_thread_pool(threads);
            println!(
                "Converting from {} to {} using {} threads ...",
                input_dir.display(),
                output_dir.display(),
                num_threads,
            );
            let start = Instant::now();
            let results = convert_dir(&input_dir, &output_dir, &opts, recursive);
            let elapsed = start.elapsed();

            print_results(&results, elapsed);
        }
    }
}
