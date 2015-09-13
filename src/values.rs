extern crate uuid;
extern crate rand;

use self::uuid::Uuid;

use std::hash::{Hash, Hasher};

use std::collections::LinkedList;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Position {
    pub fn offset(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up    => { Position { x: self.x, y: self.y + 1 }},
            Direction::Down  => { Position { x: self.x, y: self.y - 1 }},
            Direction::Left  => { Position { x: self.x - 1, y: self.y }},
            Direction::Right => { Position { x: self.x + 1, y: self.y }},
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    id: Uuid,
    pub color: Color,
}

impl Block {
    pub fn active(color: Color) -> Self {
        Block {
            id: Uuid::new_v4(),
            color: color,
        }
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
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    // TODO: These shouldn't be public
    pub blocks: [Block; 2],
    pub direction: Direction,
    pub position: Position,
}

impl Piece {
    pub fn positions(&self) -> LinkedList<Position> {
        let mut list = LinkedList::new();
        let position = self.position;

        list.push_back(position);
        list.push_back(position.offset(self.direction));

        list
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

#[derive(Copy, Clone, Debug)]
pub struct PositionedBlock {
    pub block: Block,
    pub position: Position,
}

impl PositionedBlock {
    pub fn new(block: Block, position: Position) -> Self {
        PositionedBlock {
            block: block,
            position: position,
        }
    }

    pub fn x(&self) -> i8 { self.position.x  }
    pub fn y(&self) -> i8 { self.position.y  }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow
}

impl Color {
    pub fn to_texture_name(self) -> &'static str {
        match self {
            Color::Blue => "element_blue_square.png",
            Color::Red => "element_red_square.png",
            Color::Green => "element_green_square.png",
            Color::Yellow => "element_yellow_square.png",
        }
    }

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

