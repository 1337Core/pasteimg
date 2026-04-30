use clap::Parser;
use pasteimg::{capture_clipboard_image, capture_image_from_png_path, open_in_finder};
use std::path::PathBuf;
use std::sync::atomic::Ordering;

mod ui;

#[derive(Parser)]
#[command(name = "pasteimg")]
#[command(about = "Capture clipboard image and save as JPG (or PNG with --lossless)")]
struct Args {
    #[arg(long)]
    lossless: bool,

    /// Load PNG bytes from a file instead of the clipboard (useful for automation/tests).
    #[arg(long, value_name = "PATH")]
    input: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let loading_stop = ui::loading_bar();

    let result = match args.input {
        Some(path) => capture_image_from_png_path(&path, args.lossless),
        None => capture_clipboard_image(args.lossless),
    };

    match result {
        Ok(file_path) => {
            loading_stop.store(true, Ordering::Relaxed);
            ui::clear_loading();

            ui::success(&format!("Saved image to {}", ui::path(&file_path)));

            if let Err(e) = open_in_finder(&file_path) {
                ui::warn(&format!("{}", e));
            }
        }
        Err(e) => {
            loading_stop.store(true, Ordering::Relaxed);
            ui::clear_loading();

            ui::error(&format!("{}", e));
            std::process::exit(1);
        }
    }
}
