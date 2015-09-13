extern crate uuid;
use self::uuid::*;

use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    id: uuid::Uuid,
    pub color: usize,
    pub falling: bool,
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

