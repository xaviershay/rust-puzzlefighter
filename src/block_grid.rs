use std::collections::linked_list::{Iter,LinkedList};

use values::{Block,Position};

#[derive(Copy, Clone, Debug)]
pub struct BlockCell {
    pub position: Position,
    pub block: Option<Block>,
}

pub struct BlockGrid {
    cells: Vec<Vec<BlockCell>>,
}

impl BlockGrid {
    pub fn new(width: usize, height: usize) -> BlockGrid {
        let mut rows = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let cell = BlockCell {
                    position: Position { x: x, y: y },
                    block: None,
                };
                row.push(cell);
            }
            rows.push(row);
        }

        BlockGrid {cells: rows}
    }

    pub fn top_left(&self) -> Position {
        Position { x: 0, y: self.cells.len() - 1 }
    }

    pub fn set(&mut self, position: Position, block: Option<Block>) -> BlockCell {
        let cell = BlockCell { position: position, block: block };
        self.cells[position.y][position.x] = cell;
        cell
    }

    pub fn setCell(&mut self, cell: BlockCell, block: Option<Block>) {
        self.cells[cell.position.y][cell.position.x] = BlockCell { block: block, ..cell };
    }

    pub fn below(&self, cell: BlockCell) -> Option<BlockCell> {
        if cell.position.y <= 0 {
            None
        } else {
            Some(self.cells[cell.position.y-1][cell.position.x])
        }
    }

    pub fn right(&self, cell: BlockCell) -> Option<BlockCell> {
        let ref row = self.cells[cell.position.y];

        if cell.position.x >= row.len() - 1 {
            None
        } else {
            Some(self.cells[cell.position.y][cell.position.x+1])
        }
    }

    pub fn left(&self, cell: BlockCell) -> Option<BlockCell> {
        let ref row = self.cells[cell.position.y];

        if cell.position.x <= 0 {
            None
        } else {
            Some(self.cells[cell.position.y][cell.position.x-1])
        }
    }

    pub fn blocks(&self) -> LinkedList<BlockCell> {
        let mut list = LinkedList::new();

        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                let cell = self.cells[y][x];
                if let Some(_) = cell.block {
                    list.push_back(self.cells[y][x]);
                }
            }
        }
        list
    }

    pub fn active_blocks(&self) -> LinkedList<BlockCell> {
        let mut list = LinkedList::new();

        for cell in self.blocks().iter().filter(|x| { x.block.unwrap().active }) {
            list.push_back(*cell);
        }
        list
    }
}
