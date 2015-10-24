use board::Board;
use piston_window::*;
use values::*;
use wrapper_types::*;

use std::collections::BTreeMap;
use std::collections::LinkedList;
use std::vec::Vec;

use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Score(f64);

impl Eq for Score { }

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
      self.partial_cmp(other).unwrap()
    }
}

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
    weights: (f64, f64),
}

impl RobotPlayer {
    pub fn new(weights: (f64, f64)) -> Self {
        RobotPlayer {
            current_piece: None,
            weights: weights,
        }
    }

    fn apply(&self, board: &mut Board, actions: &Vec<Actions>) {
        for a in actions.iter() {
            match a {
                &Actions::Move(direction) => {
                    board.move_piece(|current| { current.offset(direction) });
                },
                &Actions::Rotate(r) => {
                    board.rotate(r);
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64, board: &mut Board) {
        if board.current_piece().is_none() {
            return;
        }

        board.turbo(true);

        if self.current_piece.is_none() || (
            board.current_piece().unwrap().id() != self.current_piece.unwrap().id()
            ) {
            self.current_piece = board.current_piece();
            let piece = self.current_piece.unwrap();
            let mut potential_moves = Vec::new();

            let mut candidates = BTreeMap::new();

            // This technically generates dups for horizontal positions.
            // Meh.
            for x in 0..board.dimensions().w() as i8 {
                for r in 0..3 {
                    let relative = x - piece.x();
                    let mut moves = Vec::new();
                    let direction = if relative < 0 {
                        Direction::Left
                    } else {
                        Direction::Right
                    };

                    for y in 0..r {
                        moves.push(Actions::Rotate(Rotation::Clockwise))
                    }
                    for y in 0..(relative.abs()) {
                        moves.push(Actions::Move(direction))
                    }

                    potential_moves.push(moves)
                }
            }

            for moves in potential_moves {
                let mut board = board.clone();
                self.apply(&mut board, &moves);
                board.place_current_piece();
                // TODO: Resolve breaks + drops

                let mut highest = 0;
                for block in board.grid().blocks() {
                    use std::cmp::max;

                    highest = max(highest, block.y());
                }

                let groups = board.grid().count_groups() as f64;
                let weights = self.weights;

                candidates.insert(Score(highest as f64 * weights.0 + groups * weights.1), moves);
            }

            let best = candidates.values().next_back().unwrap();

            self.apply(board, best);
        }
    }
}
