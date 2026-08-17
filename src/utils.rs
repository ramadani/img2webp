use std::fs;
use std::path::{Path, PathBuf};

/// Supported image extensions for conversion.
const SUPPORTED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif"];

/// Check if a file has a supported image extension.
pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Ensure a directory exists, creating it (and parents) if needed.
pub fn ensure_dir_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Build output path by replacing the extension with `.webp`.
///
/// If `output_dir` is provided, the file is placed there.
/// Otherwise, the file is placed next to the original.
pub fn build_output_path(input: &Path, output_dir: Option<&Path>) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default();
    let file_name = format!("{}.webp", stem.to_string_lossy());

    match output_dir {
        Some(dir) => dir.join(file_name),
        None => {
            let parent = input.parent().unwrap_or_else(|| Path::new("."));
            parent.join(file_name)
        }
    }
}

/// Collect all supported image files from a directory.
///
/// If `recursive` is true, subdirectories are scanned as well.
pub fn collect_images(dir: &Path, recursive: bool) -> std::io::Result<Vec<PathBuf>> {
    let mut images = Vec::new();
    collect_images_inner(dir, recursive, &mut images)?;
    images.sort();
    Ok(images)
}

fn collect_images_inner(
    dir: &Path,
    recursive: bool,
    images: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && recursive {
            collect_images_inner(&path, recursive, images)?;
        } else if path.is_file() && is_supported_image(&path) {
            images.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_supported_image() {
        assert!(is_supported_image(Path::new("photo.png")));
        assert!(is_supported_image(Path::new("photo.JPG")));
        assert!(is_supported_image(Path::new("photo.jpeg")));
        assert!(is_supported_image(Path::new("photo.bmp")));
        assert!(is_supported_image(Path::new("photo.gif")));
        assert!(is_supported_image(Path::new("photo.tiff")));
        assert!(is_supported_image(Path::new("photo.tif")));
        assert!(!is_supported_image(Path::new("photo.webp")));
        assert!(!is_supported_image(Path::new("document.txt")));
        assert!(!is_supported_image(Path::new("no_extension")));
    }

    #[test]
    fn test_build_output_path_no_dir() {
        let result = build_output_path(Path::new("/photos/cat.png"), None);
        assert_eq!(result, PathBuf::from("/photos/cat.webp"));
    }

    #[test]
    fn test_build_output_path_with_dir() {
        let result =
            build_output_path(Path::new("/photos/cat.png"), Some(Path::new("/output")));
        assert_eq!(result, PathBuf::from("/output/cat.webp"));
    }
}
