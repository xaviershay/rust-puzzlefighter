use values::{PositionedBlock,Direction,Position,Color};
use std::collections::{LinkedList,HashSet};

pub struct BlockGrid {
    cells: Vec<Vec<Option<PositionedBlock>>>,
}

impl BlockGrid {
    pub fn new(width: usize, height: usize) -> BlockGrid {
        let mut rows = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(None);
            }
            rows.push(row);
        }

        BlockGrid {cells: rows}
    }

    pub fn set(&mut self, block: PositionedBlock) {
        self.cells[block.y() as usize()][block.x() as usize()] = Some(block);
    }

    pub fn clear(&mut self, position: Position) -> Option<PositionedBlock> {
        let existing = self.at(position);
        self.cells[position.y() as usize()][position.x() as usize()] = None;
        existing
    }

    pub fn empty(&self, position: PositionedBlock) -> bool {
        if let Some(row) = self.cells.get(position.y() as usize) {
            if let Some(cell) = row.get(position.x() as usize) {
                return cell.is_none();
            }
        }
        false
    }

    pub fn at(&self, position: Position) -> Option<PositionedBlock> {
        if let Some(row) = self.cells.get(position.y() as usize) {
            if let Some(cell) = row.get(position.x() as usize) {
                return *cell;
            }
        }
        None
    }

    pub fn find_contiguous(&self, color: Color, blocks: &mut HashSet<Position>) {
        let mut new_blocks = HashSet::new();

        for block in blocks.iter() {
            for direction in Direction::all() {
                if let Some(new_block) = self.at(block.offset(direction)) {
                    let pos = new_block.position();
                    if !blocks.contains(&pos) && new_block.color() == color {
                        new_blocks.insert(pos);
                    }
                }
            }
        }

        let recurse = !new_blocks.is_empty();

        for block in new_blocks.iter() {
            blocks.insert(*block);
        }

        if recurse {
            // TODO: More efficient to only iterate new_blocks
            self.find_contiguous(color, blocks);
        }

    }

    // Returns a positioned block dropped as far as possible.
    pub fn bottom(&self, pb: PositionedBlock) -> PositionedBlock {
        let mut cell = pb;
        loop {
            let new_cell = cell.offset(Direction::Down);

            if self.empty(new_cell) {
                cell = new_cell
            } else {
                break
            }
        }
        cell
    }

    pub fn blocks(&self) -> LinkedList<PositionedBlock> {
        let mut list = LinkedList::new();

        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                let cell = self.cells[y][x];
                if cell.is_some() {
                    list.push_back(cell.unwrap());
                }
            }
        }

        list
    }
}
