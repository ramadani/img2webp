# img2webp

A fast, parallel CLI tool for converting images to WebP format, built with Rust.

Uses [zenwebp](https://crates.io/crates/zenwebp) (pure-Rust, SIMD-accelerated) for high-performance encoding and [rayon](https://crates.io/crates/rayon) for parallel processing — ideal for batch converting large directories.

## Features

- **Single file** conversion
- **Bulk conversion** of multiple files in parallel
- **Directory conversion** with optional recursive scanning
- **Lossy & lossless** WebP encoding
- **Adjustable quality** (1–100, default: 80)
- **Optional resize** by width and/or height
- **Thread control** to manage CPU usage (`-t, --threads`)
- **Progress bar** with ETA for bulk and directory conversions
- **Preserves directory structure** when converting folders

## Supported Input Formats

PNG, JPEG, BMP, GIF, TIFF

## Installation

### From source

Requires [Rust](https://rustup.rs/) 1.85+.

```bash
git clone https://github.com/ramadani/img2webp.git
cd img2webp
cargo install --path .
```

### Build only

```bash
cargo build --release
```

The binary will be at `target/release/img2webp`.

## Usage

### Convert a single file

```bash
# Basic (outputs to same directory as input)
img2webp single photo.png

# Specify output path and quality
img2webp single photo.png -o output.webp -q 90

# Lossless encoding
img2webp single photo.png --lossless

# Resize to specific width (height scales proportionally)
img2webp single photo.png --width 800

# Resize to exact dimensions
img2webp single photo.png --width 800 --height 600
```

### Convert multiple files

```bash
# Convert several files
img2webp bulk image1.png image2.jpg image3.bmp

# Output to a specific directory
img2webp bulk *.png -o ./webp_output -q 85

# Limit to 2 threads
img2webp bulk *.png -o ./webp_output -t 2
```

### Convert an entire directory

```bash
# Convert all images in a folder
img2webp dir ./images ./webp_output

# With quality and recursive subdirectory scan
img2webp dir ./images ./webp_output -q 90 -r

# Lossless conversion of an entire folder
img2webp dir ./images ./webp_output --lossless -r

# Limit CPU usage to 4 threads
img2webp dir ./images ./webp_output -r -t 4
```

## Options Reference

### Global options

| Flag | Description |
|------|-------------|
| `-h, --help` | Print help |
| `-V, --version` | Print version |

### Shared options (available on all subcommands)

| Flag | Default | Description |
|------|---------|-------------|
| `-q, --quality <QUALITY>` | `80` | Encoding quality (1–100) |
| `-l, --lossless` | `false` | Use lossless encoding |
| `--width <WIDTH>` | — | Resize width in pixels |
| `--height <HEIGHT>` | — | Resize height in pixels |

### `bulk` and `dir` shared options

| Argument | Default | Description |
|----------|---------|-------------|
| `-t, --threads <THREADS>` | half of CPU cores | Number of parallel threads |

### `single`

| Argument | Description |
|----------|-------------|
| `<INPUT>` | Input image file path |
| `-o, --output <OUTPUT>` | Output file path (default: `<input>.webp`) |

### `bulk`

| Argument | Description |
|----------|-------------|
| `<FILES>` | Input image file paths |
| `-o, --output <OUTPUT>` | Output directory (default: same as input) |

### `dir`

| Argument | Description |
|----------|-------------|
| `<INPUT_DIR>` | Source directory |
| `<OUTPUT_DIR>` | Destination directory |
| `-r, --recursive` | Scan subdirectories recursively |

## Performance Tips

- **Always use release builds** for real workloads. Debug builds are 10–20x slower:
  ```bash
  cargo run --release -- dir ./images ./output -r
  ```
- **Thread control**: By default, `img2webp` uses half of your CPU cores. Use `-t` to adjust:
  ```bash
  # Use only 2 threads (low CPU usage)
  img2webp dir ./images ./output -r -t 2

  # Use all 8 cores (maximum speed)
  img2webp dir ./images ./output -r -t 8
  ```

## Development

```bash
# Run in development
cargo run -- single photo.png

# Run tests
cargo test

# Run linter
cargo clippy
```

## License

[MIT](LICENSE)
