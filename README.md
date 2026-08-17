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
```

### Convert an entire directory

```bash
# Convert all images in a folder
img2webp dir ./images ./webp_output

# With quality and recursive subdirectory scan
img2webp dir ./images ./webp_output -q 90 -r

# Lossless conversion of an entire folder
img2webp dir ./images ./webp_output --lossless -r
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
