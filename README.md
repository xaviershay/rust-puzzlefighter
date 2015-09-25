A puzzle fighter/swordfighting clone as an excuse for me to learn rust.

# Features

* Two-player keyboard controls (WASDC, arrows+space).
* Sprinkle attacks with combo multiplier.

![screenshot](./screenshot.png)

Play by cloning the repo and running:

    cargo run --release

See [issues](https://github.com/xaviershay/rust-puzzlefighter/issues) for major
remaining TODOs.

## Development

The block assets are auto-generated using imagemagick's `convert` tool.

    brew install imagemagick

    cargo run --example generate_tiles -- assets/src assets/gen
