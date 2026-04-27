# pasteimg

tiny cli that saves the current clipboard image to your downloads folder as a slightly compressed jpeg, the saved file is then opened in finder.

## requirements
- macos only

## installation

### homebrew

```sh
brew install --formula https://raw.githubusercontent.com/1337Core/pasteimg/main/Formula/pasteimg.rb
```

to install the latest commit from `main` instead of the latest tagged release:

```sh
brew install --HEAD --formula https://raw.githubusercontent.com/1337Core/pasteimg/main/Formula/pasteimg.rb
```

### from source

```
cargo build --release
```

 - binary output: `target/release/pasteimg`
 - to cross-build for another mac architecture, pass an explicit target, for example `cargo build --release --target x86_64-apple-darwin`.
 - install into your `path` (may require `sudo`):

```
mv target/release/pasteimg /usr/local/bin/
```

## usage
```
pasteimg
pasteimg --lossless
```

## license
- mit — see `license`
