// core functionality for clipboard image processing
use clipboard_rs::{Clipboard, ClipboardContext, common::RustImage};
use sha2::{Digest, Sha256};
use std::process::Command;
use image::{ImageFormat, DynamicImage};

use crate::Result;

// resolve ~/Downloads path efficiently (macOS-centric default)
fn downloads_dir() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::new();
    if let Ok(home) = std::env::var("HOME") {
        p.push(home);
        p.push("Downloads");
    } else {
        // fall back to conventional base; avoids alloc via format!
        p.push("/Users");
        p.push("Downloads");
    }
    p
}

// main function to grab clipboard image and save with hash name
pub fn capture_clipboard_image(lossless: bool) -> Result<String> {
    // get the image from clipboard
    let clipboard_img = get_clipboard_image()?;

    // compute PNG bytes once (used for hashing and/or saving)
    let img_data = clipboard_img
        .to_png()
        .map_err(|e| format!("Failed to convert image to PNG: {}", e))?;
    let png_bytes = img_data.get_bytes();

    // fast, deterministic short hash from PNG bytes
    let short_hash = hash_bytes_short_sha256_hex5(png_bytes);

    // downloads path (macOS default)
    let downloads_dir = downloads_dir();

    if lossless {
        let filename = format!("{}.png", short_hash);
        let file_path = downloads_dir.join(filename);
        
        std::fs::write(&file_path, png_bytes)?;
        Ok(file_path.to_string_lossy().to_string())
    } else {
        // decode once from PNG bytes, then save as JPEG (85% quality)
        let img = image::load_from_memory_with_format(png_bytes, ImageFormat::Png)
            .map_err(|e| format!("Failed to load image: {}", e))?;
        let filename = format!("{}.jpg", short_hash);
        let file_path = downloads_dir.join(filename);
        
        // save as JPEG with 85% quality
        save_as_jpeg(&img, &file_path)?;
        Ok(file_path.to_string_lossy().to_string())
    }
}

// open finder and reveal the saved file (macos only)
#[cfg(target_os = "macos")]
pub fn open_in_finder(file_path: &str) -> Result<()> {
    // use 'open -R' to reveal file in finder
    Command::new("open")
        .arg("-R")
        .arg(file_path)
        .spawn()
        .map_err(|e| format!("Failed to open Finder: {}", e))?;
    Ok(())
}

// no-op on non-macOS to avoid unnecessary process spawn attempts
#[cfg(not(target_os = "macos"))]
pub fn open_in_finder(_file_path: &str) -> Result<()> { Ok(()) }

// get image from system clipboard
fn get_clipboard_image() -> Result<impl RustImage> {
    let ctx = ClipboardContext::new()
        .map_err(|e| format!("Failed to create clipboard context: {}", e))?;
    let img = ctx
        .get_image()
        .map_err(|_| "No image found in clipboard")?;
    Ok(img)
}

// create 5-char hash from image bytes for unique filename
fn hash_bytes_short_sha256_hex5(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_str = format!("{:x}", hash);
    // take first 5 chars for short filename
    hash_str[..5].to_string()
}

// save image as JPEG with 85% quality
fn save_as_jpeg(img: &DynamicImage, file_path: &std::path::Path) -> Result<()> {
    use std::fs::File;
    use std::io::BufWriter;
    
    let file = File::create(file_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    let writer = BufWriter::new(file);
    
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(writer, 85);
    encoder.encode_image(img)
        .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_bytes_short_sha256_hex5() {
        // test that hash function produces expected output
        let h = hash_bytes_short_sha256_hex5(b"hello");
        assert_eq!(h, "2cf24");
    }

    #[test]
    fn test_filename_generation() {
        // test hash generation works
        let hash = hash_bytes_short_sha256_hex5(b"test");
        assert_eq!(hash.len(), 5);
    }
}
