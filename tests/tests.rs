extern crate puzzlefighter;

pub use self::puzzlefighter::*;
pub use std::rc::*;

pub struct FakeRenderSettings {
    _fake: bool,
}

impl FakeRenderSettings {
    pub fn new() -> Self {
        FakeRenderSettings {
            _fake: true,
        }
    }
}

impl BlockRenderer for FakeRenderSettings {
}

impl RenderSettings for FakeRenderSettings {
    fn build(&self, _position: PixelPosition, _dimensions: Dimension) -> Box<BlockRenderer> {
        Box::new(FakeRenderSettings::new())
    }
}

pub fn make_board(height: usize) -> Board {
    let fake_render_settings = FakeRenderSettings::new();

    Board::new(
        Rc::new(fake_render_settings),
        Dimension::new(10, height as u32),
        PixelPosition::new(0, 0))
}

pub fn assert_fused(board: &Board, x: i8, y: i8, sides: Sides) {
    let block = board.grid().at(GridPosition::new(x, y))
        .expect(&format!("No block at ({}, {})", x, y));

    assert!(block.is_fused(), "Block not fused: ({}, {})", x, y);
    assert!(block.borders().contains(sides), "Block at ({}, {}), has wrong borders: {:?}", x, y, block.borders());
}

pub fn assert_not_fused(board: &Board, x: i8, y: i8) {
    let block = board.grid().at(GridPosition::new(x, y))
        .expect(&format!("No block at ({}, {})", x, y));

    assert!(!block.is_fused(), "Block is fused: ({}, {})", x, y);
}

pub fn assert_block(board: &Board, x: i8, y: i8) {
    let block = board.grid().at(GridPosition::new(x, y));
    
    assert!(block.is_some(), "No block at ({}, {})", x, y);
}

pub fn assert_no_block(board: &Board, x: i8, y: i8) {
    let block = board.grid().at(GridPosition::new(x, y));
    
    assert!(block.is_none(), "Block at ({}, {})", x, y);
}

macro_rules! svec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }
    }
}

macro_rules! make_board {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            let mut board = make_board(temp_vec.len());
            board.add_blocks(temp_vec);
            board.fuse_blocks();
            board
        }
    };
}

mod test_drop;
mod test_fuse;
mod test_debug;
