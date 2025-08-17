use anyhow::anyhow;
use clipboard_rs::{Clipboard, ClipboardContext, common::RustImage};
use image::{DynamicImage, ImageFormat};
use sha2::{Digest, Sha256};
use std::process::Command;

use crate::Result;

// default to ~/downloads to keep things simple without extra things for user to do
fn downloads_dir() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::new();
    if let Ok(home) = std::env::var("HOME") {
        p.push(home);
        p.push("Downloads");
    } else {
        p.push("/Users");
        p.push("Downloads");
    }
    p
}

pub fn capture_clipboard_image(lossless: bool) -> Result<String> {
    let clipboard_img = get_clipboard_image()?;

    // hash on png bytes so filenames remain stable for identical images
    let img_data = clipboard_img
        .to_png()
        .map_err(|e| anyhow!("Failed to convert image to PNG: {}", e))?;
    let png_bytes = img_data.get_bytes();

    let short_hash = hash_bytes_short_sha256_hex5(png_bytes);

    let downloads_dir = downloads_dir();

    if lossless {
        let filename = format!("{}.png", short_hash);
        let file_path = downloads_dir.join(filename);

        std::fs::write(&file_path, png_bytes)?;
        Ok(file_path.to_string_lossy().to_string())
    } else {
        let img = image::load_from_memory_with_format(png_bytes, ImageFormat::Png)
            .map_err(|e| anyhow!("Failed to load image: {}", e))?;
        let filename = format!("{}.jpg", short_hash);
        let file_path = downloads_dir.join(filename);

        save_as_jpeg(&img, &file_path)?;
        Ok(file_path.to_string_lossy().to_string())
    }
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

fn get_clipboard_image() -> Result<impl RustImage> {
    let ctx = ClipboardContext::new()
        .map_err(|e| anyhow!("Failed to create clipboard context: {}", e))?;
    let img = ctx
        .get_image()
        .map_err(|_| anyhow!("No image found in clipboard"))?;
    Ok(img)
}

fn hash_bytes_short_sha256_hex5(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_str = format!("{:x}", hash);
    hash_str[..5].to_string()
}

// jpeg quality (85) looks diece and makes pretty small files
fn save_as_jpeg(img: &DynamicImage, file_path: &std::path::Path) -> Result<()> {
    use std::fs::File;
    use std::io::BufWriter;

    let file = File::create(file_path).map_err(|e| anyhow!("Failed to create file: {}", e))?;
    let writer = BufWriter::new(file);

    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(writer, 85);
    encoder
        .encode_image(img)
        .map_err(|e| anyhow!("Failed to encode JPEG: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_bytes_short_sha256_hex5() {
        let h = hash_bytes_short_sha256_hex5(b"hello");
        assert_eq!(h, "2cf24");
    }

    #[test]
    fn test_filename_generation() {
        let hash = hash_bytes_short_sha256_hex5(b"test");
        assert_eq!(hash.len(), 5);
    }
}
