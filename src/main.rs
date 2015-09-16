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

use piston_window::*;

use textures::Textures;
use block_grid::{BlockGrid};
use values::{Piece,Direction};
use renderer::{BlockRenderer,Renderer};

enum Phase {
    NewPiece,
    PieceFalling,
    Settling,
    Breaking(f64),
}

struct Game {
    renderer: Box<BlockRenderer>,

    // State
    grid: BlockGrid,

    // Time since last block step.
    step_accumulator: f64,

    // Seconds between block steps.
    speed: f64,

    // Currently falling fiece
    current_piece: Option<Piece>,

    // Current update phase
    phase: Phase,
}

impl Game {
    fn new(renderer: Box<BlockRenderer>, dimensions: (usize, usize)) -> Self {
        let (w, h) = dimensions;

        Game {
            renderer: renderer,
            step_accumulator: 0.0,
            speed: 0.3,
            current_piece: None,
            phase: Phase::NewPiece,
            grid: BlockGrid::new(w, h),
        }
    }

    // Attempt to modify the current piece if present. modifier will be called
    // with the current piece and should return a desired modification. If it
    // is valid (no blocks are in the way), the current piece is replaced with
    // it and true is returned. Otherwise, returns false.
    fn move_piece<F>(&mut self, modifier: F) -> bool
        where F : Fn(Piece) -> Piece {

        let ref mut grid = self.grid;

        if let Some(piece) = self.current_piece {
            let new_piece = modifier(piece);

            let occupied = new_piece.blocks().iter().any(|pb| {
                !grid.empty(*pb)
            });

            if !occupied {
                for pb in new_piece.blocks().iter() {
                    self.renderer.move_block(*pb);
                }
                self.current_piece = Some(new_piece);
                return true;
            }
        }
        false
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

            // TODO: Handle key repeat on our own timer.
            match button {
                Keyboard(Key::Up) => {
                    self.move_piece(|current| { current.anti_clockwise() });
                },
                Keyboard(Key::Down) => {
                    self.move_piece(|current| { current.clockwise() });
                },
                Keyboard(Key::Left) => {
                    self.move_piece(|current| { current.offset(Direction::Left) });
                },
                Keyboard(Key::Right) => {
                    self.move_piece(|current| { current.offset(Direction::Right) });
                },
                Keyboard(Key::Space) => {
                    self.speed = 0.05;
                    if self.step_accumulator > 0.05 {
                        self.step_accumulator = 0.05;
                    }
                }
                _ => {},
            }
        }

        e.update(|args| {
            match self.phase {
                // TODO: Is a noop phase really a phase? Probably not.
                Phase::NewPiece => {
                    let piece = Piece::rand(2, GRID_HEIGHT as i8 - 1);
                    self.current_piece = Some(piece);

                    for block in piece.blocks().iter() {
                        self.renderer.add_block(*block);
                    }

                    self.phase = Phase::PieceFalling;
                },
                Phase::PieceFalling => {
                    self.step_accumulator += args.dt;

                    if self.step_accumulator > self.speed {
                        self.step_accumulator -= self.speed;

                        if !self.move_piece(|current| current.offset(Direction::Down) ) {
                            if let Some(piece) = self.current_piece {
                                for pb in piece.blocks().iter() {
                                    let resting = self.grid.bottom(*pb);
                                    self.grid.set(resting);
                                    self.renderer.drop_block(resting);
                                }
                                self.current_piece = None;
                                self.phase = Phase::Settling;
                            }
                        }
                    }
                },
                Phase::Settling => {
                    let settled = self.grid.blocks().iter().all(|block| {
                        !self.renderer.is_animating(*block)
                    });

                    if settled {
                        let break_depth = self.break_blocks() as f64;

                        if break_depth > 0.0 {
                            self.phase = Phase::Breaking(break_depth * 0.05);
                        } else {
                            self.phase = Phase::NewPiece;
                        }
                    }
                },
                Phase::Breaking(dt) => {
                    let dt = dt - args.dt;

                    if dt > 0.0 {
                        self.phase = Phase::Breaking(dt);
                    } else {
                        for block in self.grid.blocks() {
                            let bottom = self.grid.bottom(block);

                            if bottom.position() != block.position() {
                                self.grid.set(bottom);
                                self.grid.clear(block.position());
                                self.renderer.drop_block(bottom);
                            }
                        }
                        self.phase = Phase::Settling
                    }
                }
            }
        });

        self.renderer.event(&e);
    }

    fn break_blocks(&mut self) -> u8 {
        let break_list = self.grid.find_breakers();

        if break_list.is_empty() {
            0
        } else {
            let mut highest_depth = 0;
            for (block, depth) in &break_list {
                self.grid.clear(block.position());
                self.renderer.explode_block(*block, *depth);

                if *depth > highest_depth {
                    highest_depth = *depth;
                }
            }

            highest_depth
        }
    }
}

const GRID_HEIGHT: u8 = 13;
const GRID_WIDTH: u8 = 6;
const CELL_WIDTH: f64 = 32.0;
const CELL_HEIGHT: f64 = 32.0;

fn main() {
    let width = (GRID_WIDTH as u32 * CELL_WIDTH as u32) as u32;
    let height = (GRID_HEIGHT as u32 * CELL_HEIGHT as u32) as u32;
    let window: PistonWindow =
        WindowSettings::new("Puzzle Fighter Turbo II", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let textures = Textures::new(&window);

    let renderer = Renderer::new(textures);

    let mut game = Game::new(Box::new(renderer), (GRID_WIDTH as usize, GRID_HEIGHT as usize));

    for e in window {
        game.update(&e);
    }
}
