use assert_cmd::Command;
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use std::path::Path;

fn write_fixture_png(path: &Path) {
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| {
        Rgba([(x * 80) as u8, (y * 100) as u8, 150, 255])
    });
    DynamicImage::ImageRgba8(image)
        .save_with_format(path, ImageFormat::Png)
        .unwrap();
}

#[test]
fn cli_creates_jpeg_from_input_file() {
    let home = tempfile::tempdir().unwrap();
    let input = home.path().join("input.png");
    write_fixture_png(&input);

    let mut cmd = Command::cargo_bin("pasteimg").unwrap();
    cmd.env("HOME", home.path())
        .arg("--input")
        .arg(&input)
        .assert()
        .success();

    let downloads = home.path().join("Downloads");
    let entries: Vec<_> = std::fs::read_dir(&downloads)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].extension().and_then(|s| s.to_str()), Some("jpg"));
}

#[test]
fn cli_creates_png_with_lossless_flag() {
    let home = tempfile::tempdir().unwrap();
    let input = home.path().join("input.png");
    write_fixture_png(&input);

    let mut cmd = Command::cargo_bin("pasteimg").unwrap();
    cmd.env("HOME", home.path())
        .arg("--input")
        .arg(&input)
        .arg("--lossless")
        .assert()
        .success();

    let downloads = home.path().join("Downloads");
    let entries: Vec<_> = std::fs::read_dir(&downloads)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].extension().and_then(|s| s.to_str()), Some("png"));
}

#[test]
fn cli_fails_for_missing_input_file() {
    let home = tempfile::tempdir().unwrap();
    let missing = home.path().join("missing.png");

    let mut cmd = Command::cargo_bin("pasteimg").unwrap();
    cmd.env("HOME", home.path())
        .arg("--input")
        .arg(&missing)
        .assert()
        .failure();
}
