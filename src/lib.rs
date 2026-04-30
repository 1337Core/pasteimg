pub type Result<T> = anyhow::Result<T>;

mod core;

pub use core::{
    capture_clipboard_image, capture_image_from_png_path, default_downloads_dir, open_in_finder,
};
