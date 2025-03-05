# Ditherum

Ditherum is a Rust library and command-line interface (CLI) tool designed for image dithering and color palette manipulation. It currently supports extracting color palettes from images using multithreaded variant of the [K-means clustering](https://en.wikipedia.org/wiki/K-means_clustering) algorithm, resizing and dithering images using the [Floyd-Steinberg algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering).

<p align="center">
  <img width="500" alt="Comparison dithering, thresholding and original image" src="res/doc/preview.png">
</p>

## Features

- **Extract Color Palette**: Extracts a color palette from an image.
- **Save/Load Color Palette**: Save extracted color palettes to a JSON file or load them from a JSON file.
- **Color Reduction**: Attempts to reduce the number of colors in a palette to a specified target using the K-means centroids algorithm.
- **Dithering**: Modify image so that it resembles original with highly reduced color palette.

## Installation

To use Ditherum as a library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
ditherum = { git = "https://github.com/your-username/ditherum" }
```

To install the CLI tool:

```sh
cargo install --path .
```

It will install this tool as `ditherum` executable.

## Usage

There are 2 modes in ditherum CLI: 
- palette used only to extract color palette from image and/or reduce palette
- dither used to dither image using existing palett and/or palette reduction with optional image resize

### CLI palette examples:

<p align="center">
  <img width="500" alt="8 colors palette extracted from image" src="res/doc/pink_palette_8.png">
</p>

Extract colors palette from image, reduce it to 8 colors and save to JSON:

```sh
ditherum -v palette --input image.png --output palette.json --colors 8
```

### CLI dither examples:

Dither image with default 8 colors palette, no resize:
```sh
ditherum dither --input image.png --output dithered_image.png
```

Additionaly save resulting palette:
```sh
ditherum dither --input image.png --output dithered_image.png --reduced reduced.json
```

Reuse existing colors palette:
```sh
ditherum dither --input image.png --palette existing_palette.json
```

Resize:
```sh
ditherum dither --input image.png --width 240
```

### Library

```rust
use ditherum::palette::PaletteRGB;
use ditherum::image::{
    load_image, 
    save_image, 
    ImageProcessor, 
    ProcessingAlgorithm
};

fn main() {
    let img = load_image("image.jpg").unwrap();
    
    // Create palette from image and reduce to 16 clustering colors
    let palette = PaletteRGB::from_rgbu8_image(&img)
        .try_reduce(16)
        .unwrap();

    // Save palette to file
    palette.save_to_json("palette.json").unwrap();
    
    // dither image using Floyd-Steinber algorithm
    let dithered_img = ImageProcessor::new(img, palette)
        .with_algorithm(ProcessingAlgorithm::FloydSteinbergRgb)
        .run();

    // Save image to file
    save_image("dithered.png", &dithered_img).unwrap();
}
```

## Tests & Logging
To run test with logging option.

> Log levels: error > warn > info > debug > trace.

Windows:
```cmd
set RUST_LOG=debug && cargo test --features logging -- --nocapture
```

Linux:
```sh
RUST_LOG=debug && cargo test --features logging -- --nocapture
```

### Benchmarking

Benchmarking covers:
- matrix modification using 2x3 kernel

### Depelopment test cheatsheet

Windows commands

Help:
```sh
set RUST_LOG=debug && cargo run --bin ditherum -- -h
```

Verbouse palette command with color reduction to 10:
```sh
cargo run --bin ditherum -- -v palette -i res/test_images/test_pink_300.jpg -c 10 -o res/test_results/test_pink_300.json
```

Verbouse palette command with color reduction to 10 and additional debug logging:
```sh
set RUST_LOG=debug && cargo run --bin ditherum --features logging -- -v palette -i res/test_images/test_pink_300.jpg -c 10 -o res/test_results/test_pink_300.json
```

## Roadmap

- [x] Extract color palette from image
- [x] Save and load color palette in JSON format
- [x] Reduce color palette using K-means clustering
- [x] Add CLI palette creation/reduction
- [x] Add CLI palette visualization tool
- [x] Add CLI support for dithering images using the Floyd-Steinberg algorithm
- [x] Add CLI image resize
- [ ] Parameters: proc count, timeout
- [ ] Enhance logging and error handling

## License

This project is licensed under the MIT License.