# pasteimg

Tiny CLI that saves the current clipboard image to `~/Downloads` as a JPEG (default) or PNG (with `--lossless`). The file name is a short, deterministic 5‑char SHA‑256 prefix. On macOS, the saved file is revealed in Finder.

## Platform
- macOS only. Relies on Finder integration and assumes a `~/Downloads` directory.

## Installation

```
cargo build --release
```

 - Binary output: `target/release/pasteimg`
 - Apple Silicon default: builds for `aarch64-apple-darwin`. For Intel Macs: `cargo build --release --target x86_64-apple-darwin`.
 - install into your `PATH` (may require `sudo`):

```
mv target/release/pasteimg /usr/local/bin/
```

## Usage
```
pasteimg           # saves clipboard image as JPEG
pasteimg --lossless  # saves clipboard image as PNG
```

## Behavior
- Reads the current image from the clipboard.
- Saves to `~/Downloads/<hash>.jpg` by default, or `~/Downloads/<hash>.png` with the `--lossless` flag.
- Reveals the saved file in Finder.
- Local-only: reads the clipboard and writes to disk; no network access.
 - Overwrite note: files with the same 5‑char hash overwrite existing ones.

## License
- MIT — see the `LICENSE` file for details.
