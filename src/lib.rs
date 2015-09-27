#![crate_name = "puzzlefighter"]
#[macro_use]
extern crate bitflags;

pub mod textures;
mod block_grid;
mod values;
pub mod board;
pub mod board_renderer;
pub mod human_player;

pub use self::board::*;
pub use self::values::*;
pub use self::block_grid::*;

extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_texture;
