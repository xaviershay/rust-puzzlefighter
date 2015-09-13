mod textures;
mod block_grid;
mod values;
mod renderer;

extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_texture;

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
use renderer::{BlockRenderer,Renderer};

struct Game {
    renderer: Box<BlockRenderer>,

    // State
    grid: BlockGrid,
    falling: bool, // Whether a falling block is on the screen.

    // Time since last block step.
    step_accumulator: f64,

    // Seconds between block steps.
    speed: f64,
}

impl Game {
    fn new(renderer: Box<BlockRenderer>, dimensions: (usize, usize)) -> Self {
        let (w, h) = dimensions;

        Game {
            renderer: renderer,
            falling: false,
            step_accumulator: 0.0,
            speed: 0.3,
            grid: BlockGrid::new(w, h),
        }
    }

    fn update(&mut self, e: &PistonWindow) {
        if let Some(button) = e.release_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            match button {
                Keyboard(Key::Space) => {
                    self.speed = 0.3
                },
                _ => {},
            }
        }
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            let ref mut grid = self.grid;
            let ref mut renderer = self.renderer;


            // TODO: Handle key repeat on our own timer.
            match button {
                Keyboard(Key::Left) => {
                    for cell in grid.active_blocks() {
                        let block = cell.block.unwrap();
                        if let Some(left) = grid.left(cell) {
                            if let None = left.block {
                                grid.setCell(left, Some(block));
                                grid.setCell(cell, None);

                                renderer.move_block(block, left.position);
                            }
                        }
                    }
                },
                Keyboard(Key::Right) => {
                    // TODO: Iteration order matters here.
                    for cell in grid.active_blocks() {
                        let block = cell.block.unwrap();
                        if let Some(right) = grid.right(cell) {
                            if let None = right.block {
                                grid.setCell(right, Some(block));
                                grid.setCell(cell, None);

                                renderer.move_block(block, right.position);
                            }
                        }
                    }
                },
                Keyboard(Key::Space) => {
                    self.speed = 0.05;
                }
                _ => {},
            }
        }

        e.update(|args| {
            let ref mut grid = self.grid;
            let ref mut renderer = self.renderer;

            self.step_accumulator += args.dt;

            if self.step_accumulator > self.speed {
                self.step_accumulator -= self.speed;

                // Move active blocks down
                let mut done = false;
                let active = grid.active_blocks();

                if active.is_empty() {
                    // TODO: Undup with code above.
                    let pos = grid.top_left();
                    let cell = grid.set(pos, Some(Block::active(Color::rand())));
                    renderer.add_block(cell.block.unwrap(), cell.position);
                    self.falling = true
                } else {
                    for cell in grid.active_blocks() {
                        let block = cell.block.unwrap();
                        let below = grid.below(cell);

                        if below.is_some() && below.unwrap().block == None {
                            let below = below.unwrap();

                            grid.setCell(below, Some(block));
                            grid.setCell(cell, None);

                            if self.speed < 0.3 {
                                renderer.move_block(block, below.position);
                            } else {
                                renderer.drop_block(block, below.position);
                            }
                        } else {
                            grid.setCell(cell, Some(block.make_inactive()));
                            done = true;
                        }
                    }
                }

            }
        });

        self.renderer.event(&e);
    }
}

/*
struct Sprites<I: ImageSize, R> where R: gfx::Resources {
    sprites: HashMap<Uuid, Sprite<I>>,
    textures: Textures<R>,
}
*/

const GRID_HEIGHT: usize = 13;
const GRID_WIDTH: usize = 6;
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

    let mut renderer = Renderer::new(textures);

    let mut game = Game::new(Box::new(renderer), (GRID_WIDTH, GRID_HEIGHT));

    for e in window {
        game.update(&e);
    }
}
