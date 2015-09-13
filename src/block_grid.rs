use values::{PositionedBlock,Direction};

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

    pub fn empty(&self, position: PositionedBlock) -> bool {
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

            if self.empty(new_cell) {
                cell = new_cell
            } else {
                break
            }
        }
        cell
    }
}
