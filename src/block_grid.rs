use values::{Block,Position,PositionedBlock,Direction};

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

    pub fn set(&mut self, position: Position, block: Option<Block>) -> Option<PositionedBlock> {
        let cell = match block {
            None => None,
            Some(block) => { Some(PositionedBlock::new(block, position)) },
        };
        self.cells[position.y() as usize()][position.x() as usize()] = cell;
        cell
    }

    pub fn empty(&self, position: Position) -> bool {
        if let Some(row) = self.cells.get(position.y() as usize) {
            if let Some(cell) = row.get(position.x() as usize) {
                return cell.is_none();
            }
        }
        return false
    }

    // Returns a positioned block dropped as far as possible.
    pub fn bottom(&self, pb: PositionedBlock) -> PositionedBlock {
        let mut cell = pb;
        loop {
            let new_cell = cell.offset(Direction::Down);

            if self.empty(new_cell.position) {
                cell = new_cell
            } else {
                break
            }
        }
        cell
    }
}
