use std::fs;
use std::path::Path;

use image::imageops::FilterType;
use image::ImageReader;
use rayon::prelude::*;
use zenwebp::{EncodeRequest, LosslessConfig, LossyConfig, PixelLayout};

use crate::utils::{build_output_path, collect_images, ensure_dir_exists};

/// Options for image conversion.
pub struct ConvertOptions {
    pub quality: f32,
    pub lossless: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Result of a single file conversion.
pub struct ConvertResult {
    pub input: String,
    pub output: String,
    pub original_size: u64,
    pub webp_size: u64,
    pub success: bool,
    pub error: Option<String>,
}

/// Convert a single image file to WebP.
pub fn convert_single(
    input: &Path,
    output: &Path,
    opts: &ConvertOptions,
) -> ConvertResult {
    let input_str = input.display().to_string();
    let output_str = output.display().to_string();

    let original_size = fs::metadata(input)
        .map(|m| m.len())
        .unwrap_or(0);

    match convert_file(input, output, opts) {
        Ok(webp_size) => ConvertResult {
            input: input_str,
            output: output_str,
            original_size,
            webp_size,
            success: true,
            error: None,
        },
        Err(e) => ConvertResult {
            input: input_str,
            output: output_str,
            original_size,
            webp_size: 0,
            success: false,
            error: Some(e.to_string()),
        },
    }
}

/// Convert multiple image files in parallel.
pub fn convert_bulk(
    files: &[std::path::PathBuf],
    output_dir: Option<&Path>,
    opts: &ConvertOptions,
) -> Vec<ConvertResult> {
    if let Some(dir) = output_dir {
        if let Err(e) = ensure_dir_exists(dir) {
            eprintln!("Error: failed to create output directory: {e}");
            return Vec::new();
        }
    }

    files
        .par_iter()
        .map(|input| {
            let output = build_output_path(input, output_dir);
            convert_single(input, &output, opts)
        })
        .collect()
}

/// Convert all images in a directory to an output directory.
pub fn convert_dir(
    input_dir: &Path,
    output_dir: &Path,
    opts: &ConvertOptions,
    recursive: bool,
) -> Vec<ConvertResult> {
    if let Err(e) = ensure_dir_exists(output_dir) {
        eprintln!("Error: failed to create output directory: {e}");
        return Vec::new();
    }

    let files = match collect_images(input_dir, recursive) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: failed to read directory: {e}");
            return Vec::new();
        }
    };

    if files.is_empty() {
        eprintln!("No image files found in {}", input_dir.display());
        return Vec::new();
    }

    files
        .par_iter()
        .map(|input| {
            // Preserve subdirectory structure in output.
            let relative = input
                .strip_prefix(input_dir)
                .unwrap_or(input.as_path());
            let mut out_path = output_dir.join(relative);
            out_path.set_extension("webp");

            // Ensure parent dir exists for nested files.
            if let Some(parent) = out_path.parent() {
                let _ = ensure_dir_exists(parent);
            }

            convert_single(input, &out_path, opts)
        })
        .collect()
}

/// Internal: perform the actual file conversion.
fn convert_file(input: &Path, output: &Path, opts: &ConvertOptions) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    // Decode the source image.
    let mut img = ImageReader::open(input)?.decode()?;

    // Optional resize.
    match (opts.width, opts.height) {
        (Some(w), Some(h)) => {
            img = img.resize_exact(w, h, FilterType::Lanczos3).into();
        }
        (Some(w), None) => {
            img = img.resize(w, u32::MAX, FilterType::Lanczos3).into();
        }
        (None, Some(h)) => {
            img = img.resize(u32::MAX, h, FilterType::Lanczos3).into();
        }
        (None, None) => {}
    }

    // Convert to RGBA8 pixel buffer.
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let pixels = rgba.as_raw();

    // Encode to WebP using zenwebp.
    let webp_bytes = if opts.lossless {
        let config = LosslessConfig::new();
        EncodeRequest::lossless(&config, pixels, PixelLayout::Rgba8, width, height)
            .encode()?
    } else {
        let config = LossyConfig::new().with_quality(opts.quality);
        EncodeRequest::lossy(&config, pixels, PixelLayout::Rgba8, width, height)
            .encode()?
    };

    // Write output file.
    fs::write(output, &webp_bytes)?;

    Ok(webp_bytes.len() as u64)
}

/// Print a summary of conversion results.
pub fn print_results(results: &[ConvertResult], elapsed: std::time::Duration) {
    let total = results.len();
    let success = results.iter().filter(|r| r.success).count();
    let failed = total - success;

    println!();

    for result in results {
        if result.success {
            let ratio = if result.original_size > 0 {
                (result.webp_size as f64 / result.original_size as f64) * 100.0
            } else {
                0.0
            };
            println!(
                "  ✓ {} → {} ({} → {}, {:.1}%)",
                result.input,
                result.output,
                format_size(result.original_size),
                format_size(result.webp_size),
                ratio,
            );
        } else {
            println!(
                "  ✗ {} — {}",
                result.input,
                result.error.as_deref().unwrap_or("unknown error"),
            );
        }
    }

    println!();
    println!(
        "Done: {success} succeeded, {failed} failed out of {total} files ({:.2}s)",
        elapsed.as_secs_f64()
    );
}

/// Format byte sizes for human-readable output.
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}
