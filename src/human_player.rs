use board::Board;
use piston_window::*;
use values::*;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InputAction {
    AntiClockwise,
    Clockwise,
    Left,
    Right,
    Turbo,
}

pub struct HumanPlayer {
    input_map: HashMap<Button, InputAction>,
}

impl HumanPlayer {
    pub fn new(left: bool) -> Self {
        let mut inputs = HashMap::new();

        if left {
            inputs.insert(Button::Keyboard(Key::W), InputAction::AntiClockwise);
            inputs.insert(Button::Keyboard(Key::S), InputAction::Clockwise);
            inputs.insert(Button::Keyboard(Key::A), InputAction::Left);
            inputs.insert(Button::Keyboard(Key::D), InputAction::Right);
            inputs.insert(Button::Keyboard(Key::C), InputAction::Turbo);
        } else {
            inputs.insert(Button::Keyboard(Key::Up), InputAction::AntiClockwise);
            inputs.insert(Button::Keyboard(Key::Down), InputAction::Clockwise);
            inputs.insert(Button::Keyboard(Key::Left), InputAction::Left);
            inputs.insert(Button::Keyboard(Key::Right), InputAction::Right);
            inputs.insert(Button::Keyboard(Key::Space), InputAction::Turbo);
        }

        HumanPlayer {
            input_map: inputs,
        }
    }

    pub fn update(&self, e: &PistonWindow, board: &mut Board) {
        if let Some(button) = e.release_args() {
            let action = self.input_map.get(&button);

            match action {
                Some(&InputAction::Turbo) => {
                    board.turbo(false)
                },
                _ => {},
            }
        }
        if let Some(button) = e.press_args() {
            let action = self.input_map.get(&button);

            // TODO: Handle key repeat on our own timer.
            match action {
                Some(&InputAction::AntiClockwise) => {
                    board.move_piece(|current| { current.anti_clockwise() });
                },
                Some(&InputAction::Clockwise) => {
                    board.move_piece(|current| { current.clockwise() });
                },
                Some(&InputAction::Left) => {
                    board.move_piece(|current| { current.offset(Direction::Left) });
                },
                Some(&InputAction::Right) => {
                    board.move_piece(|current| { current.offset(Direction::Right) });
                },
                Some(&InputAction::Turbo) => {
                    board.turbo(true);
                }
                None => {},
            }
        }
    }
}
