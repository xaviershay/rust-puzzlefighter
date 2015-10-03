use values::*;
use std::collections::{LinkedList,HashMap};

pub struct BlockGrid {
    cells: Vec<Vec<Option<PositionedBlock>>>,
}

impl BlockGrid {
    pub fn new(dimensions: Dimension) -> BlockGrid {
        let width = dimensions.w() as usize;
        let height = (dimensions.h() * 2) as usize;

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

    pub fn debug(&self) {
        let height = self.cells.len();
        for y in 0..height {
            let y = height - y - 1;
            for x in 0..self.cells[y].len() {
                let cell = self.cells[y][x];
                if cell.is_some() {
                    print!("{}", &cell.unwrap().debug_char());
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn set(&mut self, block: PositionedBlock) -> PositionedBlock {
        self.cells[block.y() as usize()][block.x() as usize()] = Some(block);
        block
    }

    pub fn clear(&mut self, position: GridPosition) -> Option<PositionedBlock> {
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

    pub fn at(&self, position: GridPosition) -> Option<PositionedBlock> {
        if let Some(row) = self.cells.get(position.y() as usize) {
            if let Some(cell) = row.get(position.x() as usize) {
                return *cell;
            }
        }
        None
    }

    pub fn find_opposite_corner(&self, anchor: &PositionedBlock, direction: Direction) -> PositionedBlock {
        let mut corner = *anchor;

        while !corner.borders().intersects(direction.to_side()) {
            corner = self
                .at(corner.position().offset(direction))
                .expect("Bad fuse state")
        }

        corner
    }

    pub fn find_breakers(&self) -> HashMap<PositionedBlock, u8> {
        let mut result = HashMap::new();
        for block in self.blocks() {
            if block.breaker() {
                if self.breaker_recurse(block, 1, &mut result) {
                    result.insert(block, 0);
                }
            }
        }
        result
    }

    fn breaker_recurse(&self, block: PositionedBlock, depth: u8, result: &mut HashMap<PositionedBlock, u8>) -> bool {
        let mut found = false;
        for direction in Direction::all() {
            if let Some(candidate) = self.at(block.position().offset(direction)) {
                if candidate.color() == block.color() && candidate.is_breakable() {
                    let replace = {
                        let default = 255; // TODO: u8::MAX
                        let existing = result.get(&candidate).unwrap_or(&default);
                        depth < *existing
                    };

                    if replace {
                        found = true;
                        result.insert(candidate, depth);
                        let modifier = if candidate.is_fused() {
                            0
                        } else {
                            1
                        };
                        self.breaker_recurse(candidate, depth + modifier, result);
                    }
                }
            }
        }
        found
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

    // Returns a list of blocks ordered left to right, bottom to top.
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

    pub fn age(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                match *cell {
                    Some(x) => {
                        *cell = Some(x.do_age())
                    },
                    None => {}
                }
            }
        }
    }
}
