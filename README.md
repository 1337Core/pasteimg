# pasteimg

A tiny CLI that saves the current clipboard image to your Downloads folder as a compressed JPEG (default) or a lossless PNG (`--lossless`).

## requirements
- macOS for clipboard + Finder reveal UX
- Linux/other platforms support file conversion mode (`--input`) and save output, but Finder reveal is a no-op.

## installation

### homebrew

```sh
brew tap 1337Core/pasteimg https://github.com/1337Core/pasteimg
brew install pasteimg
```

To install the latest commit from `main` instead of the latest tagged release:

```sh
brew install --HEAD pasteimg
```

### from source

```sh
cargo build --release
```

- binary output: `target/release/pasteimg`
- to cross-build for another mac architecture, pass an explicit target, for example `cargo build --release --target x86_64-apple-darwin`.
- install into your `PATH` (requires `sudo`):

```sh
mv target/release/pasteimg /usr/local/bin/
```

## usage

```sh
pasteimg
pasteimg --lossless
```

### automated / CI usage (no clipboard dependency)

```sh
pasteimg --input ./fixture.png
pasteimg --input ./fixture.png --lossless
```

## testing

```sh
cargo test
```

The test suite includes:
- unit tests for hashing, encoding, and output persistence
- integration tests that run the CLI end-to-end using `--input`

## license
- MIT (`LICENSE`)
