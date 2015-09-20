use board::Board;
use piston_window::*;
use values::*;

pub struct HumanPlayer {
    _blah: bool,
}

impl HumanPlayer {
    pub fn new() -> Self {
        HumanPlayer {
            _blah: false,
        }
    }

    pub fn update(&self, e: &PistonWindow, board: &mut Board) {
        if let Some(button) = e.release_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            match button {
                Keyboard(Key::Space) => {
                    board.turbo(false)
                },
                _ => {},
            }
        }
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            // TODO: Handle key repeat on our own timer.
            match button {
                Keyboard(Key::Up) => {
                    board.move_piece(|current| { current.anti_clockwise() });
                },
                Keyboard(Key::Down) => {
                    board.move_piece(|current| { current.clockwise() });
                },
                Keyboard(Key::Left) => {
                    board.move_piece(|current| { current.offset(Direction::Left) });
                },
                Keyboard(Key::Right) => {
                    board.move_piece(|current| { current.offset(Direction::Right) });
                },
                Keyboard(Key::Space) => {
                    board.turbo(true);
                }
                _ => {},
            }
        }
    }
}
