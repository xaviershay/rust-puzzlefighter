use values::*;
use renderer::*;
use block_grid::*;

use std::rc::Rc;

use piston_window::*;

enum Phase {
    NewPiece,
    PieceFalling,
    Settling,
    Breaking(f64),
}

pub struct Board {
    // Public
    dimensions: Dimension,

    // Private
    // State
    grid: BlockGrid,
    grid_renderer: Box<BlockRenderer>,
    next_renderer: Box<BlockRenderer>,

    // Time since last block step.
    step_accumulator: f64,

    // Seconds between block steps.
    speed: f64,

    // Currently falling fiece
    current_piece: Option<Piece>,

    // Current update phase
    phase: Phase,
}

const SLOW_SPEED: f64 = 0.5;
const TURBO_SPEED: f64 = 0.05;

impl Board {
    pub fn new(render_settings: Rc<RenderSettings>,
               dimensions: Dimension,
               position: PixelPosition) -> Self {

        // TODO: Get cell dimension from renderer
        Board {
            dimensions: dimensions,

            step_accumulator: 0.0,
            speed: SLOW_SPEED,
            current_piece: None,
            phase: Phase::NewPiece,

            grid: BlockGrid::new(dimensions),
            grid_renderer: render_settings.build(position.add(PixelPosition::new(16 + 32, 0)), dimensions),
            next_renderer: render_settings.build(position, Dimension::new(1, 2))
        }
    }

    pub fn update(&mut self, event: &PistonWindow) {
        event.update(|args| {
            match self.phase {
                // TODO: Is a noop phase really a phase? Probably not.
                Phase::NewPiece => {
                    let piece = Piece::rand(2, self.dimensions.h() as i8 - 1);
                    self.current_piece = Some(piece);

                    for block in piece.blocks().iter() {
                        self.grid_renderer.add_block(*block);
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
                                    self.grid_renderer.drop_block(resting);
                                }
                                self.current_piece = None;
                                self.phase = Phase::Settling;
                            }
                        }
                    }
                },
                Phase::Settling => {
                    let settled = self.grid.blocks().iter().all(|block| {
                        !self.grid_renderer.is_animating(*block)
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
                                self.grid_renderer.drop_block(bottom);
                            }
                        }
                        self.phase = Phase::Settling
                    }
                }
            }
        });
        self.grid_renderer.event(&event);
        self.next_renderer.event(&event);
    }

    // Attempt to modify the current piece if present. modifier will be called
    // with the current piece and should return a desired modification. If it
    // is valid (no blocks are in the way), the current piece is replaced with
    // it and true is returned. Otherwise, returns false.
    pub fn move_piece<F>(&mut self, modifier: F) -> bool
        where F : Fn(Piece) -> Piece {

        let ref mut grid = self.grid;

        if let Some(piece) = self.current_piece {
            let new_piece = modifier(piece);

            let occupied = new_piece.blocks().iter().any(|pb| {
                !grid.empty(*pb)
            });

            if !occupied {
                for pb in new_piece.blocks().iter() {
                    self.grid_renderer.move_block(*pb);
                }
                self.current_piece = Some(new_piece);
                return true;
            }
        }
        false
    }

    fn break_blocks(&mut self) -> u8 {
        let break_list = self.grid.find_breakers();

        if break_list.is_empty() {
            0
        } else {
            let mut highest_depth = 0;
            for (block, depth) in &break_list {
                self.grid.clear(block.position());
                self.grid_renderer.explode_block(*block, *depth);

                if *depth > highest_depth {
                    highest_depth = *depth;
                }
            }

            highest_depth
        }
    }

    pub fn turbo(&mut self, enable: bool) {
        if enable {
            self.speed = TURBO_SPEED;
            if self.step_accumulator > TURBO_SPEED {
                self.step_accumulator = TURBO_SPEED;
            }
        } else {
            self.speed = SLOW_SPEED;
        }
    }
}
