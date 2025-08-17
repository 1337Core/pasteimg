# pasteimg

tiny cli that saves the current clipboard image to your downloads folder as a slightly compressed jpeg or lossless png (with `--lossless`). the saved file is then revealed in finder.

## requirements
- macos only

## installation

```
cargo build --release
```

 - binary output: `target/aarch64-apple-darwin/release/pasteimg`
 - apple silicon default: builds for `aarch64-apple-darwin`. for intel macs: `cargo build --release --target x86_64-apple-darwin`.
 - install into your `path` (may require `sudo`):

```
mv target/aarch64-apple-darwin/release/pasteimg /usr/local/bin/
```

## usage
```
pasteimg
pasteimg --lossless
```

## license
- mit — see `license`
