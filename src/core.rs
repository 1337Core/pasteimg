use anyhow::{Context, anyhow};
use clipboard_rs::{Clipboard, ClipboardContext, common::RustImage};
use image::ImageFormat;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
#[cfg(target_os = "macos")]
use std::process::Command;

use crate::Result;

const JPEG_QUALITY: u8 = 85;
const HASH_PREFIX_LEN: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Png,
    Jpeg,
}

impl OutputFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
        }
    }
}

pub fn default_downloads_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/Users"))
        .join("Downloads")
}

pub fn capture_clipboard_image(lossless: bool) -> Result<String> {
    let format = if lossless {
        OutputFormat::Png
    } else {
        OutputFormat::Jpeg
    };

    let png_bytes = read_clipboard_png_bytes()?;
    let (filename, output_bytes) = build_output(&png_bytes, format)?;
    let path = persist_output(&default_downloads_dir(), &filename, &output_bytes)?;

    Ok(path.to_string_lossy().into_owned())
}

pub fn capture_image_from_png_path(input_path: &Path, lossless: bool) -> Result<String> {
    let format = if lossless {
        OutputFormat::Png
    } else {
        OutputFormat::Jpeg
    };

    let png_bytes = std::fs::read(input_path)
        .with_context(|| format!("Failed to read input file {}", input_path.display()))?;

    let (filename, output_bytes) = build_output(&png_bytes, format)?;
    let path = persist_output(&default_downloads_dir(), &filename, &output_bytes)?;

    Ok(path.to_string_lossy().into_owned())
}

pub fn build_output(png_bytes: &[u8], format: OutputFormat) -> Result<(String, Vec<u8>)> {
    let short_hash = short_sha256_hex_prefix(png_bytes, HASH_PREFIX_LEN);
    let filename = format!("{}.{}", short_hash, format.extension());

    let output_bytes = match format {
        OutputFormat::Png => png_bytes.to_vec(),
        OutputFormat::Jpeg => encode_jpeg_from_png(png_bytes)?,
    };

    Ok((filename, output_bytes))
}

pub fn persist_output(dir: &Path, filename: &str, bytes: &[u8]) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("Failed to create output directory {}", dir.display()))?;

    let file_path = dir.join(filename);
    std::fs::write(&file_path, bytes)
        .with_context(|| format!("Failed to write {}", file_path.display()))?;

    Ok(file_path)
}

#[cfg(target_os = "macos")]
pub fn open_in_finder(file_path: &str) -> Result<()> {
    Command::new("open")
        .arg("-R")
        .arg(file_path)
        .spawn()
        .map_err(|e| anyhow!("Failed to open Finder: {}", e))?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn open_in_finder(_file_path: &str) -> Result<()> {
    Ok(())
}

fn read_clipboard_png_bytes() -> Result<Vec<u8>> {
    let ctx = ClipboardContext::new()
        .map_err(|e| anyhow!("Failed to create clipboard context: {}", e))?;
    let img = ctx
        .get_image()
        .map_err(|_| anyhow!("No image found in clipboard"))?;

    let img_data = img
        .to_png()
        .map_err(|e| anyhow!("Failed to convert image to PNG: {}", e))?;

    Ok(img_data.get_bytes().to_vec())
}

fn short_sha256_hex_prefix(data: &[u8], prefix_len: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = format!("{:x}", hasher.finalize());
    hash.chars().take(prefix_len).collect()
}

fn encode_jpeg_from_png(png_bytes: &[u8]) -> Result<Vec<u8>> {
    use std::io::Cursor;

    let img = image::load_from_memory_with_format(png_bytes, ImageFormat::Png)
        .map_err(|e| anyhow!("Failed to load image: {}", e))?;

    let mut output = Cursor::new(Vec::new());
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, JPEG_QUALITY);
    encoder
        .encode_image(&img)
        .map_err(|e| anyhow!("Failed to encode JPEG: {}", e))?;

    Ok(output.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageBuffer, Rgba};

    fn sample_png() -> Vec<u8> {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| {
            Rgba([(x * 100) as u8, (y * 120) as u8, 90, 255])
        });
        let mut bytes = Vec::new();
        DynamicImage::ImageRgba8(image)
            .write_to(&mut std::io::Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();
        bytes
    }

    #[test]
    fn hash_prefix_is_stable() {
        assert_eq!(short_sha256_hex_prefix(b"hello", 5), "2cf24");
    }

    #[test]
    fn build_output_for_png_keeps_bytes_and_extension() {
        let png = sample_png();
        let (filename, bytes) = build_output(&png, OutputFormat::Png).unwrap();

        assert!(filename.ends_with(".png"));
        assert_eq!(bytes, png);
    }

    #[test]
    fn build_output_for_jpeg_creates_jpeg() {
        let png = sample_png();
        let (filename, bytes) = build_output(&png, OutputFormat::Jpeg).unwrap();

        assert!(filename.ends_with(".jpg"));
        let format = image::guess_format(&bytes).unwrap();
        assert_eq!(format, ImageFormat::Jpeg);
    }

    #[test]
    fn persist_output_writes_file() {
        let temp = tempfile::tempdir().unwrap();
        let path = persist_output(temp.path(), "abc.png", b"123").unwrap();
        assert_eq!(std::fs::read(path).unwrap(), b"123");
    }
}
