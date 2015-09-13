extern crate sprite;
extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_texture;

use sprite::{Sprite,Scene};
use self::uuid::Uuid;

use std::collections::HashMap;
use std::rc::Rc;

use graphics::{ Graphics, ImageSize };
use gfx_texture::{Texture};

use piston_window::{PistonWindow,WindowSettings,Flip,TextureSettings};

struct TextureFactory<R> where R: gfx::Resources {
    block_textures: Vec<Rc<Texture<R>>>,
}

impl<R> TextureFactory<R> where R: gfx::Resources  {
    fn new(window: PistonWindow) -> Self {
        let texture = Rc::new(gfx_texture::Texture::from_path(
            &mut *window.factory.borrow_mut(),
            "assets/element_red_square.png",
            Flip::None, &TextureSettings::new()
        ).unwrap());
        let block_textures = Vec::new();
        block_textures.push(texture);

        TextureFactory {
            block_textures: block_textures,
        }
    }
}

const GRID_HEIGHT: usize = 10;
const GRID_WIDTH: usize = 8;
const CELL_WIDTH: f64 = 32.0;
const CELL_HEIGHT: f64 = 32.0;

fn main() {
    let width = (GRID_WIDTH * CELL_WIDTH as usize) as u32;
    let height = (GRID_HEIGHT * CELL_HEIGHT as usize) as u32;
    let window: PistonWindow =
        WindowSettings::new("Puzzle Fighter Turbo II", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let textures = TextureFactory::new(window);
}
