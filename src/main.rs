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

    let loading_stop = ui::loading_bar();

    match capture_clipboard_image(args.lossless) {
        Ok(file_path) => {
            loading_stop.store(true, Ordering::Relaxed);
            ui::clear_loading();

            ui::success(&format!(
                "Saved clipboard image to {}",
                ui::path(&file_path)
            ));

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
