// main entry point for clipboard image capture tool
use clap::Parser;
use pasteimg::{capture_clipboard_image, open_in_finder};
use std::sync::atomic::Ordering;

mod ui;

#[derive(Parser)]
#[command(name = "pasteimg")]
#[command(about = "Capture clipboard image and save as JPG (or PNG with --lossless)")]
struct Args {
    #[arg(long)]
    lossless: bool,
}

fn main() {
    let args = Args::parse();

    // start loading bar in background thread
    let loading_stop = ui::loading_bar();

    // try to grab image from clipboard and save it
    match capture_clipboard_image(args.lossless) {
        Ok(file_path) => {
            // stop loading bar and clear line immediately
            loading_stop.store(true, Ordering::Relaxed);
            ui::clear_loading();

            // success - show where we saved it
            ui::success(&format!(
                "Saved clipboard image to {}",
                ui::path(&file_path)
            ));

            // try to open finder to reveal file (macos only)
            if let Err(e) = open_in_finder(&file_path) {
                ui::warn(&format!("{}", e));
            }
        }
        Err(e) => {
            // stop loading bar and clear line immediately
            loading_stop.store(true, Ordering::Relaxed);
            ui::clear_loading();

            // something went wrong - show error and bail
            ui::error(&format!("{}", e));
            std::process::exit(1);
        }
    }
}
