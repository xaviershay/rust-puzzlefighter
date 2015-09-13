mod textures;
mod block_grid;
mod values;

extern crate sprite;
extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;

use sprite::{Sprite,Scene};
use self::uuid::Uuid;

use std::collections::HashMap;
use std::rc::Rc;

use graphics::math::{ Matrix2d };

/*
use graphics::{ Graphics, ImageSize };
use gfx_texture::{Texture};
*/

use piston_window::*;

use textures::Textures;
use block_grid::BlockGrid;
use values::{Position,Block,Color};

struct Game {
    grid: BlockGrid,
    renderer: Box<BlockRenderer>,
}

impl Game {
    fn new(renderer: Box<BlockRenderer>, dimensions: (usize, usize)) -> Self {
        let (w, h) = dimensions;

        Game {
            renderer: renderer,
            grid: BlockGrid::new(w, h),
        }
    }

    fn update(&mut self, e: &PistonWindow) {
        self.renderer.event(&e);
    }
}

struct Sprites<I: ImageSize, R> where R: gfx::Resources {
    sprites: HashMap<Uuid, Sprite<I>>,
    textures: Textures<R>,
}

struct Renderer<I: ImageSize, R> where R: gfx::Resources {
    scene: Scene<I>,
    textures: Textures<R>,
}

impl<I: ImageSize, R> Renderer<I, R> where R: gfx::Resources {
    fn new(textures: Textures<R>, scene: Scene<I>) -> Self {
        Renderer {
            textures: textures,
            scene: scene,
        }
    }
}

trait BlockRenderer {
    fn event(&mut self, event: &PistonWindow) {}
    fn add_block(&mut self, block: Block, position: Position);
}

impl BlockRenderer for Renderer<Texture<gfx_device_gl::Resources>, gfx_device_gl::Resources> {
    fn add_block(&mut self, block: Block, position: Position) {
        let texture = self.textures.get(block.color.to_texture_name());
        let mut sprite = Sprite::from_texture(texture);
        sprite.set_anchor(0.0, 0.0);

        let id = self.scene.add_child(sprite);
        // TODO: Keep track of block -> id mapping
    }

    fn event(&mut self, event: &PistonWindow) {
        self.scene.event(event);
        event.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            self.scene.draw(c.transform, g);
        });
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

    let textures = Textures::new(&window);

    let mut scene = Scene::new();

    let mut renderer = Renderer::new(textures, scene);

    let mut game = Game::new(Box::new(renderer), (GRID_WIDTH, GRID_HEIGHT));

    for e in window {
        game.update(&e);
    }
}
