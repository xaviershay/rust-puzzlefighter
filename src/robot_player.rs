use board::Board;
use piston_window::*;
use values::*;
use wrapper_types::*;

use std::collections::BTreeMap;
use std::collections::LinkedList;
use std::vec::Vec;

use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
enum Actions {
    Move(Direction),
    Rotate(Rotation),
}

pub struct Candidate {
    score: u32,
    actions: Vec<Actions>,
}

/*
impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(other.score)
    }
}
*/

pub struct RobotPlayer {
    current_piece: Option<Piece>,
}

impl RobotPlayer {
    pub fn new() -> Self {
        RobotPlayer {
            current_piece: None,
        }
    }

    fn apply(&self, board: &mut Board, actions: &Vec<Actions>) {
        for a in actions.iter() {
            match a {
                &Actions::Move(direction) => {
                    board.move_piece(|current| { current.offset(direction) });
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self, e: &GameWindow, board: &mut Board) {
        e.update(|args| {
            board.turbo(true);

            if board.current_piece().is_some() && board.current_piece() != self.current_piece {
                self.current_piece = board.current_piece();
                let piece = self.current_piece.unwrap();

                let mut candidates = BTreeMap::new();
                for x in 0..board.dimensions().w() as i8 {
                    let relative = x - piece.x();
                    let mut moves = Vec::new();
                    let direction = if relative < 0 {
                        Direction::Left
                    } else {
                        Direction::Right
                    };

                    for y in 0..(relative.abs()) {
                        moves.push(Actions::Move(direction))
                    }

                    let mut board = board.clone();
                    self.apply(&mut board, &moves);
                    board.place_current_piece();
                    // TODO: Resolve breaks + drops

                    let mut highest = 0;
                    for block in board.grid().blocks() {
                        use std::cmp::max;

                        highest = max(highest, block.y());
                    }

                    // TODO: Higher is bad, should give lower score.
                    // Reverse next with next_back below
                    candidates.insert(highest, moves);
                }

                let best = candidates.values().next().unwrap();

                self.apply(board, best);
            }
        });
    }
}
