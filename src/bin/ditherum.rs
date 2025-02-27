use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};
// use ditherum::palette


#[derive(Debug, Parser)]
#[command(about = "Image dithering and palette extraction tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Mode
}

#[derive(Debug, Subcommand)]
enum Mode {
    /// Dither mode for image processing
    Dither(DitherModeArgs),

    /// Palette mode for color extraction
    Palette(PaletteModeArgs),
}

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

#[derive(Debug, Args)]
struct PaletteModeArgs {
    /// Input image file path (required)
    #[arg(short = 'i', long = "input", value_name = "INPUT_PATH")]
    input_path: PathBuf,

    /// Output palette JSON file (optional)
    #[arg(short = 'o', long = "output", value_name = "OUTPUT_PATH")]
    output_path: Option<PathBuf>,

    /// Number of colors in output palette (optional)
    #[arg(short = 'c', long = "colors", value_name = "COLORS_COUNT")]
    colors_count: Option<usize>,

    /// Debug mode for timing execution (optional)
    #[arg(short = 'd', long = "debug", value_name = "DEBUG_ENABLED", default_value_t = false)]
    debug: bool    
}

//test with:
//set RUST_LOG=debug && cargo run --bin ditherum -- palette -i res/test_images/karambola.PNG -c 10
//set RUST_LOG=debug && cargo run --bin ditherum --features logging -- palette -i res/test_images/karambola.PNG -c 10
fn main() {
    if cfg!(feature = "logging") {
        env_logger::init();
    }

    let cli_args = Cli::parse();
    log::debug!("Got args: '{:?}'.", cli_args);

    match cli_args.mode {
        Mode::Dither(dither_args) => run_dither(dither_args),
        Mode::Palette(palette_args) => run_palette(palette_args),
    }
}

fn run_dither(_args: DitherModeArgs) {
    unimplemented!("run_dither")
}


fn run_palette(_args: PaletteModeArgs) {
    unimplemented!("run_palette")
}






// use clap::{Parser, Subcommand, Args};
// use std::path::PathBuf;

// #[derive(Parser)]
// #[command(name = "ditherum")]
// #[command(about = "Image dither and palette extraction tool", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     mode: Mode,
// }

// #[derive(Subcommand)]
// enum Mode {
//     /// Dither mode for image processing
//     Dither(DitherArgs),

//     /// Palette mode for color extraction
//     Palette(PaletteArgs),
// }

// #[derive(Args)]
// struct DitherArgs {
//     /// Input image file (required)
//     #[arg(short, long, value_name = "INPUT_IMAGE", required = true)]
//     input: PathBuf,

//     /// Output file (optional)
//     #[arg(short, long, value_name = "OUTPUT_FILE")]
//     output: Option<PathBuf>,

//     /// Number of colors to reduce to (optional, conflicts with --palette)
//     #[arg(short = 'c', long, value_name = "COLOR_COUNT", conflicts_with = "palette")]
//     color: Option<usize>,

//     /// Path to palette file (optional, conflicts with --color)
//     #[arg(short = 'p', long, value_name = "PALETTE_FILE", conflicts_with = "color")]
//     palette: Option<PathBuf>,

//     /// Path to save the reduced palette (optional, works only with --color)
//     #[arg(short = 'r', long, value_name = "REDUCED_PALETTE", requires = "color")]
//     reduced: Option<PathBuf>,
// }

// #[derive(Args)]
// struct PaletteArgs {
//     /// Input image file (required)
//     #[arg(short, long, value_name = "INPUT_IMAGE", required = true)]
//     input: PathBuf,

//     /// Output palette JSON file (optional)
//     #[arg(short, long, value_name = "OUTPUT_FILE")]
//     output: Option<PathBuf>,

//     /// Number of colors in output palette (optional)
//     #[arg(short = 'c', long, value_name = "COLOR_COUNT")]
//     color: Option<usize>,

//     /// Debug mode for timing execution (optional)
//     #[arg(short = 'd', long, default_value_t = false)]
//     debug: bool,
// }

// fn main() {
//     let cli = Cli::parse();

//     match cli.mode {
//         Mode::Dither(args) => run_dither(args),
//         Mode::Palette(args) => run_palette(args),
//     }
// }

// fn run_dither(args: DitherArgs) {
//     println!("Running in Dither mode");

//     println!("Input Image: {:?}", args.input);

//     if let Some(output) = args.output {
//         println!("Output File: {:?}", output);
//     } else {
//         println!("No output specified, generating automatically...");
//     }

//     if let Some(color_count) = args.color {
//         println!("Reducing colors to: {}", color_count);
//         if let Some(reduced) = args.reduced {
//             println!("Saving reduced palette to: {:?}", reduced);
//         }
//     }

//     if let Some(palette) = args.palette {
//         println!("Using palette from: {:?}", palette);
//     }

//     // TODO: Add your dither logic here
// }

// fn run_palette(args: PaletteArgs) {
//     println!("Running in Palette mode");

//     println!("Input Image: {:?}", args.input);

//     if let Some(output) = args.output {
//         println!("Output Palette File: {:?}", output);
//     } else {
//         println!("No output specified, generating automatically...");
//     }

//     if let Some(color_count) = args.color {
//         println!("Extracting {} colors", color_count);
//     } else {
//         println!("Extracting full palette from the image.");
//     }

//     if args.debug {
//         println!("Debug mode is ON.");
//     }

//     // TODO: Add your palette extraction logic here
// }
