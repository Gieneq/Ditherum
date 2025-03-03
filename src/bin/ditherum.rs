//! # Ditherum - Image Dithering and Palette Extraction Tool
//! 
//! Ditherum is a command-line tool for image processing. It supports two main modes:
//! - `dither`: Reduces the number of colors in an image using dithering techniques.
//! - `palette`: Extracts a color palette from an image.
//! 
//! ## Features
//! - Reduce colors using a fixed count or a custom palette.
//! - Extract color palettes with optional reduction.
//! - Verbose output for detailed execution info.
//! 
//! ## Usage Examples
//! ```sh
//! # Dithering with color reduction
//! ditherum dither -i input.png -c 16 -o output.png
//! 
//! # Dithering using a predefined palette
//! ditherum dither -i input.png -p palette.json -o output.png
//! 
//! # Extracting a palette from an image
//! ditherum palette -i input.png -c 8 -o palette.json
//! 
//! # Verbose output
//! ditherum -v palette -i input.png
//! ```

use std::{path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use anyhow::{Context, Ok};
use clap::{Parser, Subcommand, Args};
use ditherum::palette::PaletteRGB;

/// Macro for verbose output.
/// 
/// Prints the message only if `verbose` is `true`.
/// 
/// # Examples
/// ```rust
/// vprintln!(true, "This will be printed.");
/// vprintln!(false, "This won't be printed.");
/// ```
macro_rules! vprintln {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            println!($($arg)*);
        }
    };
}

/// Main CLI structure for parsing arguments using `clap`.
/// 
/// Supports two modes:
/// - `dither`: Image dithering and color reduction.
/// - `palette`: Color palette extraction.
/// 
/// # Global Arguments
#[derive(Debug, Parser)]
#[command(version, about = "Image dithering and palette extraction tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,

    /// Additional information about execution process (optional)
    #[arg(short = 'v', long = "verbose", value_name = "VERBOSE_ENABLED", default_value_t = false)]
    verbose: bool  
}

/// Subcommands for selecting the operation mode.
/// 
/// - `Dither`: Image dithering and color reduction.
/// - `Palette`: Color palette extraction.
#[derive(Debug, Subcommand)]
enum Mode {
    /// Dither mode for image processing
    Dither(DitherModeArgs),

    /// Palette mode for color extraction
    Palette(PaletteModeArgs),  
}

/// Arguments for `dither` mode.
/// 
/// # Required Arguments
/// - `-i`, `--input`: Path to the input image file.
/// 
/// # Optional Arguments
/// - `-o`, `--output`: Path for the output image. Defaults to an auto-generated name.
/// - `-c`, `--colors`: Number of colors to reduce the image to. Conflicts with `--palette`.
/// - `-p`, `--palette`: Path to the custom palette file for dithering. Conflicts with `--colors`.
/// - `-r`, `--reduced`: Path to save the reduced palette. Requires `--colors`.
#[derive(Debug, Args)]
struct DitherModeArgs {
    /// Input image file path (required)
    #[arg(short = 'i', long = "input", value_name = "INPUT_PATH", required = true)]
    input_path: PathBuf,

    /// Output file path (optional)
    #[arg(short = 'o', long = "output", value_name = "OUTPUT_PATH")]
    output_path: Option<PathBuf>,

    /// Number of colors to reduce to (optional, conflicts with --palette)
    #[arg(short = 'c', long = "colors", value_name = "INPUT_PATH", conflicts_with = "palette_path")]
    colors_count: Option<usize>,
    
    /// Path to save the reduced palette (optional, works only with --color)
    #[arg(short = 'r', long = "reduced", value_name = "REDUCED_PALETTE_PATH", requires = "colors_count")]
    reduced_palette_path: Option<PathBuf>,

    /// Path to palette file (optional, conflicts with --color)
    #[arg(short = 'p', long = "palette", value_name = "PALETTE_PATH", conflicts_with = "colors_count")]
    palette_path: Option<PathBuf>,
}

/// Arguments for `palette` mode.
/// 
/// # Required Arguments
/// - `-i`, `--input`: Path to the input image or palette file.
/// 
/// # Optional Arguments
/// - `-o`, `--output`: Path for the output palette JSON file.
/// - `-c`, `--colors`: Number of colors in the output palette.
#[derive(Debug, Args)]
struct PaletteModeArgs {
    /// Input image or palett file path (required)
    #[arg(short = 'i', long = "input", value_name = "INPUT_PATH")]
    input_path: PathBuf,

    /// Output palette JSON file (optional)
    #[arg(short = 'o', long = "output", value_name = "OUTPUT_PATH")]
    output_path: Option<PathBuf>,

    /// Number of colors in output palette (optional)
    #[arg(short = 'c', long = "colors", value_name = "COLORS_COUNT")]
    colors_count: Option<usize>,
}

fn main() {
    if cfg!(feature = "logging") {
        env_logger::init();
    }

    let cli_args = Cli::parse();
    log::debug!("Got args: '{:?}'.", cli_args);

    if let Err(e) = run(cli_args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Main execution flow handler.
/// 
/// Calls the appropriate function based on the selected mode.
fn run(cli_args: Cli) -> anyhow::Result<()> {
    let process_start = SystemTime::now().duration_since(UNIX_EPOCH)?;

    match cli_args.mode {
        Mode::Dither(dither_args) => run_dither(cli_args.verbose, dither_args),
        Mode::Palette(palette_args) => run_palette(cli_args.verbose, palette_args),
    }?;
    
    let process_end = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let process_duration = process_end-process_start;
    vprintln!(cli_args.verbose, "Process done in {} seconds.", process_duration.as_secs());

    Ok(())
}

/// Executes the `dither` mode logic.
/// 
/// Currently unimplemented. This is where the image dithering logic goes.
fn run_dither(verbose: bool, _args: DitherModeArgs) -> anyhow::Result<()> {
    vprintln!(verbose, "Dithering started...");

    unimplemented!("run_dither")
}

/// Executes the `palette` mode logic.
/// 
/// Loads the image, extracts the palette, and optionally reduces colors.
fn run_palette(verbose: bool, args: PaletteModeArgs) -> anyhow::Result<()>  {
    vprintln!(verbose, "Palette extraction started...");

    let input_extension = args.input_path.extension().context("file missing etension")?;
    let mut palette = if input_extension.eq_ignore_ascii_case("json") {
        PaletteRGB::load_from_json(&args.input_path)?
    } else {
        let image = ditherum::image::load_image(&args.input_path)?;
        vprintln!(verbose, "Image '{:?}' loaded successfully. Pixels count {}.", args.input_path, image.len());
    
        PaletteRGB::from_rgbu8_image(&image)
    };
    vprintln!(verbose, "Got palette with {} colors.", palette.len());

    if let Some(output_colors_count) = args.colors_count {
        vprintln!(verbose, "Reducing palette to {} colors started...", output_colors_count);
        palette = palette.try_reduce(output_colors_count)?;
        vprintln!(verbose, "Reduced palette to {} colors.", palette.len());
    }

    let output_path = args.output_path.unwrap_or_else(|| {
        args.input_path.with_extension("json")
    });

    palette.save_to_json(&output_path)?;
    vprintln!(verbose, "Saved to {:?}.", output_path);
    vprintln!(verbose, "\nResulting palette:\n{}\n", palette.get_ansi_colors_visualization());

    Ok(())
}
