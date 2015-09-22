A puzzle fighter/swordfighting clone as an excuse for me to learn rust.

# Features

* Two-player keyboard controls (WASDC, arrows+space).
* Sprinkle attacks with combo multiplier.

![screenshot](./screenshot.png)

Play by cloning the repo and running:

    cargo run --release

## Development

The block assets are auto-generated using imagemagick's `convert` tool.

    brew install imagemagick

    cargo run --example generate_tiles -- assets/src assets/gen

### Major TODOs

* Some kind of endgame.
* Power blocks.
* Gamepad input.
* Different drop patterns.
* Counter blocks.
* Decent graphics.
* Sound effects.
