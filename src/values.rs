extern crate uuid;
use self::uuid::Uuid;

use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    id: Uuid,
    pub color: Color,
    pub falling: bool,
}

impl Block {
    pub fn new(color: Color) -> Self {
        Block {
            id: Uuid::new_v4(),
            color: color,
            falling: false,
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
}
