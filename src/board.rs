use values::*;
use renderer::*;
use block_grid::*;

use std::collections::LinkedList;

use std::rc::Rc;

use piston_window::*;

enum Phase {
    NewPiece,
    PieceFalling,
    Settling(u32),
    Breaking(f64, u32),
}

type StrikePattern = Vec<Color>;

struct Attack {
    strike_pattern: StrikePattern,
    sprinkles: u32,
}

impl Attack {
    fn sprinkles(strike_pattern: StrikePattern, size: u32) -> Self {
        Attack {
            strike_pattern: strike_pattern,
            sprinkles: size,
        }
    }

    fn apply(&self, dimensions: Dimension, attack_from_left: bool) -> LinkedList<PositionedBlock> {
        let mut attack = LinkedList::new();

        for i in 0..self.sprinkles+1 {
            let ref pattern = self.strike_pattern;
            let color = pattern[i as usize % pattern.len()];
            let block = Block::new(color, false);
            let x = i % dimensions.w();
            let x = if attack_from_left {
                x
            } else {
                dimensions.w() - x - 1
            };

            let position = GridPosition::new(
                x as i8,
                (i / dimensions.w() + dimensions.h()) as i8);

            let pb = PositionedBlock::new(block, position);
            attack.push_back(pb);
        }

        attack
    }
}

pub struct Board {
    // Public
    dimensions: Dimension,

    // Private
    // State
    grid: BlockGrid,
    grid_renderer: Box<BlockRenderer>,
    next_renderer: Box<BlockRenderer>,

    // Count of pending sprinkles
    attacks: LinkedList<Attack>,

    // Pending attack strength. Accumulates through combos then is dispatched
    // in a single attack when board is settled.
    strength: u32,

    // Toggles each attack, alternate which sides sprinkles fall from.
    attack_from_left: bool,

    // Time since last block step.
    step_accumulator: f64,

    // Seconds between block steps.
    speed: f64,

