extern crate uuid;
extern crate rand;

use self::uuid::Uuid;

use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition {
    x: i8,
    y: i8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PixelPosition {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dimension {
    w: u32,
    h: u32,
}

impl Dimension {
    pub fn new(w: u32, h: u32) -> Self {
        Dimension {
            w: w,
            h: h,
        }
    }

    pub fn from_tuple(tuple: (u32, u32)) -> Self {
        Dimension::new(tuple.0, tuple.1)
    }

    pub fn w(&self) -> u32 { self.w }
    pub fn h(&self) -> u32 { self.h }
}

impl PixelPosition {
    pub fn new(x: u32, y: u32) -> Self {
        PixelPosition {
            x: x,
            y: y,
        }
    }

    pub fn x(&self) -> u32 { self.x }
    pub fn y(&self) -> u32 { self.y }

    pub fn add(&self, rhs: Self) -> Self {
        PixelPosition::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl GridPosition {
    pub fn new(x: i8, y: i8) -> Self {
        GridPosition {
            x: x,
            y: y,
        }
    }

    pub fn offset(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up    => { GridPosition { x: self.x, y: self.y + 1 }},
            Direction::Down  => { GridPosition { x: self.x, y: self.y - 1 }},
            Direction::Left  => { GridPosition { x: self.x - 1, y: self.y }},
            Direction::Right => { GridPosition { x: self.x + 1, y: self.y }},
        }
    }

    pub fn x(&self) -> i8 { self.x }
    pub fn y(&self) -> i8 { self.y }
}

bitflags! {
    flags Sides: u8 {
        const SIDE_LEFT   = 0b00000001,
        const SIDE_RIGHT  = 0b00000010,
        const SIDE_TOP    = 0b00000100,
        const SIDE_BOTTOM = 0b00001000,
        const SIDE_NONE   = 0b00000000,
        const SIDE_BOTTOM_LEFT  = SIDE_LEFT.bits  | SIDE_BOTTOM.bits,
        const SIDE_BOTTOM_RIGHT = SIDE_RIGHT.bits | SIDE_BOTTOM.bits,
        const SIDE_TOP_LEFT     = SIDE_LEFT.bits  | SIDE_TOP.bits,
        const SIDE_TOP_RIGHT    = SIDE_RIGHT.bits | SIDE_TOP.bits,
        const SIDE_ALL = SIDE_LEFT.bits |
                         SIDE_RIGHT.bits |
                         SIDE_TOP.bits |
                         SIDE_BOTTOM.bits,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    id: Uuid,
    pub color: Color,
    breaker: bool,
    borders: Sides,
}

impl Block {
    pub fn breaker(&self) -> bool { self.breaker }

    pub fn debug_char(&self) -> String {
        // TODO: ANSI colors! Currently only suitable for debugging fuses.
        self.borders.debug_char().to_string()
    }

    pub fn new(color: Color, breaker: bool) -> Self {
        Block {
            id: Uuid::new_v4(),
            color: color,
            breaker: breaker,
            borders: SIDE_ALL,
        }
    }

    pub fn is_fused(&self) -> bool {
        self.borders != SIDE_ALL
    }

    pub fn to_texture_name(&self) -> String {
        let name = if self.breaker {
            match self.color {
                Color::Blue   => "blue_breaker",
                Color::Red    => "red_breaker",
                Color::Green  => "green_breaker",
                Color::Yellow => "yellow_breaker",
            }
        } else {
            match self.color {
                Color::Blue   => "blue",
                Color::Red    => "red",
                Color::Green  => "green",
                Color::Yellow => "yellow",
            }
        };

        name.to_string() + &self.borders.to_texture_suffix() + ".png"
    }

}

impl Sides {
    pub fn to_texture_suffix(&self) -> &'static str {
        match self {
            &SIDE_ALL          => { "" },
            &SIDE_BOTTOM_LEFT  => { "_bl" },
            &SIDE_BOTTOM_RIGHT => { "_br" },
            &SIDE_TOP_LEFT     => { "_tl" },
            &SIDE_TOP_RIGHT    => { "_tr" },
            &SIDE_BOTTOM       => { "_b" },
            &SIDE_TOP          => { "_t" },
            &SIDE_LEFT         => { "_l" },
            &SIDE_RIGHT        => { "_r" },
            &SIDE_NONE         => { "_m" },
            _ => { "" },
        }
    }

    pub fn debug_char(&self) -> &'static str {
        match self {
            &SIDE_ALL          => { "X" },
            &SIDE_BOTTOM_LEFT  => { "┗" },
            &SIDE_BOTTOM_RIGHT => { "┛" },
            &SIDE_TOP_LEFT     => { "┏" },
            &SIDE_TOP_RIGHT    => { "┓" },
            &SIDE_BOTTOM       => { "━" },
            &SIDE_TOP          => { "━" },
            &SIDE_LEFT         => { "┃" },
            &SIDE_RIGHT        => { "┃" },
            &SIDE_NONE         => { " " },
            _ => { "" },
        }
    }

    // Execute all the code paths to shut up warnings.
    // FIX: https://github.com/rust-lang/rust/issues/24580
    #[allow(dead_code)]
    fn _dead_code(&mut self) {
        self.is_all();
        self.is_empty();
        self.bits();
        self.intersects(*self);
        self.remove(SIDE_RIGHT);
        self.toggle(SIDE_RIGHT);
        Sides::from_bits(0b00000000);
        Sides::from_bits_truncate(0b00000000);
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Block {}
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec!(
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left
        )
    }
    pub fn clockwise(&self) -> Self {
        match *self {
            Direction::Up    => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
        }
    }

    pub fn anti_clockwise(&self) -> Self {
        match *self {
            Direction::Up    => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down  => Direction::Right,
            Direction::Left  => Direction::Down,
        }
    }

    pub fn to_side(&self) -> Sides {
        match *self {
            Direction::Up    => SIDE_TOP,
            Direction::Right => SIDE_RIGHT,
            Direction::Down  => SIDE_BOTTOM,
            Direction::Left  => SIDE_LEFT,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    // TODO: These shouldn't be public
    pub blocks: [Block; 2],
    pub direction: Direction,
    pub position: GridPosition,
}

impl Piece {
    #[cfg(debug_assertions)]
    pub fn new(b1: Block, b2: Block) -> Self {
        Piece {
            blocks: [b1, b2],
            position: GridPosition::new(0, 0),
            direction: Direction::Up,
        }
    }

    pub fn rand(x: i8, y: i8) -> Self {
        use self::rand::*;

        let mut rng = thread_rng();
        let pos = GridPosition::new(x, y);
        let block1 = Block::new(Color::rand(), rng.gen_weighted_bool(4));
        let block2 = Block::new(Color::rand(), rng.gen_weighted_bool(4));

        Piece {
            blocks: [block1, block2],
            position: pos,
            direction: Direction::Up,
        }
    }

    pub fn dup_to(&self, position: GridPosition, direction: Direction) -> Self {
        Piece {
            blocks: self.blocks,
            position: position,
            direction: direction,
        }
    }

    // Return blocks with positions, bottom to top ordered.
    pub fn blocks(&self) -> [PositionedBlock; 2] {
        let position = self.position;

        let positions = [
            PositionedBlock::new(self.blocks[0], position),
            PositionedBlock::new(self.blocks[1], position.offset(self.direction)),
        ];
        match self.direction {
            Direction::Down => { [positions[1], positions[0]] },
            _               => { positions },
        }
    }

    pub fn offset(&self, direction: Direction) -> Self {
        let position = self.position.offset(direction);

        Piece {
            position: position,
            ..*self
        }
    }

    pub fn clockwise(&self) -> Self {
        let direction = self.direction.clockwise();

        Piece {
            direction: direction,
            ..*self
        }
    }

    pub fn anti_clockwise(&self) -> Self {
        let direction = self.direction.anti_clockwise();

        Piece {
            direction: direction,
            ..*self
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PositionedBlock {
    block: Block,
    position: GridPosition,
}

impl PositionedBlock {
    pub fn new(block: Block, position: GridPosition) -> Self {
        PositionedBlock {
            block: block,
            position: position,
        }
    }

    pub fn x(&self) -> i8 { self.position.x() }
    pub fn y(&self) -> i8 { self.position.y() }
    pub fn block(&self) -> Block { self.block }
    pub fn position(&self) -> GridPosition { self.position }
    pub fn color(&self) -> Color { self.block.color }
    pub fn borders(&self) -> Sides { self.block.borders }
    pub fn breaker(&self) -> bool { self.block.breaker() }
    pub fn is_fused(&self) -> bool { self.block.is_fused() }
    pub fn fuse(&self, borders: Sides) -> Self {
        let block = Block {
            borders: borders,
            ..self.block
        };

        PositionedBlock {
            block: block,
            ..*self
        }
    }

    pub fn can_fuse_with(&self, other: PositionedBlock) -> bool {
        self.color() == other.color() && !self.breaker() && !other.breaker()
    }

    pub fn offset(&self, direction: Direction) -> Self {
        let position = self.position.offset(direction);

        PositionedBlock {
            position: position,
            ..*self
        }
    }

    pub fn drop(&self, height: i8) -> Self {
        let mut result = *self;
        for _ in 0..height {
            result = result.offset(Direction::Down);
        }
        result
    }

    pub fn debug_char(&self) -> String { self.block.debug_char() }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow
}

impl Color {
    pub fn rand() -> Self {
        use self::rand::*;

        let all = vec![
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Yellow
        ];
        let mut rng = rand::thread_rng();

        *rng.choose(&all).unwrap()
    }
}

