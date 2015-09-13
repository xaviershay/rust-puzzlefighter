use values::{Block,Position,PositionedBlock,Direction};

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
                    position: Position { x: x as i8, y: y as i8 },
                    block: None,
                };
                row.push(cell);
            }
            rows.push(row);
        }

        BlockGrid {cells: rows}
    }

    pub fn set(&mut self, position: Position, block: Option<Block>) -> BlockCell {
        let cell = BlockCell { position: position, block: block };
        self.cells[position.y as usize()][position.x as usize()] = cell;
        cell
    }

    pub fn empty(&self, position: Position) -> bool {
        if let Some(cell) = self.cell_at(position) {
            cell.block.is_none()
        } else {
            false
        }
    }

    pub fn cell_at(&self, position: Position) -> Option<BlockCell> {
        if let Some(row) = self.cells.get(position.y as usize) {
            if let Some(cell) = row.get(position.x as usize) {
                return Some(*cell);
            }
        }

        None
    }

    pub fn below(&self, cell: BlockCell) -> Option<BlockCell> {
        self.cell_at(cell.position.offset(Direction::Down))
    }

    // Returns the further cell this block could fall to. Returns self if block
    // cannot fall.
    pub fn bottom(&self, pb: PositionedBlock) -> BlockCell {
        let mut bottom_cell = self.cell_at(pb.position).unwrap();
        loop {
            match self.below(bottom_cell) {
                Some(new_cell) => {
                    if new_cell.block.is_some() {
                        break;
                    }
                    bottom_cell = new_cell;
                },
                None => {
                    break;
                }
            };
        }
        bottom_cell
    }
}