    // Currently and next falling pieces
    current_piece: Option<Piece>,
    next_piece: Option<Piece>,

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
        let mut board = Board {
            dimensions: dimensions,

            step_accumulator: 0.0,
            speed: SLOW_SPEED,
            current_piece: None,
            next_piece: None,
            attacks: LinkedList::new(),
            strength: 0,
            attack_from_left: false,
            phase: Phase::NewPiece,

            grid: BlockGrid::new(dimensions),
            grid_renderer: render_settings.build(position.add(PixelPosition::new(16 + 32, 0)), dimensions),
            next_renderer: render_settings.build(position, Dimension::new(1, 2))
        };
        board.generate_next_piece();
        board
    }

    // Dumps a debug representation to stdout
    pub fn debug(&self) {
        self.grid().debug();
    }

    // Helper method for testing. Provides a string syntax for specifying a
    // board. Capital first letter of color makes a block, lower case makes a
    // breaker.
    pub fn add_blocks(&mut self, lines: Vec<&'static str>) {
        let height = lines.len();
        for y in 0..height {
            let mut x = 0;
            let line = lines[y];
            for c in line.chars() {
                let y = (height - y - 1) as i8;
                let block = match c {
                    'R' => Some(Block::new(Color::Red, false)),
                    'G' => Some(Block::new(Color::Green, false)),
                    'B' => Some(Block::new(Color::Blue, false)),
                    'Y' => Some(Block::new(Color::Yellow, false)),
                    'r' => Some(Block::new(Color::Red, true)),
                    'g' => Some(Block::new(Color::Green, true)),
                    'b' => Some(Block::new(Color::Blue, true)),
                    'y' => Some(Block::new(Color::Yellow, true)),
                    _   => None
                };

                if let Some(block) = block {
                    let position = GridPosition::new(x, y);
                    self.grid_renderer.add_block(
                        self.grid.set(PositionedBlock::new(block, position))
                    );
                }
                x += 1
            }
        }
    }

    // For use in testing and AIs
    pub fn grid(&self) -> &BlockGrid {
        &self.grid
    }

    pub fn attack(&mut self, strength: u32) {
        if strength > 0 {
            // basic foil/chunli pattern
            let strikes = vec!(
                Color::Red, Color::Red, Color::Green, Color::Green, Color::Blue, Color::Blue
            );

            self.attacks.push_back(Attack::sprinkles(strikes, strength));
        }
    }

    pub fn generate_next_piece(&mut self) {
        if let Some(piece) = self.next_piece {
            // Remove existing
            for block in piece.blocks().iter() {
                self.next_renderer.remove_block(*block);
            }
        }

        // New random
        let piece = Piece::rand(0, 0);
        self.next_piece = Some(piece);

        for block in piece.blocks().iter() {
            self.next_renderer.add_block(*block);
        }
    }

    pub fn update(&mut self, event: &PistonWindow, enemy: &mut Board) {
        event.update(|args| {
            match self.phase {
                // TODO: Is a noop phase really a phase? Probably not.
                Phase::NewPiece => {
                    // Apply attack
                    if let Some(attack) = self.attacks.pop_front() {
                        let blocks = attack.apply(self.dimensions, self.attack_from_left);

                        for pb in blocks {
                            self.grid_renderer.add_block(pb);
                            let resting = self.grid.bottom(pb);
                            self.grid.set(resting);
                            self.grid_renderer.drop_block(resting);
                        }

                        self.attack_from_left = !self.attack_from_left;
                        self.phase = Phase::Settling(0);
                    }

                    // Create new piece
                    self.current_piece = Some(self.next_piece.unwrap().dup_to(
                        GridPosition::new(3, self.dimensions.h() as i8),
                        Direction::Up));

                    for block in self.current_piece.unwrap().blocks().iter() {
                        self.grid_renderer.add_block(*block);
                    }

                    self.generate_next_piece();

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
                                self.phase = Phase::Settling(0);
                            }
                        }
                    }
                },
                Phase::Settling(combo_depth) => {
                    let settled = self.grid.blocks().iter().all(|block| {
                        !self.grid_renderer.is_animating(*block)
                    });

                    if settled {
                        self.fuse_blocks();

                        let break_depth = self.break_blocks(combo_depth) as f64;

                        if break_depth > 0.0 {
                            self.phase = Phase::Breaking(break_depth * 0.05, combo_depth + 1);
                        } else {
                            enemy.attack(self.strength);
                            self.strength = 0;
                            self.phase = Phase::NewPiece;
                        }
                    }
                },
                Phase::Breaking(dt, combo_depth) => {
                    let dt = dt - args.dt;

                    if dt > 0.0 {
                        self.phase = Phase::Breaking(dt, combo_depth);
                    } else {
                        self.drop_blocks();
                        self.phase = Phase::Settling(combo_depth)
                    }
                }
            }
        });
        self.grid_renderer.event(&event);
        self.next_renderer.event(&event);
    }

    // Scan the board looking for blocks that should be dropped down to a lower
    // position. Assumes that blocks are iterated bottom-to-top, since lower
    // blocks need to move out of the way for higher ones to drop into that
    // space. The fused block logic further assumes left-to-right iteration.
    //
    // Assumes a well formed board (which, short of bugs, will always be true).
    pub fn drop_blocks(&mut self) {
        let mut fused_list = LinkedList::new();
        let mut fused_depth = self.dimensions.h() as i8;

        for block in self.grid.blocks() {
            // If this block is part of a fuse, then accumulate the block
            // rather than moving it immediately. We need to drop the block
            // only as far as the bottom side that can move the least, so that
            // it will "rest" on a shelf as needed.
            if block.is_fused() && block.borders().intersects(SIDE_BOTTOM) {
                use std::cmp;

                let bottom = self.grid.bottom(block);
                fused_depth = cmp::min(block.y() - bottom.y(), fused_depth);
                fused_list.push_back(block);

                // Only once the entire bottom side is accumulated do we drop
                // them all.
                if block.borders() == SIDE_BOTTOM_RIGHT {
                    for block in fused_list.into_iter() {
                        self.drop_block(block, block.drop(fused_depth));
                    }

                    fused_depth = self.dimensions.h() as i8;
                    fused_list = LinkedList::new();
                }
            } else {
                let bottom = self.grid.bottom(block);
                self.drop_block(block, bottom);
            }
        }
    }

    fn drop_block(&mut self, block: PositionedBlock, bottom: PositionedBlock) {
        if bottom.position() != block.position() {
            self.grid.clear(block.position());
            self.grid.set(bottom);
            self.grid_renderer.drop_block(bottom);
        }
    }

    // Scan the board looking for blocks that can be fused together. In
    // general, non-special blocks of 2x2 or more of the same color "fuse"
    // together to form a single larger "block" that is both aesthetically
    // pleasing and provides more strength when broken. Internally, they are
    // still tracked as individual blocks but with extra attributes indicating
    // they are part of a larger piece.
    pub fn fuse_blocks(&mut self) {
        for block in self.grid.blocks() {
            // Extract a 2x2 square to examine
            let block   = self.grid.at(block.position()).unwrap();

            let up      = self.grid.at(block.position().offset(Direction::Up));
            let right   = self.grid.at(block.position().offset(Direction::Right));
            let upright = self.grid.at(block.position().offset(Direction::Up).offset(Direction::Right));

            // If the cells do not call contain bricks, move on. No fusing will
            // be possible.
            if up.is_some() && right.is_some() && upright.is_some() {
                let up      = up.unwrap();
                let right   = right.unwrap();
                let upright = upright.unwrap();

                if !block.is_fused() {
                    // Base case is that all blocks are the same color,
                    // non-special, and not already fused. In which case, fuse
                    // then all together to make a 2x2.
                    //
                    // This is the only case that is checked for unfused
                    // blocks. Everything else is covered below.

                    let fuse = vec!(block, up, right, upright).into_iter().all(|x| {
                        x.can_fuse_with(block) && !x.is_fused()
                    });

                    if fuse {
                        let x = self.grid.set(block.fuse(SIDE_BOTTOM | SIDE_LEFT));
                        self.grid_renderer.transition_block(x);

                        let x = self.grid.set(up.fuse(SIDE_TOP | SIDE_LEFT));
                        self.grid_renderer.transition_block(x);

                        let x = self.grid.set(right.fuse(SIDE_BOTTOM | SIDE_RIGHT));
                        self.grid_renderer.transition_block(x);

                        let x = self.grid.set(upright.fuse(SIDE_TOP | SIDE_RIGHT));
                        self.grid_renderer.transition_block(x);
                    }
                } else {
                    // All other fusing is done by "extending" existing fused
                    // blocks.
                    //
                    // The particular corners used as starting points below are
                    // such that they are the first potential starting point
                    // that will be examined.
                    //
                    // BOTTOM_LEFT blocks there were just created by the above base
                    // case are not re-examined (i.e. this is done in an else
                    // clause). Due to iteration order, any case where it would
                    // extend left or down would have already been covered.
                    match block.borders() {
                        SIDE_TOP_LEFT => {
                            self.extrude(block, Direction::Up, Direction::Right);
                        },
                        SIDE_BOTTOM_RIGHT => {
                            self.extrude(block, Direction::Right, Direction::Up);
                        },
                        SIDE_BOTTOM_LEFT => {
                            self.extrude(block, Direction::Left, Direction::Up);
                            self.extrude(block, Direction::Down, Direction::Right);
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    // Try extruding a fused block (identified by the given anchor corner) in a
    // particular extrude_direction. traverse_direction indicates the direction
    // to find the opposite corner block - in theory it could be derived but
    // it's simpler to just specify it.
    fn extrude(&mut self, anchor: PositionedBlock, extrude_direction: Direction, traverse_direction: Direction) {
        let ref mut grid = self.grid;

        // If extruding up, the "extrude side" is up.
        let extrude_side    = extrude_direction.to_side();
        // If extruding up, the "fused sides" are left and right.
        let fused_sides     = extrude_direction.clockwise().to_side() |
                              extrude_direction.anti_clockwise().to_side();
        let opposite_corner = extrude_side | traverse_direction.to_side();

        let mut block = anchor;
        let mut good = true;

        // Iterate over every block on the edge, and compare it to the
        // adjancent block that would potentially fuse with it. This loop is
        // only guaranteed to terminate with a well formed board (which, short
        // of bugs, should always be the case).
        while good {
            let pos = block.position();
            let border_pos = pos.offset(extrude_direction);

            // Assume the adjacent block cannot fuse. There are many conditions
            // that must be met!
            good = false;

            // There actually has to be an adjacent block.
            if let Some(alt) = grid.at(border_pos) {
                let can_fuse = block.can_fuse_with(alt) && (
                    // This extra check prevents fusing across corners. A 2x2
                    // block with a 2x3 and a 2x2 sitting next to each other on
                    // top of it should not fuse.
                    !alt.is_fused() || (
                        block.borders() & fused_sides ==
                        alt.borders()   & fused_sides
                    ));

                if can_fuse {
                    // This is potentially a good fuse!
                    good = true;

                    if block.borders().contains(opposite_corner) {
                        // We found the opposite corner, so exit the loop.
                        break;
                    } else {
                        // Move to the next block on the edge.
                        block = grid
                            .at(pos.offset(traverse_direction))
                            .expect("Bad fuse state")
                    }
                }
            }
        }

        if good {
            // Now that we've determined a fuse should occur, traverse again
            // over the edge but actually perform the fuse.
            let mut block = anchor;

            // This is the same loop as before, but we since we know the fuse
            // is good we can assume the break will be hit.
            loop {
                let pos = block.position();
                let border_pos = pos.offset(extrude_direction);
                let alt = grid.at(border_pos)
                    .expect("Present per loop above");

                // Calculate correct sides for the newly fused blocks.
                let sides = block.borders() & fused_sides;
                let alt_sides = if alt.is_fused() {
                    SIDE_NONE
                } else {
                    extrude_side
                };
                let alt_sides = alt_sides | sides;


                let x = grid.set(alt.fuse(alt_sides));
                self.grid_renderer.transition_block(x);

                let x = grid.set(block.fuse(sides));
                self.grid_renderer.transition_block(x);

                if block.borders().contains(opposite_corner) {
                    // We found the opposite corner, so exit the loop.
                    break;
                }
                block = grid
                    .at(pos.offset(traverse_direction))
                    .expect("Present per loop above")
            }
        }
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

    fn break_blocks(&mut self, combo_depth: u32) -> u8 {
        let break_list = self.grid.find_breakers();

        if break_list.is_empty() {
            0
        } else {
            let mut attack: u32 = 0;

            // Find bottom left corners
            for (block, _) in &break_list {
                if block.is_fused() {
                    if block.borders().contains(SIDE_BOTTOM_LEFT) {
                        use std::cmp::min;

                        let mut top_left     = self.grid.find_opposite_corner(block, Direction::Up);
                        let mut bottom_right = self.grid.find_opposite_corner(block, Direction::Right);

                        let w = bottom_right.x() - block.x() + 1;
                        let h = top_left.y() - block.y() + 1;

                        let fuse_multiplier = min(w, h) as u32;
                        let fuse_attack = (w * h) as u32 * fuse_multiplier;

                        attack += fuse_attack;
                    }
                } else if !block.breaker() {
                    attack += 1
                }
            }

            self.strength += (attack / 2) * (combo_depth + 1);

            // Destroy everything
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
