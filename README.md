A puzzle fighter/swordfighting clone as an excuse for me to learn rust.

# Features

* Two-player keyboard (WASDC, arrows+space) or gamepad controls.
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

Some debug keys are enabled in non-release builds:

    Q:   Load blocks from board.txt
    1-4: Set next piece to a colored breaker
    5:   Drop an attack.
