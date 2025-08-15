pub type Result<T> = anyhow::Result<T>;

mod core;

pub use core::{capture_clipboard_image, open_in_finder};
