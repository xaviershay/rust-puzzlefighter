use board::Board;
use piston_window::*;
use values::*;
use wrapper_types::*;

use std::collections::{HashMap,HashSet,LinkedList};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InputAction {
    AntiClockwise,
    Clockwise,
    Left,
    Right,
    Turbo,
    DebugLoadBoard,
    DebugAttack,
    DebugBreaker(Color),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum JoystickDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InputButton {
    Piston(Button),
    Us(JoystickDirection),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InputEvent {
    Press(InputButton),
    Release(InputButton),
}

pub struct HumanPlayer {
    input_map: HashMap<InputButton, InputAction>,
    held: HashSet<JoystickDirection>,
    joystick: i32,
}

impl HumanPlayer {
    pub fn new(left: bool, joystick: i32) -> Self {
        let mut inputs = HashMap::new();

        if left {
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::W)), InputAction::AntiClockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::S)), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::A)), InputAction::Left);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D)), InputAction::Right);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::C)), InputAction::Turbo);
            inputs.insert(InputButton::Piston(Button::Joystick(0, 3)), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Joystick(0, 1)), InputAction::AntiClockwise);
            inputs.insert(InputButton::Us(JoystickDirection::Left), InputAction::Left);
            inputs.insert(InputButton::Us(JoystickDirection::Right), InputAction::Right);
            inputs.insert(InputButton::Us(JoystickDirection::Down), InputAction::Turbo);
        } else {
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Up)), InputAction::AntiClockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Down)), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Left)), InputAction::Left);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Right)), InputAction::Right);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Space)), InputAction::Turbo);
            inputs.insert(InputButton::Piston(Button::Joystick(1, 3)), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Joystick(1, 1)), InputAction::AntiClockwise);
            inputs.insert(InputButton::Us(JoystickDirection::Left), InputAction::Left);
            inputs.insert(InputButton::Us(JoystickDirection::Right), InputAction::Right);
            inputs.insert(InputButton::Us(JoystickDirection::Down), InputAction::Turbo);
        }

        if cfg!(debug_assertions) {
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Q)), InputAction::DebugLoadBoard);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D1)), InputAction::DebugBreaker(Color::Red));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D2)), InputAction::DebugBreaker(Color::Green));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D3)), InputAction::DebugBreaker(Color::Blue));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D4)), InputAction::DebugBreaker(Color::Yellow));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D5)), InputAction::DebugAttack);
        }

        HumanPlayer {
            input_map: inputs,
            held: HashSet::new(),
            joystick: joystick,
        }
    }

    pub fn update(&mut self, e: &GameWindow, board: &mut Board) {
        let mut events: LinkedList<InputEvent> = LinkedList::new();

        if let Some((which, axis, val)) = e.joystick_axis_args() {
            if which == self.joystick {
                let dead_zone = 0.8;
                if val > dead_zone || val < -dead_zone {
                    let direction = if val > dead_zone {
                        match axis {
                            0 => Some(JoystickDirection::Right),
                            1 => Some(JoystickDirection::Down),
                            _ => {
                                println!("Unknown joystick axis: {}", axis);
                                None
                            }
                        }
                    } else if val < -dead_zone {
                        match axis {
                            0 => Some(JoystickDirection::Left),
                            1 => Some(JoystickDirection::Up),
                            _ => {
                                println!("Unknown joystick axis: {}", axis);
                                None
                            }
                        }
                    } else {
                        unreachable!()
                    };

                    match direction {
                        Some(direction) => {
                            self.held.insert(direction);
                            events.push_back(InputEvent::Press(InputButton::Us(direction)));
                        },
                        None => {}
                    }
                } else {
                    // Generate up events for all held directions.
                    for direction in self.held.iter() {
                        events.push_back(InputEvent::Release(InputButton::Us(*direction)));
                    }
                    self.held = HashSet::new();
                }
            }
        }

        if let Some(button) = e.release_args() {
            events.push_back(InputEvent::Release(InputButton::Piston(button)));
        }
        if let Some(button) = e.press_args() {
            events.push_back(InputEvent::Press(InputButton::Piston(button)));
        }

        for event in events {
            match event {
                InputEvent::Press(button) => {
                    if let Some(action) = self.input_map.get(&button) {
                        // TODO: Handle key repeat on our own timer.
                        match action {
                            &InputAction::AntiClockwise => {
                                board.rotate(Rotation::AntiClockwise);
                            },
                            &InputAction::Clockwise => {
                                board.rotate(Rotation::Clockwise);
                            },
                            &InputAction::Left => {
                                board.move_piece(|current| { current.offset(Direction::Left) });
                            },
                            &InputAction::Right => {
                                board.move_piece(|current| { current.offset(Direction::Right) });
                            },
                            &InputAction::Turbo => {
                                board.turbo(true);
                            },
                            &InputAction::DebugBreaker(color) => {
                                if cfg!(debug_assertions) {
                                    board.set_next_piece(Piece::new(
                                        Block::new(color, true),
                                        Block::new(color, true),
                                    ))
                                }

                            },
                            &InputAction::DebugAttack => {
                                if cfg!(debug_assertions) {
                                    board.attack(6);
                                }
                            },
                            &InputAction::DebugLoadBoard => {
                                if cfg!(debug_assertions) {
                                    use std::io::{BufReader,BufRead};
                                    use std::fs::File;

                                    let f = File::open("board.txt");

                                    match f {
                                        Err(e) => {
                                            println!("Could not open file: {}", e);
                                        },
                                        Ok(f) => {
                                            let reader = BufReader::new(&f);
                                            let eol: &[_] = &['\n', '\r'];
                                            let lines: Vec<_> = reader.lines().map(|x| {
                                                x.ok().unwrap()
                                                    .trim_right_matches(eol)
                                                    .to_string()
                                            }).collect();
                                            board.add_blocks(lines);
                                            board.fuse_blocks();
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                InputEvent::Release(button) => {
                    if let Some(action) = self.input_map.get(&button) {
                        match action {
                            &InputAction::Turbo => {
                                board.turbo(false)
                            },
                            _ => {},
                        }
                    }
                },
            }
        }
    }
}
